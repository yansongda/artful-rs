use artisan_http::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
use artisan_http::{Plugin, Shortcut};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
struct TestShortcut;

impl Shortcut for TestShortcut {
    fn get_plugins(&self, _params: &HashMap<String, serde_json::Value>) -> Vec<Arc<dyn Plugin>> {
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
    let shortcut = TestShortcut;
    let plugins = shortcut.get_plugins(&HashMap::new());

    assert_eq!(plugins.len(), 4);
}
