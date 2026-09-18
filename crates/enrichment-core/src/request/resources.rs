//! The finite resource catalog shares operation contracts with the tool and RPC catalogs.
use super::Operation;
use crate::native_union::Rule;
use serde_json::{Value, json};

crate::native_union! { pub enum ResourceSource {
    Guidance = "guidance" { package_path: String => Rule::MemberPath },
    Operation = "operation" {
        operation: Operation => Rule::Text,
        parameter: String => Rule::NonEmpty,
    },
} }
crate::native_struct! { pub struct ResourceDefinition {
    name: String => Rule::NonEmpty,
    uri: String => Rule::NonEmpty,
    description: String => Rule::Text,
    mime_type: String => Rule::NonEmpty,
    source: ResourceSource => Rule::Text,
} }

pub fn definitions() -> Vec<ResourceDefinition> {
    let mut values = vec![ResourceDefinition {
        name: "research_workflow".into(),
        uri: "library-evidence://workflow".into(),
        description: "Current selection, coverage, jobs, recovery and result-reading guidance."
            .into(),
        mime_type: "text/markdown".into(),
        source: ResourceSource::Guidance {
            package_path: "_guidance/tool-contract.md".into(),
        },
    }];
    values.extend(
        [
            (
                "artifact",
                "library-evidence://artifacts/{artifact_id}",
                Operation::ReadArtifact,
                "artifact_id",
            ),
            (
                "context_overview",
                "library-evidence://contexts/{context_id}/overview",
                Operation::Overview,
                "context_id",
            ),
            (
                "snapshot_manifest",
                "library-evidence://snapshots/{snapshot_id}/manifest",
                Operation::SnapshotManifest,
                "snapshot_id",
            ),
            (
                "job_result",
                "library-evidence://jobs/{job_id}/result",
                Operation::Job,
                "job_id",
            ),
        ]
        .into_iter()
        .map(|(name, uri, operation, parameter)| ResourceDefinition {
            name: name.into(),
            uri: uri.into(),
            description: operation.definition().description,
            mime_type: "application/json".into(),
            source: ResourceSource::Operation {
                operation,
                parameter: parameter.into(),
            },
        }),
    );
    values
}

/// Parameter schemas are projected from the operation binding, never re-declared by Python.
pub fn bindings(operations: &[Value]) -> Vec<Value> {
    definitions()
        .into_iter()
        .map(|definition| {
            let mut value = serde_json::to_value(&definition).expect("declared resource");
            if let ResourceSource::Operation {
                operation,
                parameter,
            } = &definition.source
            {
                let binding = operations
                    .iter()
                    .find(|item| item["rpc"] == operation.rpc())
                    .expect("resource operation is declared");
                let input = &binding["input_schema"];
                let field = input["properties"]
                    .get(parameter)
                    .expect("resource parameter is declared");
                value["rpc"] = json!(operation.rpc());
                value["parameters"] = json!({
                    "type":"object", "properties":{parameter:field},
                    "required":[parameter], "additionalProperties":false,
                    "$defs":input.get("$defs").cloned().unwrap_or_else(||json!({})),
                });
            }
            value
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_union::NativeStruct;

    #[test]
    fn resource_catalog_projects_exact_operation_fields() {
        let operations = super::super::operation_bindings();
        let resources = bindings(&operations);
        assert_eq!(
            ResourceDefinition::batch(&definitions())
                .unwrap()
                .num_rows(),
            5
        );
        assert_eq!(resources.len(), 5);
        assert_eq!(resources[0]["source"]["kind"], "guidance");
        let mut names = std::collections::BTreeSet::new();
        for resource in resources {
            assert!(names.insert(resource["name"].as_str().unwrap().to_owned()));
            if resource["source"]["kind"] == "operation" {
                let operation = operations
                    .iter()
                    .find(|item| item["rpc"] == resource["rpc"])
                    .unwrap();
                let parameter = resource["parameters"]["required"][0].as_str().unwrap();
                assert!(
                    resource["uri"]
                        .as_str()
                        .unwrap()
                        .contains(&format!("{{{parameter}}}"))
                );
                assert_eq!(
                    resource["parameters"]["properties"][parameter],
                    operation["input_schema"]["properties"][parameter]
                );
                assert_eq!(resource["parameters"]["additionalProperties"], false);
            }
        }
    }
}
