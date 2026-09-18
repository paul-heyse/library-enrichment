//! Finite Rust enum declarations generate Arrow variants and mechanical boundary codecs.
//! There is no JSON intermediate representation for known variants. Only the explicitly open
//! producer extension is encoded as arrow.json.
use crate::evidence::arrow_model::cells::{self, Row};
use arrow::{
    array::{ArrayRef, UInt16Array, UInt32Array, UInt64Array},
    datatypes::{DataType, Field, Fields},
    error::ArrowError,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Symbol,
    Definition,
    Release,
    Environment,
    Context,
    Snapshot,
    Artifact,
    ProducerBinding,
    Interest,
    Job,
    Attempt,
    EffectGrant,
    OperationPolicy,
    ProcessOperation,
    ProcessEffect,
    StaticWorkerEffect,
    RustdocDecoderEffect,
    SemanticConversation,
    RegistryCapture,
    RevisionCapture,
    RetentionPolicy,
    RetentionLease,
    CleanupObligation,
    MaintenanceRun,
    PrivateDirectory,
    PhysicalOwner,
    StorageReservation,
    Cohort,
    SchemaContract,
}

impl Domain {
    /// Rendered wire prefixes share the native key declaration.
    pub fn prefix(self) -> &'static str {
        use crate::native_key::Key;
        match self {
            Self::Symbol => Key::PublicBinding.prefix(),
            Self::Definition => Key::Definition.prefix(),
            Self::Release => Key::Release.prefix(),
            Self::Environment => Key::Environment.prefix(),
            Self::Context => Key::Context.prefix(),
            Self::Snapshot => Key::Snapshot.prefix(),
            Self::Artifact => "art",
            Self::ProducerBinding => Key::ProducerBinding.prefix(),
            Self::Interest => "interest",
            Self::Job => "job",
            Self::Attempt => "attempt",
            Self::EffectGrant => Key::EffectGrant.prefix(),
            Self::OperationPolicy => Key::OperationPolicy.prefix(),
            Self::ProcessOperation => Key::ProcessOperation.prefix(),
            Self::ProcessEffect => Key::ProcessEffect.prefix(),
            Self::StaticWorkerEffect => Key::StaticWorkerEffect.prefix(),
            Self::RustdocDecoderEffect => Key::RustdocDecoderEffect.prefix(),
            Self::SemanticConversation => Key::SemanticConversation.prefix(),
            Self::RegistryCapture => Key::RegistryCapture.prefix(),
            Self::RevisionCapture => Key::RevisionCapture.prefix(),
            Self::RetentionPolicy => Key::RetentionPolicy.prefix(),
            Self::RetentionLease => "retention_lease",
            Self::CleanupObligation => "cleanup_obligation",
            Self::MaintenanceRun => "maintenance_run",
            Self::PrivateDirectory => "private_directory",
            Self::PhysicalOwner => "physical_owner",
            Self::StorageReservation => "storage_reservation",
            Self::Cohort => "cohort",
            Self::SchemaContract => Key::SchemaContract.prefix(),
        }
    }
    /// The exact release is bound by the admitted snapshot; other domains name evidence keys.
    pub fn evidence_target(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Symbol => Some(("symbols", "symbol_id")),
            Self::Definition => Some(("definitions", "definition_id")),
            Self::Artifact => Some(("input_artifacts", "artifact_id")),
            Self::ProducerBinding => Some(("producer_runs", "producer_binding_id")),
            Self::Release
            | Self::Environment
            | Self::Context
            | Self::Snapshot
            | Self::Interest
            | Self::Job
            | Self::Attempt
            | Self::EffectGrant
            | Self::OperationPolicy
            | Self::ProcessOperation
            | Self::ProcessEffect
            | Self::StaticWorkerEffect
            | Self::RustdocDecoderEffect
            | Self::SemanticConversation
            | Self::RegistryCapture
            | Self::RevisionCapture
            | Self::RetentionPolicy
            | Self::RetentionLease
            | Self::CleanupObligation
            | Self::MaintenanceRun
            | Self::PrivateDirectory
            | Self::PhysicalOwner
            | Self::StorageReservation
            | Self::Cohort
            | Self::SchemaContract => None,
        }
    }
    /// Identity width is part of its domain, never inferred from a supplied value.
    pub fn byte_width(self) -> i32 {
        match self {
            Self::Interest
            | Self::Job
            | Self::Attempt
            | Self::RetentionLease
            | Self::CleanupObligation
            | Self::MaintenanceRun
            | Self::PrivateDirectory
            | Self::PhysicalOwner
            | Self::StorageReservation
            | Self::Cohort => 16,
            _ => 32,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    LineOneBased,
    LineZeroBased,
    Utf8Byte,
    Utf16CodeUnit,
    ByteOffset,
    Ordinal,
    RustdocItem,
}

pub trait NativeStruct: Sized {
    fn fields() -> Fields;
    fn encode(rows: &[Option<&Self>]) -> Result<ArrayRef, ArrowError>;
    fn decode(row: Row<'_>) -> Result<Self, ArrowError>;
    fn batch(rows: &[Self]) -> Result<arrow::record_batch::RecordBatch, ArrowError> {
        let array = Self::encode(&rows.iter().map(Some).collect::<Vec<_>>())?;
        let value = array
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .ok_or_else(|| cells::invalid("native record requires StructArray"))?;
        Ok(arrow::record_batch::RecordBatch::from(value.clone()))
    }
}

#[macro_export]
macro_rules! native_struct {
    // One declaration generates the checked wire input and native decoder as well as the
    // public record. Cross-field boundary validation runs for both Arrow and Serde input.
    (@checked $raw:ident; $(#[$attr:meta])* $vis:vis struct $name:ident { $($(#[$member_attr:meta])* $member:ident : $ty:ty => $rule:expr),* $(,)? }) => {
        #[derive(serde::Deserialize, schemars::JsonSchema)]
        #[serde(deny_unknown_fields)]
        #[schemars(inline)]
        struct $raw {
            $(#[serde(deserialize_with = "enrichment_core::native_wire::deserialize")]
                $member: $ty,)*
        }
        $crate::native_struct! { @record [#[serde(deny_unknown_fields)]]
            [validate |value: Self| Self::try_from($raw { $($member: value.$member,)* })
                .map_err($crate::evidence::arrow_model::cells::invalid)]
            $(#[$attr])* $vis struct $name { $($(#[$member_attr])* $member: $ty => $rule,)* }
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident { $($(#[$member_attr:meta])* $member:ident : $ty:ty => $rule:expr),* $(,)? } $(ephemeral { $($ephemeral:ident : $ephemeral_ty:ty = $initial:expr),* $(,)? })?) => {
        $crate::native_struct! { @record [#[serde(deny_unknown_fields)]] [validate Ok]
            $(#[$attr])* $vis struct $name { $($(#[$member_attr])* $member: $ty => $rule,)* }
            $(ephemeral { $($ephemeral: $ephemeral_ty = $initial,)* })?
        }
    };
    // Open external source records preserve unmodeled fields in the raw source artifact.
    // Service-owned declarations remain closed by default.
    (@source $(#[$attr:meta])* $vis:vis struct $name:ident { $($(#[$member_attr:meta])* $member:ident : $ty:ty => $rule:expr),* $(,)? }) => {
        $crate::native_struct! { @record [] [validate Ok] $(#[$attr])* $vis struct $name { $($(#[$member_attr])* $member: $ty => $rule,)* } }
    };
    (@record [$($contract:tt)*] [validate $validate:expr] $(#[$attr:meta])* $vis:vis struct $name:ident { $($(#[$member_attr:meta])* $member:ident : $ty:ty => $rule:expr),* $(,)? } $(ephemeral { $($ephemeral:ident : $ephemeral_ty:ty = $initial:expr),* $(,)? })?) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        $(#[$attr])*
        $($contract)*
        $vis struct $name { $($(#[$member_attr])* #[schemars(transform = |schema: &mut schemars::Schema| enrichment_core::native_wire::field_rule(schema, $rule))] #[serde(serialize_with = "enrichment_core::native_wire::serialize", deserialize_with = "enrichment_core::native_wire::deserialize")] pub $member: $ty,)* $($(#[serde(skip)] pub $ephemeral: $ephemeral_ty,)*)? }
        impl $crate::native_union::NativeStruct for $name {
            fn fields() -> arrow::datatypes::Fields {
                let fields: Vec<arrow::datatypes::Field> = vec![$($crate::native_union::field::<$ty>(stringify!($member), $rule)),*];
                fields.into()
            }
            fn encode(rows: &[Option<&Self>]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                $crate::evidence::arrow_model::cells::structure(vec![
                    $(($crate::native_union::field::<$ty>(stringify!($member), $rule), <$ty as $crate::native_union::Cell>::encode(
                        &rows.iter().map(|row| row.map(|row| &row.$member)).collect::<Vec<_>>()
                    )?)),*
                ], Some(rows.iter().map(Option::is_some).collect()))
            }
            fn decode(row: $crate::evidence::arrow_model::cells::Row<'_>) -> Result<Self, arrow::error::ArrowError> {
                row.exact_fields(&Self::fields())?;
                ($validate)(Self { $($member: <$ty as $crate::native_union::Cell>::decode(row, stringify!($member))?,)* $($($ephemeral: $initial,)*)? })
            }
        }
        impl $crate::native_union::Cell for $name {
            fn data_type() -> arrow::datatypes::DataType {
                arrow::datatypes::DataType::Struct(<Self as $crate::native_union::NativeStruct>::fields())
            }
            fn encode(rows: &[Option<&Self>]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                <Self as $crate::native_union::NativeStruct>::encode(rows)
            }
            fn decode(row: $crate::evidence::arrow_model::cells::Row<'_>, name: &str) -> Result<Self, arrow::error::ArrowError> {
                <Self as $crate::native_union::NativeStruct>::decode(row.structure(name)?)
            }
            fn empty_record() -> Result<Self, arrow::error::ArrowError> {
                let batch = arrow::record_batch::RecordBatch::from(arrow::array::StructArray::new_empty_fields(1, None));
                let rows = $crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
                <Self as $crate::native_union::NativeStruct>::decode(rows.row(0))
            }
        }
    };
}

/// A record may place selected large fields in sibling columns while retaining one Rust,
/// wire and semantic declaration. Compact-body codecs are generated from that declaration.
#[macro_export]
macro_rules! native_split_record {
    ($(#[$attr:meta])* $vis:vis struct $name:ident {
        $($member:ident : $ty:ty => $rule:expr),* $(,)?
    } separated {
        $($separate:ident : $separate_ty:ty => $separate_rule:expr),* $(,)?
    }) => {
        $crate::native_struct! { $(#[$attr])* $vis struct $name {
            $($member: $ty => $rule,)* $($separate: $separate_ty => $separate_rule,)*
        } }
        impl $name {
            pub fn body_fields() -> arrow::datatypes::Fields {
                vec![$($crate::native_union::field::<$ty>(stringify!($member), $rule)),*].into()
            }
            pub fn encode_body(rows: &[&Self]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                $crate::evidence::arrow_model::cells::structure(vec![
                    $(($crate::native_union::field::<$ty>(stringify!($member), $rule), <$ty as $crate::native_union::Cell>::encode(
                        &rows.iter().map(|row| Some(&row.$member)).collect::<Vec<_>>()
                    )?)),*
                ], None)
            }
            pub fn separate_columns(rows: &[&Self]) -> Result<Vec<(arrow::datatypes::Field, arrow::array::ArrayRef)>, arrow::error::ArrowError> {
                Ok(vec![$((
                    $crate::native_union::field::<$separate_ty>(stringify!($separate), $separate_rule),
                    <$separate_ty as $crate::native_union::Cell>::encode(&rows.iter().map(|row| Some(&row.$separate)).collect::<Vec<_>>())?
                )),*])
            }
            pub fn decode_body(row: $crate::evidence::arrow_model::cells::Row<'_>, $($separate: $separate_ty),*) -> Result<Self, arrow::error::ArrowError> {
                row.exact_fields(&Self::body_fields())?;
                Ok(Self { $($member: <$ty as $crate::native_union::Cell>::decode(row, stringify!($member))?,)* $($separate,)* })
            }
        }
    };
}

/// Rules are explicit declarations, never inferred from field names or presentation roles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum Rule {
    Text,
    /// A native record whose members are flattened at the JSON transport boundary.
    Flatten,
    /// Independently addressable result section, generated from the payload declaration.
    Section(String),
    Documentation,
    NonEmpty,
    Sha256,
    ArtifactIdentity {
        digest: String,
    },
    Reference(Domain),
    ScopedReference {
        domain: Domain,
        scope: Vec<ScopeKey>,
    },
    /// A key in the same bound catalog/schema. Its declaration travels with the field;
    /// the caller supplies the immutable namespace, never an ambient search path.
    ForeignKey {
        table: String,
        field: Vec<String>,
        scope: Vec<ScopeKey>,
    },
    Coordinate(Unit),
    RangeEnd {
        unit: Unit,
        start: String,
    },
    MemberPath,
    DocumentUri,
    Json,
    PythonDeclarationOrigin,
    Sequence,
    SequenceBounds {
        min: u64,
        max: u64,
    },
    BinaryBytes {
        max: u64,
    },
    UnsignedRange {
        min: u64,
        max: u64,
    },
    /// Order is immaterial; repeated values, including repeated NULLs, are invalid.
    /// Canonical hashing normalizes order but never substitutes for admission.
    Set,
    Vocabulary(Vec<String>),
    Map,
}

/// One reference descriptor drives native joins and catalog discovery.
pub enum ReferenceTarget {
    Evidence(Domain),
    Relation { table: String, field: Vec<String> },
}

impl Rule {
    #[must_use]
    pub fn foreign_key(table: &str, field: &str) -> Self {
        Self::ForeignKey {
            table: table.into(),
            field: vec![field.into()],
            scope: Vec::new(),
        }
    }

    #[must_use]
    pub fn reference(self) -> Option<(ReferenceTarget, Vec<ScopeKey>)> {
        match self {
            Self::Reference(domain) => Some((ReferenceTarget::Evidence(domain), Vec::new())),
            Self::ScopedReference { domain, scope } => {
                Some((ReferenceTarget::Evidence(domain), scope))
            }
            Self::ForeignKey {
                table,
                field,
                scope,
            } => Some((ReferenceTarget::Relation { table, field }, scope)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeKey {
    /// Source fields are relative to the record containing the reference.
    pub source: Vec<String>,
    pub target: Vec<String>,
    pub null: ScopeNull,
}

impl ScopeKey {
    #[must_use]
    pub fn exact(source: &[&str], target: &[&str]) -> Self {
        Self {
            source: source.iter().map(|part| (*part).into()).collect(),
            target: target.iter().map(|part| (*part).into()).collect(),
            null: ScopeNull::Exact,
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeNull {
    Exact,
    Unspecified,
}

#[macro_export]
macro_rules! native_vocabulary {
    ($(#[$attr:meta])* $vis:vis enum $name:ident { $($(#[$variant_attr:meta])* $variant:ident = $token:literal),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        $(#[$attr])*
        $vis enum $name { $($(#[$variant_attr])* #[serde(rename = $token)] $variant),* }
        impl $name {
            pub const VALUES: &'static [&'static str] = &[$($token),*];
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $token),* } }
            pub fn parse(value: &str) -> Option<Self> { match value { $($token => Some(Self::$variant)),*, _ => None } }
        }
        impl $crate::native_union::Cell for $name {
            fn data_type() -> arrow::datatypes::DataType { arrow::datatypes::DataType::Utf8 }
            fn metadata() -> std::collections::HashMap<String, String> {
                std::collections::HashMap::from([("enrichment.vocabulary".into(), serde_json::to_string(Self::VALUES).expect("finite vocabulary"))])
            }
            fn encode(values: &[Option<&Self>]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                Ok($crate::evidence::arrow_model::cells::optional(values.iter().map(|value| value.map(|value| value.as_str()))))
            }
            fn decode(row: $crate::evidence::arrow_model::cells::Row<'_>, name: &str) -> Result<Self, arrow::error::ArrowError> {
                Self::parse(row.text(name)?).ok_or_else(|| $crate::evidence::arrow_model::cells::invalid(concat!("unknown ", stringify!($name))))
            }
        }
    };
}

pub trait Cell: Sized + serde::Serialize + serde::de::DeserializeOwned {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize(serializer)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Self::deserialize(deserializer)
    }
    fn empty_record() -> Result<Self, ArrowError> {
        Err(cells::invalid("native cell is not an empty record"))
    }
    fn data_type() -> DataType;
    fn metadata() -> HashMap<String, String> {
        HashMap::new()
    }
    fn nullable() -> bool {
        false
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError>;
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError>;
}

impl<const N: usize> Cell for [u8; N]
where
    [u8; N]: serde::Serialize + serde::de::DeserializeOwned,
{
    fn data_type() -> DataType {
        DataType::FixedSizeBinary(i32::try_from(N).expect("declared binary width fits Arrow"))
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(
            arrow::array::FixedSizeBinaryArray::try_from_sparse_iter_with_size(
                values.iter().copied(),
                i32::try_from(N)
                    .map_err(|_| cells::invalid("declared binary width exceeds Arrow"))?,
            )?,
        ))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        row.fixed_binary(name)
    }
}

impl Cell for String {
    fn data_type() -> DataType {
        DataType::Utf8
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(cells::optional(
            values.iter().map(|value| value.map(String::as_str)),
        ))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        Ok(row.text(name)?.into())
    }
}

impl Cell for std::path::PathBuf {
    fn data_type() -> DataType {
        DataType::Utf8
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let values = values
            .iter()
            .map(|value| {
                value
                    .map(|value| {
                        value
                            .to_str()
                            .ok_or_else(|| cells::invalid("native policy path is not UTF-8"))
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(cells::optional(values))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        Ok(row.text(name)?.into())
    }
}

impl Cell for bool {
    fn data_type() -> DataType {
        DataType::Boolean
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(arrow::array::BooleanArray::from(
            values
                .iter()
                .map(|value| value.copied())
                .collect::<Vec<_>>(),
        )))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        row.boolean(name)
    }
}
impl Cell for i32 {
    fn data_type() -> DataType {
        DataType::Int32
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(arrow::array::Int32Array::from(
            values
                .iter()
                .map(|value| value.copied())
                .collect::<Vec<_>>(),
        )))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        Self::try_from(row.signed(name)?).map_err(|error| cells::invalid(error.to_string()))
    }
}
impl Cell for i64 {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        crate::native_wire::signed(deserializer)
    }
    fn data_type() -> DataType {
        DataType::Int64
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(arrow::array::Int64Array::from(
            values
                .iter()
                .map(|value| value.copied())
                .collect::<Vec<_>>(),
        )))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        row.signed(name)
    }
}
impl<T: Cell> Cell for Vec<T> {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.iter().map(crate::native_wire::Ref))
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Vec::<crate::native_wire::Owned<T>>::deserialize(deserializer)
            .map(|values| values.into_iter().map(|value| value.0).collect())
    }
    fn data_type() -> DataType {
        DataType::List(Arc::new(
            field::<T>("item", Rule::Text).with_nullable(T::nullable()),
        ))
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let flat = values
            .iter()
            .flat_map(|value| value.iter().flat_map(|value| value.iter().map(Some)))
            .collect::<Vec<_>>();
        let array = T::encode(&flat)?;
        let mut offsets = vec![0i32];
        let mut offset = 0i32;
        for value in values {
            offset = offset
                .checked_add(
                    i32::try_from(value.map_or(0, Vec::len))
                        .map_err(|error| cells::invalid(error.to_string()))?,
                )
                .ok_or_else(|| cells::invalid("list offset overflow"))?;
            offsets.push(offset);
        }
        Ok(Arc::new(arrow::array::ListArray::try_new(
            Arc::new(field::<T>("item", Rule::Text).with_nullable(T::nullable())),
            arrow::buffer::OffsetBuffer::new(offsets.into()),
            array,
            Some(arrow::buffer::NullBuffer::from(
                values.iter().map(Option::is_some).collect::<Vec<_>>(),
            )),
        )?))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        let values = row.list_values(name)?;
        let batch = arrow::record_batch::RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![Field::new(
                "item",
                values.data_type().clone(),
                true,
            )])),
            vec![values],
        )?;
        let rows = cells::RowSet::batch(&batch)?;
        (0..batch.num_rows())
            .map(|index| T::decode(rows.row(index), "item"))
            .collect()
    }
}

impl<T: Cell + Ord + Clone> Cell for std::collections::BTreeSet<T> {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.iter().map(crate::native_wire::Ref))
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let values = Vec::<crate::native_wire::Owned<T>>::deserialize(deserializer)?;
        let size = values.len();
        let set: Self = values.into_iter().map(|value| value.0).collect();
        if set.len() != size {
            return Err(serde::de::Error::custom(
                "native set contains repeated values",
            ));
        }
        Ok(set)
    }
    fn data_type() -> DataType {
        Vec::<T>::data_type()
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let rows = values
            .iter()
            .map(|value| value.map(|value| value.iter().cloned().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        Vec::<T>::encode(&rows.iter().map(Option::as_ref).collect::<Vec<_>>())
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        let values = Vec::<T>::decode(row, name)?;
        let size = values.len();
        let set: Self = values.into_iter().collect();
        if set.len() != size {
            return Err(cells::invalid("native set contains repeated values"));
        }
        Ok(set)
    }
}

impl<T: Cell> Cell for std::collections::BTreeMap<String, T> {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(
            self.iter()
                .map(|(key, value)| (key, crate::native_wire::Ref(value))),
        )
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        std::collections::BTreeMap::<String, crate::native_wire::Owned<T>>::deserialize(
            deserializer,
        )
        .map(|values| {
            values
                .into_iter()
                .map(|(key, value)| (key, value.0))
                .collect()
        })
    }
    fn data_type() -> DataType {
        DataType::Map(
            Arc::new(Field::new(
                "entries",
                DataType::Struct(
                    vec![
                        field::<String>("key", Rule::Text).with_nullable(false),
                        field::<T>("value", Rule::Text),
                    ]
                    .into(),
                ),
                false,
            )),
            false,
        )
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let flat = values
            .iter()
            .flat_map(|value| value.iter().flat_map(|value| value.iter()))
            .collect::<Vec<_>>();
        let DataType::Map(entries, sorted) = Self::data_type() else {
            unreachable!("declared map");
        };
        let DataType::Struct(fields) = entries.data_type() else {
            unreachable!("declared entries");
        };
        let array = arrow::array::StructArray::try_new(
            fields.clone(),
            vec![
                String::encode(&flat.iter().map(|(key, _)| Some(*key)).collect::<Vec<_>>())?,
                T::encode(
                    &flat
                        .iter()
                        .map(|(_, value)| Some(*value))
                        .collect::<Vec<_>>(),
                )?,
            ],
            None,
        )?;
        let mut offsets = vec![0i32];
        let mut offset = 0i32;
        for value in values {
            offset = offset
                .checked_add(
                    i32::try_from(value.map_or(0, std::collections::BTreeMap::len))
                        .map_err(|error| cells::invalid(error.to_string()))?,
                )
                .ok_or_else(|| cells::invalid("map offset overflow"))?;
            offsets.push(offset);
        }
        Ok(Arc::new(arrow::array::MapArray::try_new(
            entries,
            arrow::buffer::OffsetBuffer::new(offsets.into()),
            array,
            Some(arrow::buffer::NullBuffer::from(
                values.iter().map(Option::is_some).collect::<Vec<_>>(),
            )),
            sorted,
        )?))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        let batch = arrow::record_batch::RecordBatch::from(row.map_entries(name)?);
        let rows = cells::RowSet::batch(&batch)?;
        let mut result = Self::new();
        for index in 0..batch.num_rows() {
            let row = rows.row(index);
            if result
                .insert(row.text("key")?.into(), T::decode(row, "value")?)
                .is_some()
            {
                return Err(cells::invalid("duplicate map key"));
            }
        }
        Ok(result)
    }
}
macro_rules! unsigned_cell {
    ($value:ty, $kind:ident, $array:ident) => {
        impl Cell for $value {
            fn data_type() -> DataType {
                DataType::$kind
            }
            fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
                Ok(Arc::new($array::from(
                    values
                        .iter()
                        .map(|value| value.copied())
                        .collect::<Vec<_>>(),
                )))
            }
            fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
                Self::try_from(row.number(name)?).map_err(|error| cells::invalid(error.to_string()))
            }
        }
    };
}
unsigned_cell!(u16, UInt16, UInt16Array);
unsigned_cell!(u32, UInt32, UInt32Array);
impl Cell for u64 {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        crate::native_wire::unsigned(deserializer)
    }
    fn data_type() -> DataType {
        DataType::UInt64
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        Ok(Arc::new(UInt64Array::from(
            values
                .iter()
                .map(|value| value.copied())
                .collect::<Vec<_>>(),
        )))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        row.number(name)
    }
}
impl Cell for usize {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Self::try_from(crate::native_wire::unsigned(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
    fn data_type() -> DataType {
        DataType::UInt64
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let values = values
            .iter()
            .map(|value| value.map(|value| u64::try_from(*value)).transpose())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| cells::invalid(e.to_string()))?;
        Ok(Arc::new(UInt64Array::from(values)))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        Self::try_from(row.number(name)?).map_err(|e| cells::invalid(e.to_string()))
    }
}
impl<T: Cell> Cell for Box<T> {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        T::serialize_wire(self.as_ref(), serializer)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        T::deserialize_wire(deserializer).map(Box::new)
    }
    fn data_type() -> DataType {
        T::data_type()
    }
    fn metadata() -> HashMap<String, String> {
        T::metadata()
    }
    fn nullable() -> bool {
        T::nullable()
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        T::encode(
            &values
                .iter()
                .map(|value| value.map(|value| value.as_ref()))
                .collect::<Vec<_>>(),
        )
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        T::decode(row, name).map(Box::new)
    }
}
impl<T: Cell> Cell for Option<T> {
    fn serialize_wire<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref()
            .map(crate::native_wire::Ref)
            .serialize(serializer)
    }
    fn deserialize_wire<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Option::<crate::native_wire::Owned<T>>::deserialize(deserializer)
            .map(|value| value.map(|value| value.0))
    }
    fn metadata() -> HashMap<String, String> {
        T::metadata()
    }
    fn data_type() -> DataType {
        T::data_type()
    }
    fn nullable() -> bool {
        true
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        T::encode(
            &values
                .iter()
                .map(|value| value.and_then(Option::as_ref))
                .collect::<Vec<_>>(),
        )
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        if row.is_null(name)? {
            Ok(None)
        } else {
            T::decode(row, name).map(Some)
        }
    }
}
impl Cell for serde_json::Value {
    fn data_type() -> DataType {
        DataType::Utf8
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        let encoded = values
            .iter()
            .map(|value| value.map(serde_json::to_string).transpose())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| ArrowError::JsonError(error.to_string()))?;
        Ok(cells::optional(encoded.iter().map(Option::as_deref)))
    }
    fn decode(row: Row<'_>, name: &str) -> Result<Self, ArrowError> {
        serde_json::from_str(row.text(name)?)
            .map_err(|error| ArrowError::JsonError(error.to_string()))
    }
}
pub fn field<T: Cell>(name: &str, rule: Rule) -> Field {
    let mut field = Field::new(name, T::data_type(), true).with_metadata(HashMap::from([
        (
            "enrichment.null".into(),
            if T::nullable() {
                "unobserved"
            } else {
                "forbidden"
            }
            .into(),
        ),
        (
            "enrichment.rule".into(),
            serde_json::to_string(&rule).expect("finite rule serialization"),
        ),
    ]));
    let mut metadata = field.metadata().clone();
    metadata.extend(T::metadata());
    if let Rule::Section(name) = &rule {
        metadata.insert("enrichment.section".into(), name.clone());
    }
    field = field.with_metadata(metadata);
    if matches!(rule, Rule::Json) {
        field
            .try_with_extension_type(arrow_schema::extension::Json::default())
            .expect("JSON rule has a declared text representation");
    }
    field
}

pub fn discriminator(tags: &[&str]) -> Field {
    discriminator_named("kind", tags)
}
pub fn discriminator_named(name: &str, tags: &[&str]) -> Field {
    Field::new(name, DataType::Utf8, false).with_metadata(HashMap::from([
        ("enrichment.null".into(), "forbidden".into()),
        (
            "enrichment.union.tags".into(),
            serde_json::to_string(tags).expect("finite variant serialization"),
        ),
    ]))
}

pub trait NativeUnion: Sized {
    fn kind(&self) -> &'static str;
    fn fields() -> Fields;
    fn encode(rows: &[&Self]) -> Result<ArrayRef, ArrowError>;
    fn decode(row: Row<'_>) -> Result<Self, ArrowError>;
}

pub fn optional_union<T: NativeUnion>(rows: &[Option<&T>]) -> Result<ArrayRef, ArrowError> {
    let dense = rows.iter().filter_map(|row| *row).collect::<Vec<_>>();
    let values = T::encode(&dense)?;
    let mut next = 0usize;
    let indices = rows
        .iter()
        .map(|row| {
            if row.is_none() {
                return Ok(None);
            }
            let index = u32::try_from(next).map_err(|error| cells::invalid(error.to_string()))?;
            next += 1;
            Ok(Some(index))
        })
        .collect::<Result<Vec<_>, ArrowError>>()?;
    arrow::compute::take(values.as_ref(), &UInt32Array::from(indices), None)
}

#[macro_export]
macro_rules! native_union_cell {
    ($name:ident) => {
        impl $crate::native_union::Cell for $name {
            fn data_type() -> arrow::datatypes::DataType {
                arrow::datatypes::DataType::Struct(
                    <Self as $crate::native_union::NativeUnion>::fields(),
                )
            }
            fn encode(
                rows: &[Option<&Self>],
            ) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                $crate::native_union::optional_union(rows)
            }
            fn decode(
                row: $crate::evidence::arrow_model::cells::Row<'_>,
                name: &str,
            ) -> Result<Self, arrow::error::ArrowError> {
                <Self as $crate::native_union::NativeUnion>::decode(row.structure(name)?)
            }
        }
    };
}

/// Each variant and member is written once. The same declaration produces the Rust enum,
/// Serde/Schemars wire projection, native fields, discriminator rules and Arrow codecs.
#[macro_export]
macro_rules! native_union {
    (@decode $variant:ident, $row:ident, $tag:literal { $($member:ident : $ty:ty),* }) => {{
        let payload = $row.structure($tag)?;
        let fields = <Self as $crate::native_union::NativeUnion>::fields();
        let arrow::datatypes::DataType::Struct(members) = fields.find($tag).expect("generated variant").1.data_type() else { unreachable!("generated struct"); };
        payload.exact_fields(members)?;
        Self::$variant { $($member: <$ty as $crate::native_union::Cell>::decode(payload, stringify!($member))?),* }
    }};
    (@decode $variant:ident, $row:ident, $tag:literal) => { Self::$variant };
    ($(#[$attr:meta])* $vis:vis enum $name:ident { $($body:tt)* }) => {
        $crate::native_union! { @tag "kind"; $(#[$attr])* $vis enum $name { $($body)* } }
    };
    (@tag $discriminator:literal; $(#[$attr:meta])* $vis:vis enum $name:ident {
        $($(#[$variant_attr:meta])* $variant:ident = $tag:literal $({ $($member:ident : $ty:ty => $rule:expr),* $(,)? })?),* $(,)?
    }) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        $(#[$attr])*
        #[serde(tag = $discriminator, deny_unknown_fields)]
        $vis enum $name {
            $($(#[$variant_attr])* #[serde(rename = $tag)] $variant $({ $(#[schemars(transform = |schema: &mut schemars::Schema| enrichment_core::native_wire::field_rule(schema, $rule))] #[serde(serialize_with = "enrichment_core::native_wire::serialize", deserialize_with = "enrichment_core::native_wire::deserialize")] $member: $ty),* })?),*
        }
        $crate::native_union_cell!($name);
        impl $crate::native_union::NativeUnion for $name {
            fn kind(&self) -> &'static str {
                match self { $(Self::$variant $({ $($member: _,)* })? => $tag),* }
            }
            fn fields() -> arrow::datatypes::Fields {
                let mut fields = vec![$crate::native_union::discriminator_named($discriminator, &[$($tag),*])];
                $( $( {
                    let members = vec![$($crate::native_union::field::<$ty>(stringify!($member), $rule)),*];
                    fields.push(arrow::datatypes::Field::new($tag, arrow::datatypes::DataType::Struct(members.into()), true));
                } )? )*
                fields.into()
            }
            fn encode(rows: &[&Self]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                let fields = Self::fields();
                let mut columns = vec![(fields[0].as_ref().clone(), $crate::evidence::arrow_model::cells::text(rows.iter().map(|row| row.kind())))];
                $( $( {
                    let payload = $crate::evidence::arrow_model::cells::structure(vec![
                        $(($crate::native_union::field::<$ty>(stringify!($member), $rule), <$ty as $crate::native_union::Cell>::encode(&rows.iter().map(|row| {
                            if let Self::$variant { $member, .. } = row { Some($member) } else { None }
                        }).collect::<Vec<_>>())?)),*
                    ], Some(rows.iter().map(|row| matches!(row, Self::$variant { $($member: _,)* })).collect()))?;
                    columns.push((fields.find($tag).expect("generated variant").1.as_ref().clone(), payload));
                } )? )*
                $crate::evidence::arrow_model::cells::structure(columns, None)
            }
            fn decode(row: $crate::evidence::arrow_model::cells::Row<'_>) -> Result<Self, arrow::error::ArrowError> {
                row.exact_fields(&Self::fields())?;
                let tag = row.text($discriminator)?;
                row.variant_named($discriminator, &[tag])?;
                Ok(match tag {
                    $($tag => $crate::native_union!(@decode $variant, row, $tag $({ $($member : $ty),* })?)),*,
                    _ => return Err($crate::evidence::arrow_model::cells::invalid(concat!("unknown ", stringify!($name), " variant"))),
                })
            }
        }
    };
}

#[cfg(test)]
mod declaration_tests {
    use super::*;
    use crate::evidence::arrow_model::cells::RowSet;
    use arrow::{array::StructArray, record_batch::RecordBatch};

    crate::native_union! {
        enum ContractAddition {
            Empty = "empty",
            Added = "added" {
                label: String => Rule::NonEmpty,
                observed: Option<Vec<String>> => Rule::Sequence,
            },
        }
    }

    #[test]
    fn a_single_variant_declaration_reaches_fields_wire_and_mechanical_codecs()
    -> Result<(), ArrowError> {
        let values = [
            ContractAddition::Empty,
            ContractAddition::Added {
                label: "absent".into(),
                observed: None,
            },
            ContractAddition::Added {
                label: "empty".into(),
                observed: Some(vec![]),
            },
        ];
        let array = <ContractAddition as NativeUnion>::encode(&values.iter().collect::<Vec<_>>())?;
        let batch = RecordBatch::from(
            array
                .as_any()
                .downcast_ref::<StructArray>()
                .expect("generated struct")
                .clone(),
        );
        let rows = RowSet::batch(&batch)?;
        for (index, expected) in values.iter().enumerate() {
            assert_eq!(
                <ContractAddition as NativeUnion>::decode(rows.row(index))?,
                *expected
            );
        }
        crate::native_schema::validate(&batch.schema()).expect("generated schema");
        let schema = schemars::schema_for!(ContractAddition)
            .as_value()
            .to_string();
        assert!(schema.contains("observed") && schema.contains("added"));
        assert_ne!(
            crate::native_identity::record_bytes(
                "declaration-test",
                <ContractAddition as NativeUnion>::encode(&[&values[1]])?
            )
            .expect("identity"),
            crate::native_identity::record_bytes(
                "declaration-test",
                <ContractAddition as NativeUnion>::encode(&[&values[2]])?
            )
            .expect("identity")
        );
        Ok(())
    }
}
