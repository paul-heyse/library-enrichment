//! Authoritative typed identity expressions, shared by native plans and bounded DTO ingress.
use arrow::{
    datatypes::{Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    common::{DFSchema, ScalarValue},
    error::{DataFusionError, Result},
    functions::{crypto::expr_fn::sha256, encoding::expr_fn::encode, string::expr_fn::concat},
    logical_expr::{Expr, simplify::SimplifyContext},
    optimizer::simplify_expressions::ExprSimplifier,
    prelude::{col, lit},
};
use std::sync::Arc;

/// An ordered contract is data; the native function graph is identical at every consumer.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    SchemaContract,
    OperationContract,
    RetentionPolicy,
    RegistryCapture,
    RevisionCapture,
    ContractField,
    ContractChange,
    Release,
    Environment,
    Context,
    Snapshot,
    SnapshotDescriptor,
    Projection,
    PublicPath,
    PublicBinding,
    Definition,
    ApiObservation,
    TextFragment,
    Citation,
    InputArtifact,
    Coverage,
    Relationship,
    ExecutionObservation,
    ReleaseMetadata,
    ProducerBinding,
    ProducerPlan,
    ArtifactReceipt,
    SnapshotAttempt,
    OperationPolicy,
    OperationCommand,
    EffectGrant,
    ExecutionConfiguration,
    ProcessOperation,
    SemanticConversation,
    ProcessEffect,
    StaticWorkerEffect,
    RustdocDecoderEffect,
    RowCursor,
    SearchCursor,
    SearchSelection,
    RuntimeWitness,
    ComparisonCursor,
    AlternativeCursor,
    ArtifactCursor,
    ArtifactSelection,
    ResearchInvocation,
    AcquisitionConfiguration,
    NormalizationConfiguration,
    InspectionConfiguration,
    RustdocBuildConfiguration,
    CapsuleIdentity,
    ProducerImplementation,
    VerificationConfiguration,
    ContainmentIdentity,
    ProcessInventory,
    PhysicalInventory,

    InspectionSelection,
    DiscoverySelection,
    ComparisonSelection,
}
impl Key {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::SchemaContract => "schema_contract",
            Self::OperationContract => "operation_contract",
            Self::RetentionPolicy => "retention_policy",
            Self::RegistryCapture => "registry_capture",
            Self::RevisionCapture => "revision_capture",
            Self::ContractField => "contract_field",
            Self::ContractChange => "contract_change",
            Self::Release => "rel",
            Self::Environment => "env",
            Self::Context => "ctx",
            Self::Snapshot => "snap",
            Self::SnapshotDescriptor => "snapshot_descriptor",
            Self::Projection => "projection",
            Self::PublicPath => "path",
            Self::PublicBinding => "symbol",
            Self::Definition => "def",
            Self::ApiObservation => "obs",
            Self::TextFragment => "fragment",
            Self::Citation => "ev",
            Self::InputArtifact => "input",
            Self::Coverage => "coverage",
            Self::Relationship => "relationship",
            Self::ExecutionObservation => "exec",
            Self::ReleaseMetadata => "metadata",
            Self::ProducerBinding => "producer",
            Self::ProducerPlan => "producer_plan",
            Self::ArtifactReceipt => "receipt",
            Self::SnapshotAttempt => "association",
            Self::OperationPolicy => "policy",
            Self::OperationCommand => "command",
            Self::EffectGrant => "grant",
            Self::ExecutionConfiguration => "execution_config",
            Self::ProcessOperation => "process",
            Self::SemanticConversation => "conversation",
            Self::ProcessEffect => "process_effect",
            Self::StaticWorkerEffect => "static_worker_effect",
            Self::RustdocDecoderEffect => "rustdoc_decoder_effect",
            Self::RowCursor => "row_cursor",
            Self::SearchCursor => "search_cursor",
            Self::SearchSelection => "search_selection",
            Self::RuntimeWitness => "runtime_witness",
            Self::ComparisonCursor => "comparison_cursor",
            Self::AlternativeCursor => "alternative_cursor",
            Self::ArtifactCursor => "artifact_cursor",
            Self::ArtifactSelection => "artifact_selection",
            Self::ResearchInvocation => "research_invocation",
            Self::AcquisitionConfiguration => "acquisition_configuration",
            Self::NormalizationConfiguration => "normalization_configuration",
            Self::InspectionConfiguration => "inspection_configuration",
            Self::RustdocBuildConfiguration => "rustdoc_build_configuration",
            Self::CapsuleIdentity => "capsule",
            Self::ProducerImplementation => "producer_implementation",
            Self::VerificationConfiguration => "verification_configuration",
            Self::ContainmentIdentity => "containment_identity",
            Self::ProcessInventory => "process_inventory",
            Self::PhysicalInventory => "physical_inventory",

            Self::InspectionSelection => "inspection_selection",
            Self::DiscoverySelection => "discovery_selection",
            Self::ComparisonSelection => "comparison_selection",
        }
    }
    pub fn schema(self) -> SchemaRef {
        match self {
            Self::SchemaContract => return native_fields::<crate::native_contract::Manifest>(),
            Self::OperationContract => return native_fields::<crate::operation::Contract>(),
            Self::RetentionPolicy => return native_fields::<crate::operation::retention::RetentionPolicy>(),
            Self::RegistryCapture => return native_fields::<<crate::identity::RegistryCaptureId as crate::identity::DefinitionId>::Record>(),
            Self::RevisionCapture => return native_fields::<<crate::identity::RevisionCaptureId as crate::identity::DefinitionId>::Record>(),
            Self::ContractField => return native_fields::<crate::native_contract::ContractField>(),
            Self::ContractChange => return native_fields::<crate::native_contract::Change>(),
            Self::ProducerImplementation => return native_fields::<crate::operation::identities::ProducerImplementation>(),
            Self::VerificationConfiguration => return native_fields::<crate::operation::identities::VerificationConfiguration>(),
            Self::RustdocBuildConfiguration => return native_fields::<crate::operation::identities::RustdocBuildConfiguration>(),
            Self::ContainmentIdentity => return native_fields::<crate::operation::identities::ContainmentIdentity>(),
            Self::ProcessInventory => return native_fields::<crate::operation::identities::ProcessInventory>(),
            Self::PhysicalInventory => return native_fields::<crate::operation::identities::PhysicalInventory>(),
            Self::ResearchInvocation => return native_fields::<crate::operation::identities::ResearchInvocation>(),
            Self::AcquisitionConfiguration => return native_fields::<crate::operation::identities::AcquisitionConfiguration>(),
            Self::NormalizationConfiguration => return native_fields::<crate::operation::identities::NormalizationConfiguration>(),
            Self::InspectionConfiguration => return native_fields::<crate::operation::identities::InspectionConfiguration>(),
            Self::CapsuleIdentity => return native_fields::<crate::operation::identities::CapsuleIdentity>(),
            Self::ComparisonCursor => return without_check::<crate::compare::page::ComparisonCursor>(),
            Self::AlternativeCursor => return without_check::<crate::compare::page::AlternativeCursor>(),
            Self::ArtifactCursor => return without_check::<crate::search::Cursor>(),
            Self::ArtifactSelection => return Arc::new(Schema::new(<crate::operation::selections::ArtifactSelection as crate::native_union::NativeStruct>::fields())),
            Self::RuntimeWitness => return native_fields::<crate::operation::selections::RuntimeWitness>(),
            Self::SearchSelection => return Arc::new(Schema::new(<crate::operation::selections::SearchSelection as crate::native_union::NativeStruct>::fields())),
            Self::InspectionSelection => return Arc::new(Schema::new(<crate::operation::selections::InspectionSelection as crate::native_union::NativeStruct>::fields())),
            Self::DiscoverySelection => return Arc::new(Schema::new(<crate::operation::selections::DiscoverySelection as crate::native_union::NativeStruct>::fields())),
            Self::ComparisonSelection => return Arc::new(Schema::new(<crate::operation::selections::ComparisonSelection as crate::native_union::NativeStruct>::fields())),
            Self::RowCursor => return Arc::new(Schema::new(<crate::search::row_page::RowCursorBinding as crate::native_union::NativeStruct>::fields())),
            Self::SearchCursor => return Arc::new(Schema::new(<crate::search::page::SearchCursorBinding as crate::native_union::NativeStruct>::fields())),
            Self::PublicPath => return Arc::new(Schema::new(<crate::evidence::path::PathParts as crate::native_union::NativeStruct>::fields())),
            Self::PublicBinding => return Arc::new(Schema::new(<crate::evidence::relational::PublicBindingIdentity as crate::native_union::NativeStruct>::fields())),
            Self::SnapshotAttempt => return Arc::new(Schema::new(<crate::evidence::catalog::SnapshotAttemptIdentity as crate::native_union::NativeStruct>::fields())),
            Self::Release => return Arc::new(Schema::new(<crate::identity::ReleaseKey as crate::native_union::NativeStruct>::fields())),
            Self::Environment => return Arc::new(Schema::new(<crate::identity::EnvironmentKey as crate::native_union::NativeStruct>::fields())),
            Self::Context => return Arc::new(Schema::new(<crate::identity::ContextKey as crate::native_union::NativeStruct>::fields())),
            Self::Snapshot => return Arc::new(Schema::new(<crate::identity::SnapshotInputs as crate::native_union::NativeStruct>::fields())),
            Self::Definition => return Arc::new(Schema::new(<crate::evidence::model::DefinitionIdentity as crate::native_union::NativeStruct>::fields())),
            Self::Citation => return Arc::new(Schema::new(<crate::wire::evidence::CitationIdentity as crate::native_union::NativeStruct>::fields())),
            Self::Projection => return Arc::new(Schema::new(<crate::operation::projections::ProjectionIdentity as crate::native_union::NativeStruct>::fields())),
            Self::SnapshotDescriptor => return Arc::new(Schema::new(<crate::evidence::snapshot::SnapshotDescriptor as crate::native_union::NativeStruct>::fields())),
            Self::OperationPolicy => return native_fields::<<crate::identity::OperationPolicyId as crate::identity::DefinitionId>::Record>(),
            Self::OperationCommand => return crate::operation::command_key_schema(),
            Self::EffectGrant => return crate::operation::grant_key_schema(),
            Self::ExecutionConfiguration => return crate::operation::execution_schema(),
            Self::ProcessOperation => return native_fields::<<crate::identity::ProcessOperationId as crate::identity::DefinitionId>::Record>(),
            Self::SemanticConversation => return native_fields::<<crate::identity::SemanticConversationId as crate::identity::DefinitionId>::Record>(),
            Self::ProcessEffect => return native_fields::<<crate::identity::ProcessEffectId as crate::identity::DefinitionId>::Record>(),
            Self::StaticWorkerEffect => return native_fields::<<crate::identity::StaticWorkerEffectId as crate::identity::DefinitionId>::Record>(),
            Self::RustdocDecoderEffect => return native_fields::<<crate::identity::RustdocDecoderEffectId as crate::identity::DefinitionId>::Record>(),
            _ => {}
        }
        if matches!(self, Self::ArtifactReceipt) {
            return Arc::new(Schema::new(vec![Field::new(
                "artifact",
                crate::evidence::arrow_model::acquisitions::data_type(),
                false,
            )]));
        }
        use crate::evidence::arrow_model::{encode, execution, metadata, provenance, relations};
        if matches!(self, Self::ProducerBinding | Self::ProducerPlan) {
            let batch = provenance::producer_fields(&[]).expect("native producer contract");
            let mut names = vec![
                "producer",
                "producer_version",
                "config_digest",
                "inputs",
                "profile",
            ];
            if matches!(self, Self::ProducerBinding) {
                names.extend(["outcome", "gaps"]);
            }
            return Arc::new(Schema::new(
                names
                    .into_iter()
                    .map(|name| {
                        batch
                            .schema()
                            .field_with_name(name)
                            .expect("native producer field")
                            .clone()
                    })
                    .collect::<Vec<_>>(),
            ));
        }
        let native = match self {
            Self::ApiObservation => Some((encode::observations_fields(&[]), "observation_id")),
            Self::TextFragment => Some((relations::fragments_fields(&[]), "fragment_id")),
            Self::InputArtifact => Some((provenance::input_artifacts_fields(&[]), "input_id")),
            Self::Coverage => Some((provenance::coverage_fields(&[]), "coverage_id")),
            Self::Relationship => Some((relations::relationships_fields(&[]), "relationship_id")),
            Self::ExecutionObservation => Some((execution::fields(&[]), "observation_id")),
            Self::ReleaseMetadata => Some((metadata::fields(&[]), "metadata_id")),
            _ => None,
        };
        if let Some((batch, key)) = native {
            let batch = batch.expect("declared Arrow evidence contract");
            return Arc::new(Schema::new(
                batch
                    .schema()
                    .fields()
                    .iter()
                    .filter(|field| field.name() != key)
                    .cloned()
                    .collect::<Vec<_>>(),
            ));
        }
        unreachable!("native evidence schema returned above")
    }
    /// Build a native identity expression over the named contract fields.
    pub fn expression(self) -> Expr {
        self.expression_for(
            self.schema()
                .fields()
                .iter()
                .map(|field| col(field.name()))
                .collect(),
        )
    }
    /// The same defining-field graph used by constructors, with a checked binary domain.
    pub fn identity(self) -> Result<Expr> {
        self.identity_expression(
            self.schema()
                .fields()
                .iter()
                .map(|field| col(field.name()))
                .collect(),
        )
    }

    /// Bind the canonical defining fields to a domain-preserving native hash.
    pub fn identity_expression(self, inputs: Vec<Expr>) -> Result<Expr> {
        use crate::native_union::Domain;
        let domain = match self {
            Self::Release => Domain::Release,
            Self::Environment => Domain::Environment,
            Self::Context => Domain::Context,
            Self::Snapshot => Domain::Snapshot,
            Self::EffectGrant => Domain::EffectGrant,
            Self::RetentionPolicy => Domain::RetentionPolicy,
            Self::OperationPolicy => Domain::OperationPolicy,
            Self::ProcessOperation => Domain::ProcessOperation,
            Self::ProcessEffect => Domain::ProcessEffect,
            Self::StaticWorkerEffect => Domain::StaticWorkerEffect,
            Self::RustdocDecoderEffect => Domain::RustdocDecoderEffect,
            Self::SemanticConversation => Domain::SemanticConversation,
            Self::RegistryCapture => Domain::RegistryCapture,
            Self::RevisionCapture => Domain::RevisionCapture,

            _ => {
                return datafusion::common::plan_err!("key has no deployed binary identity domain");
            }
        };
        let fields = self.schema().fields().clone();
        if inputs.len() != fields.len() {
            return datafusion::common::plan_err!("native identity input arity");
        }
        let bytes = crate::native_identity::canonical_bytes(
            format!("enrichment/identity/10/{}", self.prefix()),
            fields,
        )
        .call(inputs);
        Ok(crate::native_id::from_hash(domain, sha256(bytes)))
    }
    /// Bind the declared fields to expressions supplied by a native normalization plan.
    pub fn bind(self, inputs: Vec<Expr>) -> Result<Expr> {
        if inputs.len() != self.schema().fields().len() {
            return Err(DataFusionError::Plan(
                "identity contract input arity".into(),
            ));
        }
        Ok(self.expression_for(inputs))
    }
    fn expression_for(self, inputs: Vec<Expr>) -> Expr {
        let fields = self.schema().fields().clone();
        let bytes = crate::native_identity::canonical_bytes(
            format!("enrichment/identity/10/{}", self.prefix()),
            fields,
        )
        .call(inputs);
        concat(vec![
            lit(format!("{}_", self.prefix())),
            encode(sha256(bytes), lit("hex")),
        ])
    }
    /// Mechanical single-record ingress, then DataFusion constant folding of the same graph.
    /// Corpus computation uses `expression`; this path never creates a private session/runtime.
    /// Derive identity from generated native fields without a Serde/JSON intermediate.
    pub fn record<T: crate::native_union::NativeStruct>(self, record: &T) -> Result<String> {
        self.batch_value(&T::batch(std::slice::from_ref(record))?)
    }
    /// Hex is a mechanical boundary representation of the native binary hash.
    pub fn hex_digest<T: crate::native_union::NativeStruct>(self, record: &T) -> Result<String> {
        Ok(self
            .record_digest(record)?
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
    /// Native hash bytes, with no Serde or JSON ingress.
    pub fn record_digest<T: crate::native_union::NativeStruct>(
        self,
        record: &T,
    ) -> Result<[u8; 32]> {
        let batch = T::batch(std::slice::from_ref(record))?;
        let schema = self.schema();
        let inputs = schema
            .fields()
            .iter()
            .map(|field| {
                let actual = batch.schema().field_with_name(field.name())?.clone();
                crate::native_schema::function_arguments(
                    &[Arc::new(actual.clone())],
                    std::slice::from_ref(field),
                )?;
                let array = batch.column_by_name(field.name()).ok_or_else(|| {
                    DataFusionError::Plan(format!("missing identity field {}", field.name()))
                })?;
                Ok(Expr::Literal(
                    ScalarValue::try_from_array(array, 0)?,
                    Some(datafusion::common::metadata::FieldMetadata::from(&actual)),
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let bytes = crate::native_identity::canonical_bytes(
            format!("enrichment/identity/10/{}", self.prefix()),
            schema.fields().clone(),
        )
        .call(inputs);
        match constant(sha256(bytes))? {
            Expr::Literal(ScalarValue::Binary(Some(bytes)), _) => bytes.try_into().map_err(|_| {
                DataFusionError::Internal("SHA256 width differs from its contract".into())
            }),
            _ => Err(DataFusionError::Internal(
                "native digest failed to reduce to one binary scalar".into(),
            )),
        }
    }
    /// Derive a bounded boundary key from the authoritative mechanical Arrow fields.
    pub fn batch_value(self, batch: &RecordBatch) -> Result<String> {
        if batch.num_rows() != 1 {
            return Err(DataFusionError::Plan(
                "identity boundary requires exactly one row".into(),
            ));
        }
        let inputs = self
            .schema()
            .fields()
            .iter()
            .map(|field| {
                let column = batch.column_by_name(field.name()).ok_or_else(|| {
                    DataFusionError::Plan(format!("missing identity field {}", field.name()))
                })?;
                Ok(Expr::Literal(
                    ScalarValue::try_from_array(column, 0)?,
                    Some(datafusion::common::metadata::FieldMetadata::from(
                        batch.schema().field_with_name(field.name())?,
                    )),
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let value = constant(self.expression_for(inputs))?;
        match value {
            Expr::Literal(
                ScalarValue::Utf8(Some(value)) | ScalarValue::Utf8View(Some(value)),
                _,
            ) => Ok(value),
            _ => Err(DataFusionError::Internal(
                "native identity failed to reduce to one text scalar".into(),
            )),
        }
    }
}

fn constant(expression: Expr) -> Result<Expr> {
    let context = SimplifyContext::builder()
        .with_schema(Arc::new(DFSchema::empty()))
        .build();
    ExprSimplifier::new(context).simplify(expression)
}

/// Mechanical one-value protocol boundary around native ordered-set expressions.
/// Element types and enum codecs come from the declaration, never a second vocabulary.
pub fn ordered_set<T: crate::native_union::Cell + Clone>(values: &[T]) -> Result<Vec<T>> {
    use crate::{
        evidence::arrow_model::cells::RowSet,
        native_union::{Cell, Rule},
    };
    use datafusion::functions_nested::expr_fn::{array_distinct, array_sort};
    let values = values.to_vec();
    let array = <Vec<T> as Cell>::encode(&[Some(&values)])?;
    let input = ScalarValue::try_from_array(&array, 0)?;
    let sorted = constant(array_sort(
        array_distinct(lit(input)),
        lit("ASC"),
        lit("NULLS FIRST"),
    ))?;
    let Expr::Literal(value, _) = sorted else {
        return Err(DataFusionError::Internal(
            "native ordered set did not reduce".into(),
        ));
    };
    let field = crate::native_union::field::<Vec<T>>("values", Rule::Set);
    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![field])),
        vec![value.to_array_of_size(1)?],
    )?;
    let rows = RowSet::batch(&batch)?;
    Ok(<Vec<T> as Cell>::decode(rows.row(0), "values")?)
}

pub fn ordered_strings(values: &[String]) -> Result<Vec<String>> {
    ordered_set(values)
}

fn without_check<T: crate::native_union::NativeStruct>() -> SchemaRef {
    Arc::new(Schema::new(
        T::fields()
            .iter()
            .filter(|field| field.name() != "check")
            .cloned()
            .collect::<Vec<_>>(),
    ))
}

fn native_fields<T: crate::native_union::NativeStruct>() -> SchemaRef {
    Arc::new(Schema::new(T::fields()))
}
