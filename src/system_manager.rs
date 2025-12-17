use std::{any::TypeId, hash::Hash, marker::PhantomData};

use crate::ComponentManager;

#[derive(Default)]
pub struct SystemManager<Id> {
    systems: Vec<Box<dyn OpaqueSystem<Id>>>,
}

impl<Id: Copy + Eq + Hash + 'static> SystemManager<Id> {
    pub fn schedule<F, Input>(&mut self, system: F)
    where
        F: IntoOpaqueSystem<Id, Input>,
        <F as IntoOpaqueSystem<Id, Input>>::System: 'static,
    {
        self.systems.push(Box::new(system.into_opaque_system()))
    }

    pub fn run_systems(&self, component_manager: &mut ComponentManager<Id>) {
        for system in &self.systems {
            system.run(component_manager);
        }
    }
}

pub(crate) struct System<Input, F> {
    call: F,
    _signature: PhantomData<Input>,
}

pub(crate) trait IntoOpaqueSystem<Id, Input> {
    type System: OpaqueSystem<Id>;

    fn into_opaque_system(self) -> Self::System;
}

trait OpaqueSystem<Id> {
    fn run(&self, component_manager: &mut ComponentManager<Id>);
}

impl<Id: Copy + Eq + Hash + 'static, C: 'static, F: Fn(&mut C) + 'static> IntoOpaqueSystem<Id, (C,)>
    for F
{
    type System = System<(C,), F>;

    fn into_opaque_system(self) -> Self::System {
        Self::System {
            call: self,
            _signature: PhantomData,
        }
    }
}

impl<Id: Copy + Eq + Hash + 'static, C: 'static, D: 'static, F: Fn(&mut C, &mut D) + 'static>
    IntoOpaqueSystem<Id, (C, D)> for F
{
    type System = System<(C, D), F>;

    fn into_opaque_system(self) -> Self::System {
        Self::System {
            call: self,
            _signature: PhantomData,
        }
    }
}

impl<Id: Copy + Eq + Hash + 'static, C: 'static, F: Fn(&mut C) + 'static> OpaqueSystem<Id>
    for System<(C,), F>
{
    fn run(&self, component_manager: &mut ComponentManager<Id>) {
        if let Some(components) = component_manager.get_mut_store::<C>() {
            for c in components.as_mut_slice() {
                (self.call)(c);
            }
        }
    }
}

impl<Id: Copy + Eq + Hash + 'static, C: 'static, D: 'static, F: Fn(&mut C, &mut D) + 'static>
    OpaqueSystem<Id> for System<(C, D), F>
{
    fn run(&self, component_manager: &mut ComponentManager<Id>) {
        let mut result = component_manager.query(&[TypeId::of::<C>(), TypeId::of::<D>()]);
        let ds = result.pop().unwrap();
        let cs = result.pop().unwrap();
        for (c, d) in cs.into_iter().zip(ds) {
            (self.call)(c.downcast_mut().unwrap(), d.downcast_mut().unwrap());
        }
    }
}
