use std::ops::RangeFrom;

pub(crate) struct EntityManager<Id, G> {
    alive: Vec<Id>,
    dead: Vec<Id>,
    generator: G,
}

impl<Id: Copy + PartialEq, G: Iterator<Item = Id>> EntityManager<Id, G> {
    pub(crate) fn as_slice(&self) -> &[Id] {
        self.alive.as_slice()
    }

    pub(crate) fn contains(&self, id: Id) -> bool {
        self.alive.contains(&id)
    }

    pub(crate) fn spawn(&mut self) -> Id {
        let id = self
            .dead
            .pop()
            .unwrap_or_else(|| self.generator.next().expect("Out of entity IDs"));

        self.alive.push(id);

        id
    }

    pub(crate) fn despawn(&mut self, id: Id) {
        let i = self
            .alive
            .iter()
            .position(|other| *other == id)
            .expect("Invalid entity ID");
        self.alive.swap_remove(i);

        self.dead.push(id);
    }
}

impl<Id: PartialOrd<Id> + Default> Default for EntityManager<Id, RangeFrom<Id>> {
    fn default() -> Self {
        Self {
            alive: vec![],
            dead: vec![],
            generator: Id::default()..,
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

        let id: i32 = manager.spawn();
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
        let e: u8 = manager.spawn();
        manager.despawn(e);
        manager.despawn(e);
    }
}
