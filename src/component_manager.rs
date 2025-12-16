use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
};

#[derive(Default)]
pub struct ComponentManager<E> {
    storage: HashMap<TypeId, Box<dyn OpaqueHashMap<E>>>,
}

trait OpaqueHashMap<E> {
    fn is_empty(&self) -> bool;
    fn remove(&mut self, k: &E);
    fn shrink_to_fit(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<K: Eq + Hash + 'static, V: 'static> OpaqueHashMap<K> for HashMap<K, V> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn remove(&mut self, k: &K) {
        HashMap::remove(self, k);
    }

    fn shrink_to_fit(&mut self) {
        HashMap::shrink_to_fit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
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

    #[test]
    fn insert_one() {
        let mut manager = ComponentManager::default();
        let player = "Player";
        let position = Position(3, -5);
        manager.insert(player, position);
        assert_eq!(manager.get(&player), Some(&position));
    }
}
