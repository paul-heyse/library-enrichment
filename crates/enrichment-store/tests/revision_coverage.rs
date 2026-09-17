use enrichment_core::{
    archive::{ArchiveOmission, OmittedEntryKind},
    native_union::NativeStruct,
    producer::revision::{RevisionInputs, SourceClosure},
};
use enrichment_store::runtime::{QueryLimits, QueryRuntime};

#[tokio::test]
async fn revision_required_link_is_incomplete() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    for (omitted, missing, expected) in [
        (vec!["repo/CLAUDE.md"], vec![], vec![]),
        (vec!["repo/pkg-extra/NOTICE"], vec![], vec![]),
        (vec!["repo/pkg/NOTICE"], vec![], vec!["repo/pkg/NOTICE"]),
        (vec!["repo/Cargo.toml"], vec![], vec!["repo/Cargo.toml"]),
        (vec!["repo/shared"], vec![], vec!["repo/shared"]),
        (
            vec!["repo/shared/src/linked.rs"],
            vec![],
            vec!["repo/shared/src/linked.rs"],
        ),
        (vec!["repo/shared-extra/linked.rs"], vec![], vec![]),
        (vec![], vec!["repo/pkg/build.rs"], vec![]),
        (
            vec!["repo/pkg/src/linked.rs"],
            vec!["repo/pkg/src/linked.rs"],
            vec!["repo/pkg/src/linked.rs"],
        ),
    ] {
        let inputs = RevisionInputs {
            archive_sha256: "a".repeat(64),
            policy: enrichment_core::archive::REVISION_EXTRACTION_POLICY.into(),
            selected_package: "repo/pkg".into(),
            source_roots: vec!["repo/pkg".into(), "repo/shared".into()],
            declared_inputs: [
                "repo/Cargo.toml",
                "repo/pkg/Cargo.toml",
                "repo/shared/Cargo.toml",
            ]
            .map(str::to_owned)
            .to_vec(),
            missing_inputs: missing.iter().map(|p| (*p).to_owned()).collect(),
            omissions: omitted
                .into_iter()
                .map(|path| ArchiveOmission {
                    path: path.into(),
                    entry_kind: OmittedEntryKind::Symlink,
                    target: "uninterpreted-target".into(),
                    reason: "safe omission".into(),
                })
                .collect(),
        };
        let result = enrichment_store::coverage::assess_revision_inputs(
            &runtime,
            runtime
                .session()
                .read_batch(RevisionInputs::batch(&[inputs]).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(result.affected_omissions, expected);
        assert_eq!(
            result.source_closure,
            if !missing.is_empty() || !expected.is_empty() {
                SourceClosure::Incomplete
            } else {
                // Archive presence is never proof of generated, submodule or LFS closure.
                SourceClosure::Unknown
            }
        );
    }
    let root_inputs = RevisionInputs {
        archive_sha256: "b".repeat(64),
        policy: enrichment_core::archive::REVISION_EXTRACTION_POLICY.into(),
        selected_package: String::new(),
        source_roots: vec![String::new()],
        declared_inputs: vec![],
        missing_inputs: vec![],
        omissions: vec![ArchiveOmission {
            path: "src/link.rs".into(),
            entry_kind: OmittedEntryKind::Symlink,
            target: "../untrusted".into(),
            reason: "link omitted".into(),
        }],
    };
    let root = runtime
        .session()
        .read_batch(RevisionInputs::batch(&[root_inputs]).unwrap())
        .unwrap();
    let result = enrichment_store::coverage::assess_revision_inputs(&runtime, root)
        .await
        .unwrap();
    assert_eq!(result.affected_omissions, ["src/link.rs"]);
    assert_eq!(result.source_closure, SourceClosure::Incomplete);
    runtime.close_diagnostics().await.unwrap();
}
