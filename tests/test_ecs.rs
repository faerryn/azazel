#[cfg(test)]
mod tests {
    use azazel::{EntityManager, SystemManager};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Name(String);

    #[test]
    fn test_ecs() {
        let mut em = EntityManager::default();
        let mut sm = SystemManager::<usize>::default();

        let player = *em.spawn();
        let player2 = *em.spawn();
        sm.cm
            .lock()
            .unwrap()
            .insert::<Name>(player, Name("Johnny Silverhand".to_string()));
        sm.cm
            .lock()
            .unwrap()
            .insert::<Name>(player2, Name("Steve".to_string()));

        sm.insert::<Name, _>(|names: &mut [Name]| {
            for name in names {
                name.0.push_str(" the VI");
            }
        });
        sm.run_systems();

        assert_eq!(
            sm.cm.lock().unwrap().get(&player),
            Some(&Name("Johnny Silverhand the VI".to_string()))
        );
        assert_eq!(
            sm.cm.lock().unwrap().get(&player2),
            Some(&Name("Steve the VI".to_string()))
        );

        sm.run_systems();
        assert_eq!(
            sm.cm.lock().unwrap().get(&player2),
            Some(&Name("Steve the VI the VI".to_string()))
        );
    }
}
