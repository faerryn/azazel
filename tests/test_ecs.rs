#[cfg(test)]
mod tests {
    use azazel::Engine;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Name(String);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Position(i32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Speed(i32);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Player;

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

    #[test]
    fn test_query3() {
        let mut engine = Engine::default();

        let player = engine.spawn();
        engine.insert_entity_component(&player, Position(5));
        engine.insert_entity_component(&player, Speed(-3));
        engine.insert_entity_component(&player, Player);

        let npc = engine.spawn();
        engine.insert_entity_component(&npc, Position(10));
        engine.insert_entity_component(&npc, Speed(10));

        engine.schedule(
            |position: &mut Position, speed: &mut Speed, _player: &mut Player| {
                position.0 += 2 * speed.0;
            },
        );

        engine.schedule(|position: &mut Position, speed: &mut Speed| {
            position.0 += speed.0;
        });

        // player should move 3 times as often

        engine.run_systems();
        assert_eq!(engine.get_entity_component(&player), Some(&Position(-4)));
        assert_eq!(engine.get_entity_component(&npc), Some(&Position(20)));

        engine.run_systems();
        assert_eq!(engine.get_entity_component(&player), Some(&Position(-13)));
        assert_eq!(engine.get_entity_component(&npc), Some(&Position(30)));
    }
}
