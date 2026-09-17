//! Probes PB01 and PB03 — the two round-2 gating questions that needed a Rust compile.
//!
//! PB03  Which Arrow types can actually be persisted, and what happens to the ones that cannot?
//! PB01  Does Arrow extension-type metadata survive a Delta write/read cycle?
//!
//! Both are run against the exact pins the plan targets (see Cargo.toml). Every phase prints
//! its construction alongside its result, because a result without the construction that
//! produced it is not reproducible.
//!
//! Discipline borrowed from the surrounding capability repositories: a probe without a control
//! proves nothing. Each phase below names its control explicitly.

use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{Array, RecordBatch, StringArray, new_null_array};
use arrow_schema::{DataType, Field, Fields, Schema, SchemaRef, TimeUnit};
use deltalake::DeltaTableBuilder;
use deltalake::kernel::StructField;
// `try_from_arrow` is a TRAIT method, not an inherent one. The skill's flat method list
// shows the signature on `DataType` and the trait requirement is easy to miss.
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::operations::write::WriteBuilder;

/// Arrow types the plan wanted (evidence 03) paired with why they were wanted.
/// The `expect_rejected` flag records what evidence 04 §1 *derived* from the kernel's
/// `PrimitiveType` enum — this probe exists to confirm or refute that derivation.
fn candidates() -> Vec<(&'static str, DataType, bool, &'static str)> {
    vec![
        // --- controls: these MUST be accepted, or the probe itself is broken -------------
        ("Utf8", DataType::Utf8, false, "control"),
        ("Int64", DataType::Int64, false, "control"),
        ("Int32", DataType::Int32, false, "control"),
        ("Boolean", DataType::Boolean, false, "control"),
        ("Binary", DataType::Binary, false, "control"),
        (
            "Timestamp(us,None)",
            DataType::Timestamp(TimeUnit::Microsecond, None),
            false,
            "control",
        ),
        (
            "Struct",
            DataType::Struct(Fields::from(vec![
                Field::new("start", DataType::Int32, false),
                Field::new("end", DataType::Int32, false),
            ])),
            false,
            "control: SourceAnchor.native_range",
        ),
        (
            "List<Utf8>",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
            "control: ordered child records",
        ),
        (
            "Map<Utf8,Utf8>",
            DataType::Map(
                Arc::new(Field::new(
                    "entries",
                    DataType::Struct(Fields::from(vec![
                        Field::new("keys", DataType::Utf8, false),
                        Field::new("values", DataType::Utf8, true),
                    ])),
                    false,
                )),
                false,
            ),
            false,
            "control: assumptions / free-form detail",
        ),
        // --- derived as REJECTED in evidence 04 §1; this probe tests that ---------------
        (
            "Dictionary(Int32,Utf8)",
            DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8)),
            true,
            "wanted for: enum-like vocabularies (kind, origin, precision)",
        ),
        (
            "FixedSizeBinary(32)",
            DataType::FixedSizeBinary(32),
            true,
            "wanted for: content_hash, entity_id",
        ),
        ("UInt32", DataType::UInt32, true, "wanted for: byte ranges"),
        ("UInt64", DataType::UInt64, true, "wanted for: counts"),
        (
            "Utf8View",
            DataType::Utf8View,
            true,
            "wanted for: retained source text",
        ),
        (
            "BinaryView",
            DataType::BinaryView,
            true,
            "wanted for: retained native artifacts",
        ),
        (
            "Float16",
            DataType::Float16,
            true,
            "not wanted; boundary check",
        ),
        (
            "LargeList<Utf8>",
            DataType::LargeList(Arc::new(Field::new("item", DataType::Utf8, true))),
            true,
            "wanted for: very wide child lists",
        ),
        (
            "LargeUtf8",
            DataType::LargeUtf8,
            true,
            "wanted for: long doc strings",
        ),
        (
            "Decimal128(10,2)",
            DataType::Decimal128(10, 2),
            false,
            "boundary check: Delta has Decimal",
        ),
        ("Date32", DataType::Date32, false, "boundary check"),
    ]
}

// ---------------------------------------------------------------------------- PB03 phase A

/// Ask the kernel directly whether it can represent each Arrow type.
fn phase_a_kernel_conversion() -> Vec<(&'static str, bool, String)> {
    println!("== PB03-A · deltalake::DataType::try_from_arrow(&ArrowDataType) ==");
    println!("   Does the Delta kernel have a representation for this Arrow type at all?\n");
    println!(
        "   {:<24} {:<10} {}",
        "arrow type", "accepted", "delta type or error"
    );
    println!("   {}", "-".repeat(84));

    let mut results = Vec::new();
    for (name, data_type, _expect_rejected, _why) in candidates() {
        let (accepted, detail) = match deltalake::kernel::DataType::try_from_arrow(&data_type) {
            Ok(delta_type) => (true, format!("{delta_type:?}")),
            Err(error) => (false, format!("{error}")),
        };
        println!("   {name:<24} {:<10} {}", accepted, truncate(&detail, 46));
        results.push((name, accepted, detail));
    }
    println!();
    results
}

// ---------------------------------------------------------------------------- PB03 phase B

/// `normalize_for_delta` is INFALLIBLE — it returns a schema, never an error. So the real
/// question is not "what does delta-rs reject" but "what does it silently rewrite", which has
/// a very different design consequence: a silent rewrite means the schema you read back is not
/// the schema you wrote, and nothing raised.
fn phase_b_normalisation() -> Vec<(&'static str, String)> {
    println!("== PB03-B · deltalake::cast::normalize_for_delta(&SchemaRef) ==");
    println!("   Signature returns SchemaRef, not Result — so any change here is SILENT.\n");
    println!("   {:<24} {}", "arrow type in", "arrow type out");
    println!("   {}", "-".repeat(84));

    let mut results = Vec::new();
    for (name, data_type, _expect_rejected, _why) in candidates() {
        let schema: SchemaRef =
            Arc::new(Schema::new(vec![Field::new("c", data_type.clone(), true)]));
        let normalised = deltalake::cast::normalize_for_delta(&schema);
        let out = normalised.field(0).data_type().to_string();
        let changed = if out == data_type.to_string() {
            " "
        } else {
            "*"
        };
        println!("   {name:<24} {changed} {}", truncate(&out, 50));
        results.push((name, out));
    }
    println!("\n   (* marks a silent rewrite)\n");
    results
}

// ---------------------------------------------------------------------------- PB03 phase C

/// End-to-end: create a Delta table and write a batch, one column per type. Whatever the two
/// phases above suggest, this is the truth — it is the path the real ingestion will take.
async fn phase_c_end_to_end(root: &std::path::Path) -> Vec<(&'static str, bool, String)> {
    println!("== PB03-C · CreateBuilder + WriteBuilder, one table per type ==");
    println!("   The path real ingestion takes. This is the authoritative answer.\n");
    println!(
        "   {:<24} {:<10} {}",
        "arrow type", "written", "schema read back / error"
    );
    println!("   {}", "-".repeat(84));

    let mut results = Vec::new();
    for (name, data_type, _expect_rejected, _why) in candidates() {
        let location = root.join(format!("t_{}", sanitise(name)));
        let outcome = write_one(&location, &data_type).await;
        let (ok, detail) = match outcome {
            Ok(read_back) => (true, read_back),
            Err(error) => (false, format!("{error}")),
        };
        println!("   {name:<24} {:<10} {}", ok, truncate(&detail, 46));
        results.push((name, ok, detail));
    }
    println!();
    results
}

async fn write_one(
    location: &std::path::Path,
    data_type: &DataType,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = url::Url::from_directory_path(location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;

    let arrow_schema: SchemaRef =
        Arc::new(Schema::new(vec![Field::new("c", data_type.clone(), true)]));
    let delta_fields: Vec<StructField> = arrow_schema
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())?;
            Ok::<StructField, Box<dyn std::error::Error>>(StructField::new(
                f.name().clone(),
                dt,
                f.is_nullable(),
            ))
        })
        .collect::<Result<_, _>>()?;

    let table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields)
        .await?;

    let array = new_null_array(data_type, 1);
    let batch = RecordBatch::try_new(arrow_schema, vec![array])?;
    let _ = WriteBuilder::new(log_store, table.state.map(|s| s.snapshot().clone()))
        .with_input_batches(vec![batch])
        .await?;

    let mut reopened = DeltaTableBuilder::from_url(url)?.load().await?;
    reopened.load().await?;
    let read_schema = reopened.snapshot()?.snapshot().arrow_schema();
    Ok(read_schema.field(0).data_type().to_string())
}

// ---------------------------------------------------------------------------- PB01

/// Does Arrow extension-type metadata survive a write/read cycle?
///
/// Control: an ordinary metadata key on a second field. If BOTH vanish, all field metadata is
/// stripped; if only the extension keys vanish, extension types specifically are not carried.
/// Without the control the two are indistinguishable, and they have different remedies.
async fn pb01_extension_metadata(root: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("== PB01 · Does Arrow extension-type metadata survive a Delta round trip? ==");
    println!("   Control: an ordinary metadata key on a second field.\n");

    let mut extension_meta = HashMap::new();
    extension_meta.insert(
        "ARROW:extension:name".to_string(),
        "codesearch.entity_id".to_string(),
    );
    extension_meta.insert(
        "ARROW:extension:metadata".to_string(),
        "{\"v\":1}".to_string(),
    );

    let mut ordinary_meta = HashMap::new();
    ordinary_meta.insert("codesearch.family".to_string(), "rustdoc-json".to_string());
    ordinary_meta.insert("codesearch.precision".to_string(), "exact".to_string());

    let typed =
        Field::new("entity_id", DataType::Utf8, false).with_metadata(extension_meta.clone());
    let control = Field::new("note", DataType::Utf8, true).with_metadata(ordinary_meta.clone());
    let arrow_schema: SchemaRef = Arc::new(Schema::new(vec![typed, control]));

    println!(
        "   wrote field `entity_id` metadata : {:?}",
        sorted(&extension_meta)
    );
    println!(
        "   wrote field `note`      metadata : {:?}",
        sorted(&ordinary_meta)
    );

    // What does normalisation alone do to it, before any I/O?
    let normalised = deltalake::cast::normalize_for_delta(&arrow_schema);
    println!(
        "   after normalize_for_delta         : entity_id={:?} note={:?}",
        sorted(normalised.field(0).metadata()),
        sorted(normalised.field(1).metadata())
    );

    let location = root.join("t_pb01");
    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;

    let delta_fields: Vec<StructField> = arrow_schema
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())?;
            // `with_metadata` REPLACES the field's metadata; it does not extend it. Calling it
            // once per entry in a loop therefore keeps only whichever entry happened to be last
            // out of the HashMap. The first draft of this probe did exactly that, and both the
            // subject and the control came back having lost precisely one key -- which is what
            // the control is for: a delta-rs behaviour would not have damaged both identically.
            let all: Vec<(String, String)> = f
                .metadata()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let sf = StructField::new(f.name().clone(), dt, f.is_nullable()).with_metadata(all);
            Ok::<StructField, Box<dyn std::error::Error>>(sf)
        })
        .collect::<Result<_, _>>()?;

    let table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields)
        .await?;

    let batch = RecordBatch::try_new(
        arrow_schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["e1"])) as Arc<dyn Array>,
            Arc::new(StringArray::from(vec![Some("hello")])) as Arc<dyn Array>,
        ],
    )?;
    let _ = WriteBuilder::new(log_store, table.state.map(|s| s.snapshot().clone()))
        .with_input_batches(vec![batch])
        .await?;

    let mut reopened = DeltaTableBuilder::from_url(url)?.load().await?;
    reopened.load().await?;
    let read_schema = reopened.snapshot()?.snapshot().arrow_schema();

    println!(
        "   read back  `entity_id` metadata   : {:?}",
        sorted(read_schema.field(0).metadata())
    );
    println!(
        "   read back  `note`      metadata   : {:?}",
        sorted(read_schema.field(1).metadata())
    );

    let ext_survived = read_schema
        .field(0)
        .metadata()
        .contains_key("ARROW:extension:name");
    let control_survived = read_schema
        .field(1)
        .metadata()
        .contains_key("codesearch.family");

    println!("\n   VERDICT");
    println!("     extension metadata survived : {ext_survived}");
    println!("     control metadata survived   : {control_survived}");
    println!(
        "     interpretation              : {}",
        match (ext_survived, control_survived) {
            (true, true) =>
                "all field metadata round-trips; extension types are a storage property",
            (false, true) =>
                "extension keys specifically are dropped; ordinary metadata survives, so re-attach from a companion key",
            (false, false) =>
                "ALL field metadata is dropped; metadata must live in schema-level or DomainMetadata",
            (true, false) => "unexpected: extension survived but control did not -- re-run",
        }
    );
    println!();
    Ok(())
}

// ---------------------------------------------------------------------------- helpers

fn sorted(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut items: Vec<_> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    items.sort();
    items
}

fn truncate(text: &str, limit: usize) -> String {
    let flat = text.replace('\n', " ");
    if flat.chars().count() <= limit {
        flat
    } else {
        flat.chars()
            .take(limit.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}

fn sanitise(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let root = std::path::PathBuf::from(root);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root)?;

    println!("delta-arrow-probe");
    println!("  deltalake git rev 58f07cd62bfbce3649a7e1c87c696288068ae184");
    println!("  arrow 59.3.0 (canonical_extension_types), datafusion 55.1.0");
    println!("  scratch root {}\n", root.display());

    let kernel = phase_a_kernel_conversion();
    let normalised = phase_b_normalisation();
    let end_to_end = phase_c_end_to_end(&root).await;
    pb01_extension_metadata(&root).await?;

    println!("== PB03 summary ==\n");
    println!(
        "   {:<24} {:<9} {:<9} {}",
        "arrow type", "kernel", "written", "normalised to"
    );
    println!("   {}", "-".repeat(84));
    for (index, (name, _dt, expected_rejected, why)) in candidates().into_iter().enumerate() {
        let kernel_ok = kernel[index].1;
        let write_ok = end_to_end[index].1;
        let norm = &normalised[index].1;
        let surprise = if kernel_ok == expected_rejected {
            "  <-- CONTRADICTS evidence 04 §1"
        } else {
            ""
        };
        println!(
            "   {name:<24} {:<9} {:<9} {}{}",
            kernel_ok,
            write_ok,
            truncate(norm, 22),
            surprise
        );
        let _ = why;
    }
    println!();
    Ok(())
}
