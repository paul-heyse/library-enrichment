use enrichment_core::producer::python::requirements::{MarkerEnvironment, Requirement};
use enrichment_store::{
    dependency_plan::{Frontier, Package, Work, active_demands},
    runtime::{QueryLimits, QueryRuntime},
};

fn package(name: &str, requirements: &[&str]) -> Package {
    Package {
        name: name.into(),
        version: "1.5".into(),
        filename: format!("{name}-1.5-py3-none-any.whl"),
        url: format!("https://example.org/{name}.whl"),
        sha256: "a".repeat(64),
        requirements: requirements.iter().map(|s| (*s).into()).collect(),
        extras: vec![],
        downloaded_bytes: 100,
    }
}

fn environment() -> MarkerEnvironment {
    MarkerEnvironment {
        python_full_version: "3.14.7".into(),
        implementation_version: "3.14.7".into(),
        implementation_name: "cpython".into(),
        os_name: "posix".into(),
        platform_machine: "x86_64".into(),
        platform_system: "Linux".into(),
        platform_python_implementation: "CPython".into(),
        sys_platform: "linux".into(),
    }
}

#[tokio::test]
async fn native_frontier_converges_on_cycles_and_new_extras_and_emits_ordered_lock() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let mut frontier =
        Frontier::new(&runtime, package("root", &["alpha>=1", "alpha<2", "beta"])).unwrap();
    let mut acquired = vec![];
    let mut alpha_expansions = vec![];
    for _ in 0..20 {
        let Some(work) = frontier.next().await.unwrap() else {
            break;
        };
        match work {
            Work::Expand {
                name,
                requirements,
                extras,
            } => {
                if name == "alpha" {
                    alpha_expansions.push(extras.clone());
                }
                let requirements = requirements
                    .iter()
                    .map(|s| Requirement::parse(s).unwrap())
                    .collect::<Vec<_>>();
                let demands = active_demands(&runtime, &requirements, &environment(), &extras)
                    .await
                    .unwrap();
                frontier.expanded(&name, &extras, demands).await.unwrap();
            }
            Work::Acquire {
                name,
                specifiers,
                extras,
            } => {
                let mut selected = match name.as_str() {
                    "alpha" => {
                        assert!(specifiers.contains(">=1") && specifiers.contains("<2"));
                        package(&name, &["root>=1", "gamma; extra == 'fast'"])
                    }
                    "beta" => package(&name, &["alpha[fast]"]),
                    "gamma" => package(&name, &[]),
                    _ => panic!("unexpected dependency: {name}"),
                };
                selected.extras = extras;
                acquired.push(name);
                frontier.acquired(selected).unwrap();
            }
        }
    }
    assert!(frontier.next().await.unwrap().is_none());
    assert_eq!(acquired, ["alpha", "beta", "gamma"]);
    assert_eq!(
        alpha_expansions,
        [Vec::<String>::new(), vec!["fast".into()]]
    );
    let (lock, requirements) = frontier.finish().await.unwrap();
    let lock: serde_json::Value = serde_json::from_slice(&lock).unwrap();
    let names = lock["packages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, ["alpha", "beta", "gamma", "root"]);
    assert_eq!(
        requirements,
        names
            .iter()
            .map(|name| format!("{name}==1.5 --hash=sha256:{}\n", "a".repeat(64)))
            .collect::<String>()
    );
}

#[tokio::test]
async fn native_frontier_refuses_conflicting_retained_choice() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let mut frontier = Frontier::new(&runtime, package("root", &["child>=2"])).unwrap();
    let demands = active_demands(
        &runtime,
        &[Requirement::parse("child>=2").unwrap()],
        &environment(),
        &[],
    )
    .await
    .unwrap();
    frontier.expanded("root", &[], demands).await.unwrap();
    frontier.acquired(package("child", &[])).unwrap();
    let error = frontier.finish().await.unwrap_err().to_string();
    assert!(
        error.contains("dependency_selected_version_conflict"),
        "{error}"
    );
}
