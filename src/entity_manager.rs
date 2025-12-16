pub struct EntityManager<E> {
    active: Vec<E>,
    free: Vec<E>,
    generator: Box<dyn FnMut() -> E>,
}

impl<E: Eq> EntityManager<E> {
    pub fn new(generator: Box<dyn FnMut() -> E>) -> Self {
        Self {
            active: vec![],
            free: vec![],
            generator,
        }
    }

    pub fn contains(&self, entity: &E) -> bool {
        self.active.contains(entity)
    }

    pub fn spawn(&mut self) -> &E {
        let e = self.free.pop().unwrap_or_else(|| self.generator.as_mut()());
        self.active.push(e);
        self.active.last().unwrap()
    }

    pub fn despawn(&mut self, entity: &E) -> bool {
        match self.active.iter().position(|e| e == entity) {
            Some(i) => {
                self.free.push(self.active.swap_remove(i));
                true
            }
            None => false,
        }
    }
}

impl Default for EntityManager<usize> {
    fn default() -> Self {
        let mut it = 0usize..;
        Self::new(Box::new(move || it.next().expect("Error: out of entities")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_despawn() {
        let mut manager = EntityManager::default();

        assert_eq!(manager.active.len(), 0);
        assert_eq!(manager.free.len(), 0);

        let e = *manager.spawn();

        assert_eq!(e, 0);
        assert_eq!(manager.active.len(), 1);
        assert_eq!(manager.free.len(), 0);

        manager.despawn(&e);

        assert_eq!(manager.active.len(), 0);
        assert_eq!(manager.free.len(), 1);
    }

    #[test]
    fn test_bad_despawn() {
        let mut manager = EntityManager::default();

        assert!(!manager.despawn(&3));
    }
}
