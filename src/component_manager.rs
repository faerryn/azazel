use std::{any::TypeId, collections::HashMap, hash::Hash};

use crate::component_storage::{AnyComponentStorage, ComponentStorage};

pub struct ComponentManager<I> {
    stores: HashMap<TypeId, Box<dyn AnyComponentStorage<I>>>,
}

impl<I: Copy + Eq + Hash + 'static> ComponentManager<I> {
    // Entity stuff
    fn c_get<C: 'static>(&self) -> Option<&ComponentStorage<I, C>> {
        self.stores.get(&TypeId::of::<C>()).map(|store| {
            store
                .as_any()
                .downcast_ref::<ComponentStorage<I, C>>()
                .expect("Error: type mismatch")
        })
    }

    // Components stuff
    pub(crate) fn c_get_mut<C: 'static>(&mut self) -> Option<&mut ComponentStorage<I, C>> {
        self.stores.get_mut(&TypeId::of::<C>()).map(|store| {
            store
                .as_any_mut()
                .downcast_mut::<ComponentStorage<I, C>>()
                .expect("Error: type mismatch")
        })
    }

    fn c_get_or_default<C: 'static>(&mut self) -> &mut ComponentStorage<I, C> {
        self.stores
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(ComponentStorage::<I, C>::default()))
            .as_any_mut()
            .downcast_mut()
            .expect("Error: type mismatch")
    }

    pub fn get<C: 'static>(&self, entity: &I) -> Option<&C> {
        self.c_get::<C>().and_then(|c| c.get(entity))
    }

    pub fn get_mut<C: 'static>(&mut self, entity: &I) -> Option<&mut C> {
        self.c_get_mut::<C>().and_then(|c| c.get_mut(entity))
    }

    pub fn insert<C: 'static>(&mut self, entity: I, component: C) -> Option<C> {
        self.c_get_or_default::<C>().insert(entity, component)
    }

    pub fn remove<C: 'static>(&mut self, entity: &I) -> Option<C> {
        self.c_get_mut::<C>().and_then(|c| c.remove(entity))
    }

    pub fn remove_entity(&mut self, entity: &I) {
        self.stores.retain(|_, store| {
            store.remove(entity);
            !store.is_empty()
        });
    }

    pub fn shrink_to_fit(&mut self) {
        for store in self.stores.values_mut() {
            store.shrink_to_fit();
        }
        self.stores.retain(|_, store| !store.is_empty());
    }
}

impl<I> Default for ComponentManager<I> {
    fn default() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct StringID(&'static str);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Position(i32, i32);

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Dialogue {
        greeting: String,
    }

    #[test]
    fn insert_get() {
        let mut manager = ComponentManager::default();
        let player = StringID("Player");
        let position = Position(3, -5);

        assert_eq!(manager.stores.len(), 0);

        manager.insert(player, position);

        assert_eq!(manager.get(&player), Some(&position));
        assert_eq!(manager.stores.len(), 1);
        assert_eq!(manager.c_get::<Position>().unwrap().data.len(), 1);
    }

    #[test]
    fn bad_get() {
        let mut manager = ComponentManager::default();

        let player = "Player";
        let position = Position(3, -5);

        let npc = "Gale";
        let dialogue = Dialogue {
            greeting: "The Orb.".to_string(),
        };

        manager.insert(player, position);
        manager.insert(npc, dialogue);

        assert_eq!(manager.get::<Position>(&npc), None);
        assert_eq!(manager.get::<Dialogue>(&player), None);
    }

    #[test]
    fn bad_get_empty() {
        let manager = ComponentManager::default();

        let player = "Player";
        let npc = "Gale";

        assert_eq!(manager.get::<Position>(&npc), None);
        assert_eq!(manager.get::<Dialogue>(&player), None);
    }

    #[test]
    fn get_mut() {
        let mut manager = ComponentManager::default();

        let player = "Player";
        let position = Position(3, -5);

        let npc = "Gale";
        let dialogue = Dialogue {
            greeting: "The Orb.".to_string(),
        };

        manager.insert(player, position);
        manager.insert(npc, dialogue);

        let new_dialogue = Dialogue {
            greeting: "The Karsite Weave.".to_string(),
        };

        {
            let dialogue_mut = manager.get_mut::<Dialogue>(&npc).unwrap();
            *dialogue_mut = new_dialogue.clone();
        }

        assert_eq!(manager.get::<Dialogue>(&npc), Some(&new_dialogue));
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
