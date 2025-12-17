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
    pub fn spawn<C: 'static>(&mut self, component: C) -> Entity<I> {
        let id = self.entity_manager.spawn();
        self.component_manager.insert(id, component);
        Entity { id }
    }

    pub fn despawn(&mut self, entity: Entity<I>) {
        self.component_manager.remove_entity(&entity.id);
        self.entity_manager.despawn(entity.id)
    }

    pub fn schedule<C: 'static, S: Fn(&mut [C]) + 'static>(&mut self, system: S) {
        self.system_manager.insert(system);
    }

    pub fn run_systems(&mut self) {
        self.system_manager.run_systems(&mut self.component_manager);
    }

    pub fn entity_get_component<C: 'static>(&self, entity: &Entity<I>) -> Option<&C> {
        self.component_manager.get(&entity.id)
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
