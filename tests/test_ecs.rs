#[cfg(test)]
mod tests {
    use azazel::{ComponentManager, EntityManager};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Name(String);

    #[test]
    fn test_entity_component_manager() {
        let mut em = EntityManager::default();
        let mut cm = ComponentManager::default();

        let player = *em.spawn();
        let name = Name("Johnny Silverhand".to_string());

        cm.insert(player, name.clone());
        assert_eq!(cm.get(&player), Some(&name));
    }
}
