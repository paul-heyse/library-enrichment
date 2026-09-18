//! Finite operation inputs and effective configuration represented as typed Arrow contracts.
pub mod identities;
pub mod jobs;
pub mod ownership;
pub mod projections;
pub mod results;
pub mod retention;
pub mod selections;
pub mod sources;
use crate::execution::VerifyRequest;
pub use crate::request::{Arguments, CommandKind};
use arrow::datatypes::{DataType, Schema, SchemaRef};
use std::sync::Arc;
crate::native_vocabulary! {
    pub enum Durability { Immediate = "immediate", Durable = "durable" }
}
crate::native_vocabulary! {
    pub enum ResponsePolicy { Research = "research", Verification = "verification", Job = "job", Status = "status" }
}
crate::native_struct! {
    /// One executable operation definition also supplies the transport catalog.
    pub struct Definition {
        operation: crate::request::Operation => crate::native_union::Rule::Text,
        name: String => crate::native_union::Rule::NonEmpty,
        rpc: String => crate::native_union::Rule::NonEmpty,
        description: String => crate::native_union::Rule::NonEmpty,
        effect: crate::wire::bindings::Effect => crate::native_union::Rule::Text,
        published: bool => crate::native_union::Rule::Text,
        durability: Durability => crate::native_union::Rule::Text,
        response: ResponsePolicy => crate::native_union::Rule::Text,
    }
}
crate::native_struct! {
    /// Cache and cursor witnesses bind declaration values as well as field/codec contracts.
    pub struct Contract {
        schema: crate::native_contract::Manifest => crate::native_union::Rule::Text,
        operation: Definition => crate::native_union::Rule::Text,
        aspects: Vec<crate::wire::research::AspectDefinition> => crate::native_union::Rule::Set,
        discovery: Vec<crate::wire::research::DiscoveryDefinition> => crate::native_union::Rule::Set,
        execution: Vec<crate::evidence::execution::ExecutionDefinition> => crate::native_union::Rule::Set,
    }
}
impl Contract {
    pub fn new(operation: Definition, schema: crate::native_contract::Manifest) -> Self {
        Self {
            schema,
            operation,
            aspects: crate::wire::research::InspectionAspect::definitions(),
            discovery: crate::wire::research::DiscoveryKind::definitions(),
            execution: crate::evidence::execution::ExecutionKind::definitions(),
        }
    }
    pub fn identity(&self) -> datafusion::common::Result<String> {
        crate::native_key::Key::OperationContract.record(self)
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
pub fn definition_binding_type() -> DataType {
    <crate::delta_reference::DeltaVersionRef as crate::native_union::Cell>::data_type()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, native_key::Key, request::ResolveRequest};

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
            &DataType::Struct(<Config as crate::native_union::NativeStruct>::fields()),
        );
        let identity = crate::identity::OperationPolicyId::try_from_record(&config).unwrap();
        let mut changed = config.clone();
        changed.network.max_redirects += 1;
        assert_ne!(
            identity,
            crate::identity::OperationPolicyId::try_from_record(&changed).unwrap()
        );
        changed = config;
        changed.policy.enabled_profiles.clear();
        assert_ne!(
            identity,
            crate::identity::OperationPolicyId::try_from_record(&changed).unwrap()
        );
        crate::native_struct! { struct Input {
            operation_revision: String => crate::native_union::Rule::Text,
            policy_id: crate::identity::OperationPolicyId => crate::native_union::Rule::Text,
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
    source: crate::delta_reference::DeltaVersionRef => crate::native_union::Rule::Text,
}
}
