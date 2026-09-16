use enrichment_core::registry::{SelectionError, facts};
use enrichment_store::{
    registry::RustIndex,
    runtime::{QueryLimits, QueryRuntime},
};

#[tokio::test]
async fn native_markers_use_supplied_environment_and_independent_extra_rows() {
    use enrichment_core::producer::python::requirements::{MarkerEnvironment, Requirement};
    async fn indices(
        runtime: &QueryRuntime,
        requirements: &[Requirement],
        environment: &MarkerEnvironment,
        extras: &[String],
    ) -> datafusion::error::Result<Vec<u64>> {
        use arrow::array::UInt64Array;
        let frame = enrichment_store::python_registry::active_requirement_plan(
            runtime,
            requirements,
            environment,
            extras,
        )
        .await?;
        let result = runtime.execute(frame).await?;
        Ok(result
            .batches
            .iter()
            .flat_map(|batch| {
                batch
                    .column(0)
                    .as_any()
                    .downcast_ref::<UInt64Array>()
                    .unwrap()
                    .values()
                    .iter()
                    .copied()
            })
            .collect())
    }
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let mut environment = MarkerEnvironment {
        python_full_version: "3.14.7".into(),
        implementation_version: "3.14.7".into(),
        implementation_name: "cpython".into(),
        os_name: "posix".into(),
        platform_machine: "x86_64".into(),
        platform_system: "Linux".into(),
        platform_python_implementation: "CPython".into(),
        sys_platform: "linux".into(),
    };
    let requirements = [
        "Some_Package[fast]>=1.2,<2; python_version >= '3.10' and (sys_platform == 'linux' or extra == 'other')",
        "optional; extra == 'speed'",
        "reverse; '3.10' < python_version",
        "platform; sys_platform == 'win32'",
        "both; extra == 'speed' and extra == 'foo'",
        "different; extra != 'foo'",
        "normalized; extra == 'FAST_mode'",
    ].map(|text| Requirement::parse(text).unwrap());
    assert_eq!(requirements[0].name, "some-package");
    assert_eq!(requirements[0].extras, ["fast"]);
    assert_eq!(
        indices(&runtime, &requirements, &environment, &[])
            .await
            .unwrap(),
        [0, 2, 5]
    );
    // Each requested extra is an independent environment assignment. The conjunction does
    // not become true by combining two different assignments into one marker evaluation.
    assert_eq!(
        indices(
            &runtime,
            &requirements,
            &environment,
            &["speed".into(), "foo".into(), "fast-mode".into()]
        )
        .await
        .unwrap(),
        [0, 1, 2, 5, 6]
    );
    environment.python_full_version = "3.9.0".into();
    environment.sys_platform = "win32".into();
    assert_eq!(
        indices(&runtime, &requirements, &environment, &["foo".into()])
            .await
            .unwrap(),
        [3]
    );
    assert!(Requirement::parse("pkg; extra == 'off' and unknown == 'x'").is_err());
}

#[tokio::test]
async fn native_python_selection_obeys_pep440_environment_and_artifact_policy() {
    use enrichment_core::{
        identity::Ecosystem, producer::python::DistributionFile, request::ResolveRequest,
    };
    use enrichment_store::python_registry;
    use std::collections::BTreeMap;
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let file = |name: &str| DistributionFile {
        filename: name.into(),
        packagetype: "bdist_wheel".into(),
        url: format!("https://example.org/{name}"),
        digests: BTreeMap::from([("sha256".into(), "a".repeat(64))]),
        requires_python: Some(">=3.10".into()),
        yanked: false,
    };
    let mut request = ResolveRequest {
        ecosystem: Ecosystem::Python,
        name: "sample".into(),
        python_version: Some("3.12".into()),
        ..Default::default()
    };
    let mut source = file("sample-1.0.post1.tar.gz");
    source.packagetype = "sdist".into();
    let mut yanked = file("sample-1.1-py3-none-any.whl");
    yanked.yanked = true;
    let releases = BTreeMap::from([
        (
            "1.0.post1".into(),
            vec![source, file("sample-1.0.post1-py3-none-any.whl")],
        ),
        ("1.1".into(), vec![yanked]),
        ("2.0b1".into(), vec![file("sample-2.0b1-py3-none-any.whl")]),
    ]);
    let selected = python_registry::select(&runtime, &releases, &request, None, false)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(selected.version, "1.0.post1");
    assert!(selected.file.filename.ends_with(".whl"));
    request.version = Some("1.0post1".into()); // Equivalent PEP 440 spelling is an exact match.
    assert_eq!(
        python_registry::select(&runtime, &releases, &request, None, false)
            .await
            .unwrap()
            .unwrap()
            .version,
        "1.0.post1"
    );
    request.version = Some("1.1".into());
    assert!(
        python_registry::select(&runtime, &releases, &request, None, false)
            .await
            .unwrap()
            .is_none()
    );
    request.allow_yanked = true;
    assert!(
        python_registry::select(&runtime, &releases, &request, None, false)
            .await
            .unwrap()
            .is_some()
    );
    request.version = None;
    request.allow_yanked = false;
    request.allow_prerelease = true;
    assert_eq!(
        python_registry::select(&runtime, &releases, &request, None, false)
            .await
            .unwrap()
            .unwrap()
            .version,
        "2.0b1"
    );
    request.allow_prerelease = false;
    for (wheel, target, expected) in [
        ("sample-1.0-cp312-cp312-linux_x86_64.whl", None, false),
        (
            "sample-1.0-cp312-cp312-linux_x86_64.whl",
            Some("linux_x86_64"),
            true,
        ),
        (
            "sample-1.0-cp313-cp313-linux_x86_64.whl",
            Some("linux_x86_64"),
            false,
        ),
        (
            "sample-1.0-cp310-abi3-linux_x86_64.whl",
            Some("linux_x86_64"),
            true,
        ),
        ("sample-1.0-py2.py3-none-any.whl", None, true),
    ] {
        request.target = target.map(str::to_owned);
        let inputs = BTreeMap::from([("1.0".into(), vec![file(wheel)])]);
        assert_eq!(
            python_registry::select(&runtime, &inputs, &request, Some(">=1,<2"), true)
                .await
                .unwrap()
                .is_some(),
            expected,
            "{wheel}"
        );
    }
    request.target = None;
    request.python_version = None;
    assert!(
        python_registry::select(&runtime, &releases, &request, Some(">=2"), true)
            .await
            .unwrap()
            .is_none()
    );
    let versions = ["1.0", "1.0.post1", "1!0.1", "1.0b1", "not-a-version"].map(str::to_owned);
    assert_eq!(
        python_registry::ordered_versions(&runtime, &versions, false, 2)
            .await
            .unwrap(),
        ["1!0.1", "1.0.post1"]
    );
}

#[tokio::test]
async fn native_registry_preserves_exact_requests_source_lines_and_nested_acquisition_facts() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let document = [
        r#"{"name":"sample","vers":"0.1.0","cksum":"digest","deps":[{"name":"dep","req":"^1","optional":true}],"features":{"default":["std"]},"features2":{"extended":["dep:dep"]}}"#,
        "",
        r#"{"name":"sample","vers":"0.2.0","cksum":"new"}"#,
        r#"{"name":"sample","vers":"0.3.0","cksum":"yanked","yanked":true}"#,
        r#"{"name":"sample","vers":"0.4.0-beta.2","cksum":"preview"}"#,
        r#"{"name":"sample","vers":"invalid-version","cksum":"invalid"}"#,
    ].join("\n");
    let index = RustIndex::new(&runtime, facts::decode(&document, 2).unwrap()).unwrap();
    let exact = index
        .select(Some("0.1.0"), false, false)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(exact.entry.vers, "0.1.0");
    assert_eq!(exact.source_line, 1);
    assert_eq!(exact.entry.features["default"], ["std"]);
    assert_eq!(exact.entry.features2["extended"], ["dep:dep"]);
    assert!(exact.entry.deps[0].optional);
    assert!(exact.entry.deps[0].default_features);
    assert_eq!(exact.entry.v, 1);
    assert_eq!(exact.upstream.newest_stable.as_deref(), Some("0.2.0"));
    assert_eq!(exact.upstream.newest_any.as_deref(), Some("0.4.0-beta.2"));
    assert!(!exact.upstream.resolved_is_newest_stable);
    assert_eq!(exact.upstream.published_versions, 5);
    for (pre, yanked, expected) in [
        (false, false, "0.2.0"),
        (true, false, "0.4.0-beta.2"),
        (false, true, "0.3.0"),
    ] {
        let selected = index.select(None, pre, yanked).await.unwrap().unwrap();
        assert_eq!(selected.entry.vers, expected);
        if expected == "0.2.0" {
            assert_eq!(selected.source_line, 3);
        }
    }
    assert!(matches!(
        index.select(Some("0.3.0"), false, false).await.unwrap(),
        Err(SelectionError::Yanked { .. })
    ));
    assert!(
        index
            .select(Some("0.3.0"), false, true)
            .await
            .unwrap()
            .is_ok()
    );
    assert!(
        index
            .select(Some("0.4.0-beta.2"), false, false)
            .await
            .unwrap()
            .is_ok()
    );
    assert!(matches!(
        index.select(Some("nonsense"), false, false).await.unwrap(),
        Err(SelectionError::InvalidVersion { .. })
    ));
    match index.select(Some("0.2.5"), false, false).await.unwrap() {
        Err(SelectionError::VersionNotFound { nearest, .. }) => {
            assert_eq!(nearest, ["0.1.0", "0.2.0", "0.3.0", "0.4.0-beta.2"]);
        }
        other => panic!("unexpected {other:?}"),
    }
    let empty = RustIndex::new(&runtime, facts::decode("", 2).unwrap()).unwrap();
    assert!(matches!(
        empty.select(None, false, false).await.unwrap(),
        Err(SelectionError::NoVersions)
    ));
}
