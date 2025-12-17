use std::{hash::Hash, ops::RangeFrom};

use crate::{ComponentManager, EntityManager, SystemManager, system_manager::IntoOpaqueSystem};

pub struct Entity<Id> {
    id: Id,
}

pub struct Engine<Id, G> {
    entity_manager: EntityManager<Id, G>,
    component_manager: ComponentManager<Id>,
    system_manager: SystemManager<Id>,
}

impl<Id: Copy + Eq + Hash + 'static, G: Iterator<Item = Id>> Engine<Id, G> {
    pub fn spawn(&mut self) -> Entity<Id> {
        let id = self.entity_manager.spawn();
        Entity { id }
    }

    pub fn despawn(&mut self, entity: Entity<Id>) {
        self.component_manager.delete_entity_components(&entity.id);
        self.entity_manager.despawn(entity.id)
    }

    pub fn insert_entity_component<C: 'static>(
        &mut self,
        entity: &Entity<Id>,
        component: C,
    ) -> Option<C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.insert(entity.id, component)
    }

    pub fn remove_entity_component<C: 'static>(&mut self, entity: &Entity<Id>) -> Option<C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.remove(&entity.id)
    }

    pub fn get_entity_component<C: 'static>(&self, entity: &Entity<Id>) -> Option<&C> {
        if !self.entity_manager.contains(entity.id) {
            panic!("Invalid entity");
        }
        self.component_manager.get(&entity.id)
    }

    pub fn delete_component_for_all_entities<C: 'static>(&mut self) {
        self.component_manager.remove_store::<C>();
    }

    pub fn schedule<Input, S>(&mut self, system: S)
    where
        S: IntoOpaqueSystem<Id, Input>,
        <S as IntoOpaqueSystem<Id, Input>>::System: 'static,
    {
        self.system_manager.schedule(system);
    }

    pub fn run_systems(&mut self) {
        self.system_manager.run_systems(&mut self.component_manager);
    }

    pub fn shrink_to_fit(&mut self) {
        self.component_manager.shrink_to_fit();
    }
}

impl Default for Engine<u32, RangeFrom<u32>> {
    fn default() -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_manager: ComponentManager::default(),
            system_manager: SystemManager::default(),
        }
    }
}
