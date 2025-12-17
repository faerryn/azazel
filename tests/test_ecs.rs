#[cfg(test)]
mod tests {
    use azazel::Engine;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Name(String);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Position(i32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Speed(i32);

    #[test]
    fn test_ecs() {
        let mut engine = Engine::default();
        let player = engine.spawn();
        engine.insert_entity_component(&player, Name("Johnny Silverhand".to_string()));

        engine.schedule(|name: &mut Name| {
            name.0.push_str(" the VI");
        });

        engine.run_systems();
        assert_eq!(
            engine.get_entity_component(&player),
            Some(&Name("Johnny Silverhand the VI".to_string()))
        );

        engine.run_systems();
        assert_eq!(
            engine.get_entity_component(&player),
            Some(&Name("Johnny Silverhand the VI the VI".to_string()))
        );
    }

    #[test]
    fn test_query2() {
        let mut engine = Engine::default();
        let player = engine.spawn();
        engine.insert_entity_component(&player, Position(5));
        engine.insert_entity_component(&player, Speed(-3));

        engine.schedule(|position: &mut Position, speed: &mut Speed| {
            position.0 += speed.0;
        });

        engine.run_systems();
        assert_eq!(engine.get_entity_component(&player), Some(&Position(2)));

        engine.run_systems();
        assert_eq!(engine.get_entity_component(&player), Some(&Position(-1)));
    }
}
