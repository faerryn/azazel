use crate::opaque_value_hash_map::OpaqueValueHashMap;
use std::{any::TypeId, collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct ComponentManager<E> {
    storage: HashMap<TypeId, Box<dyn OpaqueValueHashMap<E>>>,
}

impl<E: Eq + Hash + 'static> ComponentManager<E> {
    // Entity stuff
    fn c_get<C: 'static>(&self) -> Option<&HashMap<E, C>> {
        self.storage.get(&TypeId::of::<C>()).map(|store| {
            store
                .as_any()
                .downcast_ref::<HashMap<E, C>>()
                .expect("Error: type mismatch")
        })
    }

    // Components stuff
    fn c_get_mut<C: 'static>(&mut self) -> Option<&mut HashMap<E, C>> {
        self.storage.get_mut(&TypeId::of::<C>()).map(|store| {
            store
                .as_any_mut()
                .downcast_mut::<HashMap<E, C>>()
                .expect("Error: type mismatch")
        })
    }

    fn c_get_or_default<C: 'static>(&mut self) -> &mut HashMap<E, C> {
        self.storage
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(HashMap::<E, C>::new()))
            .as_any_mut()
            .downcast_mut()
            .expect("Error: type mismatch")
    }

    pub fn get<C: 'static>(&self, entity: &E) -> Option<&C> {
        self.c_get::<C>().and_then(|c| c.get(entity))
    }

    pub fn get_mut<C: 'static>(&mut self, entity: &E) -> Option<&mut C> {
        self.c_get_mut::<C>().and_then(|c| c.get_mut(entity))
    }

    pub fn insert<C: 'static>(&mut self, entity: E, component: C) -> Option<C> {
        self.c_get_or_default::<C>().insert(entity, component)
    }

    pub fn remove<C: 'static>(&mut self, entity: &E) -> Option<C> {
        self.c_get_mut::<C>().and_then(|c| c.remove(entity))
    }

    pub fn remove_entity(&mut self, entity: &E) {
        self.storage.retain(|_, store| {
            store.remove(entity);
            !store.is_empty()
        });
    }

    pub fn shrink_to_fit(&mut self) {
        for store in self.storage.values_mut() {
            store.shrink_to_fit();
        }
        self.storage.retain(|_, store| !store.is_empty());
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

        assert_eq!(manager.storage.len(), 0);

        manager.insert(player, position);

        assert_eq!(manager.get(&player), Some(&position));
        assert_eq!(manager.storage.len(), 1);
        assert_eq!(manager.c_get::<Position>().unwrap().len(), 1);
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
}
