//! Finite Arrow extension contracts shared with the DataFusion session registry.
use arrow::datatypes::{DataType, Field, TimeUnit};
use arrow_schema::{
    ArrowError,
    extension::{ExtensionType, Json},
};
use datafusion::{
    common::{Result, types::DFExtensionType},
    logical_expr::registry::{
        ExtensionTypeRegistration, ExtensionTypeRegistry, MemoryExtensionTypeRegistry,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

crate::native_vocabulary! {
    #[derive(Hash)]
    pub enum ClockMeaning {
        Event = "event", Acquisition = "acquisition", Observation = "observation",
        Submission = "submission", Update = "update", Expiry = "expiry",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeMetadata<T> {
    version: u32,
    meaning: T,
}
impl<T> TypeMetadata<T> {
    pub fn meaning(&self) -> &T {
        &self.meaning
    }
    pub fn new(meaning: T) -> Self {
        Self {
            version: 1,
            meaning,
        }
    }
}

macro_rules! extension {
    ($name:ident, $token:literal, $meaning:ty, $physical:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            kind: DataType,
            metadata: TypeMetadata<$meaning>,
        }
        impl ExtensionType for $name {
            const NAME: &'static str = $token;
            type Metadata = TypeMetadata<$meaning>;
            fn metadata(&self) -> &Self::Metadata {
                &self.metadata
            }
            fn serialize_metadata(&self) -> Option<String> {
                Some(serde_json::to_string(&self.metadata).expect("finite extension metadata"))
            }
            fn deserialize_metadata(
                value: Option<&str>,
            ) -> std::result::Result<Self::Metadata, ArrowError> {
                let value = value.ok_or_else(|| {
                    ArrowError::InvalidArgumentError("missing semantic extension metadata".into())
                })?;
                if value.len() > 1024 {
                    return Err(ArrowError::InvalidArgumentError(
                        "semantic extension metadata bound".into(),
                    ));
                }
                let metadata: Self::Metadata = serde_json::from_str(value)
                    .map_err(|e| ArrowError::InvalidArgumentError(e.to_string()))?;
                if metadata.version != 1 {
                    return Err(ArrowError::InvalidArgumentError(
                        "unknown semantic extension version".into(),
                    ));
                }
                Ok(metadata)
            }
            fn supports_data_type(&self, kind: &DataType) -> std::result::Result<(), ArrowError> {
                let physical = ($physical)(self.metadata.meaning());
                if kind == &physical {
                    Ok(())
                } else {
                    Err(ArrowError::InvalidArgumentError(format!(
                        "{} requires {}, received {kind}",
                        Self::NAME,
                        physical
                    )))
                }
            }
            fn try_new(
                kind: &DataType,
                metadata: Self::Metadata,
            ) -> std::result::Result<Self, ArrowError> {
                if metadata.version != 1 {
                    return Err(ArrowError::InvalidArgumentError(
                        "unknown semantic extension version".into(),
                    ));
                }
                let value = Self {
                    kind: kind.clone(),
                    metadata,
                };
                value.supports_data_type(kind)?;
                Ok(value)
            }
        }
        impl DFExtensionType for $name {
            fn storage_type(&self) -> DataType {
                self.kind.clone()
            }
            fn serialize_metadata(&self) -> Option<String> {
                ExtensionType::serialize_metadata(self)
            }
        }
    };
}

extension!(
    ClockType,
    "enrichment.clock",
    ClockMeaning,
    |_: &ClockMeaning| DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into()))
);
extension!(
    IdentityType,
    "enrichment.identity",
    crate::native_union::Domain,
    |domain: &crate::native_union::Domain| DataType::FixedSizeBinary(domain.byte_width())
);
crate::native_vocabulary! { pub enum DigestAlgorithm { Sha256 = "sha256" } }
extension!(
    DigestType,
    "enrichment.digest",
    DigestAlgorithm,
    |_: &DigestAlgorithm| DataType::FixedSizeBinary(32)
);

/// The same finite registrations serve field admission and every native session.
/// # Errors
/// An invalid registration refuses runtime creation.
pub fn registry() -> Result<Arc<MemoryExtensionTypeRegistry>> {
    build_registry()
}
fn build_registry() -> Result<Arc<MemoryExtensionTypeRegistry>> {
    let registry = MemoryExtensionTypeRegistry::new_empty();
    macro_rules! register {
        ($type:ty) => {
            registry.add_extension_type_registration(ExtensionTypeRegistration::new_arc(
                <$type>::NAME,
                |kind, metadata| {
                    Ok(Arc::new(<$type>::try_new(
                        kind,
                        <$type>::deserialize_metadata(metadata)?,
                    )?))
                },
            ))?;
        };
    }
    register!(ClockType);
    register!(IdentityType);
    register!(DigestType);
    // Opaque producer JSON is the sole currently declared canonical Arrow extension.
    let canonical = MemoryExtensionTypeRegistry::new_with_canonical_extension_types();
    registry.add_extension_type_registration(canonical.extension_type_registration(Json::NAME)?)?;
    Ok(Arc::new(registry))
}

/// Refuse unknown extensions and invalid metadata/physical shape before publication.
/// # Errors
/// A field whose extension is not in the finite native registry is rejected.
pub fn validate_field(field: &Field) -> Result<()> {
    if field
        .metadata()
        .contains_key(arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY)
        && !field
            .metadata()
            .contains_key(arrow_schema::extension::EXTENSION_TYPE_NAME_KEY)
    {
        return datafusion::common::plan_err!("extension metadata without a name");
    }
    if field
        .metadata()
        .contains_key(arrow_schema::extension::EXTENSION_TYPE_NAME_KEY)
    {
        // This private validator cannot be changed through a SessionState registry handle.
        static VALIDATOR: std::sync::LazyLock<Arc<MemoryExtensionTypeRegistry>> =
            std::sync::LazyLock::new(|| {
                build_registry().expect("finite native extension registrations")
            });
        VALIDATOR.create_extension_type_for_field(field)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_refuses_unknown_meaning_version_and_physical_shape() {
        let base = Field::new("id", DataType::FixedSizeBinary(32), false);
        for (name, metadata) in [
            ("unknown.identity", r#"{"version":1,"meaning":"symbol"}"#),
            (IdentityType::NAME, r#"{"version":2,"meaning":"symbol"}"#),
            (IdentityType::NAME, r#"{"version":1,"meaning":"unknown"}"#),
            (
                IdentityType::NAME,
                r#"{"version":1,"meaning":"symbol","extra":true}"#,
            ),
        ] {
            let field = base.clone().with_metadata(
                [
                    ("ARROW:extension:name".into(), name.into()),
                    ("ARROW:extension:metadata".into(), metadata.into()),
                ]
                .into(),
            );
            assert!(validate_field(&field).is_err());
        }
        assert!(
            IdentityType::try_new(
                &DataType::Utf8,
                TypeMetadata::new(crate::native_union::Domain::Symbol)
            )
            .is_err()
        );
        assert!(
            ClockType::try_new(
                &DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".into())),
                TypeMetadata::new(ClockMeaning::Event)
            )
            .is_err()
        );
        let clock = Field::new(
            "time",
            DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())),
            false,
        )
        .with_extension_type(
            ClockType::try_new(
                &DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())),
                TypeMetadata::new(ClockMeaning::Event),
            )
            .unwrap(),
        );
        assert!(validate_field(&clock).is_ok());
    }
}
