//! The Arrow schema registry and Parquet round trips for the evidence tables (blueprint §6.3).
//!
//! Three tables per snapshot: `symbols`, `relationships` and `fragments`. The Arrow schemas
//! are declared here, next to the conversions from and to the core's typed records, and a
//! semantic round-trip test proves the two agree. Nothing else in the workspace defines a
//! column.
//!
//! Columns are flat and portable (Utf8, Boolean, UInt32); the one structured value, a
//! fragment's locator, is stored as canonical JSON text so it survives any Parquet reader.

use std::fs::File;
use std::io;
use std::path::Path;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, BooleanArray, StringArray, UInt32Array};
use arrow::record_batch::RecordBatch;
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use enrichment_core::canonical;
use enrichment_core::evidence::{
    Deprecated, EvidenceFragment, FragmentKind, RelationKind, Relationship, Symbol, SymbolKind,
};
use enrichment_core::wire::EvidenceClass;
use parquet::arrow::ArrowWriter;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;

/// File names within a snapshot directory.
pub const SYMBOLS_FILE: &str = "symbols.parquet";
/// File name of the relationships table.
pub const RELATIONSHIPS_FILE: &str = "relationships.parquet";
/// File name of the fragments table.
pub const FRAGMENTS_FILE: &str = "fragments.parquet";
/// File name of the manifest.
pub const MANIFEST_FILE: &str = "manifest.json";

fn utf8(name: &str, nullable: bool) -> Field {
    Field::new(name, DataType::Utf8, nullable)
}

/// The `symbols` table schema.
#[must_use]
pub fn symbols_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        utf8("symbol_id", false),
        utf8("definition_id", false),
        utf8("path", false),
        utf8("name", false),
        utf8("kind", false),
        utf8("parent_path", true),
        utf8("signature", true),
        utf8("doc_summary", true),
        utf8("docs", true),
        Field::new("deprecated", DataType::Boolean, false),
        utf8("deprecated_since", true),
        utf8("deprecated_note", true),
        utf8("span_file", true),
        Field::new("span_line", DataType::UInt32, true),
        Field::new("is_reexport", DataType::Boolean, false),
        utf8("definition_path", false),
        utf8("defined_in_crate", false),
        Field::new("producer_local_id", DataType::UInt32, false),
        utf8("cfg_hints", false),
    ]))
}

/// The `relationships` table schema.
#[must_use]
pub fn relationships_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        utf8("relationship_id", false),
        utf8("source_id", false),
        utf8("source_path", false),
        utf8("target_id", true),
        utf8("target_path", false),
        utf8("relation", false),
        utf8("detail", true),
        utf8("producer", false),
    ]))
}

/// The `fragments` table schema.
#[must_use]
pub fn fragments_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        utf8("fragment_id", false),
        utf8("kind", false),
        utf8("subject", false),
        utf8("artifact_id", false),
        utf8("locator", false),
        utf8("text", false),
        utf8("evidence_class", false),
        utf8("producer", false),
        utf8("producer_version", false),
    ]))
}

fn strings<'a>(values: impl Iterator<Item = &'a str>) -> ArrayRef {
    Arc::new(StringArray::from_iter_values(values))
}

fn optional_strings<'a>(values: impl Iterator<Item = Option<&'a str>>) -> ArrayRef {
    Arc::new(StringArray::from_iter(values))
}

/// Symbols as a record batch.
///
/// # Errors
///
/// Fails only if the columns disagree with the schema, which would be a defect here.
pub fn symbols_to_batch(symbols: &[Symbol]) -> Result<RecordBatch, arrow::error::ArrowError> {
    let columns: Vec<ArrayRef> = vec![
        strings(symbols.iter().map(|s| s.symbol_id.as_str())),
        strings(symbols.iter().map(|s| s.definition_id.as_str())),
        strings(symbols.iter().map(|s| s.path.as_str())),
        strings(symbols.iter().map(|s| s.name.as_str())),
        strings(symbols.iter().map(|s| s.kind.as_str())),
        optional_strings(symbols.iter().map(|s| s.parent_path.as_deref())),
        optional_strings(symbols.iter().map(|s| s.signature.as_deref())),
        optional_strings(symbols.iter().map(|s| s.doc_summary.as_deref())),
        optional_strings(symbols.iter().map(|s| s.docs.as_deref())),
        Arc::new(BooleanArray::from_iter(
            symbols.iter().map(|s| Some(s.deprecated.is_some())),
        )),
        optional_strings(
            symbols
                .iter()
                .map(|s| s.deprecated.as_ref().and_then(|d| d.since.as_deref())),
        ),
        optional_strings(
            symbols
                .iter()
                .map(|s| s.deprecated.as_ref().and_then(|d| d.note.as_deref())),
        ),
        optional_strings(symbols.iter().map(|s| s.span_file.as_deref())),
        Arc::new(UInt32Array::from_iter(symbols.iter().map(|s| s.span_line))),
        Arc::new(BooleanArray::from_iter(
            symbols.iter().map(|s| Some(s.is_reexport)),
        )),
        strings(symbols.iter().map(|s| s.definition_path.as_str())),
        strings(symbols.iter().map(|s| s.defined_in_crate.as_str())),
        Arc::new(UInt32Array::from_iter_values(
            symbols.iter().map(|s| s.producer_local_id),
        )),
        Arc::new(StringArray::from_iter_values(symbols.iter().map(|s| {
            serde_json::to_string(&s.cfg_hints).unwrap_or_else(|_| "[]".into())
        }))),
    ];
    RecordBatch::try_new(symbols_schema(), columns)
}

fn str_col<'a>(batch: &'a RecordBatch, name: &str) -> Option<&'a StringArray> {
    batch
        .column_by_name(name)?
        .as_any()
        .downcast_ref::<StringArray>()
}

fn bool_col<'a>(batch: &'a RecordBatch, name: &str) -> Option<&'a BooleanArray> {
    batch
        .column_by_name(name)?
        .as_any()
        .downcast_ref::<BooleanArray>()
}

fn u32_col<'a>(batch: &'a RecordBatch, name: &str) -> Option<&'a UInt32Array> {
    batch
        .column_by_name(name)?
        .as_any()
        .downcast_ref::<UInt32Array>()
}

fn opt_str(col: Option<&StringArray>, i: usize) -> Option<String> {
    col.filter(|c| !c.is_null(i)).map(|c| c.value(i).to_owned())
}

fn req_str(col: Option<&StringArray>, i: usize) -> String {
    opt_str(col, i).unwrap_or_default()
}

/// Symbols from a record batch. Rows whose enum spelling is unknown are skipped rather than
/// guessed.
#[must_use]
pub fn symbols_from_batch(batch: &RecordBatch) -> Vec<Symbol> {
    let mut out = Vec::with_capacity(batch.num_rows());
    for i in 0..batch.num_rows() {
        let Some(kind) = SymbolKind::parse(&req_str(str_col(batch, "kind"), i)) else {
            continue;
        };
        let deprecated = bool_col(batch, "deprecated")
            .is_some_and(|c| c.value(i))
            .then(|| Deprecated {
                since: opt_str(str_col(batch, "deprecated_since"), i),
                note: opt_str(str_col(batch, "deprecated_note"), i),
            });
        let cfg_hints: Vec<String> =
            serde_json::from_str(&req_str(str_col(batch, "cfg_hints"), i)).unwrap_or_default();
        out.push(Symbol {
            symbol_id: req_str(str_col(batch, "symbol_id"), i),
            definition_id: req_str(str_col(batch, "definition_id"), i),
            path: req_str(str_col(batch, "path"), i),
            name: req_str(str_col(batch, "name"), i),
            kind,
            parent_path: opt_str(str_col(batch, "parent_path"), i),
            signature: opt_str(str_col(batch, "signature"), i),
            doc_summary: opt_str(str_col(batch, "doc_summary"), i),
            docs: opt_str(str_col(batch, "docs"), i),
            deprecated,
            span_file: opt_str(str_col(batch, "span_file"), i),
            span_line: u32_col(batch, "span_line")
                .filter(|c| !c.is_null(i))
                .map(|c| c.value(i)),
            is_reexport: bool_col(batch, "is_reexport").is_some_and(|c| c.value(i)),
            definition_path: req_str(str_col(batch, "definition_path"), i),
            defined_in_crate: req_str(str_col(batch, "defined_in_crate"), i),
            producer_local_id: u32_col(batch, "producer_local_id").map_or(0, |c| c.value(i)),
            cfg_hints,
        });
    }
    out
}

/// Relationships as a record batch.
///
/// # Errors
///
/// Fails only if the columns disagree with the schema.
pub fn relationships_to_batch(
    relationships: &[Relationship],
) -> Result<RecordBatch, arrow::error::ArrowError> {
    let columns: Vec<ArrayRef> = vec![
        strings(relationships.iter().map(|r| r.relationship_id.as_str())),
        strings(relationships.iter().map(|r| r.source_id.as_str())),
        strings(relationships.iter().map(|r| r.source_path.as_str())),
        optional_strings(relationships.iter().map(|r| r.target_id.as_deref())),
        strings(relationships.iter().map(|r| r.target_path.as_str())),
        strings(relationships.iter().map(|r| r.relation.as_str())),
        optional_strings(relationships.iter().map(|r| r.detail.as_deref())),
        strings(relationships.iter().map(|r| r.producer.as_str())),
    ];
    RecordBatch::try_new(relationships_schema(), columns)
}

/// Relationships from a record batch.
#[must_use]
pub fn relationships_from_batch(batch: &RecordBatch) -> Vec<Relationship> {
    let mut out = Vec::with_capacity(batch.num_rows());
    for i in 0..batch.num_rows() {
        let Some(relation) = RelationKind::parse(&req_str(str_col(batch, "relation"), i)) else {
            continue;
        };
        out.push(Relationship {
            relationship_id: req_str(str_col(batch, "relationship_id"), i),
            source_id: req_str(str_col(batch, "source_id"), i),
            source_path: req_str(str_col(batch, "source_path"), i),
            target_id: opt_str(str_col(batch, "target_id"), i),
            target_path: req_str(str_col(batch, "target_path"), i),
            relation,
            detail: opt_str(str_col(batch, "detail"), i),
            producer: req_str(str_col(batch, "producer"), i),
        });
    }
    out
}

/// Fragments as a record batch.
///
/// # Errors
///
/// Fails only if the columns disagree with the schema.
pub fn fragments_to_batch(
    fragments: &[EvidenceFragment],
) -> Result<RecordBatch, arrow::error::ArrowError> {
    let locators: Vec<String> = fragments
        .iter()
        .map(|f| canonical::to_canonical_string(&serde_json::Value::Object(f.locator.clone())))
        .collect();
    let classes: Vec<String> = fragments
        .iter()
        .map(|f| {
            serde_json::to_value(f.evidence_class)
                .ok()
                .and_then(|v| v.as_str().map(str::to_owned))
                .unwrap_or_default()
        })
        .collect();
    let columns: Vec<ArrayRef> = vec![
        strings(fragments.iter().map(|f| f.fragment_id.as_str())),
        strings(fragments.iter().map(|f| f.kind.as_str())),
        strings(fragments.iter().map(|f| f.subject.as_str())),
        strings(fragments.iter().map(|f| f.artifact_id.as_str())),
        strings(locators.iter().map(String::as_str)),
        strings(fragments.iter().map(|f| f.text.as_str())),
        strings(classes.iter().map(String::as_str)),
        strings(fragments.iter().map(|f| f.producer.as_str())),
        strings(fragments.iter().map(|f| f.producer_version.as_str())),
    ];
    RecordBatch::try_new(fragments_schema(), columns)
}

/// Fragments from a record batch.
#[must_use]
pub fn fragments_from_batch(batch: &RecordBatch) -> Vec<EvidenceFragment> {
    let mut out = Vec::with_capacity(batch.num_rows());
    for i in 0..batch.num_rows() {
        let Some(kind) = FragmentKind::parse(&req_str(str_col(batch, "kind"), i)) else {
            continue;
        };
        let class_text = req_str(str_col(batch, "evidence_class"), i);
        let Ok(evidence_class) =
            serde_json::from_value::<EvidenceClass>(serde_json::Value::String(class_text))
        else {
            continue;
        };
        let locator =
            serde_json::from_str::<serde_json::Value>(&req_str(str_col(batch, "locator"), i))
                .ok()
                .and_then(|v| v.as_object().cloned())
                .unwrap_or_default();
        out.push(EvidenceFragment {
            fragment_id: req_str(str_col(batch, "fragment_id"), i),
            kind,
            subject: req_str(str_col(batch, "subject"), i),
            artifact_id: req_str(str_col(batch, "artifact_id"), i),
            locator,
            text: req_str(str_col(batch, "text"), i),
            evidence_class,
            producer: req_str(str_col(batch, "producer"), i),
            producer_version: req_str(str_col(batch, "producer_version"), i),
        });
    }
    out
}

/// Write one batch to a Parquet file with the key-value metadata every table carries.
///
/// # Errors
///
/// Fails on I/O or encoding error.
pub fn write_parquet(
    path: &Path,
    batch: &RecordBatch,
    metadata: &[(&str, &str)],
) -> io::Result<u64> {
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(
            ZstdLevel::try_new(3).map_err(|e| io::Error::other(e.to_string()))?,
        ))
        .set_key_value_metadata(Some(
            metadata
                .iter()
                .map(|(k, v)| {
                    parquet::file::metadata::KeyValue::new((*k).to_owned(), (*v).to_owned())
                })
                .collect(),
        ))
        .build();
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))
        .map_err(|e| io::Error::other(e.to_string()))?;
    writer
        .write(batch)
        .map_err(|e| io::Error::other(e.to_string()))?;
    writer
        .close()
        .map_err(|e| io::Error::other(e.to_string()))?;
    Ok(batch.num_rows() as u64)
}

/// Read every batch of a Parquet file, concatenated.
///
/// # Errors
///
/// Fails on I/O or decoding error.
pub fn read_parquet(path: &Path) -> io::Result<Vec<RecordBatch>> {
    let file = File::open(path)?;
    let reader = ParquetRecordBatchReaderBuilder::try_new(file)
        .map_err(|e| io::Error::other(e.to_string()))?
        .build()
        .map_err(|e| io::Error::other(e.to_string()))?;
    reader
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::other(e.to_string()))
}

/// Count rows across batches.
#[must_use]
pub fn row_count(batches: &[RecordBatch]) -> u64 {
    batches.iter().map(|b| b.num_rows() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol(path: &str, reexport: bool) -> Symbol {
        Symbol {
            symbol_id: Symbol::symbol_id_for("c", path, SymbolKind::Function, None),
            definition_id: Symbol::definition_id_for(
                "c",
                "c::inner::f",
                SymbolKind::Function,
                None,
            ),
            path: path.to_owned(),
            name: "f".to_owned(),
            kind: SymbolKind::Function,
            parent_path: Some("c".to_owned()),
            signature: Some("pub fn c::f()".to_owned()),
            doc_summary: None,
            docs: Some("docs".to_owned()),
            deprecated: Some(Deprecated {
                since: Some("0.1.0".to_owned()),
                note: None,
            }),
            span_file: Some("src/lib.rs".to_owned()),
            span_line: Some(7),
            is_reexport: reexport,
            definition_path: "c::inner::f".to_owned(),
            defined_in_crate: "c".to_owned(),
            producer_local_id: 42,
            cfg_hints: vec!["#[cfg(unix)]".to_owned()],
        }
    }

    #[test]
    fn symbols_round_trip_through_parquet_semantically() {
        let symbols = vec![symbol("c::f", true), symbol("c::inner::f", false)];
        let batch = symbols_to_batch(&symbols).expect("batch");
        let dir = tempfile::tempdir().expect("dir");
        let path = dir.path().join(SYMBOLS_FILE);
        let rows = write_parquet(&path, &batch, &[("snapshot_id", "snap_test")]).expect("write");
        assert_eq!(rows, 2);
        let back: Vec<Symbol> = read_parquet(&path)
            .expect("read")
            .iter()
            .flat_map(symbols_from_batch)
            .collect();
        assert_eq!(back, symbols);
    }

    #[test]
    fn relationships_and_fragments_round_trip() {
        let rel = Relationship::new(
            "a",
            "c::a",
            Some("b"),
            "c::b",
            RelationKind::Implements,
            Some("trait_impl"),
            "p",
        );
        let batch = relationships_to_batch(std::slice::from_ref(&rel)).expect("batch");
        assert_eq!(relationships_from_batch(&batch), vec![rel]);

        let frag = EvidenceFragment::new(
            FragmentKind::DocText,
            "c::a",
            "art_x",
            serde_json::json!({"z": 1, "a": [1, 2]}),
            "text".to_owned(),
            EvidenceClass::StaticallyExtracted,
            "p",
            "1",
        );
        let batch = fragments_to_batch(std::slice::from_ref(&frag)).expect("batch");
        let dir = tempfile::tempdir().expect("dir");
        let path = dir.path().join(FRAGMENTS_FILE);
        write_parquet(&path, &batch, &[]).expect("write");
        let back: Vec<EvidenceFragment> = read_parquet(&path)
            .expect("read")
            .iter()
            .flat_map(fragments_from_batch)
            .collect();
        assert_eq!(back, vec![frag]);
    }

    #[test]
    fn the_schemas_have_the_declared_columns() {
        assert_eq!(symbols_schema().fields().len(), 19);
        assert_eq!(relationships_schema().fields().len(), 8);
        assert_eq!(fragments_schema().fields().len(), 9);
        assert!(symbols_to_batch(&[]).expect("empty batch").num_rows() == 0);
    }
}
