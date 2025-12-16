use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::ComponentManager;

#[derive(Default)]
pub struct SystemManager<E> {
    systems: Vec<Box<dyn OpaqueSystem<E>>>,
    pub cm: Arc<Mutex<ComponentManager<E>>>,
}

impl<E: Eq + Hash + 'static> SystemManager<E> {
    pub fn insert<C, S>(&mut self, system: S)
    where
        C: 'static,
        S: Fn(&mut [C]) + 'static,
    {
        self.systems.push(Box::new(System {
            cm: self.cm.clone(),
            system: Box::new(system),
        }))
    }

    pub fn run_systems(&self) {
        for system in &self.systems {
            system.run();
        }
    }
}

trait OpaqueSystem<E> {
    fn run(&self);
}

struct System<E, C> {
    cm: Arc<Mutex<ComponentManager<E>>>,
    system: Box<dyn Fn(&mut [C])>,
}

impl<E: Eq + Hash + 'static, C: 'static> OpaqueSystem<E> for System<E, C> {
    fn run(&self) {
        let mut cm = self.cm.lock().unwrap();
        if let Some(components) = cm.c_get_mut::<C>() {
            for c in components.values_mut() {
                self.system.as_ref()(std::slice::from_mut(c));
            }
        }
    }
}
