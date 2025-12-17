use std::{any::Any, collections::HashMap, hash::Hash};

pub(crate) struct ComponentStorage<I, C> {
    id_to_index: HashMap<I, usize>,
    index_to_id: Vec<I>,
    pub(crate) data: Vec<C>,
}

impl<I: Copy + Eq + Hash, C> ComponentStorage<I, C> {
    pub(crate) fn get(&self, id: &I) -> Option<&C> {
        self.id_to_index
            .get(id)
            .map(|i| self.data.get(*i).expect("Id maps to invalid index"))
    }

    pub(crate) fn get_mut(&mut self, id: &I) -> Option<&mut C> {
        self.id_to_index
            .get(id)
            .map(|i| self.data.get_mut(*i).expect("Id maps to invalid index"))
    }

    pub(crate) fn insert(&mut self, id: I, mut component: C) -> Option<C> {
        let i = self.data.len();
        self.id_to_index.insert(id, i);
        self.index_to_id.push(id);
        if i < self.data.len() {
            std::mem::swap(self.data.get_mut(i).unwrap(), &mut component);
            Some(component)
        } else {
            self.data.push(component);
            None
        }
    }

    pub(crate) fn remove(&mut self, id: &I) -> Option<C> {
        let last_index = self.data.len() - 1;
        if let Some(i) = self.id_to_index.remove(id) {
            if i < last_index {
                let last_id = self
                    .index_to_id
                    .pop()
                    .expect("Non-empty storage has empty index_to_id");
                let previous_index = self.id_to_index.insert(last_id, i);
                assert_eq!(
                    previous_index,
                    Some(last_index),
                    "last_id should mapped to last_index before"
                );
                Some(self.data.swap_remove(i))
            } else {
                self.index_to_id
                    .pop()
                    .expect("Non-empty storage has empty index_to_id");
                Some(
                    self.data
                        .pop()
                        .expect("Empty storage has non-empty id_to_index"),
                )
            }
        } else {
            None
        }
    }
}

impl<I, C> Default for ComponentStorage<I, C> {
    fn default() -> Self {
        Self {
            id_to_index: HashMap::new(),
            index_to_id: vec![],
            data: vec![],
        }
    }
}

pub(crate) trait OpaqueComponentStorage<I> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn remove(&mut self, id: &I);
    fn shrink_to_fit(&mut self);
}

impl<I: Eq + Hash, C> OpaqueComponentStorage<I> for ComponentStorage<I, C> {
    fn is_empty(&self) -> bool {
        self.id_to_index.is_empty()
    }

    fn len(&self) -> usize {
        self.id_to_index.len()
    }

    fn remove(&mut self, id: &I) {
        self.id_to_index
            .remove(id)
            .expect("Tried to remove non-existent component");
    }

    fn shrink_to_fit(&mut self) {
        self.id_to_index.shrink_to_fit();
    }
}

pub(crate) trait AnyComponentStorage<I>: OpaqueComponentStorage<I> {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<I: Eq + Hash + 'static, C: 'static> AnyComponentStorage<I> for ComponentStorage<I, C> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
