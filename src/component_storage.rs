use std::{any::Any, collections::HashMap, hash::Hash};

pub(crate) struct ComponentStorage<I, C> {
    id_to_index: HashMap<I, usize>,
    pub(crate) data: Vec<C>,
}

impl<I: Eq + Hash, C> ComponentStorage<I, C> {
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
        if i < self.data.len() {
            std::mem::swap(self.data.get_mut(i).unwrap(), &mut component);
            Some(component)
        } else {
            self.data.push(component);
            None
        }
    }

    // TODO: ideally have a bijective map here
    // pub(crate) fn remove(&mut self, id: &I) -> Option<C> {
    //     self.id_to_index.remove(id).map(|i| {
    //         self.data.swap_remove(i)
    //     })
    // }
}

impl<I, C> Default for ComponentStorage<I, C> {
    fn default() -> Self {
        Self {
            id_to_index: HashMap::new(),
            data: vec![],
        }
    }
}

pub(crate) trait OpaqueComponentStorage<I> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn remove(&mut self, id: &I);
    fn shrink_to_fit(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<I: Eq + Hash + 'static, C: 'static> OpaqueComponentStorage<I> for ComponentStorage<I, C> {
    fn is_empty(&self) -> bool {
        self.id_to_index.is_empty()
    }

    fn len(&self) -> usize {
        self.id_to_index.len()
    }

    fn remove(&mut self, id: &I) {
        self.id_to_index
            .remove(&id)
            .expect("Tried to remove non-existent component");
    }

    fn shrink_to_fit(&mut self) {
        self.id_to_index.shrink_to_fit();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
