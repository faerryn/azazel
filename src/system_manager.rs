use std::{any::TypeId, hash::Hash, marker::PhantomData};

use crate::ComponentManager;

#[derive(Default)]
pub struct SystemManager<I> {
    systems: Vec<Box<dyn OpaqueSystem<I>>>,
}

impl<I: Copy + Eq + Hash + 'static> SystemManager<I> {
    pub fn insert<S, Input>(&mut self, system: S)
    where
        S: IntoOpaqueSystem<I, Input>,
        <S as IntoOpaqueSystem<I, Input>>::System: 'static,
    {
        self.systems.push(Box::new(system.into_opaque_system()))
    }

    pub fn run_systems(&self, component_manager: &mut ComponentManager<I>) {
        for system in &self.systems {
            system.run(component_manager);
        }
    }
}

pub(crate) struct System<Input, S> {
    call: S,
    _signature: PhantomData<Input>,
}

pub(crate) trait IntoOpaqueSystem<I, Input> {
    type System: OpaqueSystem<I>;

    fn into_opaque_system(self) -> Self::System;
}

trait OpaqueSystem<I> {
    fn run(&self, component_manager: &mut ComponentManager<I>);
}

impl<I: Copy + Eq + Hash + 'static, C: 'static, S: Fn(&mut C) + 'static> IntoOpaqueSystem<I, (C,)>
    for S
{
    type System = System<(C,), S>;

    fn into_opaque_system(self) -> Self::System {
        Self::System {
            call: self,
            _signature: PhantomData,
        }
    }
}

impl<I: Copy + Eq + Hash + 'static, C: 'static, D: 'static, S: Fn(&mut C, &mut D) + 'static>
    IntoOpaqueSystem<I, (C, D)> for S
{
    type System = System<(C, D), S>;

    fn into_opaque_system(self) -> Self::System {
        Self::System {
            call: self,
            _signature: PhantomData,
        }
    }
}

impl<I: Copy + Eq + Hash + 'static, C: 'static, S: Fn(&mut C) + 'static> OpaqueSystem<I>
    for System<(C,), S>
{
    fn run(&self, component_manager: &mut ComponentManager<I>) {
        if let Some(components) = component_manager.get_mut_store::<C>() {
            for c in components.as_mut_slice() {
                (self.call)(c);
            }
        }
    }
}

impl<I: Copy + Eq + Hash + 'static, C: 'static, D: 'static, S: Fn(&mut C, &mut D) + 'static>
    OpaqueSystem<I> for System<(C, D), S>
{
    fn run(&self, component_manager: &mut ComponentManager<I>) {
        let mut result = component_manager.query(&[TypeId::of::<C>(), TypeId::of::<D>()]);
        let ds = result.pop().unwrap();
        let cs = result.pop().unwrap();
        for (c, d) in cs.into_iter().zip(ds) {
            (self.call)(c.downcast_mut().unwrap(), d.downcast_mut().unwrap());
        }
    }
}
