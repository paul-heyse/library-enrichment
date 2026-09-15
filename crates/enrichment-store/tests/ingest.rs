#[path = "support/producer_records.rs"]
mod producer_records;
use std::collections::BTreeSet;
#[path = "support/native_ingest.rs"]
mod native_ingest;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind, EvidenceFragment, EvidenceKind, FragmentKind,
        ingest::{IngestContext, ProducerBatch},
        relational::{Locator, SubjectRef},
    },
    identity::Ecosystem,
    policy::ExecutionProfile,
    producer::{
        ProducerRun, RunOutcome, normalize as rust_normalize, python, python::ObservationOrigin,
    },
    wire::{EvidenceClass, SourceVersionMatch},
};
use native_ingest::normalize;
fn context(
    artifact: Artifact,
    ecosystem: Ecosystem,
    owner: &str,
    producer: &str,
    role: &str,
) -> IngestContext {
    IngestContext {
        ecosystem,
        symbol_package: owner.into(),
        release_id: "rel_fixture".into(),
        environment_id: "env_fixture".into(),
        source_version_match: SourceVersionMatch::Exact,
        producing_attempt: "attempt-fixture".into(),
        producer_runs: vec![ProducerRun {
            attempt_id: "attempt-fixture".into(),
            producer: producer.into(),
            producer_version: "2".into(),
            config_digest: "configuration".into(),
            inputs: [(role.into(), artifact.sha256.clone())]
                .into_iter()
                .collect(),
            profile: ExecutionProfile::Static,
            started_at: "2026-09-14T00:00:00Z".into(),
            finished_at: "2026-09-14T00:00:01Z".into(),
            outcome: RunOutcome::Succeeded,
            gaps: vec![],
            log: None,
        }],
        artifacts: vec![artifact],
        component_versions: [
            ("rustdoc-json".into(), "2".into()),
            ("griffe-static".into(), python::VERSION.into()),
        ]
        .into_iter()
        .collect(),
        indexed: vec![EvidenceKind::PublicApi, EvidenceKind::Documentation],
        missing: vec![],
        gaps: vec![],
    }
}

#[test]
fn checked_in_rustdoc_normalizes_shared_definitions_and_qualified_members() {
    let payload = include_str!("../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
    let artifact = Artifact::describe(
        payload.as_bytes(),
        ArtifactKind::RustdocJson,
        "application/json",
        "https://docs.rs/enr-fixture/0.1.0.json",
        "2026-09-14T00:00:00Z",
    );
    let produced = producer_records::collect(
        rust_normalize::prepare(&rust_normalize::NormalizeInput {
            payload,
            rustdoc_artifact_id: &artifact.artifact_id,
            json_path: None,
            summary_chars: 240,
        })
        .expect("prepared fixture producer"),
    )
    .expect("rustdoc");
    let context = context(
        artifact,
        Ecosystem::Rust,
        &produced.source.crate_name,
        "rustdoc-json",
        "rustdoc_json",
    );
    let expected_symbols = produced.symbols.len();
    let result = normalize(
        context,
        ProducerBatch {
            symbols: produced.symbols,
            relationships: produced.relationships,
            fragments: produced.fragments,
        },
    )
    .expect("normalized relations");
    assert_eq!(result.symbols.len(), expected_symbols);
    assert!(
        result.definitions.len() < result.symbols.len(),
        "aliases share definitions"
    );
    assert_eq!(result.api_observations.len(), result.symbols.len());
    assert!(
        result.symbols.iter().any(|s| s.qualifier.is_some()),
        "trait members retain qualifiers"
    );
    assert!(
        result
            .fragments
            .iter()
            .all(|f| matches!(f.source.locator, Locator::RustdocItem { .. }))
    );
    assert!(result.input_artifacts.iter().all(|a| a.validate().is_ok()));
}

#[test]
fn python_source_and_stub_survive_and_inferred_parents_are_not_api_facts() {
    let artifact = Artifact::describe(
        b"worker JSON",
        ArtifactKind::Other,
        "application/json",
        "worker://fixture",
        "2026-09-14T00:00:00Z",
    );
    let mut observations = vec![];
    for (origin, file, signature) in [
        (
            ObservationOrigin::Source,
            "pkg/inner.py",
            "f(x: int) -> int",
        ),
        (ObservationOrigin::Stub, "pkg/inner.pyi", "f(x: str) -> str"),
    ] {
        observations.push(python::Observation {
            path: "pkg.inner.f".into(),
            kind: "function".into(),
            origin,
            file: file.into(),
            line: Some(2),
            signature: Some(signature.into()),
            overloads: vec![],
            docs: Some("A function".into()),
            alias_target: None,
            bases: vec![],
            publicness: python::Publicness::default(),
        });
    }
    let raw = python::WorkerResponse {
        schema_version: "1.0".into(),
        griffe_version: "2.3.0".into(),
        worker_python: "3.14.7".into(),
        observations,
        processed_files: vec!["pkg/inner.py".into(), "pkg/inner.pyi".into()],
        gaps: vec![],
    };
    let produced = producer_records::collect(
        python::normalize::prepare("demo-lib", raw.clone(), &artifact.artifact_id)
            .expect("prepared Python fixture"),
    )
    .expect("producer");
    assert_eq!(
        produced.symbols.len(),
        1,
        "namespace parents are derived navigation only"
    );
    let context = context(
        artifact,
        Ecosystem::Python,
        "python:demo-lib",
        "python-static",
        "worker",
    );
    let result = normalize(
        context,
        ProducerBatch {
            symbols: produced.symbols,
            relationships: produced.relationships,
            fragments: produced.fragments,
        },
    )
    .expect("typed relations");
    assert_eq!(result.symbols.len(), 1);
    assert_eq!(result.api_observations.len(), 2);
    assert_ne!(
        result.api_observations[0].origin,
        result.api_observations[1].origin
    );
    assert_ne!(
        result.api_observations[0].payload.signature,
        result.api_observations[1].payload.signature
    );
    assert!(
        result
            .fragments
            .iter()
            .all(|f| matches!(f.source.locator, Locator::PythonDeclaration { .. }))
    );
}

#[test]
fn malformed_or_unbound_producer_evidence_is_rejected() {
    let artifact = Artifact::describe(
        b"readme",
        ArtifactKind::Readme,
        "text/markdown",
        "https://example.org/README.md",
        "2026-09-14T00:00:00Z",
    );
    let fragment = EvidenceFragment::new(
        FragmentKind::ReadmeSection,
        "p",
        &artifact.artifact_id,
        serde_json::json!({"path":"README.md","heading":"p","line":1}),
        "Documentation".into(),
        EvidenceClass::Declared,
        "source",
        "1",
    )
    .expect("valid fixture locator");
    let mut context = context(artifact, Ecosystem::Rust, "p", "source", "readme");
    context.producer_runs[0].inputs.clear();
    assert!(
        normalize(
            context,
            ProducerBatch {
                fragments: vec![fragment],
                ..ProducerBatch::default()
            }
        )
        .is_err()
    );
}

#[test]
fn python_kind_alternatives_and_ambiguous_aliases_do_not_depend_on_worker_order() {
    let declaration = |path: &str, kind: &str, origin, alias: Option<&str>| python::Observation {
        path: path.into(),
        kind: kind.into(),
        origin,
        file: if origin == python::ObservationOrigin::Source {
            "pkg.py"
        } else {
            "pkg.pyi"
        }
        .into(),
        line: Some(1),
        signature: None,
        overloads: vec![],
        docs: Some(format!("{kind} declaration")),
        alias_target: alias.map(String::from),
        bases: vec![],
        publicness: python::Publicness::default(),
    };
    let mut raw = python::WorkerResponse {
        schema_version: "1.0".into(),
        griffe_version: "2.3.0".into(),
        worker_python: "3.14.7".into(),
        observations: vec![
            declaration("pkg.item", "class", python::ObservationOrigin::Source, None),
            declaration(
                "pkg.item",
                "attribute",
                python::ObservationOrigin::Stub,
                None,
            ),
            declaration(
                "pkg.alias",
                "alias",
                python::ObservationOrigin::Source,
                Some("pkg.item"),
            ),
        ],
        processed_files: vec!["pkg.py".into(), "pkg.pyi".into()],
        gaps: vec![],
    };
    let artifact = Artifact::describe(
        b"worker result",
        ArtifactKind::Other,
        "application/json",
        "worker:fixture",
        "2026-09-14T00:00:00Z",
    );
    let mut expected_ids = None;
    for _ in 0..2 {
        let produced = producer_records::collect(
            python::normalize::prepare("demo", raw.clone(), &artifact.artifact_id)
                .expect("prepared Python fixture"),
        )
        .expect("normalize variants");
        assert_eq!(
            produced.source.unresolved, 1,
            "conflicting target kinds cannot pick a winner"
        );
        assert_eq!(produced.symbols.len(), 3);
        let result = normalize(
            context(
                artifact.clone(),
                Ecosystem::Python,
                "python:demo",
                "python-static",
                "worker",
            ),
            ProducerBatch {
                symbols: produced.symbols,
                relationships: produced.relationships,
                fragments: produced.fragments,
            },
        )
        .expect("typed variant subjects");
        assert_eq!(result.api_observations.len(), 3);
        let ids = result
            .symbols
            .iter()
            .map(|s| s.symbol_id.clone())
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), 3);
        if let Some(expected) = &expected_ids {
            assert_eq!(&ids, expected);
        } else {
            expected_ids = Some(ids);
        }
        assert!(result.fragments.iter().all(|f| matches!(&f.subject, SubjectRef::Symbol { symbol_id } if result.symbols.iter().any(|s| &s.symbol_id == symbol_id))));
        raw.observations.reverse();
    }
    raw.observations[0].kind = "unknown_kind".into();
    assert!(python::normalize::prepare("demo", raw, &artifact.artifact_id).is_err());
}
