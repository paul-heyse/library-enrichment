//! The retrieval methods over a snapshot published from the fixture upstream: gates **R02**,
//! **R05**, **R06**, **R07**, plus cursors, budgets, the manifest resource and the cold-then-
//! offline half of the blueprint's Phase 1 gate, all measured at the RPC boundary.
//!
//! The upstream is `tests/support/fixture_upstream.py` over `tests/fixtures/upstream/`; the
//! daemon's real fetcher, zstd path, normalizer and publisher run. Every service here has
//! explicit roots under a temp directory; nothing reads `LIBENR_*`.

#[path = "../../enrichment-store/tests/support/producer_records.rs"]
mod producer_records;

#[path = "support/complete_answer.rs"]
mod complete_answer;
use complete_answer::complete_answer;

#[path = "../../enrichment-store/tests/support/read_parquet.rs"]
mod parquet_read;
#[path = "../../enrichment-store/tests/support/write_parquet.rs"]
mod parquet_write;

#[path = "../../enrichment-store/tests/support/native_ingest.rs"]
mod native_ingest;

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
    let before = service.repository.runtime.diagnostic_summary();
    let frame = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": method, "params": params
    })
    .to_string();
    let response = server::dispatch(service, &frame)
        .await
        .response
        .expect("a response");
    assert!(response.error.is_none(), "{method}: {:?}", response.error);
    if std::env::var_os("LIBENR_MEASURE_QUERIES").is_some() && method != "artifact.read" {
        eprintln!(
            "PLAN11_QUERY_DIAGNOSTICS {}",
            serde_json::json!({
                "method":method,"before":before,"after":service.repository.runtime.diagnostic_summary(),
                "recent":service.repository.runtime.diagnostics(),
            })
        );
    }
    let result = response.result.expect("an envelope");
    if method == "library.resolve" {
        complete_answer::wait_for_answer(service, result).await
    } else {
        complete_answer(service, result).await
    }
}

async fn comparison_result(service: &Service, response: serde_json::Value) -> serde_json::Value {
    complete_answer::wait_for_answer(service, response).await
}

#[tokio::test]
async fn cold_version_comparison_waits_for_exact_acquisitions() {
    let dir = tempfile::tempdir().unwrap();
    let upstream = Upstream::start();
    let mut config = config_for(&upstream);
    config.limits.inline_wait_seconds = 0;
    let request = serde_json::json!({"ecosystem":"rust", "name":"enr-fixture", "from_version":"0.1.0", "to_version":"0.2.0", "scopes":["api"]});
    let (job_id, result, active) = {
        let service = service_with(config.clone(), dir.path());
        let pending = call(&service, "library.compare", request.clone()).await;
        assert_eq!(pending["status"], "pending", "{pending}");
        let job_id = pending["data"]["job_id"].as_str().unwrap().to_owned();
        let active = std::fs::read(
            dir.path()
                .join("data/jobs/active")
                .join(format!("{job_id}.json")),
        )
        .unwrap();
        let result = comparison_result(&service, pending).await;
        if std::env::var_os("LIBENR_MEASURE_QUERIES").is_some() {
            let observed = service
                .repository
                .runtime
                .operation_diagnostics()
                .into_iter()
                .find(|v| v.operation_id == job_id)
                .expect("completed comparison operation");
            eprintln!(
                "PLAN13_COMPARISON_MEASUREMENT {}",
                serde_json::to_string(&observed).unwrap()
            );
        }
        assert!(
            matches!(result["status"].as_str(), Some("ok" | "partial")),
            "{result}"
        );
        assert_eq!(result["data"]["before"]["release"]["version"], "0.1.0");
        assert_eq!(result["data"]["after"]["release"]["version"], "0.2.0");
        let published = service
            .repository
            .catalog
            .pin()
            .await
            .unwrap()
            .comparison_publication(&service.repository.runtime, &job_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            published.after_snapshot_id.as_str(),
            result["data"]["after"]["snapshot_id"].as_str().unwrap()
        );
        (job_id, result, active)
    };
    // Model the precise crash boundary: catalog committed, terminal journal rename absent.
    std::fs::write(
        dir.path()
            .join("data/jobs/active")
            .join(format!("{job_id}.json")),
        active,
    )
    .unwrap();
    std::fs::remove_file(
        dir.path()
            .join("data/jobs/terminal")
            .join(format!("{job_id}.json")),
    )
    .unwrap();
    drop(upstream);
    let service = service_with(config, dir.path());
    let journal = call(
        &service,
        "job.control",
        serde_json::json!({"job_id":job_id,"action":"status"}),
    )
    .await;
    let journal = complete_answer(&service, journal).await;
    assert!(
        journal["data"]["stage"]
            .as_str()
            .unwrap()
            .contains("recovered committed result")
    );
    let terminal = &journal["data"]["result"];
    let mut descriptor = journal.clone();
    descriptor["delivery"] = terminal["delivery"].clone();
    descriptor["context_id"] = terminal["context_id"].clone();
    descriptor["snapshot_id"] = terminal["snapshot_id"].clone();
    assert_eq!(
        complete_answer(&service, descriptor).await["data"],
        result["data"]
    );
    let pending = call(&service, "library.compare", request).await;
    let warm = comparison_result(&service, pending).await;
    assert_eq!(
        warm["data"], result["data"],
        "offline exact evidence is reused"
    );
}

#[tokio::test]
async fn signature_projection_retains_qualified_ids_without_documentation() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().unwrap();
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let context = ctx(&resolved);
    let signature = call(&service, "symbol.inspect", serde_json::json!({
        "context_id": context, "symbol_path": "enr_fixture::Widget", "selection": {"mode":"explicit","aspects":[{"aspect":"signature"}]}
    })).await;
    let signature = complete_answer(&service, signature).await;
    assert_ne!(signature["status"], "error", "{signature}");
    assert!(signature["data"]["symbol"]["docs"].is_null());
    let projected = signature["data"]["observations"].as_array().unwrap();
    assert!(!projected.is_empty());
    assert!(
        projected
            .iter()
            .all(|o| o["docs_included"] == false && o["payload"]["docs"].is_null())
    );
    let complete = call(&service, "symbol.inspect", serde_json::json!({
        "context_id": context, "symbol_path": "enr_fixture::Widget", "selection": {"mode":"explicit","aspects":[{"aspect":"signature"},{"aspect":"documentation"}]}
    })).await;
    let complete = complete_answer(&service, complete).await;
    assert_ne!(complete["status"], "error", "{complete}");
    let facts = complete["data"]["observations"].as_array().unwrap();
    assert_eq!(projected.len(), facts.len());
    for (selection, full) in projected.iter().zip(facts) {
        assert_eq!(selection["observation_id"], full["observation_id"]);
        assert_eq!(selection["source"], full["source"]);
        assert_eq!(
            selection["payload"]["signature"],
            full["payload"]["signature"]
        );
        assert_eq!(full["docs_included"], false);
    }
    let fragments = complete["data"]["fragments"].as_array().unwrap();
    assert!(fragments.iter().any(|item| {
        item["fragment"]["text"]
            .as_str()
            .is_some_and(|text| !text.is_empty())
    }));
}

/// Resolve the fixture release that has hosted JSON, and return its envelope.
async fn resolve_0_2_0(service: &Service) -> serde_json::Value {
    let envelope = call(
        service,
        "library.resolve",
        serde_json::json!({ "name": "enr-fixture", "version": "0.2.0" }),
    )
    .await;
    let envelope = complete_answer(service, envelope).await;
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
    assert_eq!(
        snapshot["normalizer_version"],
        enrichment_core::producer::rustdoc::NORMALIZER_VERSION
    );
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
    let discovery = data["discovery"].as_array().expect("discovery pages");
    let features = discovery
        .iter()
        .find(|facet| facet["kind"] == "features")
        .unwrap();
    assert!(
        features["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["fragment"]["subject"] == "extra"),
        "{features}"
    );
    for kind in ["documentation", "release_notes"] {
        assert!(
            !discovery
                .iter()
                .find(|facet| facet["kind"] == kind)
                .unwrap()["items"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
    let examples = discovery
        .iter()
        .find(|facet| facet["kind"] == "examples")
        .unwrap();
    assert_eq!(
        examples["items"][0]["fragment"]["subject"],
        "examples/basic.rs"
    );

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
        let normalized = producer_records::collect(
            normalize::prepare(&NormalizeInput {
                payload: &payload,
                rustdoc_artifact_id: "art_00000000000000000000000000000000",
                json_path: Some(&root.join(name)),
                summary_chars: 200,
            })
            .expect("prepared fixture producer"),
        )
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
    let resolved = complete_answer(&service, resolved).await;
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
            "selection": {"mode":"explicit","aspects":[{"aspect":"signature"},{"aspect":"availability"}]}
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
    let resolved = complete_answer(&service, resolved).await;
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
            "selection": {"mode":"explicit","aspects":[{"aspect":"availability"}]}
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
            "selection": {"mode":"explicit","aspects":[{"aspect":"signature"},{"aspect":"relationships"}]}
        }),
    )
    .await;
    let inner = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id": context_id, "symbol_path": "enr_fixture::inner::Widget",
            "selection": {"mode":"explicit","aspects":[{"aspect":"signature"},{"aspect":"relationships"}]}
        }),
    )
    .await;
    let at_root = complete_answer(&service, at_root).await;
    let inner = complete_answer(&service, inner).await;
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
        edges.iter().any(|e| e["relation"] == "reexports"),
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
    // The root alias and its seven public associated-item paths are all re-exports.
    assert_eq!(overview["data"]["reexports"], 8);

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
    assert_eq!(first["data"]["page"]["returned"], 1);
    assert_eq!(first["data"]["page"]["has_more"], true);
    let cursor = first["data"]["page"]["next_cursor"]
        .as_str()
        .expect("a cursor")
        .to_owned();
    assert!(cursor.starts_with("search2_"));

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
    assert_eq!(empty["data"]["page"]["returned"], 0);
    assert!(!strings(&empty["data"]["searched"]).is_empty());
    let limitations = strings(&empty["coverage"]["limitations"]);
    assert!(
        limitations.iter().any(|l| l.contains("does not establish")),
        "absence is not evidence of absence: {limitations:?}"
    );
}

#[tokio::test]
async fn discovery_keeps_successful_facets_when_one_cursor_is_invalid() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().unwrap();
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let result = call(
        &service,
        "library.overview",
        serde_json::json!({
            "context_id": ctx(&resolved),
            "discovery": [
                {"kind":"documentation", "cursor":"foreign-cursor"},
                {"kind":"features", "max_items":1}
            ]
        }),
    )
    .await;
    assert_eq!(result["status"], "partial", "{result}");
    assert!(result["error"].is_null());
    let facets = result["data"]["discovery"].as_array().unwrap();
    assert_eq!(facets[0]["state"], "failed");
    assert_eq!(facets[0]["diagnostic"]["cause"], "invalid_input");
    assert!(facets[0]["page"].is_null());
    assert_eq!(facets[1]["state"], "available");
    assert_eq!(facets[1]["items"].as_array().unwrap().len(), 1);
    assert!(!result["data"]["namespaces"].as_array().unwrap().is_empty());
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

    // A budget below the full metadata envelope is an explicit error, never oversized JSON.
    let tiny = call(
        &service,
        "artifact.read",
        serde_json::json!({"artifact_id":readme_id,"max_bytes":1024}),
    )
    .await;
    assert_eq!(tiny["error"]["code"], "BUDGET_EXCEEDED");
    assert!(serde_json::to_vec(&tiny).expect("json").len() <= 1024);
    // Page through with a small complete-envelope budget; the pieces reassemble to the whole.
    let mut cursor: Option<String> = None;
    let mut assembled = String::new();
    let mut pages = 0;
    loop {
        let mut params = serde_json::json!({ "artifact_id": readme_id, "max_bytes": 2048 });
        if let Some(c) = &cursor {
            params["cursor"] = serde_json::Value::String(c.clone());
        }
        let page = call(&service, "artifact.read", params).await;
        assert_ne!(page["status"], "error", "{}", page["summary"]);
        assert!(serde_json::to_vec(&page).expect("json").len() <= 2048);
        assert_eq!(page["data"]["encoding"], "utf8");
        assembled.push_str(page["data"]["content"].as_str().expect("text"));
        pages += 1;
        match page["data"]["page"]["next_cursor"].as_str() {
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
        serde_json::json!({ "artifact_id": readme_id, "section": {"kind":"markdown","heading": heading} }),
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
        serde_json::json!({ "artifact_id": tarball["artifact_id"], "max_bytes": 4096 }),
    )
    .await;
    assert_eq!(slice["data"]["encoding"], "base64", "{slice}");
    assert!(slice["data"]["content_digest"].is_string());

    // A read by handle makes no claim about versions, and must not look like one. `source_uri`
    // on the stored record is the FIRST retrieval's -- a file unchanged between two releases
    // hashes identically, so it can name a different release of the same package. Stamping
    // `exact` beside that URI would read as a guarantee the service cannot make; the digest is
    // the guarantee. See register row R-17.
    assert_eq!(
        slice["freshness"]["source_version_match"], "unknown",
        "read_artifact is reached by digest with no version asked about"
    );
    assert!(slice["freshness"]["registry_checked_at"].is_null());

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
    offline_fixture(None).await;
}

#[tokio::test]
async fn workstation_every_tool_answers_offline_from_the_snapshot_published_cold() {
    let config = Config::from_path(&repo_root().join("config/service.workstation.toml"))
        .expect("workstation configuration");
    assert_eq!(config.arrow.memory_bytes, 32 * 1024 * 1024 * 1024);
    assert_eq!(config.arrow.partitions, 16);
    assert_eq!(config.arrow.concurrency, 16);
    offline_fixture(Some(config)).await;
}

/// Manual W11 experiment: actual native operations, fixed fixture, no acceptance latency
/// threshold. The caller records binary/source identities and process resources separately.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore = "manual one/eight-client operation measurement"]
async fn measure_research_operations_one_and_eight_clients() {
    use std::time::Instant;
    let dir = tempfile::tempdir().unwrap();
    let upstream = Upstream::start();
    let mut config = config_for(&upstream);
    let resources =
        Config::from_path(&repo_root().join("config/service.workstation.toml")).unwrap();
    config.arrow = resources.arrow;
    config.limits = resources.limits;
    let service = std::sync::Arc::new(service_with(config, dir.path()));
    let acquisition = Instant::now();
    let resolved = resolve_0_2_0(&service).await;
    let context = ctx(&resolved);
    eprintln!(
        "PLAN13_ACQUISITION {}",
        serde_json::json!({
            "elapsed_micros": acquisition.elapsed().as_micros(), "status":resolved["status"],
            "operations": service.repository.runtime.operation_diagnostics(),
        })
    );
    drop(upstream);
    let workloads = [
        (
            "known_api",
            "symbol.inspect",
            serde_json::json!({"context_id":context,"symbol_path":"enr_fixture::Shape"}),
        ),
        (
            "relationships",
            "symbol.inspect",
            serde_json::json!({"context_id":context,"symbol_path":"enr_fixture::Shape","selection":{"mode":"explicit","aspects":[{"aspect":"relationships","max_items":32}]}}),
        ),
        (
            "search",
            "evidence.search",
            serde_json::json!({"context_id":context,"query":"perimeter"}),
        ),
        (
            "overview",
            "library.overview",
            serde_json::json!({"context_id":context}),
        ),
    ];
    let mut expected = std::collections::BTreeMap::new();
    for clients in [1, 8] {
        for round in 0..3 {
            for (task, method, params) in &workloads {
                let started = Instant::now();
                let mut tasks = tokio::task::JoinSet::new();
                for _ in 0..clients {
                    let service = std::sync::Arc::clone(&service);
                    let params = params.clone();
                    let method = *method;
                    tasks.spawn(async move {
                        let start = Instant::now();
                        let frame = serde_json::json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}).to_string();
                        let response = server::dispatch(&service, &frame).await.response.unwrap();
                        assert!(response.error.is_none(), "{:?}", response.error);
                        let answer = response.result.unwrap();
                        let first_response_micros = start.elapsed().as_micros();
                        let observation = service.repository.runtime.operation_diagnostics().into_iter().find(|v| Some(v.operation_id.as_str()) == answer["request_id"].as_str()).expect("the response owns its native query observations");
                        let (answer, delivery) = complete_answer::complete_answer_measured(&service, answer).await;
                        (start.elapsed().as_micros(), first_response_micros, answer, observation, delivery)
                    });
                }
                let mut samples = Vec::new();
                let mut operations = Vec::new();
                while let Some(result) = tasks.join_next().await {
                    let (elapsed, first_response, answer, observation, delivery) = result.unwrap();
                    assert!(
                        matches!(answer["status"].as_str(), Some("ok" | "partial")),
                        "{answer}"
                    );
                    let digest = enrichment_core::canonical::digest_hex(&answer["data"]);
                    assert_eq!(
                        expected.entry(task).or_insert_with(|| digest.clone()),
                        &digest,
                        "equal results for {task}"
                    );
                    operations.push(observation);
                    samples.push(serde_json::json!({"elapsed_micros":elapsed,"first_response_micros":first_response,"data_digest":digest,"encoded_result_bytes":serde_json::to_vec(&answer).unwrap().len(),"delivery":delivery}));
                }
                assert_eq!(
                    operations.len(),
                    clients,
                    "each request has separately owned observations"
                );
                eprintln!(
                    "PLAN13_OPERATION_MEASUREMENTS {}",
                    serde_json::json!({
                        "task":task,"clients":clients,"round":round,"wall_micros":started.elapsed().as_micros(),
                        "samples":samples,"operations":operations,"scope":"native service dispatch; no MCP transport; retained fixture inputs; first round cold readers",
                    })
                );
            }
        }
    }
}

async fn offline_fixture(resources: Option<Config>) {
    let dir = tempfile::tempdir().expect("dir");
    let upstream = Upstream::start();
    let mut config = config_for(&upstream);
    if let Some(resources) = resources {
        config.arrow = resources.arrow;
        config.limits = resources.limits;
        config.source = resources.source;
    }

    let (context_id, snapshot_id, cold, cold_resolution) = {
        let service = service_with(config.clone(), dir.path());
        let resolved = resolve_0_2_0(&service).await;
        let context_id = ctx(&resolved);
        let snapshot_id = resolved["snapshot_id"].as_str().expect("snap").to_owned();
        let cold = retrieval_round(&service, &context_id, &resolved).await;
        (context_id, snapshot_id, cold, resolved)
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
    assert_eq!(offline["status"], cold_resolution["status"]);
    for field in ["scope", "assessments", "indexed", "missing"] {
        assert_eq!(
            offline["coverage"][field], cold_resolution["coverage"][field],
            "cold/offline coverage {field}"
        );
    }
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
            "context_id": context_id, "symbol_path": "enr_fixture::Shape", "selection": {"mode":"explicit","aspects":[{"aspect":"signature"},{"aspect":"availability"},{"aspect":"source"}]}
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

#[tokio::test]
async fn missing_json_publishes_readable_metadata_and_source() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = call(
        &service,
        "library.resolve",
        serde_json::json!({
            "name": "enr-fixture", "version": "0.1.0"
        }),
    )
    .await;
    let resolved = complete_answer(&service, resolved).await;
    assert_eq!(resolved["status"], "partial", "{resolved}");
    assert!(resolved["snapshot_id"].as_str().is_some());
    let overview = call(
        &service,
        "library.overview",
        serde_json::json!({
            "context_id": ctx(&resolved), "snapshot_id": resolved["snapshot_id"]
        }),
    )
    .await;
    assert_eq!(overview["status"], "partial", "{overview}");
    assert!(overview["data"]["observed_configuration"].is_null());
    assert!(strings(&overview["coverage"]["missing"]).contains(&"public_api".to_owned()));
    let manifest = call(
        &service,
        "snapshot.manifest",
        serde_json::json!({
            "snapshot_id": resolved["snapshot_id"]
        }),
    )
    .await;
    let readme = resolved["data"]["artifacts"]
        .as_array()
        .expect("artifacts")
        .iter()
        .find(|a| a["kind"] == "readme")
        .expect("README")["artifact_id"]
        .clone();
    let read = call(
        &service,
        "artifact.read",
        serde_json::json!({"artifact_id": readme}),
    )
    .await;
    assert_eq!(read["status"], "ok", "{read}");
    assert!(!read["data"]["content"].as_str().expect("text").is_empty());
    let runs = manifest["data"]["producer_runs"].as_array().expect("runs");
    assert!(
        runs.iter()
            .any(|run| run["producer"] == "rust-source-snapshot")
    );
}

#[tokio::test]
async fn invalid_catalog_selection_cannot_redirect_retained_resolution() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let original = resolve_0_2_0(&service).await;
    let context = enrichment_core::identity::ContextId::try_from(ctx(&original)).expect("context");
    let different =
        enrichment_core::identity::SnapshotId::try_from("snap_0123456789abcdef".to_owned())
            .expect("id");
    let original_id = enrichment_core::identity::SnapshotId::try_from(
        original["snapshot_id"]
            .as_str()
            .expect("snapshot")
            .to_owned(),
    )
    .expect("id");
    assert!(
        service
            .repository
            .catalog
            .commit(enrichment_store::catalog_generation::CatalogDelta {
                publication: None,
                selection: Some(enrichment_store::catalog_generation::SelectionChange {
                    context_id: context,
                    snapshot_id: different,
                    expected_base: Some(original_id)
                }),
                ..Default::default()
            })
            .await
            .is_err()
    );
    let replay = call(
        &service,
        "library.resolve",
        serde_json::json!({
            "name": "enr-fixture", "version": "0.2.0", "freshness": "offline"
        }),
    )
    .await;
    assert_eq!(replay["snapshot_id"], original["snapshot_id"]);
    assert_eq!(
        replay["snapshot_id"],
        replay["data"]["snapshot"]["snapshot_id"]
    );
}

#[tokio::test]
async fn corrupted_stored_observations_are_errors_across_retrieval_boundaries() {
    use arrow::array::StringArray;
    use arrow::record_batch::RecordBatch;
    use std::sync::Arc;
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let snapshot_id = enrichment_core::identity::SnapshotId::try_from(
        resolved["snapshot_id"].as_str().expect("id").to_owned(),
    )
    .expect("id");
    let snapshot = service.paths.snapshots().join(snapshot_id.as_str());
    // Deliberate corruption of service-owned test evidence: malformed present fields must
    // never become empty evidence. A real file read follows, not a mocked query Result.
    let path = snapshot.join("fragments.parquet");
    let batches = parquet_read::read_parquet(&path).expect("read");
    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    let mut columns = batch.columns().to_vec();
    let text_index = batch.schema().index_of("text").expect("text column");
    columns[text_index] = Arc::new(StringArray::from(vec![
        Some("tampered content");
        batch.num_rows()
    ]));
    let corrupted = RecordBatch::try_new(batch.schema(), columns).expect("batch");
    parquet_write::write_parquet(&path, &corrupted, &[]).expect("corrupt test snapshot");
    for (method, mut params) in [
        (
            "symbol.inspect",
            serde_json::json!({"symbol_path":"enr_fixture::Widget"}),
        ),
        ("library.overview", serde_json::json!({})),
        (
            "evidence.search",
            serde_json::json!({"query":"Widget", "kinds":["docs"]}),
        ),
    ] {
        params["context_id"] = resolved["context_id"].clone();
        let result = call(&service, method, params).await;
        assert_eq!(result["status"], "error", "{method}: {result}");
        assert_eq!(result["error"]["code"], "QUERY_FAILED");
        assert_eq!(
            result["error"]["diagnostic"]["cause"], "corrupt_state",
            "{result}"
        );
    }
}

// A deterministic comparison fixture derived from real normalized Rust input. Only the
// synthetic release record, declared configuration and a stored release note change.
async fn publish_comparison_variant(
    service: &Service,
    resolved: &serde_json::Value,
    version: &str,
    environment: enrichment_core::identity::Environment,
    note: &str,
    api_complete: bool,
) -> (String, String) {
    use enrichment_core::{
        evidence::{
            Artifact, ArtifactKind, EvidenceFragment, EvidenceKind, FragmentKind,
            ingest::{IngestContext, ProducerBatch},
            snapshot::SnapshotMetadata,
        },
        identity::Context,
        policy::ExecutionProfile,
        producer::{ProducerRun, RunOutcome},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    let original = enrichment_daemon::ops::common::open_context(service, &ctx(resolved), None)
        .await
        .expect("context");
    let mut release = original.release.clone();
    release.key.version = version.into();
    release.release_id = release.key.id();
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        original.context.mode,
    );
    let artifact = service
        .blobs
        .put(note.as_bytes(), |_| {
            Artifact::describe(
                note.as_bytes(),
                ArtifactKind::Changelog,
                "text/plain",
                "fixture:behavior-note",
                "2026-09-14T00:00:00Z",
            )
        })
        .expect("note")
        .acquired;
    let input = original
        .reader
        .inputs_of_kind(ArtifactKind::RustdocJson)
        .await
        .expect("inputs")
        .remove(0);
    let payload =
        String::from_utf8(service.blobs.read(&input.sha256).expect("rustdoc bytes")).expect("utf8");
    let produced = producer_records::collect(
        normalize::prepare(&NormalizeInput {
            payload: &payload,
            rustdoc_artifact_id: &input.artifact_id,
            json_path: Some(&service.blobs.path_for(&input.sha256)),
            summary_chars: 240,
        })
        .expect("prepared fixture producer"),
    )
    .expect("normalize actual Rust fixture");
    let source = original
        .reader
        .artifacts()
        .await
        .expect("acquisitions")
        .into_iter()
        .find(|a| a.artifact_id == input.artifact_id && a.source_uri == input.source_uri)
        .expect("rustdoc source");
    let mut fragments = produced.fragments;
    fragments.push(
        EvidenceFragment::new(
            FragmentKind::ChangelogSection,
            "Behavior",
            &artifact.artifact_id,
            serde_json::json!({"path":"CHANGELOG.md","line":1,"heading":"Behavior"}),
            note.into(),
            EvidenceClass::Declared,
            "comparison-fixture",
            "1",
        )
        .expect("valid fixture locator"),
    );
    let components = fragments
        .iter()
        .map(|f| (f.producer.clone(), f.producer_version.clone()))
        .collect();
    let run = ProducerRun {
        attempt_id: format!("fixture-{version}-{}", environment.environment_id),
        producer: "comparison-fixture".into(),
        producer_version: "1".into(),
        config_digest: "fixture-normalization".into(),
        inputs: [
            ("rustdoc_json".into(), input.sha256),
            ("note".into(), artifact.sha256.clone()),
        ]
        .into_iter()
        .collect(),
        profile: ExecutionProfile::Static,
        started_at: "2026-09-14T00:00:00Z".into(),
        finished_at: "2026-09-14T00:00:01Z".into(),
        outcome: RunOutcome::Succeeded,
        gaps: vec![],
        log: None,
    };
    let evidence = native_ingest::normalize(
        IngestContext {
            ecosystem: release.key.ecosystem,
            symbol_package: produced.source.crate_name.clone(),
            release_id: release.release_id.to_string(),
            environment_id: environment.environment_id.to_string(),
            source_version_match: SourceVersionMatch::Unknown,
            producing_attempt: run.attempt_id.clone(),
            producer_runs: vec![run],
            artifacts: vec![artifact, source],
            component_versions: components,
            indexed: vec![
                EvidenceKind::PublicApi,
                EvidenceKind::Documentation,
                EvidenceKind::ReleaseNotes,
            ],
            missing: vec![],
            gaps: if api_complete {
                vec![]
            } else {
                vec![enrichment_core::evidence::Gap {
                    kind: EvidenceKind::PublicApi,
                    reason: enrichment_core::evidence::GapReason::ExtractionFailed,
                    detail: "Fixture declaration could not be extracted".into(),
                    planned_fallback: None,
                }]
            },
        },
        ProducerBatch {
            symbols: produced.symbols,
            relationships: produced.relationships,
            fragments,
        },
    )
    .expect("typed fixture evidence");
    let published = service
        .repository
        .publish(
            SnapshotMetadata {
                context: context.clone(),
                release,
                environment,
                symbol_package: produced.source.crate_name.clone(),
                crate_name: produced.source.crate_name,
                crate_version: Some(version.into()),
                normalizer_version: original.reader.manifest().normalizer_version.clone(),
                observed_configuration: original.reader.manifest().observed_configuration.clone(),
                producer_items: produced.source.producer_items,
            },
            evidence,
            None,
        )
        .await
        .expect("publish typed fixture");
    (
        context.context_id.to_string(),
        published.snapshot_id.to_string(),
    )
}

#[tokio::test]
async fn comparison_surfaces_behavior_only_release_notes_with_unchanged_rust_api() {
    use enrichment_core::identity::Environment;
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let (before, before_snapshot) = publish_comparison_variant(
        &service,
        &resolved,
        "0.2.1",
        Environment::unspecified(),
        "Whitespace is trimmed.",
        true,
    )
    .await;
    let (after, after_snapshot) = publish_comparison_variant(
        &service,
        &resolved,
        "0.2.2",
        Environment::unspecified(),
        "Whitespace is now preserved, fixing data loss.",
        true,
    )
    .await;
    drop(upstream);
    let result = call(&service,"library.compare",serde_json::json!({"before_context_id":before,"after_context_id":after,"before_snapshot_id":before_snapshot,"after_snapshot_id":after_snapshot,"scopes":["api","release_notes"]})).await;
    assert_ne!(result["status"], "error", "{result}");
    let changes = result["data"]["changes"].as_array().expect("changes");
    assert!(changes.iter().all(|c| c["scope"] != "api"));
    let note = changes
        .iter()
        .find(|c| c["scope"] == "release_notes")
        .expect("behavior note");
    assert!(note["after"].to_string().contains("preserved"));
    let artifact = note["after"][0]["source"]["artifact_id"]
        .as_str()
        .expect("source");
    assert!(service.blobs.find(artifact).expect("read").is_some());
    assert!(!result["data"]["same_release"].as_bool().expect("identity"));
}

#[tokio::test]
async fn partial_api_observations_never_establish_complete_unchanged_api() {
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let (context, _) = publish_comparison_variant(
        &service,
        &resolved,
        "0.2.1",
        enrichment_core::identity::Environment::unspecified(),
        "Same partial API",
        false,
    )
    .await;
    let result = call(
        &service,
        "library.compare",
        serde_json::json!({
            "before_context_id": context, "after_context_id": context, "scopes": ["api"]
        }),
    )
    .await;
    assert_ne!(result["status"], "error", "{result}");
    assert_eq!(result["data"]["total_changes"], 0);
    assert_eq!(result["data"]["api_complete"], false);
    assert_eq!(result["data"]["comparable"], false);
    assert!(
        result["coverage"]["assessments"]
            .as_array()
            .unwrap()
            .iter()
            .any(
                |assessment| assessment["kind"] == "public_api" && assessment["state"] != "indexed"
            )
    );
}

#[tokio::test]
async fn same_release_configuration_differences_are_separate_from_release_changes() {
    use enrichment_core::identity::Environment;
    let upstream = Upstream::start();
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(config_for(&upstream), dir.path());
    let resolved = resolve_0_2_0(&service).await;
    let (before, _) = publish_comparison_variant(
        &service,
        &resolved,
        "0.2.1",
        Environment::declared(
            Some("x86_64-unknown-linux-gnu".into()),
            Some(vec![]),
            Some(true),
        ),
        "Same behavior.",
        true,
    )
    .await;
    let (after, _) = publish_comparison_variant(
        &service,
        &resolved,
        "0.2.1",
        Environment::declared(
            Some("aarch64-unknown-linux-gnu".into()),
            Some(vec!["extra".into()]),
            Some(false),
        ),
        "Same behavior.",
        true,
    )
    .await;
    let result = call(
        &service,
        "library.compare",
        serde_json::json!({"before_context_id":before,"after_context_id":after,"scopes":["api"]}),
    )
    .await;
    assert_eq!(result["status"], "partial", "{result}");
    assert_eq!(result["data"]["same_release"], true);
    assert_eq!(result["data"]["comparable"], false);
    assert!(
        result["data"]["changes"]
            .as_array()
            .expect("changes")
            .is_empty()
    );
    let fields: Vec<_> = result["data"]["configuration_differences"]
        .as_array()
        .expect("differences")
        .iter()
        .map(|d| d["field"].as_str().expect("field"))
        .collect();
    assert!(
        fields.contains(&"target")
            && fields.contains(&"features")
            && fields.contains(&"default_features")
    );
}

#[tokio::test]
async fn escaped_unicode_answers_fit_complete_envelope_and_overflow_is_retrievable() {
    use enrichment_core::evidence::{Artifact, ArtifactKind};
    let dir = tempfile::tempdir().expect("dir");
    let service = service_with(Config::default(), dir.path());
    let text = "\"\\\n\t漢字🙂".repeat(4000);
    let artifact = service
        .blobs
        .put(text.as_bytes(), |_| {
            Artifact::describe(
                text.as_bytes(),
                ArtifactKind::Other,
                "text/plain",
                "fixture:unicode",
                "2026-09-13T00:00:00Z",
            )
        })
        .expect("artifact")
        .artifact;
    let mut cursor = None;
    let mut recovered = String::new();
    loop {
        let page = call(&service,"artifact.read",serde_json::json!({"artifact_id":artifact.artifact_id,"max_bytes":4096,"cursor":cursor})).await;
        assert_ne!(page["status"], "error", "{page}");
        assert!(serde_json::to_vec(&page).expect("json").len() <= 4096);
        recovered.push_str(page["data"]["content"].as_str().expect("text"));
        cursor = page["data"]["page"]["next_cursor"]
            .as_str()
            .map(str::to_owned);
        if cursor.is_none() {
            break;
        }
    }
    assert_eq!(recovered, text);
    let answer = enrichment_daemon::envelope::ok(
        "large answer",
        serde_json::json!({"text":text})
            .as_object()
            .expect("object")
            .clone(),
        enrichment_core::wire::Coverage {
            details: None,
            assessments: Vec::new(),
            scope: "fixture".into(),
            indexed: Default::default(),
            missing: Default::default(),
            limitations: vec![],
        },
    );
    let bounded =
        enrichment_daemon::ops::common::enforce_budget(&service, answer.clone(), Some(1024));
    assert!(serde_json::to_vec(&bounded).expect("json").len() <= 1024);
    assert_eq!(bounded.status(), enrichment_core::wire::Status::Ok);
    let enrichment_core::wire::DeliveryDescriptor::Artifact {
        artifact_id: id, ..
    } = &bounded.delivery
    else {
        panic!("artifact delivery");
    };
    let mut second = answer.clone();
    second.request_id = enrichment_daemon::envelope::new_request_id();
    let second = enrichment_daemon::ops::common::enforce_budget(&service, second, Some(1024));
    assert!(
        matches!(&second.delivery, enrichment_core::wire::DeliveryDescriptor::Artifact { artifact_id, .. } if artifact_id == id)
    );
    let artifact = service.blobs.find(id).expect("lookup").expect("saved");
    let saved: serde_json::Value =
        serde_json::from_slice(&service.blobs.read(&artifact.sha256).expect("read")).expect("JSON");
    assert_eq!(saved["result"]["data"], serde_json::json!(answer.data));
    assert_eq!(saved["result"]["request_id"], "req_retained");
}

#[tokio::test]
async fn job_lookup_distinguishes_unknown_permission_and_corrupt_journals() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().expect("root");
    let service = service_with(Config::default(), root.path());
    let id = "job_00000000000000000000000000000000";
    let request = serde_json::json!({"job_id": id});
    let unknown = call(&service, "job.control", request.clone()).await;
    assert_eq!(
        unknown["error"]["diagnostic"]["cause"], "not_found",
        "{unknown}"
    );
    assert_eq!(unknown["error"]["retryable"], false);
    assert_eq!(
        unknown["error"]["diagnostic"]["actions"][0]["kind"],
        "change_request"
    );
    let journal = root
        .path()
        .join("data/jobs/terminal")
        .join(format!("{id}.json"));
    std::fs::write(&journal, b"{not valid json}").expect("journal fixture");
    std::fs::set_permissions(&journal, std::fs::Permissions::from_mode(0o000)).expect("deny read");
    let denied = call(&service, "job.control", request.clone()).await;
    std::fs::set_permissions(&journal, std::fs::Permissions::from_mode(0o600))
        .expect("restore read");
    assert_eq!(
        denied["error"]["diagnostic"]["cause"], "permission_denied",
        "{denied}"
    );
    assert_eq!(
        denied["error"]["diagnostic"]["actions"][0]["kind"],
        "operator_setup"
    );
    let corrupt = call(&service, "job.control", request).await;
    assert_eq!(
        corrupt["error"]["diagnostic"]["cause"], "corrupt_state",
        "{corrupt}"
    );
    assert_eq!(
        corrupt["error"]["diagnostic"]["actions"][0]["kind"],
        "report_defect"
    );
    for result in [unknown, denied, corrupt] {
        assert_eq!(result["error"]["diagnostic"]["stage"], "job_lookup");
        assert_eq!(result["error"]["diagnostic"]["affected_ids"][0], id);
        assert_eq!(result["error"]["retryable"], false);
    }
}

#[tokio::test]
async fn static_service_reports_the_same_execution_prerequisites_at_every_route() {
    let upstream = Upstream::start();
    let root = tempfile::tempdir().expect("root");
    let service = service_with(config_for(&upstream), root.path());
    let resolved = resolve_0_2_0(&service).await;
    let context = ctx(&resolved);
    let status = call(&service, "service.status", serde_json::json!({})).await;
    let route = status["data"]["sandbox"]["execution_routes"]
        .as_array()
        .expect("routes")
        .iter()
        .find(|route| route["ecosystem"] == "rust" && route["profile"] == "build")
        .expect("rust build route");
    assert_eq!(route["available"], false);
    assert!(
        route["prerequisites"]
            .as_array()
            .expect("prerequisites")
            .contains(&serde_json::json!("enabled_profile"))
    );
    let verify = call(
        &service,
        "usage.verify",
        serde_json::json!({
            "context_id":context, "snippet":"fn main() {}", "mode":"compile", "profile":"build"
        }),
    )
    .await;
    let inspect = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id":context, "symbol_path":"enr_fixture::Shape",
            "selection":{"mode":"explicit","aspects":[{"aspect":"semantics"}]},
            "execution":{"intent":"execute_on_miss", "profile":"build"}
        }),
    )
    .await;
    for answer in [verify, inspect] {
        assert_eq!(answer["status"], "error", "{answer}");
        assert_eq!(
            answer["error"]["diagnostic"]["stage"], "execution_readiness",
            "{answer}"
        );
        assert_eq!(answer["error"]["diagnostic"]["actions"], route["actions"]);
    }
    let retained = call(
        &service,
        "symbol.inspect",
        serde_json::json!({
            "context_id":context, "symbol_path":"enr_fixture::Shape"
        }),
    )
    .await;
    assert_eq!(retained["status"], "ok", "{retained}");
}
