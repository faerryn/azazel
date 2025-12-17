use std::{any::Any, collections::HashMap, hash::Hash};

pub(crate) struct ComponentStorage<Id, C> {
    id_to_index: HashMap<Id, usize>,
    index_to_id: Vec<Id>,
    components: Vec<C>,
}

impl<Id: Copy + Eq + Hash, C> ComponentStorage<Id, C> {
    pub(crate) fn get(&self, id: &Id) -> Option<&C> {
        self.id_to_index
            .get(id)
            .map(|i| self.components.get(*i).expect("ID maps to invalid index"))
    }

    pub(crate) fn as_slice(&self) -> &[C] {
        self.components.as_slice()
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [C] {
        self.components.as_mut_slice()
    }

    pub(crate) fn get_mut(&mut self, id: &Id) -> Option<&mut C> {
        self.id_to_index.get(id).map(|i| {
            self.components
                .get_mut(*i)
                .expect("ID maps to invalid index")
        })
    }

    pub(crate) fn insert(&mut self, id: Id, mut component: C) -> Option<C> {
        let i = self.components.len();
        self.id_to_index.insert(id, i);
        self.index_to_id.push(id);
        if i < self.components.len() {
            std::mem::swap(self.components.get_mut(i).unwrap(), &mut component);
            Some(component)
        } else {
            self.components.push(component);
            None
        }
    }

    pub(crate) fn remove(&mut self, id: &Id) -> Option<C> {
        let last_index = self.components.len() - 1;
        if let Some(i) = self.id_to_index.remove(id) {
            if i < last_index {
                let last_id = self.index_to_id.pop().expect("Empty index_to_id");
                let previous_index = self.id_to_index.insert(last_id, i);
                assert_eq!(previous_index, Some(last_index), "Bijection broken");
                Some(self.components.swap_remove(i))
            } else {
                self.index_to_id.pop().expect("Empty index_to_id");
                Some(self.components.pop().expect("Empty components"))
            }
        } else {
            None
        }
    }
}

impl<Id, C> Default for ComponentStorage<Id, C> {
    fn default() -> Self {
        Self {
            id_to_index: HashMap::new(),
            index_to_id: vec![],
            components: vec![],
        }
    }
}

pub(crate) trait OpaqueComponentStorage<Id> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn delete(&mut self, id: &Id);
    fn shrink_to_fit(&mut self);
    fn ids(&self) -> &[Id];
    fn get_any(&self, id: &Id) -> Option<&dyn Any>;
    fn get_mut_any(&mut self, id: &Id) -> Option<&mut dyn Any>;
    fn sample_mut_any(&mut self, ids: &[Id]) -> Vec<Option<&mut dyn Any>>;
}

impl<Id: Copy + Eq + Hash, C: 'static> OpaqueComponentStorage<Id> for ComponentStorage<Id, C> {
    fn is_empty(&self) -> bool {
        self.id_to_index.is_empty()
    }

    fn len(&self) -> usize {
        self.id_to_index.len()
    }

    fn delete(&mut self, id: &Id) {
        self.remove(id);
    }

    fn shrink_to_fit(&mut self) {
        self.id_to_index.shrink_to_fit();
        self.index_to_id.shrink_to_fit();
        self.components.shrink_to_fit();
    }

    fn ids(&self) -> &[Id] {
        &self.index_to_id
    }

    fn get_any(&self, id: &Id) -> Option<&dyn Any> {
        self.get(id).map(|c| c as &dyn Any)
    }

    fn get_mut_any(&mut self, id: &Id) -> Option<&mut dyn Any> {
        self.get_mut(id).map(|c| c as &mut dyn Any)
    }

    fn sample_mut_any(&mut self, ids: &[Id]) -> Vec<Option<&mut dyn Any>> {
        let mut unsorted: HashMap<_, _> = self.components.iter_mut().enumerate().collect();
        let mut result = vec![];
        for id in ids {
            result.push(
                self.id_to_index
                    .get(id)
                    .and_then(|i| unsorted.remove(i))
                    .map(|c| c as _),
            );
        }
        result
    }
}

pub(crate) trait AnyComponentStorage<Id>: OpaqueComponentStorage<Id> {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<Id: Copy + Eq + Hash + 'static, C: 'static> AnyComponentStorage<Id>
    for ComponentStorage<Id, C>
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
