//! Write the wire JSON Schemas generated from the authoritative Rust types.
//!
//! Invoked by `scripts/schemas-generate.sh` and `scripts/schema-conformance.sh` as:
//!
//! ```text
//! cargo run -p enrichment-core --bin emit-schemas -- --out schemas/generated
//! ```
//!
//! Both scripts probe `cargo metadata` for a binary with exactly this name, so renaming the
//! target silently disables schema generation and the conformance gate reports `not_run`.
//!
//! Two files are written: the response envelope (the frozen Phase-0 contract's counterpart)
//! and the tool payloads (`data` shapes for the research tools). Output is byte-reproducible;
//! `schema-conformance.sh` runs `git diff --exit-code` over the output directory to prove it.
//! See `enrichment_core::wire::schema::envelope_schema` and `wire::data::tool_data_schema`.

use std::path::PathBuf;
use std::process::ExitCode;

use enrichment_core::wire::data::{TOOL_DATA_SCHEMA_FILE, tool_data_schema_json};
use enrichment_core::wire::{ENVELOPE_SCHEMA_FILE, envelope_schema_json};

const USAGE: &str = "usage: emit-schemas --out <dir>";

fn main() -> ExitCode {
    let out = match parse_out_dir(std::env::args().skip(1)) {
        Ok(out) => out,
        Err(message) => {
            eprintln!("emit-schemas: {message}\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = std::fs::create_dir_all(&out) {
        eprintln!("emit-schemas: cannot create {}: {err}", out.display());
        return ExitCode::FAILURE;
    }

    for (file, contents) in [
        (ENVELOPE_SCHEMA_FILE, envelope_schema_json()),
        (TOOL_DATA_SCHEMA_FILE, tool_data_schema_json()),
        (
            "request.schema.json",
            format!(
                "{}\n",
                serde_json::to_string_pretty(&enrichment_core::canonical::canonicalize(
                    enrichment_core::request::request_schema()
                ))
                .expect("request schema")
            ),
        ),
        (
            "worker.schema.json",
            format!(
                "{}\n",
                serde_json::to_string_pretty(&enrichment_core::canonical::canonicalize(
                    schemars::generate::SchemaSettings::draft2020_12()
                        .for_serialize()
                        .into_generator()
                        .into_root_schema_for::<enrichment_core::producer::python::WorkerProtocol>()
                        .to_value()
                ))
                .expect("schema")
            ),
        ),
    ] {
        let path = out.join(file);
        if let Err(err) = std::fs::write(&path, contents) {
            eprintln!("emit-schemas: cannot write {}: {err}", path.display());
            return ExitCode::FAILURE;
        }
        println!("emit-schemas: wrote {}", path.display());
    }
    ExitCode::SUCCESS
}

/// Parse `--out <dir>`.
///
/// One flag does not justify an argument-parsing dependency: `clap` is absent from the lock and
/// would add roughly eight crates to satisfy a single option.
fn parse_out_dir(args: impl Iterator<Item = String>) -> Result<PathBuf, String> {
    let mut out = None;
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--out needs a value".to_owned())?;
                out = Some(PathBuf::from(value));
            }
            other => return Err(format!("unexpected argument `{other}`")),
        }
    }
    out.ok_or_else(|| "--out is required".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_dir_is_parsed() {
        let args = ["--out".to_owned(), "schemas/generated".to_owned()];
        assert_eq!(
            parse_out_dir(args.into_iter()),
            Ok(PathBuf::from("schemas/generated"))
        );
    }

    #[test]
    fn a_missing_out_dir_is_an_error() {
        assert!(parse_out_dir(std::iter::empty()).is_err());
    }

    #[test]
    fn an_unknown_flag_is_an_error() {
        let args = ["--verbose".to_owned()];
        assert!(parse_out_dir(args.into_iter()).is_err());
    }
}
