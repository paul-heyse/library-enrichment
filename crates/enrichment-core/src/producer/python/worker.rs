//! Rust-owned mechanical Griffe fact transport. No evidence identities or alias decisions
//! are made by the Python encoder. The schema is shipped verbatim as an empty IPC stream.
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    error::ArrowError,
    ipc::writer::StreamWriter,
};
use std::{collections::HashMap, io::Write, sync::Arc};

pub const VERSION: &str = "python-arrow-facts/2";
pub const PROTOCOL: &str = "3.0";
pub const ENCODER: &str = "25.0.1";
pub const GRIFFE: &str = "2.3.0";
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_BATCH_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_BATCH_ROWS: usize = 256;
pub const MAX_OBSERVATIONS: usize = 100_000;
pub const MAX_FILES: usize = 65_536;

fn text(name: &str, nullable: bool) -> Field {
    Field::new(name, DataType::Utf8, nullable)
}
pub fn schema() -> SchemaRef {
    let observation = Field::new(
        "observation",
        DataType::Struct(
            <super::Observation as crate::native_union::NativeStruct>::fields()
                .iter()
                .map(|field| crate::native_schema::producer_field(field))
                .collect(),
        ),
        true,
    );
    Arc::new(Schema::new_with_metadata(vec![
        text("fact", false), text("file", true),
        Field::new("ordinal", DataType::UInt64, true), observation,
        text("detail", true), Field::new("count", DataType::UInt64, true),
        text("griffe_version", true), text("worker_python", true), text("encoder_version", true),
    ], HashMap::from([("enrichment.worker".into(), serde_json::json!({
        "version": VERSION, "protocol": PROTOCOL, "encoder": ENCODER, "griffe": GRIFFE,
        "max_bytes": MAX_BYTES, "batch_bytes": MAX_BATCH_BYTES, "batch_rows": MAX_BATCH_ROWS,
        "max_observations": MAX_OBSERVATIONS, "max_files": MAX_FILES,
    }).to_string())])))
}

/// Emit the authoritative schema without maintaining a second Python type declaration.
pub fn write_schema(output: impl Write) -> Result<(), ArrowError> {
    StreamWriter::try_new(output, &schema())?.finish()
}
