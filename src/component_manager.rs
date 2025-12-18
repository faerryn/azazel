use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::component_storage::{AnyComponentStorage, ComponentStorage};

pub(crate) struct ComponentManager<Id> {
    stores: HashMap<TypeId, Box<dyn AnyComponentStorage<Id>>>,
}

impl<Id: Copy + Eq + Hash + 'static> ComponentManager<Id> {
    pub(crate) fn get_store<C: 'static>(&self) -> Option<&ComponentStorage<Id, C>> {
        self.stores.get(&TypeId::of::<C>()).map(|store| {
            store
                .as_any()
                .downcast_ref::<ComponentStorage<Id, C>>()
                .expect("Type mismatch")
        })
    }

    pub(crate) fn get_mut_store<C: 'static>(&mut self) -> Option<&mut ComponentStorage<Id, C>> {
        self.stores.get_mut(&TypeId::of::<C>()).map(|store| {
            store
                .as_any_mut()
                .downcast_mut::<ComponentStorage<Id, C>>()
                .expect("Type mismatch")
        })
    }

    pub(crate) fn get<C: 'static>(&self, entity: &Id) -> Option<&C> {
        self.get_store::<C>().and_then(|c| c.get(entity))
    }

    pub(crate) fn get_mut<C: 'static>(&mut self, entity: &Id) -> Option<&mut C> {
        self.get_mut_store::<C>().and_then(|c| c.get_mut(entity))
    }

    pub(crate) fn insert<C: 'static>(&mut self, entity: Id, component: C) -> Option<C> {
        self.stores
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(ComponentStorage::<Id, C>::default()))
            .as_any_mut()
            .downcast_mut::<ComponentStorage<Id, C>>()
            .expect("Type mismatch")
            .insert(entity, component)
    }

    pub(crate) fn remove<C: 'static>(&mut self, entity: &Id) -> Option<C> {
        self.get_mut_store::<C>().and_then(|c| c.remove(entity))
    }

    pub(crate) fn delete_entity_components(&mut self, entity: &Id) {
        self.stores.retain(|_, store| {
            store.delete(entity);
            !store.is_empty()
        });
    }

    pub(crate) fn remove_store<C: 'static>(&mut self) -> Option<ComponentStorage<Id, C>> {
        // NOTE: removing non-existent stores is allowed
        self.stores
            .remove(&TypeId::of::<C>())
            .map(|any| *(any.into_any().downcast().expect("Type mismatch")))
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        for store in self.stores.values_mut() {
            store.shrink_to_fit();
        }
        self.stores.retain(|_, store| !store.is_empty());
    }

    fn sample_mut_store<const N: usize>(
        &mut self,
        query: [TypeId; N],
    ) -> [Option<&mut Box<dyn AnyComponentStorage<Id>>>; N] {
        let mut unsorted: HashMap<_, _> = self.stores.iter_mut().collect();
        std::array::from_fn(|i| unsorted.remove(&query[i]))
    }

    pub(crate) fn query<const N: usize>(&mut self, query: [TypeId; N]) -> Vec<[&mut dyn Any; N]> {
        if N == 0 {
            return vec![];
        }

        let mut set: Option<HashSet<Id>> = None;
        for c in &query {
            if let Some(store) = self.stores.get(c) {
                let s2: HashSet<Id> = store.ids().iter().copied().collect();
                if let Some(s1) = set {
                    set = Some(s1.intersection(&s2).copied().collect());
                } else {
                    set = Some(s2);
                }
            } else {
                return vec![];
            }
        }

        let set = if let Some(set) = set {
            set
        } else {
            return vec![];
        };

        let ids: Vec<Id> = set.into_iter().collect();
        let mut all: Vec<Vec<_>> = vec![];
        for store in self
            .sample_mut_store(query)
            .into_iter()
            .map(|store| store.expect("Store missing"))
        {
            all.push(
                store
                    .sample_mut_any(&ids)
                    .into_iter()
                    .map(|c| c.expect("Broken store"))
                    .collect(),
            );
        }
        let mut transpose = vec![];
        for _ in 0..ids.len() {
            let per_entity: [&mut dyn Any; N] = std::array::from_fn(|j| all[j].pop().unwrap());
            transpose.push(per_entity);
        }

        transpose
    }
}

impl<Id: PartialOrd<Id> + Default> Default for ComponentManager<Id> {
    fn default() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Position(i32, i32);

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Dialogue {
        greeting: String,
    }

    #[test]
    fn insert_get() {
        let mut manager = ComponentManager::default();
        let player = "Player";
        let position = Position(3, -5);

        assert_eq!(manager.stores.len(), 0);

        manager.insert(player, position);

        assert_eq!(manager.get(&player), Some(&position));
        assert_eq!(manager.stores.len(), 1);
    }

    #[test]
    fn bad_get() {
        let mut manager = ComponentManager::default();

        let player = "Player";
        let position = Position(3, -5);

        let gale = "Gale";
        let dialogue = Dialogue {
            greeting: "The Orb.".to_string(),
        };

        manager.insert(player, position);
        manager.insert(gale, dialogue);

        assert_eq!(manager.get::<Position>(&gale), None);
        assert_eq!(manager.get::<Dialogue>(&player), None);
    }

    #[test]
    fn bad_get_empty() {
        let manager = ComponentManager::default();

        let player = "Player";
        let gale = "Gale";

        assert_eq!(manager.get::<Position>(&gale), None);
        assert_eq!(manager.get::<Dialogue>(&player), None);
    }

    #[test]
    fn get_mut() {
        let mut manager = ComponentManager::default();

        let player = "Player";
        let position = Position(3, -5);

        let gale = "Gale";
        let dialogue = Dialogue {
            greeting: "The Orb.".to_string(),
        };

        manager.insert(player, position);
        manager.insert(gale, dialogue);

        let new_dialogue = Dialogue {
            greeting: "The Karsite Weave.".to_string(),
        };

        {
            let dialogue_mut = manager.get_mut::<Dialogue>(&gale).unwrap();
            *dialogue_mut = new_dialogue.clone();
        }

        assert_eq!(manager.get::<Dialogue>(&gale), Some(&new_dialogue));
    }

    #[test]
    fn remove() {
        let mut manager = ComponentManager::default();

        let player = "Player";
        let position = Position(3, -5);
        let dialogue = Dialogue {
            greeting: "Heyah".to_string(),
        };

        manager.insert(player, position);
        manager.insert(player, dialogue.clone());

        assert_eq!(manager.get::<Position>(&player), Some(&position));
        assert_eq!(manager.get::<Dialogue>(&player), Some(&dialogue));

        manager.remove::<Position>(&player);

        assert_eq!(manager.get::<Position>(&player), None);
        assert_eq!(manager.get::<Dialogue>(&player), Some(&dialogue));
    }
}
