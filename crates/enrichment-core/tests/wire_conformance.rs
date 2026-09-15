//! Acceptance gate C19, Rust leg: "inputs violate wire schema -> rejected consistently through
//! CLI/RPC/MCP boundaries."
//!
//! *Consistently* is the load-bearing word. The three negative cases below are the same three
//! `scripts/schema-conformance.py` builds in `negative_cases()`, so the Rust types and the JSON
//! Schema reject the same documents rather than merely both rejecting something.
//!
//! These are real `#[test]` functions, not doctests: `cargo nextest` does not run doctests, and
//! `docs/reports/logs/nextest.json` is what promotes the gate.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use enrichment_core::wire::{Envelope, EnvelopeError, Outcome, Status, envelope_schema_json};
use serde_json::{Value, json};

const OK_FIXTURE: &str = include_str!("../../../contracts/research-v2/examples/ok.fixture.json");
const PARTIAL_FIXTURE: &str =
    include_str!("../../../contracts/research-v2/examples/partial.fixture.json");
const PENDING_FIXTURE: &str =
    include_str!("../../../contracts/research-v2/examples/pending.fixture.json");
const ERROR_FIXTURE: &str =
    include_str!("../../../contracts/research-v2/examples/error.fixture.json");
const FROZEN_CONTRACT: &str =
    include_str!("../../../contracts/research-v2/research-envelope.schema.json");
const FROZEN_ENUMS: &str = include_str!("../../../contracts/research-v2/enums.json");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/enrichment-core sits two levels below the repository root")
        .to_path_buf()
}

/// The emitted schema, read from disk rather than `include_str!` so this crate still compiles
/// before the first `just schemas-generate`.
fn generated_schema() -> Value {
    let path = repo_root().join("schemas/generated/research-envelope.schema.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "{} is missing ({err}). Run `just schemas-generate`.",
            path.display()
        )
    });
    serde_json::from_str(&text).expect("the generated schema is valid JSON")
}

fn frozen_contract() -> Value {
    serde_json::from_str(FROZEN_CONTRACT).expect("the frozen contract is valid JSON")
}

fn frozen_enums() -> Value {
    serde_json::from_str(FROZEN_ENUMS).expect("the frozen enum extract is valid JSON")
}

/// Deserialize, re-serialize, and compare as `Value` so key order is not asserted -- the frozen
/// contract is hand-ordered and the generated one is lexicographic, both legitimately.
fn assert_round_trips(fixture: &str, expected_status: Status) {
    let envelope: Envelope =
        serde_json::from_str(fixture).expect("fixture deserializes into the wire types");
    assert_eq!(envelope.status(), expected_status);

    let reserialized = serde_json::to_value(&envelope).expect("envelope serializes");
    let original: Value = serde_json::from_str(fixture).expect("fixture is valid JSON");
    assert_eq!(
        reserialized, original,
        "round trip changed the document; a field is missing from the wire types"
    );
}

// --- 1-4: the four delivered fixtures round-trip -----------------------------------------

#[test]
fn ok_fixture_round_trips() {
    assert_round_trips(OK_FIXTURE, Status::Ok);
}

#[test]
fn partial_fixture_round_trips() {
    assert_round_trips(PARTIAL_FIXTURE, Status::Partial);
}

#[test]
fn pending_fixture_round_trips() {
    assert_round_trips(PENDING_FIXTURE, Status::Pending);
}

#[test]
fn error_fixture_round_trips() {
    assert_round_trips(ERROR_FIXTURE, Status::Error);
}

// --- 5-9: negative cases ------------------------------------------------------------------

/// Mutate the ok fixture the way `scripts/schema-conformance.py::negative_cases` does.
fn mutated_ok(mutate: impl FnOnce(&mut serde_json::Map<String, Value>)) -> String {
    let mut doc: Value = serde_json::from_str(OK_FIXTURE).expect("fixture is valid JSON");
    mutate(doc.as_object_mut().expect("the envelope is an object"));
    doc.to_string()
}

fn rejection(document: &str) -> String {
    let err = serde_json::from_str::<Envelope>(document)
        .expect_err("this document must be rejected by the wire types");
    err.to_string()
}

#[test]
fn pending_without_a_job_handle_is_rejected() {
    let doc = mutated_ok(|o| {
        o.insert("status".into(), json!("pending"));
        o.insert("job".into(), Value::Null);
    });
    assert!(
        rejection(&doc).contains(&EnvelopeError::PendingWithoutJob.to_string()),
        "expected the pending-without-job rule to fire"
    );
}

#[test]
fn error_status_without_an_error_object_is_rejected() {
    let doc = mutated_ok(|o| {
        o.insert("status".into(), json!("error"));
        o.insert("error".into(), Value::Null);
    });
    assert!(
        rejection(&doc).contains(&EnvelopeError::ErrorStatusWithoutError.to_string()),
        "expected the error-without-error-object rule to fire"
    );
}

#[test]
fn an_unknown_root_field_is_rejected() {
    let doc = mutated_ok(|o| {
        o.insert("unexpected_root_field".into(), json!(true));
    });
    assert!(
        rejection(&doc).contains("unexpected_root_field"),
        "deny_unknown_fields must reject an unknown root field"
    );
}

#[test]
fn ok_status_with_an_error_object_is_rejected() {
    let doc = mutated_ok(|o| {
        o.insert(
            "error".into(),
            json!({
                "code": "POLICY_DENIED",
                "message": "m",
                "retryable": false,
                "next_action": "n",
                "diagnostic": {"cause":"policy_denied", "stage":"admission",
                    "affected_ids":[], "rule":null, "observed":null, "allowed":null,
                    "correlation_id":null, "actions":[{"kind":"operator_setup","reason":"n"}]}
            }),
        );
    });
    assert!(
        rejection(&doc).contains(&EnvelopeError::NonErrorStatusWithError.to_string()),
        "expected the non-error-status-with-error rule to fire"
    );
}

#[test]
fn pending_status_with_an_error_object_is_rejected() {
    let doc = mutated_ok(|o| {
        o.insert("status".into(), json!("pending"));
        o.insert(
            "job".into(),
            json!({
                "job_id": "job_x",
                "state": "queued",
                "stage": "s",
                "poll_after_ms": 1000
            }),
        );
        o.insert(
            "error".into(),
            json!({
                "code": "POLICY_DENIED",
                "message": "m",
                "retryable": false,
                "next_action": "n",
                "diagnostic": {"cause":"policy_denied", "stage":"admission",
                    "affected_ids":[], "rule":null, "observed":null, "allowed":null,
                    "correlation_id":null, "actions":[{"kind":"operator_setup","reason":"n"}]}
            }),
        );
    });
    assert!(
        rejection(&doc).contains(&EnvelopeError::PendingWithError.to_string()),
        "expected the pending-with-error rule to fire"
    );
}

// --- 10-12: the conditionals as Rust facts, on the construction side ----------------------

#[test]
fn pending_outcome_always_emits_a_job_and_a_null_error() {
    let envelope: Envelope =
        serde_json::from_str(PENDING_FIXTURE).expect("pending fixture deserializes");
    assert!(matches!(envelope.outcome(), Some(Outcome::Pending { .. })));
    assert!(envelope.job().is_some(), "pending must carry a job handle");
    assert!(
        envelope.error().is_none(),
        "pending must not carry an error"
    );
}

#[test]
fn ok_and_partial_outcomes_always_emit_a_null_error() {
    for fixture in [OK_FIXTURE, PARTIAL_FIXTURE] {
        let envelope: Envelope = serde_json::from_str(fixture).expect("fixture deserializes");
        assert!(
            envelope.error().is_none(),
            "ok and partial must not carry an error object"
        );
    }
}

#[test]
fn error_outcome_always_emits_an_error_object() {
    let envelope: Envelope =
        serde_json::from_str(ERROR_FIXTURE).expect("error fixture deserializes");
    assert!(matches!(envelope.outcome(), Some(Outcome::Error { .. })));
    assert!(
        envelope.error().is_some(),
        "error status must carry an error object"
    );
}

// --- 13-21: structure, against contracts/research-v2/enums.json ----------------------------------
//
// 13-17 double as the guard against a `///` doc comment on a unit enum variant: that turns the
// emitted `enum` into `oneOf` + `const`, and these lookups return `None`.

fn enum_at(schema: &Value, pointer: &str) -> Vec<String> {
    let node = schema
        .pointer(pointer)
        .unwrap_or_else(|| panic!("{pointer} is absent; did a unit variant gain a doc comment?"));
    node.as_array()
        .unwrap_or_else(|| panic!("{pointer} is not an array; expected a string enum"))
        .iter()
        .map(|v| v.as_str().expect("enum values are strings").to_owned())
        .collect()
}

fn frozen_list(key: &str) -> Vec<String> {
    frozen_enums()[key]
        .as_array()
        .unwrap_or_else(|| panic!("contracts/research-v2/enums.json has no array `{key}`"))
        .iter()
        .map(|v| v.as_str().expect("values are strings").to_owned())
        .collect()
}

#[test]
fn status_enum_matches_the_frozen_contract() {
    let actual = enum_at(&generated_schema(), "/properties/status/enum");
    assert_eq!(actual, frozen_list("status"));
}

#[test]
fn error_code_enum_matches_the_frozen_contract() {
    let actual = enum_at(&generated_schema(), "/$defs/Error/properties/code/enum");
    assert_eq!(actual, frozen_list("error_codes"));
}

#[test]
fn evidence_class_enum_matches_the_frozen_contract() {
    let actual = enum_at(
        &generated_schema(),
        "/$defs/Evidence/properties/evidence_class/enum",
    );
    assert_eq!(actual, frozen_list("evidence_class"));
}

#[test]
fn source_version_match_enum_matches_the_frozen_contract() {
    let schema = generated_schema();
    let expected = frozen_list("source_version_match");
    // Appears twice in the contract; both sites must agree.
    for pointer in [
        "/$defs/Freshness/properties/source_version_match/enum",
        "/$defs/Evidence/properties/source_version_match/enum",
    ] {
        assert_eq!(enum_at(&schema, pointer), expected, "at {pointer}");
    }
}

#[test]
fn job_state_enum_matches_the_frozen_contract() {
    let actual = enum_at(
        &generated_schema(),
        "/$defs/JobHandle/properties/state/enum",
    );
    assert_eq!(actual, frozen_list("job_state"));
}

#[test]
fn artifact_uri_pattern_matches_the_frozen_contract() {
    let schema = generated_schema();
    let actual = schema
        .pointer("/$defs/ArtifactHandle/properties/uri/pattern")
        .and_then(Value::as_str)
        .expect("the artifact uri carries a pattern");
    assert_eq!(actual, frozen_enums()["artifact_uri_pattern"]);
}

#[test]
fn root_required_fields_match_the_frozen_contract() {
    let schema = generated_schema();
    let mut actual: Vec<String> = schema["required"]
        .as_array()
        .expect("the root has a required list")
        .iter()
        .map(|v| v.as_str().expect("names are strings").to_owned())
        .collect();
    actual.sort();
    assert_eq!(
        actual,
        frozen_list("root_required"),
        "all fourteen root fields must be required; nullability is expressed by type, not by \
         absence. A missing Option field means the schema was generated under the deserialize \
         contract."
    );
}

#[test]
fn defs_match_the_active_contract_definitions() {
    let schema = generated_schema();
    let actual: BTreeSet<String> = schema["$defs"]
        .as_object()
        .expect("the schema has $defs")
        .keys()
        .cloned()
        .collect();
    let expected: BTreeSet<String> = frozen_list("definitions").into_iter().collect();
    assert_eq!(
        actual, expected,
        "an extra $defs entry means a type that should be inlined is not, or a \
         deserialization-only helper leaked into the emitted schema"
    );
}

#[test]
fn root_forbids_additional_properties() {
    assert_eq!(generated_schema()["additionalProperties"], json!(false));
}

// --- 22-25: agreement with the frozen contract, and reproducibility -----------------------

#[test]
fn root_conditionals_match_the_frozen_contract() {
    assert_eq!(
        generated_schema()["allOf"],
        frozen_contract()["allOf"],
        "the emitted root conditionals must restate the frozen contract verbatim; they are what \
         make `pending` require a job handle and `error` require an error object"
    );
}

#[test]
fn every_ref_in_the_schema_resolves() {
    let schema = generated_schema();
    let defs = schema["$defs"].as_object().expect("the schema has $defs");

    fn collect_refs(node: &Value, out: &mut Vec<String>) {
        match node {
            Value::Object(map) => {
                for (key, value) in map {
                    if key == "$ref"
                        && let Some(target) = value.as_str()
                    {
                        out.push(target.to_owned());
                    }
                    collect_refs(value, out);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| collect_refs(item, out)),
            _ => {}
        }
    }

    let mut refs = Vec::new();
    collect_refs(&schema, &mut refs);
    assert!(
        !refs.is_empty(),
        "expected at least the JobHandle reference"
    );
    for target in refs {
        let name = target.strip_prefix("#/$defs/").unwrap_or_else(|| {
            panic!("`{target}` does not point into $defs; only local refs are emitted")
        });
        assert!(
            defs.contains_key(name),
            "`{target}` is dangling. Renaming a wire type, or dropping ErrorDetail's \
             #[schemars(rename = \"Error\")], breaks the hard-coded refs in status_conditionals."
        );
    }
}

#[test]
fn schema_generation_is_deterministic() {
    assert_eq!(
        envelope_schema_json(),
        envelope_schema_json(),
        "emission must be byte-stable; schema-conformance.sh runs `git diff --exit-code` over it"
    );
}

/// Guards the canonicalization in `wire::schema`. This test runs under workspace-wide feature
/// unification, where DataFusion turns on `serde_json/preserve_order` and `serde_json::Map`
/// becomes an insertion-ordered `IndexMap`; `just schemas-generate` builds only
/// `enrichment-core`, where it is a `BTreeMap`. Without explicit sorting the two disagree and
/// the committed file churns depending on which command wrote it.
#[test]
fn every_object_key_is_emitted_in_sorted_order() {
    fn assert_sorted(node: &Value, path: &str) {
        match node {
            Value::Object(map) => {
                let keys: Vec<&String> = map.keys().collect();
                let mut sorted = keys.clone();
                sorted.sort();
                assert_eq!(keys, sorted, "keys are not sorted at `{path}`");
                for (key, value) in map {
                    assert_sorted(value, &format!("{path}/{key}"));
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    assert_sorted(item, &format!("{path}/{index}"));
                }
            }
            _ => {}
        }
    }

    let schema: Value =
        serde_json::from_str(&envelope_schema_json()).expect("emitted schema is valid JSON");
    assert_sorted(&schema, "");
}

#[test]
fn emitted_schema_matches_the_committed_file() {
    let path = repo_root().join("schemas/generated/research-envelope.schema.json");
    let committed = std::fs::read_to_string(&path).expect("the generated schema exists");
    assert_eq!(
        envelope_schema_json(),
        committed,
        "{} is stale. Run `just schemas-generate` and commit both sides together.",
        path.display()
    );
}
