//! `library.resolve` against the fixture upstream: gates **R01** and **R03**, measured.
//!
//! The upstream is `tests/support/fixture_upstream.py`, a loopback HTTP server over the
//! committed files in `tests/fixtures/upstream/`. It is a stand-in for crates.io and docs.rs,
//! not a mock of any service component: the daemon's real fetcher, redirect policy, size
//! bounds and zstd path all run. Only the configured base URLs make loopback reachable, which
//! is itself asserted below.
//!
//! Every service here has explicit roots under a temp directory; nothing reads `LIBENR_*`.

#[path = "support/complete_answer.rs"]
mod complete_answer;

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use enrichment_core::config::Config;
use enrichment_daemon::fetch::{FetchError, Fetcher};
use enrichment_daemon::server;
use enrichment_daemon::service::Service;
use enrichment_store::StatePaths;
use url::Url;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/enrichment-daemon sits two levels below the repository root")
        .to_path_buf()
}

/// The fixture upstream, killed when dropped.
struct Upstream {
    child: Child,
    base: String,
}

impl Upstream {
    fn start() -> Self {
        let script = repo_root().join("tests/support/fixture_upstream.py");
        let root = repo_root().join("tests/fixtures/upstream");
        assert!(
            root.join("index/enr-fixture.ndjson").is_file(),
            "fixture upstream documents are missing; run `just fixtures-build`"
        );
        let mut child = Command::new("python3")
            .arg(&script)
            .arg("--root")
            .arg(&root)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("python3 is available to serve the fixture upstream");
        let stdout = child.stdout.take().expect("piped stdout");
        let mut line = String::new();
        BufReader::new(stdout)
            .read_line(&mut line)
            .expect("the server announces its port");
        let base = line
            .trim()
            .strip_prefix("listening on ")
            .expect("announcement shape")
            .to_owned();
        Self { child, base }
    }
}

impl Drop for Upstream {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A service whose registry and documentation endpoints are the fixture upstream, with only
/// the `static` profile enabled -- what a development configuration looks like.
fn service_for(upstream: &Upstream, dir: &Path) -> Service {
    let mut config = Config::default();
    config.producers.rust.crates_io_index_url = format!("{}/index", upstream.base);
    config.producers.rust.crates_io_api_url = format!("{}/api/v1", upstream.base);
    config.producers.rust.docs_rs_url = upstream.base.clone();
    config.policy.enabled_profiles = vec!["static".to_owned()];
    let paths = StatePaths::explicit(dir.join("cache"), dir.join("data"));
    Service::open(config, paths).expect("service opens")
}

async fn resolve(service: &Service, params: serde_json::Value) -> serde_json::Value {
    let frame = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "library.resolve", "params": params
    })
    .to_string();
    let response = server::dispatch(service, &frame)
        .await
        .response
        .expect("a response");
    assert!(response.error.is_none(), "{:?}", response.error);
    complete_answer::wait_for_answer(service, response.result.expect("an envelope")).await
}

fn strings(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

#[tokio::test]
async fn resolve_keeps_the_requested_version_while_a_newer_release_exists() {
    // Gate R01: the result retains 0.1.0; 0.2.0 is reported apart, never substituted.
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr-fixture", "version": "0.1.0" }),
    )
    .await;

    let data = &envelope["data"];
    assert_eq!(data["release"]["version"], "0.1.0");
    assert_eq!(data["release"]["package"], "enr-fixture");
    assert_eq!(data["release"]["lib_name"], "enr_fixture");
    assert_eq!(data["upstream"]["newest_stable"], "0.2.0");
    assert_eq!(data["upstream"]["resolved_is_newest_stable"], false);
    assert_eq!(data["upstream"]["published_versions"], 2);
    assert_eq!(data["context"]["mode"], "project");
    assert_eq!(data["environment"]["resolution"], "unspecified");
    assert_eq!(envelope["freshness"]["latest_verified"], true);
    assert!(envelope["freshness"]["registry_checked_at"].is_string());
    assert!(
        envelope["context_id"]
            .as_str()
            .is_some_and(|c| c.starts_with("ctx_")),
        "a context identity is minted"
    );
    assert_eq!(envelope["context_id"], data["context"]["context_id"]);
    // The registry line is cited as declared evidence with a locator.
    let evidence = envelope["evidence"].as_array().expect("evidence");
    assert!(evidence.iter().any(|e| {
        e["evidence_class"] == "declared"
            && e["producer"] == "crates-io-registry"
            && e["locator"]["kind"] == "index_line"
    }));
}

#[tokio::test]
async fn a_release_without_hosted_json_is_partial_with_the_fallback_named() {
    // Gate R03: 0.1.0 has no docs.rs JSON. Useful metadata remains, coverage is partial, and
    // the fallback is explicit -- including that policy does not currently allow it.
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr-fixture", "version": "0.1.0" }),
    )
    .await;

    assert_eq!(envelope["status"], "partial", "{}", envelope["summary"]);
    let indexed = strings(&envelope["coverage"]["indexed"]);
    for kind in [
        "registry_metadata",
        "crate_source",
        "documentation_build_config",
    ] {
        assert!(indexed.contains(&kind.to_owned()), "indexed: {indexed:?}");
    }
    let missing = strings(&envelope["coverage"]["missing"]);
    assert!(missing.contains(&"hosted_rustdoc_json".to_owned()));
    assert!(missing.contains(&"public_api".to_owned()));

    let data = &envelope["data"];
    assert_eq!(data["hosted_rustdoc_json"]["state"], "missing");
    assert_eq!(data["observed_configuration"]["all_features"], true);
    assert_eq!(data["observed_configuration"]["declared"], true);
    assert_eq!(
        data["observed_configuration"]["default_target"],
        "x86_64-unknown-linux-gnu"
    );
    let gap = data["gaps"]
        .as_array()
        .expect("gaps")
        .iter()
        .find(|g| g["kind"] == "hosted_rustdoc_json")
        .expect("the missing JSON is a gap");
    assert_eq!(gap["reason"], "hosted_json_missing");
    assert_eq!(
        gap["planned_fallback"]["producer"],
        enrichment_core::producer::rustdoc::LOCAL_PRODUCER,
        "the gap advertises the producer that actually runs, so a caller can recognise what it \
         got in the resulting snapshot"
    );
    assert_eq!(gap["planned_fallback"]["profile"], "build");
    assert_eq!(
        gap["planned_fallback"]["enabled"], false,
        "only `static` is enabled in this configuration"
    );
    assert!(
        !gap["planned_fallback"]["next_action"]
            .as_str()
            .unwrap_or_default()
            .is_empty()
    );
    // Artifacts were still stored: the index line, the version record, the tarball and its
    // manifest -- and every handle is a service-issued URI.
    let kinds: Vec<String> = data["artifacts"]
        .as_array()
        .expect("artifacts")
        .iter()
        .filter_map(|a| a["kind"].as_str().map(str::to_owned))
        .collect();
    for kind in [
        "registry_index_entry",
        "registry_version_metadata",
        "crate_tarball",
        "cargo_manifest",
    ] {
        assert!(kinds.contains(&kind.to_owned()), "artifacts: {kinds:?}");
    }
    assert!(!kinds.contains(&"rustdoc_json".to_owned()));
    for handle in envelope["artifacts"].as_array().expect("handles") {
        assert!(
            handle["uri"]
                .as_str()
                .is_some_and(|u| u.starts_with("library-evidence://artifacts/art_"))
        );
    }
}

#[tokio::test]
async fn a_release_with_hosted_json_stores_it_and_never_runs_cargo() {
    // 0.2.0 has docs.rs JSON in a supported format. The fixture's build script writes a marker
    // when it runs; the marker's absence after resolution is gate R02's primary oracle for the
    // acquisition half (the API index itself lands with the normalizer).
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let marker_dir = dir.path().join("marker");
    // SAFETY: this test is the only writer of this variable; nextest runs each test in its own
    // process, so no other test observes the change.
    unsafe { std::env::set_var("ENR_FIXTURE_MARKER_DIR", &marker_dir) };
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr-fixture", "version": "0.2.0" }),
    )
    .await;

    let data = &envelope["data"];
    assert_eq!(data["hosted_rustdoc_json"]["state"], "available");
    assert_eq!(data["hosted_rustdoc_json"]["format_version"], 61);
    assert_eq!(
        data["hosted_rustdoc_json"]["declared_crate_version"],
        "0.2.0"
    );
    assert_eq!(envelope["freshness"]["source_version_match"], "exact");
    assert!(strings(&envelope["coverage"]["indexed"]).contains(&"hosted_rustdoc_json".to_owned()));
    let json_artifact = data["artifacts"]
        .as_array()
        .expect("artifacts")
        .iter()
        .find(|a| a["kind"] == "rustdoc_json")
        .expect("the JSON was stored");
    assert_eq!(json_artifact["compression"], "zstd");
    assert_eq!(json_artifact["media_type"], "application/json");
    // The stored bytes are the decompressed JSON, readable by digest.
    let sha = json_artifact["sha256"].as_str().expect("digest");
    let bytes = service.blobs.read(sha).expect("blob is readable");
    assert!(bytes.starts_with(b"{"));
    assert_eq!(data["upstream"]["resolved_is_newest_stable"], true);

    assert!(
        !marker_dir.exists(),
        "the fixture's build script ran: acquisition must never invoke Cargo"
    );
}

#[tokio::test]
async fn the_underscore_spelling_finds_the_hyphenated_crate() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr_fixture", "version": "0.1.0" }),
    )
    .await;
    assert_ne!(envelope["status"], "error", "{}", envelope["summary"]);
    assert_eq!(envelope["data"]["release"]["package"], "enr-fixture");

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "no-such-crate", "version": "0.1.0" }),
    )
    .await;
    assert_eq!(envelope["status"], "error");
    assert_eq!(envelope["error"]["code"], "VERSION_NOT_FOUND");
    assert!(
        envelope["error"]["message"]
            .as_str()
            .is_some_and(|m| m.contains("no-such-crate") && m.contains("no_such_crate")),
        "both spellings were tried: {}",
        envelope["error"]["message"]
    );
}

#[tokio::test]
async fn a_missing_version_names_its_published_neighbours() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr-fixture", "version": "0.3.0" }),
    )
    .await;
    assert_eq!(envelope["status"], "error");
    assert_eq!(envelope["error"]["code"], "VERSION_NOT_FOUND");
    assert!(
        envelope["error"]["next_action"]
            .as_str()
            .is_some_and(|n| n.contains("0.2.0")),
        "{}",
        envelope["error"]["next_action"]
    );
    assert_eq!(envelope["coverage"]["indexed"], serde_json::json!([]));
}

#[tokio::test]
async fn an_unversioned_request_resolves_the_newest_stable_release_in_upstream_mode() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    let envelope = resolve(&service, serde_json::json!({ "name": "enr-fixture" })).await;
    assert_ne!(envelope["status"], "error", "{}", envelope["summary"]);
    assert_eq!(envelope["data"]["release"]["version"], "0.2.0");
    assert_eq!(envelope["data"]["context"]["mode"], "upstream");
    assert_eq!(envelope["freshness"]["latest_verified"], true);
}

#[tokio::test]
async fn a_recorded_resolution_replays_offline_with_the_same_context() {
    let dir = tempfile::tempdir().expect("dir");
    let (online_context, online_data) = {
        let upstream = Upstream::start();
        let service = service_for(&upstream, dir.path());
        let envelope = resolve(
            &service,
            serde_json::json!({ "name": "enr-fixture", "version": "0.1.0" }),
        )
        .await;
        assert_eq!(envelope["data"]["answered_from_cache"], false, "{envelope}");
        service
            .shutdown()
            .await
            .expect("owned acquisition and its terminal reconciliation drain");
        (
            envelope["context_id"].as_str().expect("ctx").to_owned(),
            envelope["data"].clone(),
        )
        // The upstream is dropped here: nothing below may open a socket.
    };

    let dead = Upstream {
        child: Command::new("true").spawn().expect("spawn true"),
        base: "http://127.0.0.1:9".to_owned(),
    };
    let service = service_for(&dead, dir.path());

    for freshness in ["offline", "cache_ok"] {
        let envelope = resolve(
            &service,
            serde_json::json!({ "name": "enr-fixture", "version": "0.1.0", "freshness": freshness }),
        )
        .await;
        assert_ne!(
            envelope["status"], "error",
            "{freshness}: {}",
            envelope["summary"]
        );
        assert_eq!(envelope["context_id"], online_context, "{freshness}");
        assert_eq!(envelope["data"]["answered_from_cache"], true, "{freshness}");
        assert_eq!(
            envelope["freshness"]["latest_verified"], false,
            "{freshness}"
        );
        assert_eq!(
            envelope["data"]["release"], online_data["release"],
            "{freshness}: the replay is the recorded resolution"
        );
        assert!(
            strings(&envelope["coverage"]["limitations"])
                .iter()
                .any(|l| l.contains("retained without age-based expiry")),
            "{freshness}: the replay says it is a replay"
        );
    }

    // A different environment is a different context, and nothing is recorded for it.
    let envelope = resolve(
        &service,
        serde_json::json!({
            "name": "enr-fixture", "version": "0.1.0", "freshness": "offline",
            "features": ["extra"]
        }),
    )
    .await;
    assert_eq!(envelope["status"], "error");
    assert_eq!(envelope["error"]["code"], "ARTIFACT_UNAVAILABLE");

    // Revalidate refuses to replay and, with the upstream gone, reports it honestly.
    let envelope = resolve(
        &service,
        serde_json::json!({ "name": "enr-fixture", "version": "0.1.0", "freshness": "revalidate" }),
    )
    .await;
    assert_eq!(envelope["status"], "error");
    assert_eq!(envelope["error"]["code"], "UPSTREAM_UNAVAILABLE");
    assert_eq!(envelope["error"]["retryable"], true);
}

async fn owned_fetch(
    service: &Service,
    url: Url,
) -> Result<enrichment_daemon::fetch::Fetched, FetchError> {
    let (record, _, fresh) = service
        .jobs
        .submit(enrichment_daemon::jobs::JobSpec::Resolve(
            enrichment_core::request::ResolveRequest {
                name: "enr-fixture".into(),
                version: Some("0.1.0".into()),
                ..Default::default()
            },
        ))
        .await
        .unwrap();
    assert!(fresh);
    let jobs = service.jobs.clone();
    let driver_jobs = jobs.clone();
    let id = record.job_id;
    let fetcher = service.fetcher.clone();
    let (send, receive) = tokio::sync::oneshot::channel();
    jobs.spawn(&service.repository.runtime, id.clone(), async move {
        assert!(driver_jobs.start(&id).await.unwrap());
        let result = fetcher.get(&url, None).await;
        send.send(result).unwrap();
    })
    .unwrap();
    let result = receive.await.unwrap();
    tokio::time::timeout(Duration::from_secs(60), async {
        while jobs.has_owned_work() {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("physical driver and durable settlement complete");
    result
}
#[tokio::test]
async fn the_fetch_policy_holds_against_the_fixture_upstream() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_for(&upstream, dir.path());

    // A redirect to a link-local address is refused at the hop.
    let url = Url::parse(&format!("{}/_test/redirect-private", upstream.base)).expect("url");
    let err = owned_fetch(&service, url).await.expect_err("refused");
    assert!(matches!(err, FetchError::RedirectRefused { .. }), "{err}");
    assert!(matches!(
        err.code(),
        enrichment_core::wire::ErrorCode::PolicyDenied
    ));

    // A redirect loop stops at the configured hop count.
    let url = Url::parse(&format!("{}/_test/redirect-loop", upstream.base)).expect("url");
    let err = owned_fetch(&service, url).await.expect_err("loop refused");
    assert!(matches!(err, FetchError::RedirectRefused { .. }), "{err}");

    // A body over the download bound is refused without being buffered.
    let mut small = service.config.clone();
    small.network.max_download_bytes = 1024;
    let small_service = Service::open(
        small,
        StatePaths::explicit(
            dir.path().join("small-cache"),
            dir.path().join("small-data"),
        ),
    )
    .unwrap();
    let url = Url::parse(&format!("{}/_test/oversized/4096", upstream.base)).expect("url");
    let err = owned_fetch(&small_service, url)
        .await
        .expect_err("too large");
    assert!(
        matches!(err, FetchError::TooLarge { limit: 1024, .. }),
        "{err}"
    );

    // Loopback is reachable only because it is configured: a default policy refuses it.
    let default_fetcher = Fetcher::new(
        &Config::default(),
        &service.repository.runtime,
        &service.paths.data_root,
    )
    .expect("client");
    let url = Url::parse(&format!("{}/index/en/r-/enr-fixture", upstream.base)).expect("url");
    let err = default_fetcher.get(&url, None).await.expect_err("refused");
    assert!(matches!(err, FetchError::Policy(_)), "{err}");

    let _ = Duration::from_secs(0);
}
