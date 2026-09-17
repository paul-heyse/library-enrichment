use enrichment_core::{
    config::Config,
    http::Fetched,
    policy::{FetchPolicy, PolicyViolation},
};
use enrichment_store::{
    http_cache::HttpCache,
    network_policy::NetworkPolicy,
    runtime::{QueryLimits, QueryRuntime},
};

#[tokio::test]
async fn native_network_admission_uses_configured_authority_and_every_redirect_hop() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let mut config = Config::default();
    config.producers.rust.docs_rs_url = "http://127.0.0.1:9000".into();
    config.producers.github_api_url = "https://configured.example:9001".into();
    config.network.max_redirects = 2;
    let policy = NetworkPolicy::new(&runtime, FetchPolicy::from_config(&config));
    for target in [
        "https://docs.rs/crate/serde/1/json",
        "https://8.8.8.8/",
        "https://[2606:4700::1111]/",
        "http://127.0.0.1:9000/fixture",
    ] {
        assert!(
            policy
                .check(&target.parse().unwrap(), 2)
                .await
                .unwrap()
                .is_ok(),
            "{target}"
        );
    }
    for target in [
        "https://127.0.0.1/",
        "https://localhost/",
        "https://api.localhost/",
        "https://10.1.2.3/",
        "https://100.100.1.1/",
        "https://172.16.5.5/",
        "https://192.168.1.1/",
        "https://169.254.169.254/",
        "https://0.0.0.0/",
        "https://255.255.255.255/",
        "https://[::1]/",
        "https://[fe80::1]/",
        "https://[fd00::1]/",
        "https://[::ffff:127.0.0.1]/",
        "https://[::ffff:10.0.0.1]/",
    ] {
        assert!(
            matches!(
                policy.check(&target.parse().unwrap(), 0).await.unwrap(),
                Err(PolicyViolation::PrivateAddress { .. })
            ),
            "{target}"
        );
    }
    assert!(matches!(
        policy
            .check(&"http://127.0.0.1:9001/".parse().unwrap(), 0)
            .await
            .unwrap(),
        Err(PolicyViolation::SchemeNotAllowed { .. })
    ));
    assert!(matches!(
        policy
            .check(&"https://docs.rs/".parse().unwrap(), 3)
            .await
            .unwrap(),
        Err(PolicyViolation::RedirectLimit { limit: 2, .. })
    ));
    let private_answer = ["127.0.0.1:443".parse().unwrap()];
    assert!(matches!(
        policy
            .check_addresses(
                &"https://public.example/".parse().unwrap(),
                0,
                &private_answer
            )
            .await
            .unwrap(),
        Err(PolicyViolation::PrivateAddress { .. })
    ));
    assert!(
        policy
            .check_addresses(
                &"https://configured.example:9001/".parse().unwrap(),
                0,
                &private_answer
            )
            .await
            .unwrap()
            .is_ok()
    );
    let public_answers = [
        "8.8.8.8:443".parse().unwrap(),
        "[2606:4700::1111]:443".parse().unwrap(),
    ];
    assert!(
        policy
            .check_addresses(
                &"https://public.example/".parse().unwrap(),
                0,
                &public_answers
            )
            .await
            .unwrap()
            .is_ok()
    );
}

#[tokio::test]
async fn delta_http_cache_reopens_exact_bytes_and_native_freshness_and_validator_decisions() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let cache = HttpCache::new(directory.path(), &runtime, 1024, 60).unwrap();
    let url = "https://example.org/package".parse().unwrap();
    assert!(cache.select(&url, None, false).await.unwrap().is_none());
    let response = Fetched {
        status: 200,
        bytes: b"exact cache source".to_vec(),
        content_type: Some("text/plain".into()),
        etag: Some("v1".into()),
        last_modified: Some("Wed, 16 Sep 2026 00:00:00 GMT".into()),
        final_url: "https://example.org/package".into(),
        retrieved_at: enrichment_core::native_time::AcquisitionTime::now().unwrap(),
    };
    cache
        .record(&url, Some("text/plain"), &response)
        .await
        .unwrap();
    drop(cache);
    let cache = HttpCache::new(directory.path(), &runtime, 1024, 60).unwrap();
    assert!(cache.select(&url, None, false).await.unwrap().is_none());
    let selected = cache
        .select(&url, Some("text/plain"), false)
        .await
        .unwrap()
        .unwrap();
    assert!(!selected.reuse);
    assert_eq!(selected.validator_name.as_deref(), Some("if-none-match"));
    assert_eq!(selected.validator_value.as_deref(), Some("v1"));
    assert_eq!(cache.body(&selected).await.unwrap(), response.bytes);
    let mut fresh = response.metadata();
    fresh.status = 304;
    fresh.content_type = None;
    fresh.etag = Some("v2".into());
    fresh.last_modified = None;
    let merged = cache.revalidated(&fresh, Some(&selected)).await.unwrap();
    assert_eq!(merged.status, 200);
    assert_eq!(merged.etag.as_deref(), Some("v2"));
    assert_eq!(merged.content_type, response.content_type);
    assert_eq!(merged.last_modified, response.last_modified);
    assert!(cache.revalidated(&fresh, None).await.is_err());
    let negative_url = "https://example.org/absent".parse().unwrap();
    let mut absent = response.clone();
    absent.status = 404;
    absent.final_url = "https://example.org/absent".into();
    cache.record(&negative_url, None, &absent).await.unwrap();
    let negative = cache
        .select(&negative_url, None, false)
        .await
        .unwrap()
        .unwrap();
    assert!(negative.reuse);
    assert!(negative.validator_name.is_none());
    assert!(
        !cache
            .select(&negative_url, None, true)
            .await
            .unwrap()
            .unwrap()
            .reuse
    );
    assert!(
        directory
            .path()
            .join("delta/http_responses/_delta_log")
            .is_dir()
    );
    assert!(!directory.path().join("http").exists());
}
