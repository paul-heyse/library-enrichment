//! The sole target snapshot contract. Table bytes and semantic identity are distinct.

use super::{EvidenceKind, ObservedConfiguration, SnapshotCounts};
use crate::identity::{
    Context, ContextId, Ecosystem, Environment, EnvironmentId, Release, ReleaseId, SnapshotId,
    SnapshotInputs,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FORMAT: &str = "8.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SnapshotMetadata {
    pub context: Context,
    pub release: Release,
    pub environment: Environment,
    pub symbol_package: String,
    pub crate_name: String,
    pub crate_version: Option<String>,
    pub normalizer_version: String,
    pub observed_configuration: Option<ObservedConfiguration>,
    pub producer_items: u64,
}

impl SnapshotMetadata {
    #[must_use]
    pub fn descriptor(&self) -> SnapshotDescriptor {
        SnapshotDescriptor {
            context_id: self.context.context_id.clone(),
            release_id: self.release.release_id.clone(),
            environment_id: self.environment.environment_id.clone(),
            ecosystem: self.release.key.ecosystem,
            symbol_package: self.symbol_package.clone(),
            crate_name: self.crate_name.clone(),
            crate_version: self.crate_version.clone(),
            normalizer_version: self.normalizer_version.clone(),
            observed_configuration: self.observed_configuration.clone(),
            producer_items: self.producer_items,
        }
    }

    /// # Errors
    /// Scope must be a valid, coherent context and identities must recompute.
    pub fn validate(&self) -> Result<(), String> {
        if self.context.release_id != self.release.release_id
            || self.release.release_id != self.release.key.id()
            || self.context.environment_id != self.environment.environment_id
            || !self.environment.has_valid_identity()
            || self.context.context_id
                != Context::new(
                    self.release.release_id.clone(),
                    self.environment.environment_id.clone(),
                    self.context.mode,
                )
                .context_id
            || self.symbol_package.is_empty()
            || self.crate_name.is_empty()
            || self.normalizer_version.is_empty()
        {
            return Err("snapshot metadata has invalid identity or scope".into());
        }
        Ok(())
    }
}

crate::native_struct! {
/// Small snapshot publication metadata. Release/environment records belong to catalog Parquet.
pub struct SnapshotDescriptor {
    context_id: ContextId => crate::native_union::Rule::Text,
    release_id: ReleaseId => crate::native_union::Rule::Text,
    environment_id: EnvironmentId => crate::native_union::Rule::Text,
    ecosystem: Ecosystem => crate::native_union::Rule::Text,
    symbol_package: String => crate::native_union::Rule::NonEmpty,
    crate_name: String => crate::native_union::Rule::NonEmpty,
    crate_version: Option<String> => crate::native_union::Rule::Text,
    normalizer_version: String => crate::native_union::Rule::NonEmpty,
    observed_configuration: Option<ObservedConfiguration> => crate::native_union::Rule::Text,
    producer_items: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct DeltaBinding {
    relation: String => crate::native_union::Rule::NonEmpty,
    table_uri: String => crate::native_union::Rule::NonEmpty,
    table_id: String => crate::native_union::Rule::NonEmpty,
    version: u64 => crate::native_union::Rule::Text,
    cohort_id: String => crate::native_union::Rule::NonEmpty,
    contract_id: String => crate::native_union::Rule::NonEmpty,
    rows: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! {
pub struct EvidenceManifest {
    snapshot_id: SnapshotId => crate::native_union::Rule::Text,
    schema_version: String => crate::native_union::Rule::Vocabulary(vec![FORMAT.into()]),
    metadata: SnapshotDescriptor => crate::native_union::Rule::Text,
    components: BTreeMap<String, String> => crate::native_union::Rule::Map,
    tables: Vec<DeltaBinding> => crate::native_union::Rule::SequenceBounds { min: 1, max: 32 },
    counts: SnapshotCounts => crate::native_union::Rule::Text,
    indexed: Vec<EvidenceKind> => crate::native_union::Rule::Set,
    missing: Vec<EvidenceKind> => crate::native_union::Rule::Set,
    published_at: crate::native_time::ObservationTime => crate::native_union::Rule::Text,
}
}

impl std::ops::Deref for EvidenceManifest {
    type Target = SnapshotDescriptor;
    fn deref(&self) -> &Self::Target {
        &self.metadata
    }
}

impl EvidenceManifest {
    /// Derive semantic identity without table bytes, clock or attempt attribution.
    /// # Errors
    /// Invalid scope or serialization cannot produce a default identity.
    pub fn derive_id(
        metadata: &SnapshotDescriptor,
        components: &BTreeMap<String, String>,
    ) -> Result<SnapshotId, String> {
        if metadata.symbol_package.is_empty()
            || metadata.crate_name.is_empty()
            || metadata.normalizer_version.is_empty()
        {
            return Err("snapshot descriptor is incomplete".into());
        }
        let qualifiers = crate::native_key::Key::SnapshotDescriptor
            .record(metadata)
            .map_err(|error| error.to_string())?;
        Ok(SnapshotId::derive(&SnapshotInputs {
            schema_version: FORMAT.into(),
            normalizer_version: metadata.normalizer_version.clone(),
            context_id: metadata.context_id.clone(),
            input_digests: components.clone(),
            producers: [("qualifiers".into(), qualifiers)].into_iter().collect(),
        }))
    }

    /// # Errors
    /// Reject foreign epochs, incoherent identities and unsafe native table references.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != FORMAT
            || self.snapshot_id != Self::derive_id(&self.metadata, &self.components)?
        {
            return Err("unsupported or inconsistent snapshot identity".into());
        }
        for table in &self.tables {
            if table.table_uri != format!("evidence_{}", table.relation)
                || !table
                    .relation
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b == b'_')
                || table.table_id.is_empty()
                || table.cohort_id.is_empty()
                || table.contract_id.len() != 64
                || !table
                    .contract_id
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err("invalid exact Delta table binding".into());
            }
        }
        Ok(())
    }
}
