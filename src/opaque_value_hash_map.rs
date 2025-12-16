use std::{any::Any, collections::HashMap, hash::Hash};

pub(crate) trait OpaqueValueHashMap<K> {
    fn is_empty(&self) -> bool;
    fn remove(&mut self, k: &K);
    fn shrink_to_fit(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<K: Eq + Hash + 'static, V: 'static> OpaqueValueHashMap<K> for HashMap<K, V> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn remove(&mut self, k: &K) {
        HashMap::remove(self, k);
    }

    fn shrink_to_fit(&mut self) {
        HashMap::shrink_to_fit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
