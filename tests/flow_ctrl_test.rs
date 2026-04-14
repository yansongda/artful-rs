use artisan::Rocket;
use artisan::flow_ctrl::FlowCtrl;
use artisan::plugin::Plugin;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

struct TestPlugin {
    name: String,
}

#[async_trait]
impl Plugin for TestPlugin {
    async fn assembly(
        &self,
        rocket: &mut Rocket,
        next: artisan::flow_ctrl::Next<'_>,
    ) -> artisan::Result<()> {
        rocket
            .payload
            .insert("visited".to_string(), serde_json::json!(self.name.clone()));
        next.call(rocket).await
    }
}

#[tokio::test]
async fn test_flow_ctrl_basic() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(TestPlugin {
            name: "plugin1".to_string(),
        }),
        Arc::new(TestPlugin {
            name: "plugin2".to_string(),
        }),
    ];

    let mut ctrl = FlowCtrl::new(plugins);
    let mut rocket = Rocket::new(HashMap::new());

    ctrl.call_next(&mut rocket).await.unwrap();

    assert!(rocket.payload.contains_key("visited"));
}

#[tokio::test]
async fn test_flow_ctrl_cease() {
    struct CeasePlugin;

    #[async_trait]
    impl Plugin for CeasePlugin {
        async fn assembly(
            &self,
            rocket: &mut Rocket,
            _next: artisan::flow_ctrl::Next<'_>,
        ) -> artisan::Result<()> {
            rocket
                .payload
                .insert("ceased".to_string(), serde_json::json!(true));
            Ok(())
        }
    }

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(CeasePlugin),
        Arc::new(TestPlugin {
            name: "should_not_run".to_string(),
        }),
    ];

    let mut ctrl = FlowCtrl::new(plugins);
    let mut rocket = Rocket::new(HashMap::new());

    ctrl.call_next(&mut rocket).await.unwrap();

    assert!(rocket.payload.contains_key("ceased"));
    assert!(!rocket.payload.contains_key("visited"));
}

#[test]
fn test_flow_ctrl_has_next() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![Arc::new(TestPlugin {
        name: "p1".to_string(),
    })];

    let ctrl = FlowCtrl::new(plugins);
    assert!(ctrl.has_next());
}
