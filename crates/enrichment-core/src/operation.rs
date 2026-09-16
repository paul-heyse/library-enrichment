//! Finite operation inputs and effective configuration represented as typed Arrow contracts.
use crate::{
    execution::VerifyRequest,
    request::{CompareRequest, InspectRequest, ResolveRequest},
};
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
fn field(name: &str, kind: DataType, nullable: bool) -> Field {
    Field::new(name, kind, nullable)
}
fn text(name: &str) -> Field {
    field(name, DataType::Utf8, false)
}
fn optional(name: &str) -> Field {
    field(name, DataType::Utf8, true)
}
fn number(name: &str, nullable: bool) -> Field {
    field(name, DataType::UInt64, nullable)
}
fn list(kind: DataType) -> DataType {
    DataType::List(Arc::new(field("item", kind, false)))
}
fn structure(fields: Vec<Field>) -> DataType {
    DataType::Struct(fields.into())
}
fn resolve() -> DataType {
    let mut fields = vec![text("ecosystem"), text("name")];
    fields.extend(
        [
            "version",
            "repository",
            "revision",
            "package_subdir",
            "mode",
            "target",
            "python_version",
        ]
        .map(optional),
    );
    fields.extend(["features", "extras"].map(|name| field(name, list(DataType::Utf8), true)));
    fields.push(field("default_features", DataType::Boolean, true));
    fields.push(text("freshness"));
    fields.extend(
        ["allow_prerelease", "allow_yanked", "allow_local_build"]
            .map(|name| field(name, DataType::Boolean, false)),
    );
    structure(fields)
}
fn verify() -> DataType {
    structure(vec![
        text("context_id"),
        optional("snapshot_id"),
        text("snippet"),
        text("mode"),
        text("profile"),
        optional("test_intent"),
        number("max_bytes", true),
    ])
}
fn inspect() -> DataType {
    let selection = structure(vec![
        text("mode"),
        field(
            "aspects",
            list(structure(vec![
                text("aspect"),
                optional("cursor"),
                number("max_items", false),
                number("max_characters", true),
            ])),
            true,
        ),
    ]);
    let execution = structure(vec![
        text("intent"),
        optional("profile"),
        optional("snippet"),
        field(
            "position",
            structure(vec![
                field("line", DataType::UInt32, false),
                field("byte", DataType::UInt32, false),
            ]),
            true,
        ),
        field("methods", list(DataType::Utf8), false),
        field(
            "runtime",
            structure(vec![
                text("module"),
                field("attributes", list(DataType::Utf8), false),
            ]),
            true,
        ),
    ]);
    structure(vec![
        text("context_id"),
        optional("snapshot_id"),
        text("symbol_path"),
        optional("definition_id"),
        field("selection", selection, false),
        number("max_bytes", true),
        field("execution", execution, true),
    ])
}
fn compare() -> DataType {
    let mut fields = [
        "alternative_cursor",
        "before_context_id",
        "after_context_id",
        "before_snapshot_id",
        "after_snapshot_id",
        "ecosystem",
        "name",
        "from_version",
        "to_version",
        "cursor",
    ]
    .map(optional)
    .to_vec();
    fields.extend([
        field("scopes", list(DataType::Utf8), true),
        number("max_items", true),
        number("max_bytes", true),
    ]);
    structure(fields)
}

/// Exactly one operation payload is admitted by the native control contract.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    pub verify: Option<VerifyRequest>,
    pub inspect: Option<InspectRequest>,
    pub resolve: Option<ResolveRequest>,
    pub compare: Option<CompareRequest>,
}
pub fn arguments_type() -> DataType {
    structure(vec![
        field("verify", verify(), true),
        field("inspect", inspect(), true),
        field("resolve", resolve(), true),
        field("compare", compare(), true),
    ])
}
pub fn command_key_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("operation_revision"),
        text("policy_id"),
        field("arguments", arguments_type(), false),
    ]))
}
pub fn grant_key_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("job_id"),
        text("job_key"),
        text("policy_id"),
        text("owner"),
        text("attempt_id"),
        number("fence", false),
    ]))
}

fn config_config() -> DataType {
    structure(vec![
        field("arrow", config_arrowconfig(), false),
        field("execution", config_execution(), false),
        field("limits", config_limits(), false),
        field("policy", config_policy(), false),
        field("freshness", config_freshnessconfig(), false),
        field("producers", config_producers(), false),
        field("network", config_network(), false),
    ])
}

fn config_arrowconfig() -> DataType {
    structure(vec![
        field("native", config_nativequeryconfig(), false),
        field("memory_bytes", DataType::UInt64, false),
        field("spill_bytes", DataType::UInt64, false),
        field("metadata_cache_bytes", DataType::UInt64, false),
        field("batch_rows", DataType::UInt64, false),
        field("partitions", DataType::UInt64, false),
        field("concurrency", DataType::UInt64, false),
        field("result_rows", DataType::UInt64, false),
        field("result_bytes", DataType::UInt64, false),
        field("query_deadline_seconds", DataType::UInt64, false),
        field("admission_deadline_seconds", DataType::UInt64, false),
        field("record_bytes", DataType::UInt64, false),
        field("batch_bytes", DataType::UInt64, false),
        field("file_bytes", DataType::UInt64, false),
        field("table_rows", DataType::UInt64, false),
    ])
}

fn config_nativequeryconfig() -> DataType {
    structure(vec![
        field("worker_stack_bytes", DataType::UInt64, false),
        field("blocking_threads", DataType::UInt64, false),
        field("decoder_filter", DataType::Boolean, false),
        field("observation_bloom", DataType::Boolean, false),
        field("reorder_filters", DataType::Boolean, false),
        field("row_group_rows", DataType::UInt64, false),
        field("row_group_bytes", DataType::UInt64, false),
        field("target_file_bytes", DataType::UInt64, false),
        field("claim_lease_seconds", DataType::UInt64, false),
        field("normalization_depth", DataType::UInt32, false),
    ])
}

fn config_execution() -> DataType {
    structure(vec![
        field("storage_root", DataType::Utf8, true),
        field("python_image", DataType::Utf8, true),
        field("rust_image", DataType::Utf8, true),
        field("deadline_seconds", DataType::UInt64, false),
        field("output_bytes", DataType::UInt64, false),
        field("cpus", DataType::UInt32, false),
        field("memory_mib", DataType::UInt64, false),
        field("scratch_mib", DataType::UInt64, false),
        field("executor_path", DataType::Utf8, true),
        field("pids", DataType::UInt32, false),
        field("queue_limit", DataType::UInt64, false),
        field("broker_path", DataType::Utf8, true),
        field("cleanup_deadline_seconds", DataType::UInt64, false),
        field("capsule_budget_mib", DataType::UInt64, false),
        field("lsp_idle_seconds", DataType::UInt64, false),
    ])
}

pub fn execution_schema() -> SchemaRef {
    let DataType::Struct(fields) = config_execution() else {
        unreachable!("declared execution configuration")
    };
    Arc::new(Schema::new(fields))
}

fn map(value: DataType) -> DataType {
    DataType::Map(
        Arc::new(field(
            "entries",
            // DF55 map_entries reports map values nullable. Preserve that physical layout;
            // the executor's typed Entry/OutputKind input and admission require real values.
            structure(vec![text("key"), field("value", value, true)]),
            false,
        )),
        false,
    )
}

/// Exact executor protocol input; directory-only fields are absent, never fabricated.
pub fn process_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        field("version", DataType::UInt32, false),
        text("mode"),
        field("argv", list(DataType::Utf8), false),
        field(
            "inputs",
            map(structure(vec![
                text("kind"),
                field("mode", DataType::UInt32, false),
                number("bytes", true),
                optional("sha256"),
            ])),
            false,
        ),
        field("outputs", map(DataType::Utf8), false),
        number("data_bytes", false),
        number("output_bytes", false),
        number("deadline_millis", false),
        text("binding"),
    ]))
}

pub fn process_effect_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        text("grant_id"),
        text("job_id"),
        text("policy_id"),
        text("profile"),
        text("ecosystem"),
        optional("environment_id"),
        optional("snapshot_id"),
        text("image_id"),
        field("acquisition", DataType::Boolean, false),
        text("operation_id"),
        field("operation_binding", definition_binding_type(), false),
    ]))
}

pub fn definition_binding_type() -> DataType {
    structure(vec![
        text("table_id"),
        number("version", false),
        text("contract_id"),
    ])
}

fn config_limits() -> DataType {
    structure(vec![
        field("inline_result_bytes", DataType::UInt64, false),
        field("search_results", DataType::UInt64, false),
        field("excerpt_characters", DataType::UInt64, false),
        field("source_lines", DataType::UInt64, false),
        field("namespace_entries", DataType::UInt64, false),
        field("verification_input_bytes", DataType::UInt64, false),
        field("inline_wait_seconds", DataType::UInt64, false),
        field("max_job_wait_seconds", DataType::UInt64, false),
        field("expensive_worker_concurrency", DataType::UInt64, false),
        field("warm_lsp_sessions", DataType::UInt64, false),
        field("rpc_message_bytes", DataType::UInt64, false),
    ])
}

fn config_policy() -> DataType {
    structure(vec![
        field("enabled_profiles", list(DataType::Utf8), false),
        field("require_sandbox_for_build", DataType::Boolean, false),
        field("require_sandbox_for_runtime", DataType::Boolean, false),
        field("execution_network", DataType::Boolean, false),
    ])
}

fn config_freshnessconfig() -> DataType {
    structure(vec![
        field("registry_ttl_seconds", DataType::UInt64, false),
        field("mutable_docs_ttl_seconds", DataType::UInt64, false),
        field("negative_cache_ttl_seconds", DataType::UInt64, false),
        field("latest_requires_revalidation", DataType::Boolean, false),
    ])
}

fn config_producers() -> DataType {
    structure(vec![
        field("github_api_url", DataType::Utf8, false),
        field("rust", config_rustproducers(), false),
        field("python", config_pythonproducers(), false),
    ])
}

fn config_rustproducers() -> DataType {
    structure(vec![
        field("prefer_hosted_rustdoc_json", DataType::Boolean, false),
        field("all_features_by_default", DataType::Boolean, false),
        field("lsp", DataType::Utf8, false),
        field("crates_io_index_url", DataType::Utf8, false),
        field("crates_io_api_url", DataType::Utf8, false),
        field("docs_rs_url", DataType::Utf8, false),
        field("user_agent", DataType::Utf8, false),
    ])
}

fn config_pythonproducers() -> DataType {
    structure(vec![
        field("pypi_url", DataType::Utf8, false),
        field("simple_url", DataType::Utf8, false),
        field("worker_python", DataType::Utf8, false),
        field("worker_timeout_seconds", DataType::UInt64, false),
    ])
}

fn config_network() -> DataType {
    structure(vec![
        field("max_download_bytes", DataType::UInt64, false),
        field("max_decompressed_bytes", DataType::UInt64, false),
        field("max_redirects", DataType::UInt32, false),
        field("request_timeout_seconds", DataType::UInt64, false),
        field("acquisition_timeout_seconds", DataType::UInt64, false),
    ])
}

pub fn policy_schema() -> SchemaRef {
    let DataType::Struct(fields) = config_config() else {
        unreachable!("declared config")
    };
    Arc::new(Schema::new(fields))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, native_key::Key};

    fn complete(value: &serde_json::Value, kind: &DataType) {
        if let (Some(object), DataType::Struct(fields)) = (value.as_object(), kind) {
            let encoded: std::collections::BTreeSet<_> =
                object.keys().map(String::as_str).collect();
            let declared: std::collections::BTreeSet<_> =
                fields.iter().map(|f| f.name().as_str()).collect();
            assert_eq!(
                encoded, declared,
                "every serialized configuration field belongs to the native policy"
            );
            for field in fields {
                complete(&object[field.name()], field.data_type());
            }
        }
    }
    #[test]
    fn policy_captures_complete_configuration_and_command_scope_changes() {
        let config = Config::default();
        complete(
            &serde_json::to_value(&config).unwrap(),
            &DataType::Struct(policy_schema().fields().clone()),
        );
        let identity = Key::OperationPolicy.value(&config).unwrap();
        let mut changed = config.clone();
        changed.network.max_redirects += 1;
        assert_ne!(identity, Key::OperationPolicy.value(&changed).unwrap());
        changed = config;
        changed.policy.enabled_profiles.clear();
        assert_ne!(identity, Key::OperationPolicy.value(&changed).unwrap());
        #[derive(Serialize)]
        struct Input<'a> {
            operation_revision: &'a str,
            policy_id: &'a str,
            arguments: Arguments,
        }
        let mut input = Input {
            operation_revision: "source/1",
            policy_id: &identity,
            arguments: Arguments {
                resolve: Some(ResolveRequest {
                    name: "fixture".into(),
                    version: Some("1.0.0".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        };
        let command = Key::OperationCommand.value(&input).unwrap();
        input.arguments.resolve.as_mut().unwrap().features = Some(vec!["extra".into()]);
        assert_ne!(command, Key::OperationCommand.value(&input).unwrap());
        input.arguments.resolve.as_mut().unwrap().features = None;
        input.operation_revision = "source/2";
        assert_ne!(command, Key::OperationCommand.value(&input).unwrap());
    }
}
