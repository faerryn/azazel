#[cfg(test)]
mod tests {
    use azazel::Engine;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Name(String);

    #[test]
    fn test_ecs() {
        let mut engine = Engine::default();
        let player = engine.spawn(Name("Johnny Silverhand".to_string()));

        engine.schedule(|names: &mut [Name]| {
            for name in names {
                name.0.push_str(" the VI");
            }
        });

        engine.run_systems();
        assert_eq!(
            engine.entity_get_component(&player),
            Some(&Name("Johnny Silverhand the VI".to_string()))
        );

        engine.run_systems();
        assert_eq!(
            engine.entity_get_component(&player),
            Some(&Name("Johnny Silverhand the VI the VI".to_string()))
        );
    }
}
