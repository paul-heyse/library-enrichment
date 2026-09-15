//! The sole target snapshot contract. Table bytes and semantic identity are distinct.

use super::{EvidenceKind, ObservedConfiguration, SnapshotCounts};
use crate::identity::{
    Context, ContextId, Ecosystem, Environment, EnvironmentId, Release, ReleaseId, SnapshotId,
    SnapshotInputs,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FORMAT: &str = "5.0";

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

/// Small snapshot publication metadata. Release/environment records belong to catalog Parquet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDescriptor {
    pub context_id: ContextId,
    pub release_id: ReleaseId,
    pub environment_id: EnvironmentId,
    pub ecosystem: Ecosystem,
    pub symbol_package: String,
    pub crate_name: String,
    pub crate_version: Option<String>,
    pub normalizer_version: String,
    pub observed_configuration: Option<ObservedConfiguration>,
    pub producer_items: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PhysicalTable {
    pub relation: String,
    pub file: String,
    pub sha256: String,
    pub bytes: u64,
    pub rows: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EvidenceManifest {
    pub snapshot_id: SnapshotId,
    pub schema_version: String,
    pub metadata: SnapshotDescriptor,
    pub components: BTreeMap<String, String>,
    pub tables: Vec<PhysicalTable>,
    pub counts: SnapshotCounts,
    pub indexed: Vec<EvidenceKind>,
    pub missing: Vec<EvidenceKind>,
    pub published_at: String,
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
        let qualifiers = crate::canonical::digest_hex(&serde_json::json!([
            "snapshot-qualifiers/1",
            metadata.symbol_package,
            metadata.crate_name,
            metadata.crate_version,
            metadata.observed_configuration,
            metadata.producer_items,
        ]));
        Ok(SnapshotId::derive(&SnapshotInputs {
            schema_version: FORMAT.into(),
            normalizer_version: metadata.normalizer_version.clone(),
            context_id: metadata.context_id.clone(),
            input_digests: components.clone(),
            producers: [("qualifiers".into(), qualifiers)].into_iter().collect(),
        }))
    }

    /// # Errors
    /// Historical versions, semantic ID mismatch and unsafe table filenames are rejected.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != FORMAT
            || self.snapshot_id != Self::derive_id(&self.metadata, &self.components)?
        {
            return Err("unsupported or inconsistent snapshot identity".into());
        }
        for table in &self.tables {
            if table.file != format!("{}.parquet", table.relation)
                || !table
                    .relation
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b == b'_')
                || table.sha256.len() != 64
                || !table
                    .sha256
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                || table.bytes == 0
            {
                return Err("invalid exact snapshot file reference".into());
            }
        }
        Ok(())
    }
}
