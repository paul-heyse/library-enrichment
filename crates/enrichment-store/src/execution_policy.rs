//! One native qualification and execution-route policy over captured physical facts.
use crate::{registry::rows, runtime::QueryRuntime};
use arrow::{
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::{SessionContext, col, lit},
};
use enrichment_core::native_union::{NativeStruct, Rule};
use enrichment_core::{
    config::{Config, ExecutionResources},
    execution::{
        ProcessObservation,
        facts::{RESOURCE_PROBE, Receipt, ResourceProbe, resource_fields},
    },
    identity::Ecosystem,
    policy::ExecutionProfile,
    wire::status::ExecutionReadiness,
};
use std::{collections::BTreeMap, sync::Arc};

/// Captured mechanism outputs, including failures as facts rather than readiness decisions.
pub struct Capture {
    pub execution_root: String,
    pub containment_identity: Option<String>,
    pub containment_error: Option<String>,
    pub receipt: Option<Receipt>,
    pub receipt_error: Option<String>,
    pub cleanup_error: Option<String>,
}

pub struct Policy {
    runtime: QueryRuntime,
    session: SessionContext,
    policy_id: String,
}

enrichment_core::native_struct! {
pub struct Qualification {
    qualified: bool => enrichment_core::native_union::Rule::Text,
    detail: String => enrichment_core::native_union::Rule::Text,
}
}

fn text(name: &str, nullable: bool) -> Field {
    Field::new(name, DataType::Utf8, nullable)
}

fn register(session: &SessionContext, name: &str, batch: RecordBatch) -> Result<()> {
    session.register_table(name, session.read_batch(batch)?.into_view())?;
    Ok(())
}

// Exact kernel output parsing is expressed with native regexp extraction, TRY_CAST and
// checked decimal arithmetic. Malformed/unlimited/overflow values remain null and cannot grant.
const PARSE: &str = r#"
WITH captured AS (
 SELECT *, regexp_match(stdout,
   '\A([0-9]+) ([0-9]+)\n([0-9]+)\n([0-9]+)\n([0-9]+)\n([^ ]+) ([0-9]+) ([0-9]+)\n?\z') AS parts
 FROM resource_input
)
SELECT *,
 TRY_CAST(parts[1] AS BIGINT UNSIGNED) AS cpu_quota_micros,
 TRY_CAST(parts[2] AS BIGINT UNSIGNED) AS cpu_period_micros,
 TRY_CAST(parts[3] AS BIGINT UNSIGNED) AS memory_bytes,
 TRY_CAST(parts[4] AS BIGINT UNSIGNED) AS swap_bytes,
 TRY_CAST(parts[5] AS INT UNSIGNED) AS pids,
 parts[6] AS scratch_filesystem,
 TRY_CAST(TRY_CAST(parts[7] AS DECIMAL(20,0)) * TRY_CAST(parts[8] AS DECIMAL(20,0)) AS BIGINT UNSIGNED) AS scratch_bytes
FROM captured
"#;

fn resource_input<'a>(
    entries: impl IntoIterator<Item = (&'a str, Option<&'a ProcessObservation>)>,
) -> Result<RecordBatch> {
    enrichment_core::native_struct! {
        struct Raw {
            ecosystem: String => Rule::Text,
            stdout: Option<String> => Rule::Text,
            image_id: Option<String> => Rule::Text,
            command: Option<Vec<String>> => Rule::Sequence,
            end: Option<enrichment_core::execution::ProcessEnd> => Rule::Text,
            exit_code: Option<i32> => Rule::Text,
            cleanup_confirmed: Option<bool> => Rule::Text,
            operation_id: Option<String> => Rule::Text,
            qualification_id: Option<String> => Rule::Text,
        }
    }
    let input: Vec<_> = entries
        .into_iter()
        .map(|(ecosystem, process)| Raw {
            ecosystem: ecosystem.into(),
            stdout: process.map(|p| p.stdout.clone()),
            image_id: process.map(|p| p.image_id.clone()),
            command: process.map(|p| p.command.clone()),
            end: process.map(|p| p.end),
            exit_code: process.and_then(|p| p.exit_code),
            cleanup_confirmed: process.map(|p| p.cleanup_confirmed),
            operation_id: process.map(|p| p.operation_id.clone()),
            qualification_id: process.and_then(|p| match &p.authority {
                enrichment_core::execution::ProcessAuthority::Qualification { definition_id } => {
                    Some(definition_id.clone())
                }
                enrichment_core::execution::ProcessAuthority::Command { .. } => None,
            }),
        })
        .collect();
    Ok(Raw::batch(&input)?)
}

impl Policy {
    pub async fn bind(runtime: &QueryRuntime, config: &Config, captured: Capture) -> Result<Self> {
        let session = runtime.session();
        let requested = config.execution.resources()?;
        enrichment_core::native_struct! {
            struct State {
                execution_root: String => Rule::Text,
                containment_identity: Option<String> => Rule::Text,
                containment_error: Option<String> => Rule::Text,
                receipt_root: Option<String> => Rule::Text,
                receipt_identity: Option<String> => Rule::Text,
                qualified_at: Option<String> => Rule::Text,
                receipt_error: Option<String> => Rule::Text,
                cleanup_error: Option<String> => Rule::Text,
                requested: ExecutionResources => Rule::Text,
            }
        }
        let receipt = captured.receipt.as_ref();
        register(
            &session,
            "policy_state",
            State::batch(&[State {
                execution_root: captured.execution_root.clone(),
                containment_identity: captured.containment_identity.clone(),
                containment_error: captured.containment_error.clone(),
                receipt_root: receipt.map(|r| r.execution_root.clone()),
                receipt_identity: receipt.map(|r| r.containment_identity.clone()),
                qualified_at: receipt.map(|r| r.qualified_at.clone()),
                receipt_error: captured.receipt_error.clone(),
                cleanup_error: captured.cleanup_error.clone(),
                requested: requested.clone(),
            }])?,
        )?;
        enrichment_core::native_struct! {
            struct Image {
                ecosystem: String => Rule::Text,
                configured_image: Option<String> => Rule::Text,
                receipt_image: Option<String> => Rule::Text,
                requested: Option<ExecutionResources> => Rule::Text,
                observed: Option<ExecutionResources> => Rule::Text,
                filesystem: Option<String> => Rule::Text,
            }
        }
        let images = [
            ("rust", config.execution.rust_image.as_deref()),
            ("python", config.execution.python_image.as_deref()),
        ];
        let inputs: Vec<_> = images
            .iter()
            .map(|(ecosystem, image)| {
                let resource = receipt.and_then(|r| r.resources.get(*ecosystem));
                Image {
                    ecosystem: (*ecosystem).into(),
                    configured_image: image.map(str::to_owned),
                    receipt_image: receipt.and_then(|r| r.images.get(*ecosystem)).cloned(),
                    requested: resource.map(|r| r.requested.clone()),
                    observed: resource.map(|r| r.observed.clone()),
                    filesystem: resource.map(|r| r.scratch_filesystem.clone()),
                }
            })
            .collect();
        register(&session, "image_input", Image::batch(&inputs)?)?;
        register(
            &session,
            "resource_input",
            resource_input(images.iter().map(|(eco, _)| {
                (
                    *eco,
                    receipt
                        .and_then(|r| r.resources.get(*eco))
                        .map(|r| &r.process),
                )
            }))?,
        )?;
        session.register_table("kernel_resources", session.sql(PARSE).await?.into_view())?;
        let profiles = RecordBatch::try_new(
            Arc::new(Schema::new(vec![text("profile", false)])),
            vec![Arc::new(arrow::array::StringArray::from(
                config.policy.enabled_profiles.clone(),
            ))],
        )?;
        register(&session, "enabled_profiles", profiles)?;
        let resource_equality = resource_fields().iter().map(|f| format!(
            "k.{} = s.requested['{}'] AND i.requested['{}'] = s.requested['{}'] AND i.observed['{}'] = k.{}",
            f.name(), f.name(), f.name(), f.name(), f.name(), f.name()
        )).collect::<Vec<_>>().join(" AND ");
        let image_plan = session.sql(&format!(r#"
        SELECT i.ecosystem, i.configured_image,
          CASE WHEN regexp_like(i.configured_image, '^sha256:[0-9a-f]{{64}}$') THEN i.configured_image END AS image_id,
          s.qualified_at,
          CASE
            WHEN i.configured_image IS NULL THEN 'No producer image is configured; run just execution-images --apply.'
            WHEN NOT regexp_like(i.configured_image, '^sha256:[0-9a-f]{{64}}$') THEN 'Configure an immutable sha256 producer image.'
            WHEN s.receipt_error IS NOT NULL THEN s.receipt_error
            WHEN s.receipt_identity IS NULL THEN 'No qualification receipt is available; run just execution-qualify --apply.'
            WHEN s.receipt_root IS DISTINCT FROM s.execution_root THEN concat('The qualification receipt names another execution root: ', s.receipt_root)
            WHEN s.containment_error IS NOT NULL THEN s.containment_error
            WHEN s.receipt_identity IS DISTINCT FROM s.containment_identity THEN 'The helper, broker, execution contract or effective limits changed; run just execution-qualify --apply.'
            WHEN TRY_CAST(s.qualified_at AS TIMESTAMP) IS NULL THEN 'Qualification receipt has no valid observation time.'
            WHEN i.receipt_image IS DISTINCT FROM i.configured_image THEN concat('The qualification receipt covers ', coalesce(i.receipt_image, 'no image'), ' for ', i.ecosystem, '; configured image is ', i.configured_image, '.')
            WHEN NOT coalesce(k.end = 'exited' AND k.exit_code = 0 AND k.cleanup_confirmed AND k.operation_id=k.qualification_id
                 AND k.command = make_array('/bin/sh', '-c', $1)
                 AND k.image_id = i.configured_image AND k.scratch_filesystem = 'tmpfs'
                 AND i.filesystem = k.scratch_filesystem AND {resource_equality}, false)
              THEN concat(i.ecosystem, ' resource qualification disagrees with the kernel observation, image or effective request; run just execution-qualify --apply.')
          END AS reason
        FROM image_input i CROSS JOIN policy_state s LEFT JOIN kernel_resources k ON i.ecosystem = k.ecosystem
        "#)).await?.with_param_values(vec![datafusion::common::ScalarValue::Utf8(Some(RESOURCE_PROBE.into()))])?;
        session.register_table("qualified_images", image_plan.into_view())?;
        Ok(Self {
            runtime: runtime.clone(),
            session,
            policy_id: enrichment_core::native_key::Key::OperationPolicy.record(config)?,
        })
    }

    pub fn identity(&self) -> &str {
        &self.policy_id
    }

    /// Acquisition and contained execution consume the same configured profile relation.
    pub async fn command_routes(&self) -> Result<DataFrame> {
        let route = self.route_plan().await?;
        let execution = route.select_columns(&["ecosystem", "profile", "available", "image_id"])?;
        execution.union(self.session.sql("SELECT i.ecosystem, 'static' AS profile, p.available, CAST(NULL AS VARCHAR) AS image_id FROM image_input i CROSS JOIN (SELECT count(*)>0 AS available FROM enabled_profiles WHERE profile='static') p").await?)
    }

    /// A finite precondition relation, shared by status and every executing route.
    pub async fn route_plan(&self) -> Result<DataFrame> {
        self.session.sql(r#"
        WITH routes AS (
          SELECT i.*, p.profile FROM qualified_images i CROSS JOIN (VALUES ('build'), ('runtime')) p(profile)
        ), unmet AS (
          SELECT r.ecosystem, r.profile, 1 AS ordinal, 'enabled_profile' AS prerequisite,
             concat('Enable the ', r.profile, ' execution profile in operator configuration.') AS reason
          FROM routes r LEFT ANTI JOIN enabled_profiles p ON p.profile = r.profile
          UNION ALL SELECT ecosystem, profile, 2, 'immutable_image', 'Configure an immutable producer image with just execution-images --apply.' FROM routes WHERE image_id IS NULL
          UNION ALL SELECT ecosystem, profile, 3, 'qualification', reason FROM routes WHERE reason IS NOT NULL
          UNION ALL SELECT ecosystem, profile, 4, 'cleanup', cleanup_error FROM routes CROSS JOIN policy_state WHERE cleanup_error IS NOT NULL
        ), grouped AS (
          SELECT ecosystem, profile, array_agg(prerequisite ORDER BY ordinal) AS prerequisites,
             array_agg(named_struct('kind', 'operator_setup', 'reason', reason) ORDER BY ordinal) AS actions
          FROM unmet GROUP BY ecosystem, profile
        )
        SELECT r.ecosystem, r.profile, g.ecosystem IS NULL AS available, r.image_id,
           coalesce(g.prerequisites, []) AS prerequisites, coalesce(g.actions, []) AS actions
        FROM routes r LEFT JOIN grouped g ON r.ecosystem = g.ecosystem AND r.profile = g.profile
        ORDER BY r.ecosystem, r.profile
        "#).await
    }

    pub async fn routes(&self) -> Result<Vec<ExecutionReadiness>> {
        rows(&self.runtime, self.route_plan().await?, 4).await
    }

    pub async fn assess(
        &self,
        ecosystem: Ecosystem,
        profile: ExecutionProfile,
    ) -> Result<ExecutionReadiness> {
        let name = match ecosystem {
            Ecosystem::Rust => "rust",
            Ecosystem::Python => "python",
        };
        let frame = self.route_plan().await?.filter(
            col("ecosystem")
                .eq(lit(name))
                .and(col("profile").eq(lit(profile.as_str()))),
        )?;
        rows(&self.runtime, frame, 1)
            .await?
            .pop()
            .ok_or_else(|| DataFusionError::Plan("unknown native execution route".into()))
    }

    pub async fn qualification(&self) -> Result<Qualification> {
        rows(&self.runtime, self.session.sql(r#"
            SELECT count(*) FILTER (WHERE configured_image IS NOT NULL) > 0
              AND count(*) FILTER (WHERE configured_image IS NOT NULL AND reason IS NOT NULL) = 0 AS qualified,
              coalesce(string_agg(reason, ' ' ORDER BY ecosystem) FILTER (WHERE configured_image IS NOT NULL),
                CASE WHEN count(*) FILTER (WHERE configured_image IS NOT NULL) = 0
                THEN 'No producer image is configured; run just execution-images --apply.'
                ELSE concat('Qualified ', max(qualified_at), ' by an actual containment run.') END) AS detail
            FROM qualified_images
        "#).await?, 1).await?.pop().ok_or_else(|| DataFusionError::Internal("qualification aggregate has no row".into()))
    }

    pub async fn admitted_images(&self) -> Result<BTreeMap<String, String>> {
        enrichment_core::native_struct! {
        struct Row {
            ecosystem: String => enrichment_core::native_union::Rule::Text,
            image_id: String => enrichment_core::native_union::Rule::Text,
        }
        }
        let rows: Vec<Row> = rows(&self.runtime, self.session.sql("SELECT ecosystem, image_id FROM qualified_images WHERE reason IS NULL ORDER BY ecosystem").await?, 2).await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.ecosystem, r.image_id))
            .collect())
    }
}

/// The operator qualification path uses the same native parser and resource equality contract.
pub async fn resource_probe(
    runtime: &QueryRuntime,
    requested: ExecutionResources,
    process: ProcessObservation,
) -> Result<ResourceProbe> {
    let session = runtime.session();
    register(
        &session,
        "resource_input",
        resource_input([("probe", Some(&process))])?,
    )?;
    session.register_table("kernel_resources", session.sql(PARSE).await?.into_view())?;
    register(
        &session,
        "requested",
        ExecutionResources::batch(std::slice::from_ref(&requested))?,
    )?;
    let same = resource_fields()
        .iter()
        .map(|f| format!("k.{} = r.{}", f.name(), f.name()))
        .collect::<Vec<_>>()
        .join(" AND ");
    let check = session.sql(&format!("SELECT 'kernel resources differ from request or probe did not complete with confirmed cleanup' AS witness FROM kernel_resources k CROSS JOIN requested r WHERE NOT coalesce(k.end = 'exited' AND k.exit_code = 0 AND k.cleanup_confirmed AND k.operation_id=k.qualification_id AND k.scratch_filesystem = 'tmpfs' AND k.command = make_array('/bin/sh', '-c', $1) AND {same}, false)")).await?.with_param_values(vec![datafusion::common::ScalarValue::Utf8(Some(RESOURCE_PROBE.into()))])?;
    runtime
        .require_empty(check, "execution_resource_qualification", "qualification")
        .await?;
    let observed: ExecutionResources = rows(runtime, session.sql("SELECT cpu_quota_micros, cpu_period_micros, memory_bytes, swap_bytes, scratch_bytes, pids FROM kernel_resources").await?, 1).await?.pop().ok_or_else(|| DataFusionError::Execution("resource observation missing".into()))?;
    Ok(ResourceProbe {
        requested,
        observed,
        scratch_filesystem: "tmpfs".into(),
        process,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{execution::ProcessEnd, wire::status::ExecutionPrerequisite};

    fn process() -> ProcessObservation {
        ProcessObservation {
            operation_id: "fixture-resource-operation".into(),
            authority: enrichment_core::execution::ProcessAuthority::Qualification {
                definition_id: "fixture-resource-operation".into(),
            },
            image_id: format!("sha256:{}", "a".repeat(64)),
            command: vec!["/bin/sh".into(), "-c".into(), RESOURCE_PROBE.into()],
            started_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-16T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
            finished_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-16T00:00:01.000000Z".to_owned(),
            )
            .unwrap(),
            exit_code: Some(0),
            end: ProcessEnd::Exited,
            stdout: "200000 100000\n1073741824\n0\n128\ntmpfs 131072 4096\n".into(),
            stderr: String::new(),
            cleanup_confirmed: true,
        }
    }

    fn config() -> Config {
        let mut config = Config::default();
        config.execution.rust_image = Some(process().image_id);
        config.execution.python_image = Some(format!("sha256:{}", "b".repeat(64)));
        config.policy.enabled_profiles = vec!["static".into(), "build".into()];
        config
    }

    fn capture(config: &Config) -> Capture {
        let requested = config.execution.resources().unwrap();
        Capture {
            execution_root: "/owned/engine".into(),
            containment_identity: Some("qualified-physical-helper".into()),
            containment_error: None,
            receipt_error: None,
            cleanup_error: None,
            receipt: Some(Receipt {
                containment_identity: "qualified-physical-helper".into(),
                qualified_at: "2026-09-16T00:00:01Z".into(),
                execution_root: "/owned/engine".into(),
                images: [("rust".into(), process().image_id)].into(),
                tools: BTreeMap::new(),
                resources: [(
                    "rust".into(),
                    ResourceProbe {
                        observed: requested.clone(),
                        requested,
                        scratch_filesystem: "tmpfs".into(),
                        process: process(),
                    },
                )]
                .into(),
            }),
        }
    }

    #[tokio::test]
    async fn native_routes_share_policy_without_coupling_independent_images() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), Default::default()).unwrap();
        let config = config();
        let policy = Policy::bind(&runtime, &config, capture(&config))
            .await
            .unwrap();
        let rust = policy
            .assess(Ecosystem::Rust, ExecutionProfile::Build)
            .await
            .unwrap();
        assert!(rust.available, "{rust:?}");
        assert!(
            !policy
                .assess(Ecosystem::Python, ExecutionProfile::Build)
                .await
                .unwrap()
                .available
        );
        let runtime_route = policy
            .assess(Ecosystem::Rust, ExecutionProfile::Runtime)
            .await
            .unwrap();
        assert_eq!(
            runtime_route.prerequisites,
            [ExecutionPrerequisite::EnabledProfile]
        );
        assert_eq!(
            policy
                .admitted_images()
                .await
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            [&"rust"]
        );
        assert!(!policy.qualification().await.unwrap().qualified);
        let mut captured = capture(&config);
        captured.cleanup_error = Some("owned container absence remains unresolved".into());
        let quarantined = Policy::bind(&runtime, &config, captured)
            .await
            .unwrap()
            .assess(Ecosystem::Rust, ExecutionProfile::Build)
            .await
            .unwrap();
        assert_eq!(quarantined.prerequisites, [ExecutionPrerequisite::Cleanup]);
    }

    #[tokio::test]
    async fn raw_observation_and_current_contract_are_required_for_a_grant() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), Default::default()).unwrap();
        let config = config();
        for fault in [
            "requested",
            "observed",
            "raw",
            "process",
            "command",
            "image",
            "root",
            "identity",
        ] {
            let mut captured = capture(&config);
            let receipt = captured.receipt.as_mut().unwrap();
            let probe = receipt.resources.get_mut("rust").unwrap();
            match fault {
                "requested" => probe.requested.memory_bytes += 1,
                "observed" => probe.observed.scratch_bytes += 1,
                "raw" => probe.process.stdout = probe.process.stdout.replace("200000", "max"),
                "process" => probe.process.cleanup_confirmed = false,
                "command" => probe.process.command[2] = "printf fabricated".into(),
                "image" => probe.process.image_id = "sha256:aa".into(),
                "root" => receipt.execution_root = "/other/engine".into(),
                "identity" => receipt.containment_identity = "changed helper".into(),
                _ => unreachable!(),
            }
            let assessed = Policy::bind(&runtime, &config, captured)
                .await
                .unwrap()
                .assess(Ecosystem::Rust, ExecutionProfile::Build)
                .await
                .unwrap();
            assert_eq!(
                assessed.prerequisites,
                [ExecutionPrerequisite::Qualification],
                "{fault}: {assessed:?}"
            );
        }
    }

    #[tokio::test]
    async fn operator_resource_receipts_use_the_same_native_kernel_observations() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), Default::default()).unwrap();
        let requested = config().execution.resources().unwrap();
        let good = resource_probe(&runtime, requested.clone(), process())
            .await
            .unwrap();
        assert_eq!(good.observed, requested);
        for output in [
            "max 100000\n1073741824\n0\n128\ntmpfs 131072 4096\n",
            "200000 100000\n1073741824\n0\n128\ntmpfs 18446744073709551615 18446744073709551615\n",
        ] {
            let mut bad = process();
            bad.stdout = output.into();
            assert!(
                resource_probe(&runtime, requested.clone(), bad)
                    .await
                    .is_err()
            );
        }
    }
    /// Synthetic qualification facts test native decision semantics, never physical qualification.
    #[tokio::test]
    async fn process_definitions_require_opt_in_exact_settings_and_a_live_command() {
        use crate::{
            control::ControlStore,
            control_jobs::{Arguments, JobStore},
            process_grants::Facts,
        };
        use enrichment_core::{
            capsule_protocol::{Mode, Operation, VERSION},
            request::ResolveRequest,
        };
        let root = tempfile::tempdir().unwrap();
        let config = config();
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let control = ControlStore::open(root.path(), runtime.clone()).unwrap();
        let jobs = JobStore::new(
            control,
            runtime.clone(),
            "process_policy_fixture".into(),
            Arc::new(config.clone()),
        );
        let policy = Policy::bind(&runtime, &config, capture(&config))
            .await
            .unwrap();
        let operation = Operation {
            version: VERSION,
            mode: Mode::Command,
            argv: vec!["/bin/tool".into()],
            inputs: [(
                "source".into(),
                enrichment_core::capsule_protocol::inventory::Entry::File {
                    mode: 0o400,
                    bytes: 7,
                    sha256: "a".repeat(64),
                },
            )]
            .into(),
            outputs: Default::default(),
            data_bytes: 1024,
            output_bytes: 1024,
            deadline_millis: 1000,
            binding: Operation::binding(
                config.execution.rust_image.as_ref().unwrap(),
                "qualified-physical-helper",
            )
            .unwrap(),
        };
        let image = config.execution.rust_image.as_ref().unwrap();
        for enabled in [false, true] {
            let id = format!("process_fixture_{enabled}");
            let command = jobs
                .command_with_id(
                    id.clone(),
                    Arguments::Resolve {
                        request: ResolveRequest {
                            name: "fixture".into(),
                            allow_local_build: enabled,
                            ..Default::default()
                        },
                    },
                )
                .await
                .unwrap();
            jobs.submit(command, format!("interest_{id}"))
                .await
                .unwrap();
            assert!(
                jobs.start(&id, "process_fixture_attempt", &policy)
                    .await
                    .unwrap()
            );
            let pin = jobs.pin().await.unwrap();
            let claim = jobs.claim(&pin, &id).await.unwrap().unwrap();
            let grant = jobs.grant(&id, claim.fence).await.unwrap();
            let result = grant
                .process(Facts {
                    execution: &config.execution,
                    capture: capture(&config),
                    image_id: image,
                    operation: &operation,
                    acquisition: false,
                })
                .await;
            if !enabled {
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("build_not_requested")
                );
            } else {
                let process = result.unwrap();
                assert_eq!(process.witness().operation_id, operation.id());
                assert_eq!(process.witness().grant_id, grant.id());
                let mut changed = config.execution.clone();
                changed.deadline_seconds += 1;
                assert!(
                    grant
                        .process(Facts {
                            execution: &changed,
                            capture: capture(&config),
                            image_id: image,
                            operation: &operation,
                            acquisition: false
                        })
                        .await
                        .unwrap_err()
                        .to_string()
                        .contains("execution_configuration_mismatch")
                );
                let wrong = format!("sha256:{}", "b".repeat(64));
                assert!(
                    grant
                        .process(Facts {
                            execution: &config.execution,
                            capture: capture(&config),
                            image_id: &wrong,
                            operation: &operation,
                            acquisition: false
                        })
                        .await
                        .unwrap_err()
                        .to_string()
                        .contains("contained_image_mismatch")
                );
                jobs.cancel(&id, &format!("interest_{id}")).await.unwrap();
                assert!(
                    process.check().await.is_err(),
                    "durable definitions never bypass revocation"
                );
            }
        }
    }
}
