//! Declared document visits from qualified immutable artifacts, never a fragment corpus.
use enrichment_core::{
    evidence::{
        Artifact, FragmentKind, document::DocumentFact, ingest::DocumentSource, relational::Locator,
    },
    producer::{docsrs::ManifestFacts, source},
};
use enrichment_store::blob::BlobStore;
use std::io::Read;

pub(super) struct SourceDocuments {
    blobs: BlobStore,
    features: Option<(ManifestFacts, Artifact)>,
    files: Vec<(String, FragmentKind, Artifact, bool)>,
    declarations: Vec<DocumentFact>,
    inventory_bytes: usize,
    inventory: Option<Inventory>,
}
struct Inventory {
    entries: Vec<enrichment_core::producer::python::inventory::Entry>,
    artifact: Artifact,
    project: String,
    version: String,
    version_match: enrichment_core::wire::SourceVersionMatch,
}
impl SourceDocuments {
    pub fn new(blobs: BlobStore) -> Self {
        Self {
            blobs,
            features: None,
            files: Vec::new(),
            declarations: Vec::new(),
            inventory_bytes: 0,
            inventory: None,
        }
    }
    fn charge(&mut self, row: &impl serde::Serialize) -> Result<(), String> {
        self.inventory_bytes = self
            .inventory_bytes
            .checked_add(
                enrichment_core::canonical::serialized_size(row, 1024 * 1024)
                    .map_err(|e| e.to_string())?,
            )
            .filter(|n| *n <= 16 * 1024 * 1024)
            .ok_or("source descriptor inventory exceeds 16 MiB")?;
        if self.files.len() + self.declarations.len() >= 8192 {
            return Err("source descriptor count exceeds 8192".into());
        }
        Ok(())
    }
    pub fn features(&mut self, facts: ManifestFacts, artifact: Artifact) -> Result<(), String> {
        self.charge(&(&facts, &artifact))?;
        if self.features.is_some() {
            return Err("source manifest already supplied".into());
        }
        self.features = Some((facts, artifact));
        Ok(())
    }
    pub fn file(
        &mut self,
        path: String,
        kind: FragmentKind,
        artifact: Artifact,
        rust_sections: bool,
    ) -> Result<(), String> {
        self.charge(&(&path, kind, &artifact))?;
        self.files.push((path, kind, artifact, rust_sections));
        Ok(())
    }
    /// Small explicit declarations such as a documentation URL, never extracted API text.
    pub fn declaration(&mut self, value: DocumentFact) -> Result<(), String> {
        self.charge(&value)?;
        self.declarations.push(value);
        Ok(())
    }
    pub fn inventory(
        &mut self,
        entries: Vec<enrichment_core::producer::python::inventory::Entry>,
        artifact: Artifact,
        project: String,
        version: String,
        version_match: enrichment_core::wire::SourceVersionMatch,
    ) -> Result<(), String> {
        let bytes = enrichment_core::canonical::serialized_size(&entries, 8 * 1024 * 1024)
            .map_err(|e| e.to_string())?;
        if entries.len() > 50000 || self.inventory.is_some() {
            return Err("invalid inventory descriptor count".into());
        }
        self.inventory_bytes = self
            .inventory_bytes
            .checked_add(bytes)
            .filter(|n| *n <= 16 * 1024 * 1024)
            .ok_or("source inventory bound exceeded")?;
        self.inventory = Some(Inventory {
            entries,
            artifact,
            project,
            version,
            version_match,
        });
        Ok(())
    }
    pub fn kinds(&self) -> Vec<FragmentKind> {
        self.files
            .iter()
            .map(|(_, kind, _, _)| *kind)
            .chain(self.declarations.iter().map(|f| f.kind))
            .chain(
                self.inventory
                    .iter()
                    .filter(|i| !i.entries.is_empty())
                    .map(|_| FragmentKind::DocText),
            )
            .chain(
                self.features
                    .iter()
                    .filter(|(facts, _)| !facts.features.is_empty())
                    .map(|_| FragmentKind::FeatureDefinition),
            )
            .fold(Vec::new(), |mut kinds, kind| {
                if !kinds.contains(&kind) {
                    kinds.push(kind);
                }
                kinds
            })
    }
}
impl DocumentSource for SourceDocuments {
    fn decode(
        &self,
        emit: &mut dyn FnMut(DocumentFact) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Some(inventory) = &self.inventory {
            for entry in &inventory.entries {
                let mut row = DocumentFact::new(
                    FragmentKind::DocText,
                    &entry.name,
                    &inventory.artifact.artifact_id,
                    Locator::SphinxInventory {
                        uri: entry.uri.clone(),
                        role: entry.role.clone(),
                        project: inventory.project.clone(),
                        inventory_version: inventory.version.clone(),
                    },
                    format!("{} ({}) → {}", entry.display, entry.role, entry.uri),
                    enrichment_core::wire::EvidenceClass::Declared,
                    "sphinx-inventory",
                    "2",
                )?;
                row.source_version_match = Some(inventory.version_match);
                row.source_uri = Some(inventory.artifact.source_uri.clone());
                emit(row)?;
            }
        }
        for row in &self.declarations {
            emit(row.clone())?;
        }
        if let Some((facts, artifact)) = &self.features {
            source::visit_features(facts, &artifact.artifact_id, &mut |mut row| {
                row.source_uri = Some(artifact.source_uri.clone());
                emit(row)
            })?;
        }
        for (path, kind, artifact, rust_sections) in &self.files {
            let mut text = String::new();
            self.blobs
                .capture(artifact, 64 * 1024 * 1024)
                .map_err(|e| e.to_string())?
                .read_to_string(&mut text)
                .map_err(|e| e.to_string())?;
            source::visit_document(
                &text,
                path,
                &artifact.artifact_id,
                *kind,
                *rust_sections,
                &mut |mut row| {
                    row.source_uri = Some(artifact.source_uri.clone());
                    if !rust_sections {
                        row.producer = "python-distribution".into();
                        row.producer_version = enrichment_core::producer::python::VERSION.into();
                    }
                    emit(row)
                },
            )?;
        }
        Ok(())
    }
}
