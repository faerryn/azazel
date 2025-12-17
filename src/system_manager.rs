use std::{hash::Hash, marker::PhantomData};

use crate::ComponentManager;

#[derive(Default)]
pub struct SystemManager<I> {
    systems: Vec<Box<dyn OpaqueSystem<I>>>,
}

impl<I: Eq + Hash + 'static> SystemManager<I> {
    pub fn insert<C, S>(&mut self, system: S)
    where
        C: 'static,
        S: Fn(&mut [C]) + 'static,
    {
        self.systems.push(Box::new(System::new(system)))
    }

    pub fn run_systems(&self, component_manager: &mut ComponentManager<I>) {
        for system in &self.systems {
            system.run(component_manager);
        }
    }
}

trait OpaqueSystem<I> {
    fn run(&self, component_manager: &mut ComponentManager<I>);
}

struct System<C, S> {
    call: S,
    _signature: PhantomData<C>,
}

impl<C, S> System<C, S> {
    fn new(call: S) -> Self {
        Self {
            call,
            _signature: PhantomData,
        }
    }
}

impl<I: Eq + Hash + 'static, C: 'static, S: Fn(&mut [C]) + 'static> OpaqueSystem<I>
    for System<C, S>
{
    fn run(&self, component_manager: &mut ComponentManager<I>) {
        if let Some(components) = component_manager.c_get_mut::<C>() {
            for c in &mut components.data {
                (self.call)(std::slice::from_mut(c));
            }
        }
    }
}
