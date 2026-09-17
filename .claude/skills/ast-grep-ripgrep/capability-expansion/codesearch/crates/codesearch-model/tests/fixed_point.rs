//! The schema fixed-point check, run against real Delta storage.
//!
//! This is the test the whole type discipline exists for. Probe PB03 measured that Delta **does
//! not reject types, it silently narrows them** -- ten of eleven supposedly-unsupported Arrow
//! types are accepted and mapped to something else, with no error anywhere. A schema that is
//! merely *plausible* therefore round-trips into a different schema, and the first symptom is
//! corrupt data rather than a failed write.
//!
//! So: create every table for real, read every schema back, and require equality.
//!
//! It also settles the `Int8` lattice encoding empirically. The design stores lattice values as
//! ordinals rather than labels, which is only sound if `Int8` survives the trip -- the kernel
//! source says it maps to Delta `BYTE` and back, and this test is what makes that a measurement
//! instead of a reading.

use codesearch_model::{ddl, schema};

/// Every authored table survives a create/read-back cycle unchanged.
#[tokio::test]
async fn every_authored_schema_is_a_fixed_point_of_the_delta_conversion() {
    let scratch = tempfile::tempdir().expect("temp dir");
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("session builds");

    let specs = ddl::all_specs();
    assert!(!specs.is_empty(), "there must be tables to check");

    for spec in &specs {
        let url = ddl::ensure_table(scratch.path(), spec, &session)
            .await
            .unwrap_or_else(|e| panic!("creating {}: {e}", spec.name));

        let actual = ddl::read_back_schema(&url)
            .await
            .unwrap_or_else(|e| panic!("reading back {}: {e}", spec.name));

        ddl::assert_fixed_point(spec.name, &spec.arrow_schema, &actual)
            .unwrap_or_else(|e| panic!("{e}"));
    }
}

/// The control: a schema that is NOT a fixed point must be caught.
///
/// Without this, the test above could pass because the comparison is vacuous rather than because
/// the schemas match. `UInt32` is the specific case that matters -- it narrows to `Int32` and
/// silently truncates any byte offset above `i32::MAX`, which is every file over 2 GiB.
#[tokio::test]
async fn a_narrowing_type_is_actually_caught() {
    use arrow_schema::{DataType, Field, Schema};
    use std::sync::Arc;

    let scratch = tempfile::tempdir().expect("temp dir");
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("session builds");

    let deliberately_wrong = ddl::TableSpec {
        name: "control.narrowing",
        arrow_schema: Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            // The trap: authored unsigned, stored signed, no error at any point.
            Field::new("bad_offset", DataType::UInt32, false),
        ])),
        partition_columns: Vec::new(),
        constraints: Vec::new(),
    };

    let url = ddl::ensure_table(scratch.path(), &deliberately_wrong, &session)
        .await
        .expect("the table is created happily -- that is the point");
    let actual = ddl::read_back_schema(&url).await.expect("read back");

    let verdict = ddl::assert_fixed_point(
        deliberately_wrong.name,
        &deliberately_wrong.arrow_schema,
        &actual,
    );
    assert!(
        verdict.is_err(),
        "UInt32 must be reported as narrowed; if this passes, the fixed-point check is vacuous \
         and proves nothing about the real tables"
    );
}

/// Lattice constraints are derived from the lattice definitions, so they cannot go stale.
#[test]
fn every_lattice_column_carries_a_range_constraint() {
    for spec in ddl::all_specs() {
        let lattice_columns: Vec<&str> = spec
            .arrow_schema
            .fields()
            .iter()
            .filter_map(|f| f.name().strip_suffix("_rank"))
            .filter(|stem| codesearch_bridge::lattice::lattice(stem).is_some())
            .collect();
        for stem in lattice_columns {
            let expected = format!("{stem}_rank_in_range");
            assert!(
                spec.constraints.iter().any(|(name, _)| name == &expected),
                "{}.{stem}_rank has no range constraint",
                spec.name
            );
        }
    }
}

/// No constraint may name a UDF or another table.
///
/// PB07 measured both failure modes: a constraint calling a UDF makes the table unwritable by any
/// session lacking that UDF, and one naming another table **panics the process**. This check is
/// cheap and the failures are expensive, so it runs over every authored constraint.
#[test]
fn no_constraint_references_a_udf_or_another_table() {
    let udf_names = ["lattice_label"];
    for spec in ddl::all_specs() {
        for (name, expr) in &spec.constraints {
            let lowered = expr.to_lowercase();
            assert!(
                !lowered.contains("select"),
                "{}.{name} contains a subquery; a constraint naming another table panics",
                spec.name
            );
            for udf in udf_names {
                assert!(
                    !lowered.contains(udf),
                    "{}.{name} calls `{udf}`; that locks the table to sessions registering it",
                    spec.name
                );
            }
        }
    }
}

/// The schema check must also inspect column *metadata*, since identity is a storage property.
///
/// PB01 measured that Arrow extension metadata survives a Delta round trip byte-for-byte. If that
/// ever stops being true, the identity discipline silently degrades to plain strings, so it is
/// worth asserting rather than assuming.
#[tokio::test]
async fn extension_metadata_survives_the_round_trip() {
    let scratch = tempfile::tempdir().expect("temp dir");
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("session builds");

    let spec = ddl::all_specs()
        .into_iter()
        .find(|s| s.name == "snapshot.source_anchor")
        .expect("the anchor table carries byte-offset and entity-key extension names");

    let url = ddl::ensure_table(scratch.path(), &spec, &session)
        .await
        .expect("create");
    let actual = ddl::read_back_schema(&url).await.expect("read back");

    let authored_offsets: Vec<&str> = spec
        .arrow_schema
        .fields()
        .iter()
        .filter(|f| {
            f.metadata().get("ARROW:extension:name").map(String::as_str)
                == Some(schema::EXT_BYTE_OFFSET)
        })
        .map(|f| f.name().as_str())
        .collect();
    assert!(
        !authored_offsets.is_empty(),
        "the fixture table must actually carry the extension name"
    );

    for name in authored_offsets {
        let field = actual.field_with_name(name).expect("column survives");
        assert_eq!(
            field
                .metadata()
                .get("ARROW:extension:name")
                .map(String::as_str),
            Some(schema::EXT_BYTE_OFFSET),
            "extension name lost on `{name}` -- identity is no longer a storage property"
        );
    }
}
