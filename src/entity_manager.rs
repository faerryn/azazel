use std::ops::RangeFrom;

pub(crate) struct EntityManager<I, G> {
    alive: Vec<I>,
    dead: Vec<I>,
    generator: G,
}

impl<I: Copy + PartialEq, G: Iterator<Item = I>> EntityManager<I, G> {
    pub fn contains(&self, id: I) -> bool {
        self.alive.contains(&id)
    }

    pub fn spawn(&mut self) -> I {
        let id = self
            .dead
            .pop()
            .unwrap_or_else(|| self.generator.next().expect("Out of entity IDs"));

        self.alive.push(id);

        id
    }

    pub fn despawn(&mut self, id: I) {
        let i = self
            .alive
            .iter()
            .position(|other| *other == id)
            .expect("Invalid entity ID");
        self.alive.swap_remove(i);

        self.dead.push(id);
    }
}

impl Default for EntityManager<usize, RangeFrom<usize>> {
    fn default() -> Self {
        Self {
            alive: vec![],
            dead: vec![],
            generator: 0..,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_despawn() {
        let mut manager = EntityManager::default();
        assert_eq!(manager.alive.len(), 0);
        assert_eq!(manager.dead.len(), 0);

        let id = manager.spawn();
        assert_eq!(id, 0);
        assert_eq!(manager.contains(id), true);

        assert_eq!(manager.alive.len(), 1);
        assert_eq!(manager.dead.len(), 0);

        manager.despawn(id);
        assert_eq!(manager.contains(id), false);

        assert_eq!(manager.alive.len(), 0);
        assert_eq!(manager.dead.len(), 1);
    }

    #[test]
    #[should_panic = "Invalid entity ID"]
    fn test_spawn_double_free() {
        let mut manager = EntityManager::default();
        let e = manager.spawn();
        manager.despawn(e);
        manager.despawn(e);
    }
}
