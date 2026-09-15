use std::sync::Arc;

use arrow::array::{ArrayRef, BooleanArray, UInt32Array, UInt64Array};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use arrow_schema::extension::Json;
use enrichment_core::evidence::relational::{
    ApiObservation, Definition, FactSource, Locator, PublicBinding, SubjectRef,
};
use enrichment_core::identity::Ecosystem;

use super::cells::{batch, column, list, optional, structure, text};

/// Definition rows, shared by public aliases.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn definitions(rows: &[Definition]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(ArrowError::InvalidArgumentError)?;
    }
    batch(
        "definitions",
        vec![
            column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "key:definition",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:symbol-kind/1",
            ),
            column(
                "definition_path",
                text(rows.iter().map(|r| r.definition_path.as_str())),
                false,
                "definition-path",
            ),
            column(
                "defined_in_package",
                text(rows.iter().map(|r| r.defined_in_package.as_str())),
                false,
                "package-name",
            ),
            column(
                "qualifier",
                optional(rows.iter().map(|r| r.qualifier.as_deref())),
                true,
                "definition-qualifier",
            ),
        ],
    )
}

/// Public binding rows. Display text is separate from exact typed component identity.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn bindings(rows: &[PublicBinding]) -> Result<RecordBatch, ArrowError> {
    let path_ids: Vec<_> = rows.iter().map(|r| r.path.id()).collect();
    let displays: Vec<_> = rows.iter().map(|r| r.path.display()).collect();
    let parents: Vec<_> = rows
        .iter()
        .map(|r| r.path.parent().map(|p| p.id()))
        .collect();
    batch(
        "symbols",
        vec![
            column(
                "symbol_id",
                text(rows.iter().map(|r| r.symbol_id.as_str())),
                false,
                "key:symbol",
            ),
            column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "ref:definition",
            ),
            column(
                "path_id",
                text(path_ids.iter().map(String::as_str)),
                false,
                "ref:path",
            ),
            column(
                "ecosystem",
                text(rows.iter().map(|r| match r.path.ecosystem() {
                    Ecosystem::Rust => "rust",
                    Ecosystem::Python => "python",
                })),
                false,
                "vocabulary:ecosystem/1",
            ),
            column(
                "components",
                list(rows.iter().map(|r| r.path.components())),
                false,
                "ordered:path-components",
            ),
            column(
                "path",
                text(displays.iter().map(String::as_str)),
                false,
                "display:public-path",
            ),
            column(
                "parent_path_id",
                optional(parents.iter().map(|p| p.as_deref())),
                true,
                "ref:path",
            ),
            column(
                "name",
                text(rows.iter().map(|r| r.name.as_str())),
                false,
                "public-name",
            ),
            column(
                "is_reexport",
                Arc::new(BooleanArray::from_iter(
                    rows.iter().map(|r| Some(r.is_reexport)),
                )),
                false,
                "reexport",
            ),
            column(
                "qualifier",
                optional(rows.iter().map(|r| r.qualifier.as_deref())),
                true,
                "binding-qualifier",
            ),
        ],
    )
}

pub(super) fn subject(rows: &[&SubjectRef]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "kind",
                text(rows.iter().map(|s| match s {
                    SubjectRef::Symbol { .. } => "symbol",
                    SubjectRef::Definition { .. } => "definition",
                    SubjectRef::Library { .. } => "library",
                    SubjectRef::Feature { .. } => "feature",
                    SubjectRef::Document { .. } => "document",
                    SubjectRef::Example { .. } => "example",
                })),
                false,
                "vocabulary:subject/1",
            ),
            column(
                "symbol_id",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Symbol { symbol_id } => Some(symbol_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:symbol",
            ),
            column(
                "definition_id",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Definition { definition_id } => Some(definition_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:definition",
            ),
            column(
                "release_id",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Library { release_id } => Some(release_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:release",
            ),
            column(
                "feature",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Feature { name } => Some(name.as_str()),
                    _ => None,
                })),
                true,
                "feature-name",
            ),
            column(
                "artifact_id",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Document { artifact_id, .. }
                    | SubjectRef::Example { artifact_id, .. } => Some(artifact_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:artifact",
            ),
            column(
                "heading",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Document { heading, .. } => Some(heading.as_str()),
                    _ => None,
                })),
                true,
                "document-heading",
            ),
            column(
                "path",
                optional(rows.iter().map(|s| match s {
                    SubjectRef::Example { path, .. } => Some(path.as_str()),
                    _ => None,
                })),
                true,
                "archive-member",
            ),
        ],
        None,
    )
}

fn locator(rows: &[&Locator]) -> Result<ArrayRef, ArrowError> {
    // This is the only open producer value. Serialization is fallible and tagged arrow.json;
    // all supported coordinates have native fields, and no query parses this extension.
    let extensions: Vec<Option<String>> = rows
        .iter()
        .map(|r| match r {
            Locator::Extension { value, .. } => serde_json::to_string(value).map(Some),
            _ => Ok(None),
        })
        .collect::<Result<_, _>>()
        .map_err(|e| ArrowError::JsonError(e.to_string()))?;
    let mut extension = column(
        "extension",
        optional(extensions.iter().map(|v| v.as_deref())),
        true,
        "producer-extension/1",
    );
    extension.0.try_with_extension_type(Json::default())?;
    structure(
        vec![
            column(
                "kind",
                text(rows.iter().map(|r| match r {
                    Locator::Artifact => "artifact",
                    Locator::Lines { .. } => "lines",
                    Locator::Bytes { .. } => "bytes",
                    Locator::ArchiveMember { .. } => "archive_member",
                    Locator::Heading { .. } => "heading",
                    Locator::ProducerItem { .. } => "producer_item",
                    Locator::RustdocItem { .. } => "rustdoc_item",
                    Locator::PythonDeclaration { .. } => "python_declaration",
                    Locator::ManifestKey { .. } => "manifest_key",
                    Locator::MarkdownSection { .. } => "markdown_section",
                    Locator::SourceStart { .. } => "source_start",
                    Locator::SphinxInventory { .. } => "sphinx_inventory",
                    Locator::WebDocument { .. } => "web_document",
                    Locator::Extension { .. } => "extension",
                })),
                false,
                "vocabulary:locator/1",
            ),
            column(
                "uri",
                optional(rows.iter().map(|r| match r {
                    Locator::SphinxInventory { uri, .. } | Locator::WebDocument { uri, .. } => {
                        Some(uri.as_str())
                    }
                    _ => None,
                })),
                true,
                "documentation-uri",
            ),
            column(
                "role",
                optional(rows.iter().map(|r| match r {
                    Locator::SphinxInventory { role, .. } => Some(role.as_str()),
                    _ => None,
                })),
                true,
                "sphinx-role",
            ),
            column(
                "project",
                optional(rows.iter().map(|r| match r {
                    Locator::SphinxInventory { project, .. } => Some(project.as_str()),
                    _ => None,
                })),
                true,
                "sphinx-project",
            ),
            column(
                "inventory_version",
                optional(rows.iter().map(|r| match r {
                    Locator::SphinxInventory {
                        inventory_version, ..
                    }
                    | Locator::WebDocument {
                        inventory_version, ..
                    } => Some(inventory_version.as_str()),
                    _ => None,
                })),
                true,
                "claimed-documentation-version",
            ),
            column(
                "file",
                optional(rows.iter().map(|r| match r {
                    Locator::Lines { file, .. } => file.as_deref(),
                    Locator::ArchiveMember { path } => Some(path.as_str()),
                    Locator::PythonDeclaration { file, .. }
                    | Locator::ManifestKey { file, .. }
                    | Locator::MarkdownSection { file, .. }
                    | Locator::SourceStart { file, .. } => Some(file.as_str()),
                    _ => None,
                })),
                true,
                "archive-member",
            ),
            column(
                "start",
                Arc::new(UInt64Array::from_iter(rows.iter().map(|r| match r {
                    Locator::Lines { start, .. } => Some(u64::from(*start)),
                    Locator::Bytes { start, .. } => Some(*start),
                    Locator::MarkdownSection { line, .. } | Locator::SourceStart { line, .. } => {
                        Some(u64::from(*line))
                    }
                    Locator::PythonDeclaration { line, .. } => line.map(u64::from),
                    _ => None,
                }))),
                true,
                "coordinate-start",
            ),
            column(
                "end",
                Arc::new(UInt64Array::from_iter(rows.iter().map(|r| match r {
                    Locator::Lines { end, .. } => Some(u64::from(*end)),
                    Locator::Bytes { end, .. } => Some(*end),
                    _ => None,
                }))),
                true,
                "coordinate-end",
            ),
            column(
                "heading",
                optional(rows.iter().map(|r| match r {
                    Locator::Heading { heading, .. } | Locator::MarkdownSection { heading, .. } => {
                        Some(heading.as_str())
                    }
                    _ => None,
                })),
                true,
                "document-heading",
            ),
            column(
                "ordinal",
                Arc::new(UInt32Array::from_iter(rows.iter().map(|r| match r {
                    Locator::Heading { ordinal, .. } => Some(*ordinal),
                    _ => None,
                }))),
                true,
                "heading-ordinal",
            ),
            column(
                "producer",
                optional(rows.iter().map(|r| match r {
                    Locator::ProducerItem { producer, .. } => Some(producer.as_str()),
                    _ => None,
                })),
                true,
                "producer-name",
            ),
            column(
                "item",
                optional(rows.iter().map(|r| match r {
                    Locator::ProducerItem { item, .. } => Some(item.as_str()),
                    _ => None,
                })),
                true,
                "producer-local-item",
            ),
            column(
                "format",
                optional(rows.iter().map(|r| match r {
                    Locator::Extension { format, .. } => Some(format.as_str()),
                    _ => None,
                })),
                true,
                "extension-format",
            ),
            column(
                "version",
                optional(rows.iter().map(|r| match r {
                    Locator::Extension { version, .. } => Some(version.as_str()),
                    _ => None,
                })),
                true,
                "extension-version",
            ),
            column(
                "declaration",
                optional(rows.iter().map(|r| match r {
                    Locator::PythonDeclaration { declaration, .. } => Some(declaration.as_str()),
                    _ => None,
                })),
                true,
                "producer-declaration-path",
            ),
            column(
                "rustdoc_id",
                Arc::new(UInt32Array::from_iter(rows.iter().map(|r| match r {
                    Locator::RustdocItem { item, .. } => Some(*item),
                    _ => None,
                }))),
                true,
                "rustdoc-local-item",
            ),
            column(
                "reported_file",
                optional(rows.iter().map(|r| match r {
                    Locator::RustdocItem { reported_file, .. } => reported_file.as_deref(),
                    _ => None,
                })),
                true,
                "producer-reported-source-path",
            ),
            column(
                "reported_line",
                Arc::new(UInt32Array::from_iter(rows.iter().map(|r| match r {
                    Locator::RustdocItem { reported_line, .. } => *reported_line,
                    _ => None,
                }))),
                true,
                "producer-reported-source-line",
            ),
            column(
                "origin",
                optional(rows.iter().map(|r| match r {
                    Locator::PythonDeclaration { origin, .. } => Some(origin.as_str()),
                    _ => None,
                })),
                true,
                "vocabulary:api-origin/1",
            ),
            column(
                "overload",
                Arc::new(UInt32Array::from_iter(rows.iter().map(|r| match r {
                    Locator::PythonDeclaration { overload, .. } => *overload,
                    _ => None,
                }))),
                true,
                "declaration-overload-ordinal",
            ),
            column(
                "table",
                optional(rows.iter().map(|r| match r {
                    Locator::ManifestKey { table, .. } => Some(table.as_str()),
                    _ => None,
                })),
                true,
                "manifest-table",
            ),
            column(
                "key",
                optional(rows.iter().map(|r| match r {
                    Locator::ManifestKey { key, .. } => Some(key.as_str()),
                    _ => None,
                })),
                true,
                "manifest-key",
            ),
            extension,
        ],
        None,
    )
}

pub(super) fn source(rows: &[&FactSource]) -> Result<ArrayRef, ArrowError> {
    use enrichment_core::wire::{EvidenceClass, SourceVersionMatch};
    structure(
        vec![
            column(
                "extractor",
                text(rows.iter().map(|r| r.extractor.as_str())),
                false,
                "extractor-name",
            ),
            column(
                "extractor_version",
                text(rows.iter().map(|r| r.extractor_version.as_str())),
                false,
                "extractor-version",
            ),
            column(
                "producer_binding_id",
                text(rows.iter().map(|r| r.producer_binding_id.as_str())),
                false,
                "ref:producer-binding",
            ),
            column(
                "artifact_id",
                text(rows.iter().map(|r| r.artifact_id.as_str())),
                false,
                "ref:artifact",
            ),
            column(
                "source_uri",
                optional(rows.iter().map(|r| r.source_uri.as_deref())),
                true,
                "acquisition-uri",
            ),
            column(
                "source_version_match",
                text(rows.iter().map(|r| match r.source_version_match {
                    SourceVersionMatch::Exact => "exact",
                    SourceVersionMatch::CompatibleClaimed => "compatible_claimed",
                    SourceVersionMatch::Unknown => "unknown",
                    SourceVersionMatch::Mismatched => "mismatched",
                })),
                false,
                "vocabulary:source-version-match/1",
            ),
            column(
                "evidence_class",
                text(rows.iter().map(|r| match r.evidence_class {
                    EvidenceClass::Declared => "declared",
                    EvidenceClass::StaticallyExtracted => "statically_extracted",
                    EvidenceClass::CompilerDerived => "compiler_derived",
                    EvidenceClass::TypecheckerObserved => "typechecker_observed",
                    EvidenceClass::RuntimeObserved => "runtime_observed",
                    EvidenceClass::AgentInferred => "agent_inferred",
                })),
                false,
                "vocabulary:evidence-class/1",
            ),
            column(
                "locator",
                locator(&rows.iter().map(|r| &r.locator).collect::<Vec<_>>())?,
                false,
                "artifact-coordinates",
            ),
        ],
        None,
    )
}

fn payload(rows: &[ApiObservation]) -> Result<ArrayRef, ArrowError> {
    let deprecation = structure(
        vec![
            column(
                "since",
                optional(rows.iter().map(|r| {
                    r.payload
                        .deprecated
                        .as_ref()
                        .and_then(|d| d.since.as_deref())
                })),
                true,
                "deprecation-version",
            ),
            column(
                "note",
                optional(rows.iter().map(|r| {
                    r.payload
                        .deprecated
                        .as_ref()
                        .and_then(|d| d.note.as_deref())
                })),
                true,
                "deprecation-note",
            ),
        ],
        Some(
            rows.iter()
                .map(|r| r.payload.deprecated.is_some())
                .collect(),
        ),
    )?;
    let python = rows
        .iter()
        .map(|r| r.payload.python.as_ref())
        .collect::<Vec<_>>();
    let python_valid = python.iter().map(|p| p.is_some()).collect();
    let publicness =
        structure(
            vec![
                column(
                    "exported",
                    Arc::new(BooleanArray::from_iter(
                        python.iter().map(|p| p.and_then(|p| p.publicness.exported)),
                    )),
                    true,
                    "declared-export",
                ),
                column(
                    "underscore",
                    Arc::new(BooleanArray::from_iter(
                        python.iter().map(|p| p.map(|p| p.publicness.underscore)),
                    )),
                    true,
                    "name-visibility",
                ),
                column(
                    "reexport",
                    Arc::new(BooleanArray::from_iter(
                        python.iter().map(|p| p.map(|p| p.publicness.reexport)),
                    )),
                    true,
                    "declared-reexport",
                ),
                column(
                    "docstring",
                    Arc::new(BooleanArray::from_iter(
                        python.iter().map(|p| p.map(|p| p.publicness.docstring)),
                    )),
                    true,
                    "docstring-present",
                ),
                column(
                    "declared_exports",
                    list(
                        python.iter().map(|p| {
                            p.map_or(&[][..], |p| p.publicness.declared_exports.as_slice())
                        }),
                    ),
                    false,
                    "ordered:declared-exports",
                ),
                column(
                    "unresolved_exports",
                    list(python.iter().map(|p| {
                        p.map_or(&[][..], |p| p.publicness.unresolved_exports.as_slice())
                    })),
                    false,
                    "ordered:unresolved-exports",
                ),
            ],
            None,
        )?;
    let python = structure(
        vec![
            column(
                "overloads",
                list(
                    python
                        .iter()
                        .map(|p| p.map_or(&[][..], |p| p.overloads.as_slice())),
                ),
                false,
                "ordered:overloads",
            ),
            column(
                "alias_target",
                optional(
                    python
                        .iter()
                        .map(|p| p.and_then(|p| p.alias_target.as_deref())),
                ),
                true,
                "declared-alias",
            ),
            column(
                "bases",
                list(
                    python
                        .iter()
                        .map(|p| p.map_or(&[][..], |p| p.bases.as_slice())),
                ),
                false,
                "ordered:declared-bases",
            ),
            column("publicness", publicness, false, "publicness-signals"),
        ],
        Some(python_valid),
    )?;
    structure(
        vec![
            column(
                "declared_kind",
                text(rows.iter().map(|r| r.payload.declared_kind.as_str())),
                false,
                "vocabulary:symbol-kind/1",
            ),
            column(
                "signature",
                optional(rows.iter().map(|r| r.payload.signature.as_deref())),
                true,
                "api-signature",
            ),
            column(
                "doc_summary",
                optional(rows.iter().map(|r| r.payload.doc_summary.as_deref())),
                true,
                "documentation-summary",
            ),
            column(
                "docs",
                optional(rows.iter().map(|r| r.payload.docs.as_deref())),
                true,
                "documentation-text",
            ),
            column("deprecated", deprecation, true, "observed-deprecation"),
            column(
                "cfg_hints",
                list(rows.iter().map(|r| r.payload.cfg_hints.as_slice())),
                false,
                "ordered:declared-cfg",
            ),
            column("python", python, true, "python-declaration"),
        ],
        None,
    )
}

/// Independently qualified API observations, preserving nulls and source/stub alternatives.
///
/// # Errors
/// Invalid observations and inconsistent Arrow shapes are refused.
pub fn observations(rows: &[ApiObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(ArrowError::InvalidArgumentError)?;
    }
    batch(
        "api_observations",
        vec![
            column(
                "observation_id",
                text(rows.iter().map(|r| r.observation_id.as_str())),
                false,
                "key:observation",
            ),
            column(
                "subject",
                subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "typed-subject",
            ),
            column(
                "origin",
                text(rows.iter().map(|r| r.origin.as_str())),
                false,
                "vocabulary:api-origin/1",
            ),
            column(
                "environment_id",
                text(rows.iter().map(|r| r.environment_id.as_str())),
                false,
                "ref:environment",
            ),
            column("payload", payload(rows)?, false, "api-payload"),
            column(
                "source",
                source(&rows.iter().map(|r| &r.source).collect::<Vec<_>>())?,
                false,
                "fact-provenance",
            ),
        ],
    )
}
