//! Immutable definition families bind payload, identity and storage selection once.
use super::*;
use crate::native_union::{Cell, NativeStruct, Rule};

mod sealed {
    pub trait Sealed {}
}

pub trait DefinitionId:
    sealed::Sealed + Cell + Clone + Send + Sync + fmt::Display + 'static
{
    type Record: NativeStruct + Clone + Send + Sync + 'static;
    const KEY: Key;
    const TABLE: &'static str;
    const COLUMN: &'static str;
    fn row_value(&self) -> RowValue;
}

macro_rules! definitions {
    ($($id:ident, $key:ident, $prefix:literal, $pattern:literal, $record:ty, $table:literal, $column:literal;)+) => {
        $(content_identity!($id, $key, $prefix, $pattern);
        impl sealed::Sealed for $id {}
        impl DefinitionId for $id {
            type Record = $record;
            const KEY: Key = Key::$key;
            const TABLE: &'static str = $table;
            const COLUMN: &'static str = $column;
            fn row_value(&self) -> RowValue { RowValue::$key { value: self.clone() } }
        })+
        crate::native_union! { pub enum RowValue {
            Text = "text" { value: String => Rule::NonEmpty },
            Cohort = "cohort" { value: CohortId => Rule::Text },
            SchemaContract = "schema_contract" { value: SchemaContractId => Rule::Text },
            $($key = $prefix { value: $id => Rule::Text },)+
        } }
        impl RowValue {
            pub fn parameter(&self) -> datafusion::common::Result<datafusion::common::metadata::ScalarAndMetadata> {
                use crate::evidence::arrow_model::expressions::parameter;
                match self {
                    Self::Text { value } => {
                        if value.is_empty() { return datafusion::common::plan_err!("empty retention row key"); }
                        parameter(value)
                    },
                    $(Self::$key { value } => parameter(value),)+
                    Self::Cohort { value } => parameter(value),
                    Self::SchemaContract { value } => parameter(value),
                }
            }
        }
    };
}

definitions! {
    OperationPolicyId, OperationPolicy, "policy", "^policy_[0-9a-f]{64}$", crate::config::Config, "operation_policies", "policy_id";
    ProcessOperationId, ProcessOperation, "process", "^process_[0-9a-f]{64}$", crate::capsule_protocol::Operation, "process_operations", "operation_id";
    ProcessEffectId, ProcessEffect, "process_effect", "^process_effect_[0-9a-f]{64}$", crate::operation::jobs::Effect, "process_effects", "effect_id";
    StaticWorkerEffectId, StaticWorkerEffect, "static_worker_effect", "^static_worker_effect_[0-9a-f]{64}$", crate::execution::static_worker::Effect, "static_worker_effects", "effect_id";
    RustdocDecoderEffectId, RustdocDecoderEffect, "rustdoc_decoder_effect", "^rustdoc_decoder_effect_[0-9a-f]{64}$", crate::execution::rustdoc_decoder::Effect, "rustdoc_decoder_effects", "effect_id";
    SemanticConversationId, SemanticConversation, "conversation", "^conversation_[0-9a-f]{64}$", crate::native_semantics::Conversation, "semantic_conversations", "conversation_id";
    RegistryCaptureId, RegistryCapture, "registry_capture", "^registry_capture_[0-9a-f]{64}$", crate::operation::sources::RegistryCapture, "registry_captures", "capture_id";
    RevisionCaptureId, RevisionCapture, "revision_capture", "^revision_capture_[0-9a-f]{64}$", crate::operation::sources::RevisionCapture, "revision_captures", "capture_id";
}
