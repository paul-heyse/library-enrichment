use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use enrichment_core::evidence::{
    Deprecated, SymbolKind,
    path::PublicPath,
    relational::{
        ApiObservation, ApiOrigin, ApiPayload, Definition, FactSource, Locator, PublicBinding,
        PythonDetails, SubjectRef,
    },
};
use enrichment_core::identity::Ecosystem;
use enrichment_core::producer::python::Publicness;
use enrichment_core::wire::{EvidenceClass, SourceVersionMatch};

use super::cells::{Row, RowSet, invalid};

/// Decode a projected definition batch. Invalid enums remain errors.
///
/// # Errors
/// Missing, null or malformed columns are rejected.
pub fn definitions(batch: &RecordBatch) -> Result<Vec<Definition>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let definition = Definition {
                definition_id: r.text("definition_id")?.into(),
                kind: SymbolKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown symbol kind"))?,
                definition_path: r.text("definition_path")?.into(),
                defined_in_package: r.text("defined_in_package")?.into(),
                qualifier: r.owned("qualifier")?,
            };
            definition.validate().map_err(invalid)?;
            Ok(definition)
        })
        .collect()
}

/// Decode public bindings using exact components, verifying their derived display/index data.
///
/// # Errors
/// Corrupt path identities, nulls or unsupported column shapes are rejected.
pub fn bindings(batch: &RecordBatch) -> Result<Vec<PublicBinding>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let ecosystem = match r.text("ecosystem")? {
                "rust" => Ecosystem::Rust,
                "python" => Ecosystem::Python,
                _ => return Err(invalid("unknown ecosystem")),
            };
            let path = PublicPath::new(ecosystem, r.list("components")?).map_err(invalid)?;
            if path.id() != r.text("path_id")?
                || path.display() != r.text("path")?
                || path.parent().map(|p| p.id()).as_deref() != r.optional_text("parent_path_id")?
            {
                return Err(invalid("path projection disagrees with components"));
            }
            Ok(PublicBinding {
                symbol_id: r.text("symbol_id")?.into(),
                definition_id: r.text("definition_id")?.into(),
                path,
                name: r.text("name")?.into(),
                is_reexport: r.boolean("is_reexport")?,
                qualifier: r.owned("qualifier")?,
            })
        })
        .collect()
}

/// Validate bindings against their native joined definitions without collecting a snapshot.
/// The plan selects `s.*` and definition kind/path/package/qualifier under `definition_*` aliases.
/// # Errors
/// Invalid identity, qualifier, package or ecosystem is rejected.
pub fn validate_binding_definitions(
    batch: &RecordBatch,
    package: &str,
    ecosystem: Ecosystem,
) -> Result<(), ArrowError> {
    let columns = RowSet::batch(batch)?;
    for (i, binding) in bindings(batch)?.into_iter().enumerate() {
        let row = columns.row(i);
        let definition = Definition {
            definition_id: binding.definition_id.clone(),
            kind: SymbolKind::parse(row.text("definition_kind")?)
                .ok_or_else(|| invalid("unknown definition kind"))?,
            definition_path: row.text("definition_path")?.into(),
            defined_in_package: row.text("definition_package")?.into(),
            qualifier: row.owned("definition_qualifier")?,
        };
        definition.validate().map_err(invalid)?;
        binding.validate(package, &definition).map_err(invalid)?;
        if binding.path.ecosystem() != ecosystem {
            return Err(invalid("binding ecosystem disagrees with context"));
        }
    }
    Ok(())
}

pub(super) fn subject(r: Row<'_>) -> Result<SubjectRef, ArrowError> {
    Ok(match r.text("kind")? {
        "symbol" => {
            r.variant(&["symbol_id"])?;
            SubjectRef::Symbol {
                symbol_id: r.text("symbol_id")?.into(),
            }
        }
        "definition" => {
            r.variant(&["definition_id"])?;
            SubjectRef::Definition {
                definition_id: r.text("definition_id")?.into(),
            }
        }
        "library" => {
            r.variant(&["release_id"])?;
            SubjectRef::Library {
                release_id: r.text("release_id")?.into(),
            }
        }
        "feature" => {
            r.variant(&["feature"])?;
            SubjectRef::Feature {
                name: r.text("feature")?.into(),
            }
        }
        "document" => {
            r.variant(&["artifact_id", "heading"])?;
            SubjectRef::Document {
                artifact_id: r.text("artifact_id")?.into(),
                heading: r.text("heading")?.into(),
            }
        }
        "example" => {
            r.variant(&["artifact_id", "path"])?;
            SubjectRef::Example {
                artifact_id: r.text("artifact_id")?.into(),
                path: r.text("path")?.into(),
            }
        }
        _ => return Err(invalid("unknown subject variant")),
    })
}

fn small_number(r: Row<'_>, field: &str) -> Result<u32, ArrowError> {
    u32::try_from(r.number(field)?).map_err(|e| invalid(e.to_string()))
}

fn optional_small_number(r: Row<'_>, field: &str) -> Result<Option<u32>, ArrowError> {
    r.optional_number(field)?
        .map(|v| u32::try_from(v).map_err(|e| invalid(e.to_string())))
        .transpose()
}

fn locator(r: Row<'_>) -> Result<Locator, ArrowError> {
    let value = match r.text("kind")? {
        "artifact" => {
            r.variant(&[])?;
            Locator::Artifact
        }
        "lines" => {
            r.variant(&["file", "start", "end"])?;
            Locator::Lines {
                file: r.owned("file")?,
                start: small_number(r, "start")?,
                end: small_number(r, "end")?,
            }
        }
        "bytes" => {
            r.variant(&["start", "end"])?;
            Locator::Bytes {
                start: r.number("start")?,
                end: r.number("end")?,
            }
        }
        "archive_member" => {
            r.variant(&["file"])?;
            Locator::ArchiveMember {
                path: r.text("file")?.into(),
            }
        }
        "heading" => {
            r.variant(&["heading", "ordinal"])?;
            Locator::Heading {
                heading: r.text("heading")?.into(),
                ordinal: small_number(r, "ordinal")?,
            }
        }
        "producer_item" => {
            r.variant(&["producer", "item"])?;
            Locator::ProducerItem {
                producer: r.text("producer")?.into(),
                item: r.text("item")?.into(),
            }
        }
        "rustdoc_item" => {
            r.variant(&["rustdoc_id", "reported_file", "reported_line"])?;
            Locator::RustdocItem {
                item: small_number(r, "rustdoc_id")?,
                reported_file: r.owned("reported_file")?,
                reported_line: optional_small_number(r, "reported_line")?,
            }
        }
        "python_declaration" => {
            r.variant(&["file", "declaration", "start", "origin", "overload"])?;
            Locator::PythonDeclaration {
                file: r.text("file")?.into(),
                declaration: r.text("declaration")?.into(),
                line: optional_small_number(r, "start")?,
                origin: match r.text("origin")? {
                    "source" => ApiOrigin::Source,
                    "stub" => ApiOrigin::Stub,
                    _ => return Err(invalid("invalid Python declaration origin")),
                },
                overload: optional_small_number(r, "overload")?,
            }
        }
        "manifest_key" => {
            r.variant(&["file", "table", "key"])?;
            Locator::ManifestKey {
                file: r.text("file")?.into(),
                table: r.text("table")?.into(),
                key: r.text("key")?.into(),
            }
        }
        "markdown_section" => {
            r.variant(&["file", "heading", "start"])?;
            Locator::MarkdownSection {
                file: r.text("file")?.into(),
                heading: r.text("heading")?.into(),
                line: small_number(r, "start")?,
            }
        }
        "source_start" => {
            r.variant(&["file", "start"])?;
            Locator::SourceStart {
                file: r.text("file")?.into(),
                line: small_number(r, "start")?,
            }
        }
        "extension" => {
            r.variant(&["format", "version", "extension"])?;
            Locator::Extension {
                format: r.text("format")?.into(),
                version: r.text("version")?.into(),
                value: serde_json::from_str(r.text("extension")?)
                    .map_err(|e| ArrowError::JsonError(e.to_string()))?,
            }
        }
        "sphinx_inventory" => {
            r.variant(&["uri", "role", "project", "inventory_version"])?;
            Locator::SphinxInventory {
                uri: r.text("uri")?.into(),
                role: r.text("role")?.into(),
                project: r.text("project")?.into(),
                inventory_version: r.text("inventory_version")?.into(),
            }
        }
        "web_document" => {
            r.variant(&["uri", "inventory_version"])?;
            Locator::WebDocument {
                uri: r.text("uri")?.into(),
                inventory_version: r.text("inventory_version")?.into(),
            }
        }
        _ => return Err(invalid("unknown locator variant")),
    };
    value.validate().map_err(invalid)?;
    Ok(value)
}

pub(super) fn source(r: Row<'_>) -> Result<FactSource, ArrowError> {
    Ok(FactSource {
        extractor: r.text("extractor")?.into(),
        extractor_version: r.text("extractor_version")?.into(),
        producer_binding_id: r.text("producer_binding_id")?.into(),
        artifact_id: r.text("artifact_id")?.into(),
        source_uri: r.owned("source_uri")?,
        source_version_match: match r.text("source_version_match")? {
            "exact" => SourceVersionMatch::Exact,
            "compatible_claimed" => SourceVersionMatch::CompatibleClaimed,
            "mismatched" => SourceVersionMatch::Mismatched,
            "unknown" => SourceVersionMatch::Unknown,
            _ => return Err(invalid("unknown source version match")),
        },
        evidence_class: match r.text("evidence_class")? {
            "declared" => EvidenceClass::Declared,
            "statically_extracted" => EvidenceClass::StaticallyExtracted,
            "compiler_derived" => EvidenceClass::CompilerDerived,
            "typechecker_observed" => EvidenceClass::TypecheckerObserved,
            "runtime_observed" => EvidenceClass::RuntimeObserved,
            "agent_inferred" => EvidenceClass::AgentInferred,
            _ => return Err(invalid("unknown evidence class")),
        },
        locator: locator(r.structure("locator")?)?,
    })
}

fn python(r: Row<'_>) -> Result<PythonDetails, ArrowError> {
    let p = r.structure("publicness")?;
    Ok(PythonDetails {
        overloads: r.list("overloads")?,
        alias_target: r.owned("alias_target")?,
        bases: r.list("bases")?,
        publicness: Publicness {
            exported: p.optional_bool("exported")?,
            underscore: p.boolean("underscore")?,
            reexport: p.boolean("reexport")?,
            docstring: p.boolean("docstring")?,
            declared_exports: p.list("declared_exports")?,
            unresolved_exports: p.list("unresolved_exports")?,
        },
    })
}

pub(super) fn payload(r: Row<'_>) -> Result<ApiPayload, ArrowError> {
    payload_projection(r, true)
}

pub(super) fn payload_projection(r: Row<'_>, docs: bool) -> Result<ApiPayload, ArrowError> {
    Ok(ApiPayload {
        declared_kind: SymbolKind::parse(r.text("declared_kind")?)
            .ok_or_else(|| invalid("unknown API declaration kind"))?,
        signature: r.owned("signature")?,
        doc_summary: r.owned("doc_summary")?,
        docs: if docs { r.owned("docs")? } else { None },
        deprecated: r
            .optional_struct("deprecated")?
            .map(|d| {
                Ok::<_, ArrowError>(Deprecated {
                    since: d.owned("since")?,
                    note: d.owned("note")?,
                })
            })
            .transpose()?,
        cfg_hints: r.list("cfg_hints")?,
        python: r.optional_struct("python")?.map(python).transpose()?,
    })
}

/// Decode observations and recompute semantic IDs. Row order and string encoding are immaterial.
///
/// # Errors
/// Malformed variants, identities or nested values are rejected instead of silently omitted.
pub fn observations(batch: &RecordBatch) -> Result<Vec<ApiObservation>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let observation = ApiObservation {
                observation_id: r.text("observation_id")?.into(),
                subject: subject(r.structure("subject")?)?,
                origin: match r.text("origin")? {
                    "rustdoc" => ApiOrigin::Rustdoc,
                    "source" => ApiOrigin::Source,
                    "stub" => ApiOrigin::Stub,
                    _ => return Err(invalid("unknown API origin")),
                },
                environment_id: r.text("environment_id")?.into(),
                payload: payload(r.structure("payload")?)?,
                source: source(r.structure("source")?)?,
            };
            observation.validate().map_err(invalid)?;
            Ok(observation)
        })
        .collect()
}

/// Decode native requested-domain subjects without JSON projection.
pub(crate) fn subjects(batch: &RecordBatch) -> Result<Vec<SubjectRef>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| subject(rows.row(i).structure("subject")?))
        .collect()
}
