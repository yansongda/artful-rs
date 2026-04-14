use artful::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
use artful::{Plugin, RocketConfig, Shortcut};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
struct TestShortcut;

impl Shortcut for TestShortcut {
    fn get_plugins(
        &self,
        _config: &RocketConfig,
        _payload: &HashMap<String, serde_json::Value>,
    ) -> Vec<Arc<dyn Plugin>> {
        vec![
            Arc::new(StartPlugin),
            Arc::new(AddPayloadBodyPlugin),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}

#[tokio::test]
async fn test_shortcut_basic() {
    let shortcut = TestShortcut::default();
    let config = RocketConfig::default();
    let plugins = shortcut.get_plugins(&config, &HashMap::new());

    assert_eq!(plugins.len(), 4);
}
