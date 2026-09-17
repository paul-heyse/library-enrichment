//! Finite operation inputs and effective configuration represented as typed Arrow contracts.
pub mod jobs;
pub mod ownership;
pub mod projections;
pub mod results;
use crate::{
    execution::VerifyRequest,
    request::{CompareRequest, InspectRequest, ResolveRequest},
};
use arrow::datatypes::{DataType, Schema, SchemaRef};
use std::sync::Arc;
crate::native_union! {
    pub enum Arguments {
        Verify = "verify" { request: VerifyRequest => crate::native_union::Rule::Text },
        Inspect = "inspect" { request: InspectRequest => crate::native_union::Rule::Text },
        Resolve = "resolve" { request: ResolveRequest => crate::native_union::Rule::Text },
        Compare = "compare" { request: CompareRequest => crate::native_union::Rule::Text },
    }
}
crate::native_struct! {
    pub struct DefinitionBinding {
        table_id: String => crate::native_union::Rule::NonEmpty,
        version: u64 => crate::native_union::Rule::Text,
        contract_id: String => crate::native_union::Rule::NonEmpty,
    }
}
impl From<VerifyRequest> for Arguments {
    fn from(request: VerifyRequest) -> Self {
        Self::Verify { request }
    }
}
pub fn arguments_type() -> DataType {
    <Arguments as crate::native_union::Cell>::data_type()
}
fn key_projection<T: crate::native_union::NativeStruct>(names: &[&str]) -> SchemaRef {
    let fields = T::fields();
    Arc::new(Schema::new(
        names
            .iter()
            .map(|name| {
                fields
                    .find(name)
                    .expect("declared identity field")
                    .1
                    .clone()
            })
            .collect::<Vec<_>>(),
    ))
}
pub fn command_key_schema() -> SchemaRef {
    key_projection::<jobs::Command>(&["operation_revision", "policy_id", "arguments"])
}
pub fn grant_key_schema() -> SchemaRef {
    key_projection::<jobs::Claim>(&[
        "job_id",
        "job_key",
        "policy_id",
        "owner",
        "attempt_id",
        "fence",
    ])
}
pub fn execution_schema() -> SchemaRef {
    Arc::new(Schema::new(
        <crate::config::Execution as crate::native_union::NativeStruct>::fields(),
    ))
}
pub fn process_schema() -> SchemaRef {
    Arc::new(Schema::new(
        <crate::capsule_protocol::Operation as crate::native_union::NativeStruct>::fields(),
    ))
}
pub fn process_effect_schema() -> SchemaRef {
    Arc::new(Schema::new(
        <jobs::Effect as crate::native_union::NativeStruct>::fields(),
    ))
}
pub fn definition_binding_type() -> DataType {
    <DefinitionBinding as crate::native_union::Cell>::data_type()
}

pub fn policy_schema() -> SchemaRef {
    Arc::new(Schema::new(
        <crate::config::Config as crate::native_union::NativeStruct>::fields(),
    ))
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
        let identity = Key::OperationPolicy.record(&config).unwrap();
        let mut changed = config.clone();
        changed.network.max_redirects += 1;
        assert_ne!(identity, Key::OperationPolicy.record(&changed).unwrap());
        changed = config;
        changed.policy.enabled_profiles.clear();
        assert_ne!(identity, Key::OperationPolicy.record(&changed).unwrap());
        crate::native_struct! { struct Input {
            operation_revision: String => crate::native_union::Rule::Text,
            policy_id: String => crate::native_union::Rule::Text,
            arguments: Arguments => crate::native_union::Rule::Text,
        } }
        use crate::native_union::NativeStruct;
        let mut input = Input {
            operation_revision: "source/1".into(),
            policy_id: identity.clone(),
            arguments: Arguments::Resolve {
                request: ResolveRequest {
                    name: "fixture".into(),
                    version: Some("1.0.0".into()),
                    ..Default::default()
                },
            },
        };
        let command = Key::OperationCommand
            .batch_value(&Input::batch(std::slice::from_ref(&input)).unwrap())
            .unwrap();
        if let Arguments::Resolve { request } = &mut input.arguments {
            request.features = Some(vec!["extra".into()]);
        }
        assert_ne!(
            command,
            Key::OperationCommand
                .batch_value(&Input::batch(std::slice::from_ref(&input)).unwrap())
                .unwrap()
        );
        if let Arguments::Resolve { request } = &mut input.arguments {
            request.features = None;
        }
        input.operation_revision = "source/2".into();
        assert_ne!(
            command,
            Key::OperationCommand
                .batch_value(&Input::batch(std::slice::from_ref(&input)).unwrap())
                .unwrap()
        );
    }
}
crate::native_struct! {
pub struct StorageCapture {
    name: String => crate::native_union::Rule::NonEmpty,
    table_id: String => crate::native_union::Rule::NonEmpty,
    version: u64 => crate::native_union::Rule::Text,
    contract_id: String => crate::native_union::Rule::NonEmpty,
}
}
