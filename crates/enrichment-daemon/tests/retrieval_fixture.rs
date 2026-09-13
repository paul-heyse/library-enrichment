//! The retrieval methods over a snapshot published from the fixture upstream: gates **R02**,
//! **R05**, **R06**, **R07**, plus cursors, budgets, the manifest resource and the cold-then-
//! offline half of the blueprint's Phase 1 gate, all measured at the RPC boundary.
//!
//! The upstream is `tests/support/fixture_upstream.py` over `tests/fixtures/upstream/`; the
//! daemon's real fetcher, zstd path, normalizer and publisher run. Every service here has
//! explicit roots under a temp directory; nothing reads `LIBENR_*`.

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use enrichment_core::config::Config;
use enrichment_core::producer::normalize::{self, NormalizeInput};
use enrichment_daemon::server;
use enrichment_daemon::service::Service;
use enrichment_store::StatePaths;

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
            root.join("docsrs/enr-fixture-0.2.0.json.zst").is_file(),
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

fn config_for(upstream: &Upstream) -> Config {
    let mut config = Config::default();
    config.producers.rust.crates_io_index_url = format!("{}/index", upstream.base);
    config.producers.rust.crates_io_api_url = format!("{}/api/v1", upstream.base);
    config.producers.rust.docs_rs_url = upstream.base.clone();
    config.policy.enabled_profiles = vec!["static".to_owned()];
    config
}

fn service_with(config: Config, dir: &Path) -> Service {
    let paths = StatePaths::explicit(dir.join("cache"), dir.join("data"));
    Service::open(config, paths).expect("service opens")
}

async fn call(service: &Service, method: &str, params: serde_json::Value) -> serde_json::Value {
    let frame = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": method, "params": params
    })
    .to_string();
    let response = server::dispatch(service, &frame)
        .await
        .response
        .expect("a response");
    assert!(response.error.is_none(), "{method}: {:?}", response.error);
    response.result.expect("an envelope")
}

/// Resolve the fixture release that has hosted JSON, and return its envelope.
async fn resolve_0_2_0(service: &Service) -> serde_json::Value {
    let envelope = call(
        service,
        "library.resolve",
        serde_json::json!({ "name": "enr-fixture", "version": "0.2.0" }),
    )
    .await;
    assert_eq!(
        envelope["status"], "ok",
        "0.2.0 has JSON and a tarball: {}",
        envelope["summary"]
    );
    assert!(
        envelope["snapshot_id"]
            .as_str()
            .is_some_and(|s| s.starts_with("snap_")),
        "a snapshot is published: {}",
        envelope["summary"]
    );
    envelope
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

fn ctx(envelope: &serde_json::Value) -> String {
    envelope["context_id"]
        .as_str()
        .expect("context id")
        .to_owned()
}

// ---------------------------------------------------------------------------------------------
// R02: hosted JSON available -> the API index succeeds without invoking Cargo compilation.
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn hosted_json_indexes_the_api_without_compiling_the_crate() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let marker_dir = dir.path().join("marker");
    // SAFETY: this test is the only writer of this variable; nextest runs each test in its own
    // process, so no other test observes the change. The fixture's build.rs writes a marker
    // here if it is ever compiled, which is the primary oracle for R02.
    unsafe { std::env::set_var("ENR_FIXTURE_MARKER_DIR", &marker_dir) };
    let service = service_with(config_for(&upstream), dir.path());

    let resolved = resolve_0_2_0(&service).await;
    let context_id = ctx(&resolved);
    let snapshot = &resolved["data"]["snapshot"];
    assert!(snapshot["counts"]["definitions"].as_u64().unwrap_or(0) >= 5);
    assert_eq!(snapshot["normalizer_version"], "1");
    let indexed = strings(&resolved["coverage"]["indexed"]);
    for kind in [
        "registry_metadata",
        "crate_source",
        "documentation_build_config",
        "hosted_rustdoc_json",
        "public_api",
        "documentation",
        "examples",
        "release_notes",
    ] {
        assert!(
            indexed.contains(&kind.to_owned()),
            "{kind} indexed: {indexed:?}"
        );
    }
    assert!(strings(&resolved["coverage"]["missing"]).is_empty());
    let runs = resolved["data"]["producer_runs"].as_array().expect("runs");
    let normalizer = runs
        .iter()
        .find(|r| r["producer"] == "rustdoc-json" && r["outcome"] == "succeeded")
        .expect("the normalizer ran and succeeded");
    assert_eq!(normalizer["profile"], "static");

    let overview = call(
        &service,
        "library.overview",
        serde_json::json!({ "context_id": context_id }),
    )
    .await;
    assert_eq!(overview["status"], "ok", "{}", overview["summary"]);
    let data = &overview["data"];
    assert_eq!(data["crate_name"], "enr_fixture");
    assert_eq!(data["crate_version"], "0.2.0");
    assert_eq!(data["snapshot"]["snapshot_id"], resolved["snapshot_id"]);
    assert_eq!(data["namespaces"][0]["path"], "enr_fixture");
    assert!(
        data["definitions_by_kind"]["function"]
            .as_u64()
            .unwrap_or(0)
            >= 3
    );
    assert_eq!(data["definitions_by_kind"]["trait"], 1);
    assert!(
        data["features"].get("extra").is_some(),
        "{}",
        data["features"]
    );
    assert!(!strings(&data["documentation_headings"]).is_empty());
    assert!(!strings(&data["release_note_headings"]).is_empty());
    assert_eq!(strings(&data["examples"]), vec!["basic".to_owned()]);

    let search = call(
        &service,
        "evidence.search",
        serde_json::json!({ "context_id": context_id, "query": "describe" }),
    )
    .await;
    assert_eq!(search["status"], "ok", "{}", search["summary"]);
    assert!(
        search["data"]["hits"]
            .as_array()
            .is_some_and(|h| !h.is_empty())
    );

    assert!(
        !marker_dir.exists()
            || std::fs::read_dir(&marker_dir).is_ok_and(|mut d| d.next().is_none()),
        "the fixture's build script ran: the crate was compiled"
    );
}

// ---------------------------------------------------------------------------------------------
// R05: default vs optional feature -- observed configuration, never an invented predicate.
// ---------------------------------------------------------------------------------------------

#[test]
fn feature_gated_items_follow_the_documented_build_configuration() {
    // The two captures of 0.2.0 differ only in features. The normalizer reports what each
    // build contained; it does not attach a feature predicate to `extra_only`.
    let root = repo_root().join("tests/fixtures/rustdoc");
    let paths_for = |name: &str| {
        let payload = std::fs::read_to_string(root.join(name)).expect(name);
        let normalized = normalize::normalize(&NormalizeInput {
            payload: &payload,
            rustdoc_artifact_id: "art_00000000000000000000000000000000",
            json_path: Some(&root.join(name)),
            summary_chars: 200,
        })
        .expect("fixture JSON normalizes");
        normalized
            .symbols
            .iter()
            .map(|s| s.path.clone())
            .collect::<Vec<_>>()
    };
    let default = paths_for("enr-fixture-0.2.0-default.json");
    let all = paths_for("enr-fixture-0.2.0-all-features.json");
    assert!(
        !default.iter().any(|p| p == "enr_fixture::extra_only"),
        "`extra_only` is compiled out of the default build"
    );
    assert!(
        all.iter().any(|p| p == "enr_fixture::extra_only"),
        "`extra_only` is present with all features"
    );
    assert!(default.iter().any(|p| p == "enr_fixture::describe"));
    assert!(all.iter().any(|p| p == "enr_fixture::describe"));
}

#[tokio::test]
async fn availability_is_reported_against_the_observed_configuration() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());

    // docs.rs builds this crate with all features (its manifest says so); a project declaring
    // only default features has not verified that `extra_only` is available to it.
    let resolved = call(
        &service,
        "library.resolve",
        serde_json::json!({
            "name": "enr-fixture", "version": "0.2.0",
            "features": [], "default_features": true
        }),
    )
    .await;
    assert_eq!(resolved["status"], "ok", "{}", resolved["summary"]);
    assert_eq!(resolved["data"]["environment"]["resolution"], "declared");
    assert_eq!(
        resolved["data"]["observed_configuration"]["all_features"],
        true
    );

    let inspected = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": ctx(&resolved), "symbol_path": "enr_fixture::extra_only",
            "aspects": ["signature", "availability"]
        }),
    )
    .await;
    assert_ne!(inspected["status"], "error", "{}", inspected["summary"]);
    let availability = &inspected["data"]["availability"];
    assert_eq!(availability["status"], "project_availability_unverified");
    assert_eq!(availability["observed_configuration"]["all_features"], true);
    assert_eq!(
        availability["requested_configuration"]["features"],
        serde_json::json!([])
    );
    let notes = strings(&availability["notes"]);
    assert!(
        notes.iter().any(|n| n.contains("all features")),
        "the note names the configuration difference: {notes:?}"
    );
    let symbol = &inspected["data"]["symbol"];
    assert_eq!(symbol["path"], "enr_fixture::extra_only");
    assert!(
        symbol.get("feature").is_none() && symbol.get("features").is_none(),
        "no per-symbol feature predicate is invented"
    );
}

// ---------------------------------------------------------------------------------------------
// R06: target-gated item -- target differences are identified before availability claims.
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn a_target_difference_is_identified_before_any_availability_claim() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());

    let resolved = call(
        &service,
        "library.resolve",
        serde_json::json!({
            "name": "enr-fixture", "version": "0.2.0", "target": "x86_64-pc-windows-msvc"
        }),
    )
    .await;
    assert_eq!(resolved["status"], "ok", "{}", resolved["summary"]);
    let observed_target = resolved["data"]["hosted_rustdoc_json"]["target"]
        .as_str()
        .expect("target")
        .to_owned();
    assert_eq!(observed_target, "x86_64-unknown-linux-gnu");

    let inspected = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": ctx(&resolved), "symbol_path": "enr_fixture::unix_only",
            "aspects": ["availability"]
        }),
    )
    .await;
    assert_ne!(inspected["status"], "error", "{}", inspected["summary"]);
    let availability = &inspected["data"]["availability"];
    assert_eq!(availability["status"], "project_availability_unverified");
    assert_eq!(
        availability["observed_configuration"]["target"],
        observed_target
    );
    assert_eq!(
        availability["requested_configuration"]["target"],
        "x86_64-pc-windows-msvc"
    );
    let notes = strings(&availability["notes"]);
    assert!(
        notes
            .iter()
            .any(|n| n.contains("x86_64-pc-windows-msvc") && n.contains(&observed_target)),
        "the target difference is named: {notes:?}"
    );
    let limitations = strings(&inspected["coverage"]["limitations"]);
    assert!(
        limitations
            .iter()
            .any(|l| l.to_lowercase().contains("target")),
        "{limitations:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// R07: re-exported public symbol -- linked to its definition, counted once.
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn a_reexport_is_linked_to_its_definition_and_counted_once() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let context_id = ctx(&resolved);

    let at_root = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": context_id, "symbol_path": "enr_fixture::Widget",
            "aspects": ["signature", "relationships"]
        }),
    )
    .await;
    let inner = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": context_id, "symbol_path": "enr_fixture::inner::Widget",
            "aspects": ["signature", "relationships"]
        }),
    )
    .await;
    assert_eq!(at_root["status"], "ok", "{}", at_root["summary"]);
    assert_eq!(inner["status"], "ok", "{}", inner["summary"]);
    let root_symbol = &at_root["data"]["symbol"];
    let inner_symbol = &inner["data"]["symbol"];
    assert_eq!(root_symbol["is_reexport"], true);
    assert_eq!(inner_symbol["is_reexport"], false);
    assert_eq!(
        root_symbol["definition_id"], inner_symbol["definition_id"],
        "both paths name one definition"
    );
    assert_ne!(root_symbol["symbol_id"], inner_symbol["symbol_id"]);
    assert_eq!(root_symbol["definition_path"], "enr_fixture::inner::Widget");
    assert!(
        strings(&at_root["data"]["also_at"]).contains(&"enr_fixture::inner::Widget".to_owned()),
        "{}",
        at_root["data"]["also_at"]
    );
    let edges = at_root["data"]["relationships"].as_array().expect("edges");
    assert!(
        edges.iter().any(|e| e["kind"] == "reexports"),
        "a reexports edge is recorded: {edges:?}"
    );

    // One definition in the counts, whichever path it is reached by.
    let overview = call(
        &service,
        "library.overview",
        serde_json::json!({ "context_id": context_id }),
    )
    .await;
    assert_eq!(overview["data"]["definitions_by_kind"]["struct"], 1);
    assert_eq!(overview["data"]["reexports"], 1);

    // Search folds the two paths into one hit that names the other path.
    let search = call(
        &service,
        "evidence.search",
        serde_json::json!({ "context_id": context_id, "query": "Widget", "kinds": ["api"] }),
    )
    .await;
    let hits = search["data"]["hits"].as_array().expect("hits");
    let widgets: Vec<&serde_json::Value> = hits
        .iter()
        .filter(|h| {
            h["hit"] == "symbol" && h["path"].as_str().is_some_and(|p| p.ends_with("Widget"))
        })
        .collect();
    assert_eq!(widgets.len(), 1, "{hits:?}");
    assert_eq!(strings(&widgets[0]["also_at"]).len(), 1);
}

// ---------------------------------------------------------------------------------------------
// Cursors, budgets, artifacts and the manifest resource.
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn search_pages_with_a_checksummed_cursor_and_rejects_a_foreign_one() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let context_id = ctx(&resolved);

    let first = call(
        &service,
        "evidence.search",
        serde_json::json!({ "context_id": context_id, "query": "shape widget describe", "max_items": 1 }),
    )
    .await;
    assert_eq!(first["status"], "ok", "{}", first["summary"]);
    assert_eq!(first["pagination"]["returned"], 1);
    assert_eq!(first["pagination"]["truncated"], true);
    let cursor = first["pagination"]["next_cursor"]
        .as_str()
        .expect("a cursor")
        .to_owned();
    assert!(cursor.starts_with("cur_"));

    let second = call(
        &service,
        "evidence.search",
        serde_json::json!({
            "context_id": context_id, "query": "shape widget describe", "max_items": 1,
            "cursor": cursor
        }),
    )
    .await;
    assert_ne!(second["status"], "error", "{}", second["summary"]);
    assert_eq!(second["data"]["offset"], 1);
    assert_ne!(
        second["data"]["hits"][0]["evidence_id"],
        first["data"]["hits"][0]["evidence_id"]
    );

    let foreign = call(
        &service,
        "evidence.search",
        serde_json::json!({
            "context_id": context_id, "query": "something else", "cursor": cursor
        }),
    )
    .await;
    assert_eq!(foreign["status"], "error");
    assert_eq!(foreign["error"]["code"], "INVALID_CURSOR");

    let mut tampered = cursor.clone();
    tampered.replace_range(4..6, "zz");
    let bad = call(
        &service,
        "evidence.search",
        serde_json::json!({
            "context_id": context_id, "query": "shape widget describe", "cursor": tampered
        }),
    )
    .await;
    assert_eq!(bad["status"], "error");
    assert_eq!(bad["error"]["code"], "INVALID_CURSOR");
}

#[tokio::test]
async fn an_empty_search_states_what_was_searched() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;

    let empty = call(
        &service,
        "evidence.search",
        serde_json::json!({ "context_id": ctx(&resolved), "query": "zzqxjv" }),
    )
    .await;
    assert_ne!(empty["status"], "error");
    assert_eq!(empty["pagination"]["returned"], 0);
    assert!(!strings(&empty["data"]["searched"]).is_empty());
    let limitations = strings(&empty["coverage"]["limitations"]);
    assert!(
        limitations.iter().any(|l| l.contains("not proof")),
        "absence is not evidence of absence: {limitations:?}"
    );
}

#[tokio::test]
async fn artifacts_are_read_by_handle_in_bounded_slices() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;

    let artifacts = resolved["data"]["artifacts"].as_array().expect("artifacts");
    let readme = artifacts
        .iter()
        .find(|a| a["kind"] == "readme")
        .expect("the README is stored as its own artifact");
    let readme_id = readme["artifact_id"].as_str().expect("id").to_owned();

    // Page through with a small budget; the pieces reassemble to the whole.
    let mut cursor: Option<String> = None;
    let mut assembled = String::new();
    let mut pages = 0;
    loop {
        let mut params = serde_json::json!({ "artifact_id": readme_id, "max_bytes": 1024 });
        if let Some(c) = &cursor {
            params["cursor"] = serde_json::Value::String(c.clone());
        }
        let page = call(&service, "artifact.read", params).await;
        assert_ne!(page["status"], "error", "{}", page["summary"]);
        assert_eq!(page["data"]["encoding"], "utf8");
        assembled.push_str(page["data"]["content"].as_str().expect("text"));
        pages += 1;
        match page["pagination"]["next_cursor"].as_str() {
            Some(next) => cursor = Some(next.to_owned()),
            None => break,
        }
    }
    let readme_size = readme["size_bytes"].as_u64().expect("size") as usize;
    assert_eq!(assembled.len(), readme_size, "after {pages} pages");
    assert!(assembled.contains("# enr-fixture") || assembled.contains("# enr_fixture"));

    // A section reads one heading's body.
    let whole = call(
        &service,
        "artifact.read",
        serde_json::json!({ "artifact_id": readme_id }),
    )
    .await;
    assert_eq!(whole["data"]["remaining"], 0);
    let heading = whole["data"]["content"]
        .as_str()
        .expect("text")
        .lines()
        .find_map(|l| l.strip_prefix("## "))
        .expect("the README has a second-level heading")
        .to_owned();
    let section = call(
        &service,
        "artifact.read",
        serde_json::json!({ "artifact_id": readme_id, "section": heading }),
    )
    .await;
    assert_ne!(section["status"], "error", "{}", section["summary"]);
    assert_eq!(section["data"]["section"], heading);
    assert!(section["data"]["end"].as_u64() < whole["data"]["end"].as_u64());

    // Binary artifacts come back base64, never as a path.
    let tarball = artifacts
        .iter()
        .find(|a| a["kind"] == "crate_tarball")
        .expect("tarball");
    let slice = call(
        &service,
        "artifact.read",
        serde_json::json!({ "artifact_id": tarball["artifact_id"], "max_bytes": 2048 }),
    )
    .await;
    assert_eq!(slice["data"]["encoding"], "base64");
    assert!(slice["data"]["content_digest"].is_string());

    // Only service-issued handles are accepted.
    let path = call(
        &service,
        "artifact.read",
        serde_json::json!({ "artifact_id": "/etc/passwd" }),
    )
    .await;
    assert_eq!(path["status"], "error");
    assert_eq!(path["error"]["code"], "ARTIFACT_UNAVAILABLE");
}

#[tokio::test]
async fn the_manifest_resource_describes_the_published_snapshot() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let snapshot_id = resolved["snapshot_id"].as_str().expect("snap").to_owned();

    let manifest = call(
        &service,
        "snapshot.manifest",
        serde_json::json!({ "snapshot_id": snapshot_id }),
    )
    .await;
    assert_eq!(manifest["status"], "ok", "{}", manifest["summary"]);
    assert_eq!(manifest["data"]["is_current"], true);
    assert_eq!(manifest["data"]["manifest"]["snapshot_id"], snapshot_id);
    assert_eq!(
        manifest["data"]["manifest"]["counts"],
        resolved["data"]["snapshot"]["counts"]
    );
    assert_eq!(manifest["context_id"], resolved["context_id"]);

    let missing = call(
        &service,
        "snapshot.manifest",
        serde_json::json!({ "snapshot_id": "snap_0000000000000000" }),
    )
    .await;
    assert_eq!(missing["status"], "error");
    assert_eq!(missing["error"]["code"], "ARTIFACT_UNAVAILABLE");
}

// ---------------------------------------------------------------------------------------------
// Phase 1 gate: cold from a clean cache, then offline from the same snapshot.
// ---------------------------------------------------------------------------------------------

#[tokio::test]
async fn every_tool_answers_offline_from_the_snapshot_published_cold() {
    let dir = tempfile::tempdir().expect("dir");
    let upstream = Upstream::start();
    let config = config_for(&upstream);

    let (context_id, snapshot_id, cold) = {
        let service = service_with(config.clone(), dir.path());
        let resolved = resolve_0_2_0(&service).await;
        let context_id = ctx(&resolved);
        let snapshot_id = resolved["snapshot_id"].as_str().expect("snap").to_owned();
        let cold = retrieval_round(&service, &context_id, &resolved).await;
        (context_id, snapshot_id, cold)
    };
    drop(upstream);

    // A fresh service over the same roots, with no upstream to reach.
    let service = service_with(config, dir.path());
    let offline = call(
        &service,
        "library.resolve",
        serde_json::json!({ "name": "enr-fixture", "version": "0.2.0", "freshness": "offline" }),
    )
    .await;
    assert_eq!(offline["status"], "ok", "{}", offline["summary"]);
    assert_eq!(offline["snapshot_id"], snapshot_id);
    assert_eq!(offline["context_id"], context_id);
    assert_eq!(offline["data"]["answered_from_cache"], true);
    let warm = retrieval_round(&service, &context_id, &offline).await;
    assert_eq!(cold, warm, "the same snapshot yields the same answers");

    // Reads pinned to the snapshot identity also work.
    let pinned = call(
        &service,
        "library.overview",
        serde_json::json!({ "context_id": context_id, "snapshot_id": snapshot_id }),
    )
    .await;
    assert_eq!(pinned["status"], "ok");
    assert_eq!(pinned["data"], cold[0]);
}

/// One pass over the four retrieval tools; the `data` payloads, for comparison.
async fn retrieval_round(
    service: &Service,
    context_id: &str,
    resolved: &serde_json::Value,
) -> Vec<serde_json::Value> {
    let overview = call(
        service,
        "library.overview",
        serde_json::json!({ "context_id": context_id }),
    )
    .await;
    let search = call(
        service,
        "evidence.search",
        serde_json::json!({ "context_id": context_id, "query": "perimeter" }),
    )
    .await;
    let inspect = call(
        service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": context_id, "symbol_path": "enr_fixture::Shape", "depth": "source"
        }),
    )
    .await;
    let readme = resolved["data"]["artifacts"]
        .as_array()
        .expect("artifacts")
        .iter()
        .find(|a| a["kind"] == "readme")
        .expect("readme")["artifact_id"]
        .clone();
    let artifact = call(
        service,
        "artifact.read",
        serde_json::json!({ "artifact_id": readme }),
    )
    .await;
    for (name, envelope) in [
        ("overview", &overview),
        ("search", &search),
        ("inspect", &inspect),
        ("artifact", &artifact),
    ] {
        assert_ne!(
            envelope["status"], "error",
            "{name}: {}",
            envelope["summary"]
        );
    }
    assert!(
        inspect["data"]["source"].is_object(),
        "depth=source reads the extracted tarball: {}",
        inspect["summary"]
    );
    vec![
        overview["data"].clone(),
        search["data"].clone(),
        inspect["data"].clone(),
        artifact["data"].clone(),
    ]
}
