//! PB19 — does DataFusion's CSV reader REJECT a row whose field count disagrees with the schema?
//!
//! `index.rs:12-14` states the guarantee the hand-rolled reader exists to provide:
//!
//! > "a silently shifted column is exactly the class of error this catalog exists to prevent: it
//! > would not fail, it would produce confidently wrong answers."
//!
//! Step 3 of the plan replaces that reader with `CsvReadOptions`. **Nothing in `evidence/` has
//! ever exercised a DataFusion file reader** -- no `read_csv`, `register_csv`, `CsvReadOptions`,
//! `ListingTable` or `FileFormat` appears in any of the seventeen dossiers or nine probes -- so
//! the guarantee would be dropped or kept by accident. This measures it first.
//!
//!     cargo run --example pb19_csv_arity
//!
//! Two of the option defaults make the question urgent rather than academic, and neither is
//! documented in the pinned API surface:
//!
//!   * `truncated_rows(bool)` -- "allow short rows, fill null". If that defaults to TRUE, a
//!     shifted column becomes precisely the silent-null failure `index.rs` was written to catch.
//!   * `file_extension` -- if it defaults to `".csv"`, every `.tsv` is rejected outright, which
//!     is loud and therefore harmless, but needs knowing.
//!
//! ARMS -- the control is first, because "short rows error" is only attributable if a conforming
//! file of the same shape, read through the same options, succeeds.
//!
//!   A  CONTROL: three conforming 3-field rows. Must read 3 rows with every cell intact.
//!   A' CONTROL: the same bytes with `file_extension` left at its default, to resolve that default
//!   B  a 2-field row      -- error, or null-padded?
//!   C  a 4-field row      -- error, or truncated?
//!   D  arm B's file with `truncated_rows(true)` -- names which default arm B exercised
//!   E  a cell CONTAINING `"` -- the skill's contract says there is no quoting; confirm the
//!      reader does not reinterpret one.
//!   F  a cell STARTING with `"` -- the arm that matters. RFC4180 readers treat a quote as special
//!      only at field start, so E passing proves nothing about F. A summary beginning with a quote
//!      character is ordinary index data, and if the reader consumes it the cell is silently
//!      wrong rather than rejected.
//!
//! The schema is three all-`Utf8` columns, which is what every `IndexSpec` in `index.rs` is.


use arrow::array::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use datafusion::prelude::{CsvReadOptions, SessionContext};

/// What one arm observed. `Err` carries the error's `to_string()` -- the message is the finding.
type Observed = Result<Vec<Vec<Option<String>>>, String>;

/// Write one TSV and read it back through the supplied options.
async fn read_tsv(
    ctx: &SessionContext,
    root: &std::path::Path,
    name: &str,
    body: &str,
    options: CsvReadOptions<'_>,
) -> Observed {
    let path = root.join(name);
    std::fs::write(&path, body).map_err(|e| format!("write {name}: {e}"))?;
    let frame = ctx
        .read_csv(path.display().to_string(), options)
        .await
        .map_err(|e| format!("plan: {e}"))?;
    let batches = frame.collect().await.map_err(|e| format!("execute: {e}"))?;
    Ok(rows_of(&batches))
}

/// Every cell, as `Option<String>`, so a null is distinguishable from an empty string -- which is
/// the entire difference between "padded" and "present but empty".
fn rows_of(batches: &[RecordBatch]) -> Vec<Vec<Option<String>>> {
    use arrow::array::{Array, StringArray};
    let mut out = Vec::new();
    for batch in batches {
        for row in 0..batch.num_rows() {
            let mut cells = Vec::with_capacity(batch.num_columns());
            for col in 0..batch.num_columns() {
                let array = batch.column(col);
                let cell = array
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .filter(|a| a.is_valid(row))
                    .map(|a| a.value(row).to_string());
                cells.push(cell);
            }
            out.push(cells);
        }
    }
    out
}

fn show(label: &str, observed: &Observed) {
    match observed {
        Err(e) => println!("     {label:<22} ERROR  {}", e.replace('\n', " ")),
        Ok(rows) => {
            println!("     {label:<22} {} row(s)", rows.len());
            for row in rows {
                let cells: Vec<String> = row
                    .iter()
                    .map(|c| match c {
                        None => "NULL".to_string(),
                        Some(v) => format!("{v:?}"),
                    })
                    .collect();
                println!("     {:<22}   [{}]", "", cells.join(", "));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("pb19-csv-arity")
            .display()
            .to_string()
    });
    let root = std::path::PathBuf::from(root);
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root)?;

    // Three all-Utf8 columns: the shape of every IndexSpec in index.rs.
    let schema = Schema::new(vec![
        Field::new("a", DataType::Utf8, true),
        Field::new("b", DataType::Utf8, true),
        Field::new("c", DataType::Utf8, true),
    ]);

    // The options the plan proposes. `.schema()` BORROWS, so `schema` must outlive every use.
    let opts = || {
        CsvReadOptions::new()
            .has_header(false)
            .delimiter(b'\t')
            .file_extension(".tsv")
            .schema(&schema)
    };

    println!("PB19 — CSV reader arity against an explicit 3-column Utf8 schema\n");

    // The defaults, read off the struct rather than inferred from behaviour.
    let d = CsvReadOptions::new();
    println!("  defaults   has_header={}  delimiter={:?}  quote={:?}", d.has_header, d.delimiter as char, d.quote as char);
    println!("             file_extension={:?}  truncated_rows={}", d.file_extension, d.truncated_rows);
    println!();

    let ctx = SessionContext::new();
    let mut failures: Vec<&str> = Vec::new();

    // ---- A: CONTROL -------------------------------------------------------------------------
    println!("  A  CONTROL  three conforming 3-field rows       must read 3 rows, cells intact");
    let a = read_tsv(&ctx, &root, "a.tsv", "1\tx\tp\n2\ty\tq\n3\tz\tr\n", opts()).await;
    show("conforming", &a);
    match &a {
        Ok(rows) if rows.len() == 3 && rows.iter().all(|r| r.iter().all(Option::is_some)) => {}
        _ => failures.push("A: the control did not read cleanly, so no other arm is attributable"),
    }
    println!();

    // ---- A': the file_extension default ------------------------------------------------------
    println!("  A' CONTROL  same bytes, file_extension DEFAULT  resolves that default");
    let a2 = read_tsv(
        &ctx,
        &root,
        "a2.tsv",
        "1\tx\tp\n2\ty\tq\n3\tz\tr\n",
        CsvReadOptions::new()
            .has_header(false)
            .delimiter(b'\t')
            .schema(&schema),
    )
    .await;
    show("default extension", &a2);
    println!();

    // ---- B: a SHORT row ----------------------------------------------------------------------
    println!("  B  a 2-field row among conforming ones          error, or null-padded?");
    let b = read_tsv(&ctx, &root, "b.tsv", "1\tx\tp\n2\ty\n3\tz\tr\n", opts()).await;
    show("short row", &b);
    println!();

    // ---- C: a LONG row -----------------------------------------------------------------------
    println!("  C  a 4-field row among conforming ones          error, or truncated?");
    let c = read_tsv(&ctx, &root, "c.tsv", "1\tx\tp\n2\ty\tq\tEXTRA\n3\tz\tr\n", opts()).await;
    show("long row", &c);
    println!();

    // ---- D: arm B under truncated_rows(true) -------------------------------------------------
    println!("  D  arm B's bytes with truncated_rows(true)      names which default B exercised");
    let d_arm = read_tsv(
        &ctx,
        &root,
        "d.tsv",
        "1\tx\tp\n2\ty\n3\tz\tr\n",
        opts().truncated_rows(true),
    )
    .await;
    show("truncated_rows(true)", &d_arm);
    // If B and D agree, the default already IS truncated_rows(true) -- which is the dangerous case.
    match (&b, &d_arm) {
        (Ok(bx), Ok(dx)) if bx == dx => {
            println!("     -> B and D AGREE: the default already allows short rows");
        }
        (Err(_), Ok(_)) => println!("     -> B errored and D did not: the default REJECTS short rows"),
        _ => println!("     -> B and D differ in another way; read the rows above"),
    }
    println!();

    // ---- E: an embedded quote ----------------------------------------------------------------
    println!("  E  a cell containing a double quote             contract says no quoting");
    let e = read_tsv(&ctx, &root, "e.tsv", "1\ta\"b\tp\n2\ty\tq\n", opts()).await;
    show("embedded quote", &e);
    match &e {
        Ok(rows) if rows.len() == 2 && rows[0][1].as_deref() == Some("a\"b") => {
            println!("     -> the quote is DATA; the contract survives");
        }
        Ok(_) => {
            println!("     -> the quote was REINTERPRETED; set .quote() explicitly");
            failures.push("E: a bare quote changed the parse; the no-quoting contract needs .quote()");
        }
        Err(_) => failures.push("E: a bare quote made the read fail; set .quote() explicitly"),
    }
    println!();

    // ---- F: a cell STARTING with a quote -----------------------------------------------------
    println!("  F  a cell STARTING with a double quote         quotes are special at field start");
    let f = read_tsv(&ctx, &root, "f.tsv", "1\t\"quoted\"\tp\n2\ty\tq\n", opts()).await;
    show("leading quote", &f);
    let f_ok = matches!(&f, Ok(rows) if rows.len() == 2 && rows[0][1].as_deref() == Some("\"quoted\""));
    if f_ok {
        println!("     -> the leading quote is DATA too; no .quote() needed");
    } else {
        println!("     -> REINTERPRETED at field start; the reader must disable quoting");
    }
    println!();

    // ---- F': the same bytes with quoting disabled --------------------------------------------
    println!("  F' arm F with quoting disabled                 the fix, if F needs one");
    let f2 = read_tsv(
        &ctx,
        &root,
        "f2.tsv",
        "1\t\"quoted\"\tp\n2\ty\tq\n",
        opts().quote(b'\0'),
    )
    .await;
    show("quote(b'\\0')", &f2);
    match &f2 {
        Ok(rows) if rows.len() == 2 && rows[0][1].as_deref() == Some("\"quoted\"") => {
            println!("     -> quoting disabled reads the cell verbatim");
        }
        _ if !f_ok => failures.push("F: a leading quote is reinterpreted and disabling quoting did not fix it"),
        _ => {}
    }
    println!();

    println!("VERDICT");
    if failures.is_empty() {
        println!("  every arm behaved as the staging reader needs.");
    } else {
        for f in &failures {
            println!("  ATTENTION  {f}");
        }
    }
    Ok(())
}
