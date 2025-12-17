use std::{hash::Hash, ops::RangeFrom};

use crate::{ComponentManager, EntityManager, SystemManager};

pub struct Entity<I> {
    id: I,
}

pub struct Engine<I, G> {
    entity_manager: EntityManager<I, G>,
    component_manager: ComponentManager<I>,
    system_manager: SystemManager<I>,
}

impl<I: Copy + Eq + Hash + 'static, G: Iterator<Item = I>> Engine<I, G> {
    pub fn spawn<C: 'static>(&mut self) -> Entity<I> {
        let id = self.entity_manager.spawn();
        Entity { id }
    }

    pub fn despawn(&mut self, entity: Entity<I>) {
        self.component_manager.delete_entity_components(&entity.id);
        self.entity_manager.despawn(entity.id)
    }

    pub fn insert_entity_component<C: 'static>(
        &mut self,
        entity: &Entity<I>,
        component: C,
    ) -> Option<C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.insert(entity.id, component)
    }

    pub fn remove_entity_component<C: 'static>(&mut self, entity: &Entity<I>) -> Option<C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.remove(&entity.id)
    }

    pub fn get_entity_component<C: 'static>(&self, entity: &Entity<I>) -> Option<&C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.get(&entity.id)
    }

    pub fn delete_component_for_all_entities<C: 'static>(&mut self) {
        self.component_manager.remove_store::<C>();
    }

    pub fn schedule<C: 'static, S: Fn(&mut [C]) + 'static>(&mut self, system: S) {
        self.system_manager.insert(system);
    }

    pub fn run_systems(&mut self) {
        self.system_manager.run_systems(&mut self.component_manager);
    }
}

impl Default for Engine<usize, RangeFrom<usize>> {
    fn default() -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_manager: ComponentManager::default(),
            system_manager: SystemManager::default(),
        }
    }
}
