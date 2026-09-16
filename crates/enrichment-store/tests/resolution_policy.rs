use enrichment_core::{
    identity::{Ecosystem, ResearchMode},
    request::FreshnessMode,
};
use enrichment_store::{
    control::ControlStore,
    resolution_policy::{ResolutionPolicy, Route, Scope},
    runtime::{QueryLimits, QueryRuntime},
};

#[tokio::test]
async fn acquisition_and_offline_routing_preserve_registry_names() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&root.path().join("spill"), QueryLimits::default()).unwrap();
    let store = ControlStore::open(root.path(), runtime.clone()).unwrap();
    let pin = store.pin().await.unwrap();
    for (ecosystem, name, expected) in [
        (Ecosystem::Rust, "Serde_JSON", "serde_json"),
        (Ecosystem::Python, "My._Package", "my-package"),
    ] {
        for (freshness, profiles) in [
            (FreshnessMode::CacheOk, vec!["static".to_owned()]),
            (FreshnessMode::Revalidate, vec!["static".to_owned()]),
            (FreshnessMode::Offline, vec!["static".to_owned()]),
            (FreshnessMode::CacheOk, vec![]),
        ] {
            let policy = ResolutionPolicy::new(
                &runtime,
                pin.clone(),
                Scope {
                    ecosystem,
                    name,
                    registry: "registry",
                    version: Some("1.0.0"),
                    environment_id: "environment",
                    mode: ResearchMode::Project,
                    allow_local_build: false,
                    freshness,
                    profiles: &profiles,
                },
            )
            .await
            .unwrap();
            match policy.route().await.unwrap() {
                Route::Acquire { normalized_name } => {
                    assert_ne!(freshness, FreshnessMode::Offline);
                    assert_eq!(profiles, ["static"]);
                    assert_eq!(normalized_name, expected);
                }
                Route::Offline => assert_eq!(freshness, FreshnessMode::Offline),
                Route::Disabled => assert!(profiles.is_empty()),
                route => panic!("empty catalog returned {route:?}"),
            }
            assert!(policy.retained().await.unwrap().is_none());
        }
    }
}
