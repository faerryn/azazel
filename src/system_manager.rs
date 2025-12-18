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

    pub fn run_systems(&mut self, component_manager: &mut ComponentManager<Id>) {
        for system in &mut self.systems {
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
    fn run(&mut self, component_manager: &mut ComponentManager<Id>);
}

macro_rules! impl_into_opaque_system {
    ($($($params:ident),+)?) => {
        impl<
            Id: Copy + Eq + Hash + 'static,
            F: FnMut($($(&mut $params),+)?) + 'static,
            $($($params: 'static),+)?
        > IntoOpaqueSystem<Id, ($($($params),+,)?)> for F
        {
            type System = System<($($($params),+,)?), F>;
            fn into_opaque_system(self) -> Self::System {
                Self::System {
                    call: self,
                    _signature: PhantomData,
                }
            }
        }
    impl<
        Id: Copy + Eq + Hash + 'static,
        F: FnMut($($(&mut $params),+)?) + 'static,
        $($($params: 'static),+)?
        >
        OpaqueSystem<Id> for System<($($($params),+,)?), F>
    {
        fn run(&mut self, component_manager: &mut ComponentManager<Id>) {
            let result = component_manager.query([
                $($(TypeId::of::<$params>()),+)?
            ]);
            for [$($($params),+)?] in result {
                (self.call)($($($params.downcast_mut().unwrap()),+)?);
            }
        }
    }

    };
}

impl_into_opaque_system!();
impl_into_opaque_system!(T1);
impl_into_opaque_system!(T1, T2);
impl_into_opaque_system!(T1, T2, T3);
impl_into_opaque_system!(T1, T2, T3, T4);
