use enrichment_core::{
    evidence::{Artifact, ArtifactKind},
    native_time::AcquisitionTime,
    operation::selections::{ArtifactReadAction, ArtifactWindow},
    request::ReadArtifactRequest,
    wire::research::{ArtifactSection, ResultSectionName},
};
use enrichment_store::{
    artifact_selection,
    runtime::{QueryLimits, QueryRuntime},
};

#[tokio::test]
async fn native_artifact_selection_distinguishes_format_missing_corrupt_and_markdown() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let mut artifact = Artifact::describe(
        b"0123456789",
        ArtifactKind::Other,
        "application/octet-stream",
        "service:fixture",
        AcquisitionTime::now().unwrap(),
    );
    let mut request = ReadArtifactRequest {
        artifact_id: artifact.artifact_id.clone(),
        ..Default::default()
    };
    let plan = artifact_selection::plan(&runtime, &artifact, &request, None)
        .await
        .unwrap();
    assert_eq!(plan.action, ArtifactReadAction::Bytes);
    assert!(!plan.is_text);
    assert_eq!((plan.window.start, plan.window.end), (0, 10));
    request.section = Some(ArtifactSection::Markdown {
        heading: " Title ".into(),
    });
    assert_eq!(
        artifact_selection::plan(&runtime, &artifact, &request, None)
            .await
            .unwrap()
            .action,
        ArtifactReadAction::Unsupported
    );
    artifact.media_type = "text/markdown".into();
    let plan = artifact_selection::plan(&runtime, &artifact, &request, None)
        .await
        .unwrap();
    assert_eq!(plan.action, ArtifactReadAction::Markdown);
    assert_eq!(plan.heading.as_deref(), Some("Title"));
    for heading in [String::new(), " ".into(), "é".repeat(257)] {
        request.section = Some(ArtifactSection::Markdown { heading });
        assert_eq!(
            artifact_selection::plan(&runtime, &artifact, &request, None)
                .await
                .unwrap()
                .action,
            ArtifactReadAction::Unsupported
        );
    }
    request.section = Some(ArtifactSection::Result {
        name: ResultSectionName::Data,
    });
    assert_eq!(
        artifact_selection::plan(&runtime, &artifact, &request, None)
            .await
            .unwrap()
            .action,
        ArtifactReadAction::Missing
    );
    let window = ArtifactWindow {
        start: 4,
        end: 10,
        section: Some("data".into()),
    };
    let plan = artifact_selection::plan(&runtime, &artifact, &request, Some(window.clone()))
        .await
        .unwrap();
    assert_eq!(plan.action, ArtifactReadAction::Bytes);
    assert_eq!(plan.window, window);
    let wrong_section = ArtifactWindow {
        section: Some("coverage".into()),
        ..window.clone()
    };
    assert_eq!(
        artifact_selection::plan(&runtime, &artifact, &request, Some(wrong_section))
            .await
            .unwrap()
            .action,
        ArtifactReadAction::Corrupt
    );
    for (start, end) in [(5, 4), (0, 11)] {
        let bad = ArtifactWindow {
            start,
            end,
            section: Some("data".into()),
        };
        assert_eq!(
            artifact_selection::plan(&runtime, &artifact, &request, Some(bad))
                .await
                .unwrap()
                .action,
            ArtifactReadAction::Corrupt
        );
    }
    let windows = vec![
        ArtifactWindow {
            start: 8,
            end: 10,
            section: Some("TITLE".into()),
        },
        ArtifactWindow {
            start: 0,
            end: 2,
            section: Some("preamble".into()),
        },
        ArtifactWindow {
            start: 3,
            end: 5,
            section: Some("Title".into()),
        },
    ];
    assert_eq!(
        artifact_selection::markdown(&runtime, windows.clone(), "tItLe")
            .await
            .unwrap()
            .unwrap(),
        windows[2]
    );
    assert!(
        artifact_selection::markdown(&runtime, windows, "missing")
            .await
            .unwrap()
            .is_none()
    );
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn native_artifact_slices_bound_unsigned_arithmetic_and_base64_expansion() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let window = ArtifactWindow {
        start: 100,
        end: 200,
        section: None,
    };
    for (text, budget, expected) in [(true, 8, 8), (false, 8, 6), (false, 7, 5), (false, 0, 0)] {
        let plan = artifact_selection::slice(&runtime, &window, 10, budget, text)
            .await
            .unwrap();
        assert_eq!(
            (plan.start, plan.end, plan.read_bytes),
            (110, 110 + expected, expected + 1)
        );
        assert!(!plan.invalid_offset);
    }
    let plan = artifact_selection::slice(&runtime, &window, 100, usize::MAX, true)
        .await
        .unwrap();
    assert_eq!((plan.start, plan.end, plan.read_bytes), (200, 200, 0));
    assert!(!plan.invalid_offset);
    let plan = artifact_selection::slice(&runtime, &window, usize::MAX, usize::MAX, false)
        .await
        .unwrap();
    assert!(plan.invalid_offset);
    let enormous = ArtifactWindow {
        start: usize::MAX - 10,
        end: usize::MAX,
        section: None,
    };
    let plan = artifact_selection::slice(&runtime, &enormous, 5, usize::MAX, false)
        .await
        .unwrap();
    assert_eq!(
        (plan.start, plan.end, plan.read_bytes),
        (usize::MAX - 5, usize::MAX, 5)
    );
    assert!(
        artifact_selection::slice(
            &runtime,
            &ArtifactWindow {
                start: 2,
                end: 1,
                section: None
            },
            0,
            10,
            true
        )
        .await
        .is_err()
    );
    runtime.close_diagnostics().await.unwrap();
}
