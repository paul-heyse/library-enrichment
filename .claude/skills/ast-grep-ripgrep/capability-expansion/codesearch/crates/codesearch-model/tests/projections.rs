//! Every `violations_*` view, with a fixture built to break it.
//!
//! A check that cannot fail proves nothing. `validate` against the real catalog reports zero
//! violations on every invariant, and that result is worth exactly as much as the evidence that a
//! violation would have shown up -- which is what this file is.
//!
//! The fixtures are built from the **authored Arrow schemas**, not from a hand-written column list,
//! so a column renamed in `schema.rs` but not in a view is a failure here rather than a view that
//! silently stops matching anything.
//!
//! Delta is deliberately absent. These are in-memory tables under the same SQL names the query
//! binary registers, because what is under test is the SQL of the views, and involving storage
//! would make a failure ambiguous between the two.

use std::collections::BTreeMap;
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, BooleanArray, Int8Array, Int32Array, Int64Array, ListArray, RecordBatch,
    StringArray,
};
use arrow::buffer::OffsetBuffer;
use arrow_schema::{DataType, SchemaRef};
use codesearch_bridge::{identity, lattice, lazy as projection_lazy, projection, session};
use codesearch_model::schema;
use datafusion::common::tree_node::{TreeNode, TreeNodeRecursion};
use datafusion::datasource::MemTable;
use datafusion::logical_expr::expr_schema::ExprSchemable;
use datafusion::logical_expr::{Expr, LogicalPlan};
use datafusion::prelude::SessionContext;

const TABLE_NAMES: &[&str] = &[
    "capability",
    "mechanism",
    "parameter",
    "surface_binding",
    "surface_entry",
    "behavior_assertion",
    "result_contract",
    "search_domain",
    "negative_space",
    "interaction",
    "interaction_atom",
    "plan_fragment",
    "extraction_run",
    "definition",
    "package",
    "signature",
    "implementation",
    "export_path",
    "crate_unit",
    "feature_declaration",
    "dependency_edge",
    "native_binding",
    "lattice",
];

const SNAPSHOT: &str = "snap:test";

/// One cell of a fixture row.
#[derive(Clone, Debug)]
enum Cell {
    S(&'static str),
    Owned(String),
    I8(i8),
    I32(i32),
    I64(i64),
    B(bool),
    Null,
}

type Row = Vec<(&'static str, Cell)>;

/// Build a batch for an authored schema, filling anything the row does not mention.
///
/// `entity_id` is derived from `entity_key` unless the row sets it explicitly. That is what makes
/// the good fixture satisfy the identity invariant without every test restating it, and it is also
/// what lets one test plant an underived id by simply naming the column.
///
/// `payload_digest` is likewise derived, by the same function the build uses. A fixture that
/// computed it its own way would agree with the build today and be the reason `compare` returns a
/// false negative later.
///
/// **The LAST entry for a column wins**, so a test takes the healthy fixture and appends the one
/// thing it is breaking. First-wins would silently ignore the override and leave the test passing
/// against unmodified data -- which is how four of these were briefly measuring nothing.
fn batch(schema: &SchemaRef, rows: &[Row]) -> RecordBatch {
    let mut arrays: Vec<ArrayRef> = Vec::new();
    for field in schema.fields() {
        let name = field.name().as_str();
        if name == identity::PAYLOAD_DIGEST {
            // A placeholder: the real value needs every other column, so it is filled below.
            arrays.push(Arc::new(StringArray::new_null(rows.len())) as ArrayRef);
            continue;
        }
        let cells: Vec<Cell> = rows
            .iter()
            .map(|row| {
                if let Some((_, cell)) = row.iter().rev().find(|(n, _)| *n == name) {
                    return cell.clone();
                }
                if name == "entity_id"
                    && let Some((_, Cell::S(key))) =
                        row.iter().rev().find(|(n, _)| *n == "entity_key")
                {
                    return Cell::Owned(identity::entity_id(key, SNAPSHOT));
                }
                default_for(name, field.data_type())
            })
            .collect();
        arrays.push(column(field.data_type(), &cells, rows.len()));
    }
    identity::batch_with_digest(schema, arrays, rows.len()).expect("fixture batch")
}

/// What an unmentioned column holds. Chosen so a fixture row is valid by default and a test has to
/// say what it is breaking.
fn default_for(name: &str, data_type: &DataType) -> Cell {
    match name {
        "snapshot_id" => Cell::S(SNAPSHOT),
        "run_id" => Cell::S("run:test"),
        "coverage_status" => Cell::S("characterised"),
        "retracted" => Cell::B(false),
        "first_seen_version" => Cell::I64(0),
        // The healthy catalog was written by a run that finished. Any test wanting otherwise says
        // so, which is the same discipline every other column here follows.
        "completion_rank" => Cell::I8(lattice::rank_of("completion", "complete").unwrap_or(0)),
        "skill_root" => Cell::S("/nowhere"),
        "index_files" => Cell::I32(0),
        "index_rows" => Cell::I64(0),
        "started_at" => Cell::I64(0),
        _ => match data_type {
            DataType::Int8 => Cell::I8(0),
            DataType::Boolean => Cell::B(false),
            _ => Cell::Null,
        },
    }
}

fn column(data_type: &DataType, cells: &[Cell], len: usize) -> ArrayRef {
    match data_type {
        DataType::Utf8 => Arc::new(StringArray::from(
            cells
                .iter()
                .map(|c| match c {
                    Cell::S(s) => Some((*s).to_string()),
                    Cell::Owned(s) => Some(s.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        )),
        DataType::Int8 => Arc::new(Int8Array::from(
            cells
                .iter()
                .map(|c| match c {
                    Cell::I8(v) => Some(*v),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        )),
        DataType::Int32 => Arc::new(Int32Array::from(
            cells
                .iter()
                .map(|c| match c {
                    Cell::I32(v) => Some(*v),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        )),
        DataType::Int64 => Arc::new(Int64Array::from(
            cells
                .iter()
                .map(|c| match c {
                    Cell::I64(v) => Some(*v),
                    _ => None,
                })
                .collect::<Vec<_>>(),
        )),
        DataType::Boolean => Arc::new(BooleanArray::from(
            cells
                .iter()
                .map(|c| matches!(c, Cell::B(true)))
                .collect::<Vec<_>>(),
        )),
        DataType::List(child) => Arc::new(ListArray::new(
            Arc::clone(child),
            OffsetBuffer::new(vec![0i32; len + 1].into()),
            Arc::new(StringArray::from(Vec::<Option<String>>::new())),
            None,
        )),
        DataType::Timestamp(arrow_schema::TimeUnit::Microsecond, tz) => {
            let a = arrow::array::TimestampMicrosecondArray::from(
                cells
                    .iter()
                    .map(|c| match c {
                        Cell::I64(v) => Some(*v),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            );
            match tz {
                Some(z) => Arc::new(a.with_timezone(z.to_string())),
                None => Arc::new(a),
            }
        }
        // Built from the children's own defaults, recursively. A hand-written struct literal here
        // would stop matching the moment a field was added to the schema, which is the failure
        // this whole file is built to avoid.
        DataType::Struct(children) => Arc::new(arrow::array::StructArray::new(
            children.clone(),
            children
                .iter()
                .map(|f| {
                    let child: Vec<Cell> = cells
                        .iter()
                        .map(|_| default_for(f.name(), f.data_type()))
                        .collect();
                    column(f.data_type(), &child, len)
                })
                .collect(),
            None,
        )),
        other => panic!("fixture builder has no case for {other:?}"),
    }
}

/// A catalog that satisfies every invariant. Tests take this and break one thing.
#[derive(Clone)]
struct Catalog {
    capability: Vec<Row>,
    mechanism: Vec<Row>,
    parameter: Vec<Row>,
    surface_binding: Vec<Row>,
    surface_entry: Vec<Row>,
    behavior_assertion: Vec<Row>,
    result_contract: Vec<Row>,
    search_domain: Vec<Row>,
    negative_space: Vec<Row>,
    interaction: Vec<Row>,
    interaction_atom: Vec<Row>,
    plan_fragment: Vec<Row>,
    /// The `library-api` family's half. Small on purpose: these tests are about the SQL of the
    /// views, and one row per table is enough to prove a join resolves.
    definition: Vec<Row>,
    package: Vec<Row>,
    signature: Vec<Row>,
    implementation: Vec<Row>,
    export_path: Vec<Row>,
    crate_unit: Vec<Row>,
    feature_declaration: Vec<Row>,
    dependency_edge: Vec<Row>,
    native_binding: Vec<Row>,
    /// The runs that wrote the rows above. `run_id` defaults to `run:test` everywhere, so the
    /// healthy fixture has exactly one, complete.
    extraction_run: Vec<Row>,
    /// The caller's request, as `facet=value` pairs joined by `;`.
    bindings: String,
    start_unit: String,
    max_depth: i64,
    include_partial: bool,
    /// Which snapshot the request is about. Empty means "the newest with a visible run", which is
    /// what the healthy fixture uses -- it holds one snapshot, so resolving it exercises the
    /// default path rather than skipping it.
    snapshot: String,
    /// `compare`'s two endpoints. Empty in every fixture but the compare ones, where they name the
    /// two snapshots the rows were planted under.
    from_snapshot: String,
    to_snapshot: String,
}

impl Catalog {
    fn healthy() -> Self {
        Self {
            capability: vec![vec![
                ("entity_key", Cell::S("cap:find-a-call")),
                ("ability", Cell::S("find-a-call")),
                ("mechanism_count", Cell::I64(1)),
            ]],
            mechanism: vec![vec![
                ("entity_key", Cell::S("mech:rg/cli/--pcre2")),
                ("tool", Cell::S("rg")),
                ("surface", Cell::S("cli")),
                ("natural_key", Cell::S("--pcre2")),
                ("precision_rank", Cell::I8(2)),
                ("observed_rank", Cell::I8(2)),
            ]],
            parameter: vec![vec![
                ("entity_key", Cell::S("param:mech:rg/cli/--pcre2/PATTERN")),
                (
                    "mechanism_id",
                    Cell::Owned(identity::entity_id("mech:rg/cli/--pcre2", SNAPSHOT)),
                ),
                ("name", Cell::S("PATTERN")),
                ("required", Cell::B(true)),
            ]],
            surface_binding: vec![vec![
                (
                    "entity_key",
                    Cell::S("bind:find-a-call/mech:rg/cli/--pcre2"),
                ),
                ("subject_key", Cell::S("cap:find-a-call")),
                ("subject_kind", Cell::S("capability")),
                ("object_key", Cell::S("mech:rg/cli/--pcre2")),
                ("object_kind", Cell::S("mechanism")),
                ("role", Cell::S("implemented_by")),
            ]],
            // One entry per canonical record, because every record must be in the denominator --
            // that is the invariant, and a fixture that skipped three of the four kinds would
            // leave three of them untested.
            surface_entry: vec![
                vec![
                    ("entity_key", Cell::S("surf:rg/flags/--pcre2")),
                    ("tool", Cell::S("rg")),
                    ("surface", Cell::S("flags")),
                    ("natural_key", Cell::S("--pcre2")),
                    ("source_table", Cell::S("flags")),
                    ("catalog_key", Cell::S("mech:rg/cli/--pcre2")),
                    ("catalog_kind", Cell::S("mechanism")),
                ],
                vec![
                    ("entity_key", Cell::S("surf:rg/exit-codes/1")),
                    ("tool", Cell::S("rg")),
                    ("surface", Cell::S("exit-codes")),
                    ("natural_key", Cell::S("rg#1")),
                    ("source_table", Cell::S("exit-codes")),
                    ("catalog_key", Cell::S("contract:rg#1")),
                    ("catalog_kind", Cell::S("result_contract")),
                ],
                vec![
                    ("entity_key", Cell::S("surf:rg/file-types/rust")),
                    ("tool", Cell::S("rg")),
                    ("surface", Cell::S("file-types")),
                    ("natural_key", Cell::S("rust")),
                    ("source_table", Cell::S("file-types")),
                    ("catalog_key", Cell::S("domain:rg/file_type/rust")),
                    ("catalog_kind", Cell::S("search_domain")),
                ],
                vec![
                    ("entity_key", Cell::S("surf:rg/unreachable/in-place")),
                    ("tool", Cell::S("rg")),
                    ("surface", Cell::S("unreachable")),
                    ("natural_key", Cell::S("in-place file rewriting")),
                    ("source_table", Cell::S("unreachable")),
                    ("catalog_key", Cell::S("absent:rg/in-place file rewriting")),
                    ("catalog_kind", Cell::S("negative_space")),
                ],
            ],
            result_contract: vec![vec![
                ("entity_key", Cell::S("contract:rg#1")),
                ("tool", Cell::S("rg")),
                ("command", Cell::S("")),
                ("code", Cell::I64(1)),
                ("meaning", Cell::S("no match")),
            ]],
            search_domain: vec![vec![
                ("entity_key", Cell::S("domain:rg/file_type/rust")),
                ("tool", Cell::S("rg")),
                ("domain_kind", Cell::S("file_type")),
                ("name", Cell::S("rust")),
                ("truth_rank", Cell::I8(2)),
            ]],
            negative_space: vec![vec![
                ("entity_key", Cell::S("absent:rg/in-place file rewriting")),
                ("tool", Cell::S("rg")),
                ("capability_text", Cell::S("in-place file rewriting")),
                ("truth_rank", Cell::I8(0)),
                ("why", Cell::S("ripgrep never modifies a file, by design")),
            ]],
            interaction: vec![vec![
                ("entity_key", Cell::S("ix:engine/lookbehind")),
                ("kind", Cell::S("requires")),
                ("affected_key", Cell::S("mech:rg/cli/--pcre2")),
                ("effect", Cell::S("needs the PCRE2 engine")),
                ("ordering_requirement", Cell::S("none")),
            ]],
            interaction_atom: vec![vec![
                ("entity_key", Cell::S("ix:engine/lookbehind#0.0")),
                ("interaction_key", Cell::S("ix:engine/lookbehind")),
                ("term_ord", Cell::I32(0)),
                ("atom_ord", Cell::I32(0)),
                ("facet", Cell::S("engine")),
                ("op", Cell::S("eq")),
                ("value", Cell::S("pcre2")),
                ("negated", Cell::B(false)),
            ]],
            // Two fragments that compose, plus a self-loop so the bound has something to
            // bound. Fragment composition is cyclic in general and this fixture says so.
            plan_fragment: vec![
                vec![
                    ("entity_key", Cell::S("plan:narrow")),
                    ("name", Cell::S("narrow")),
                    ("accepted_input", Cell::S("file_set")),
                    ("produced_output", Cell::S("file_set")),
                    ("scope_assumption", Cell::S("none")),
                    ("coordinate_preservation", Cell::S("preserved")),
                    ("recall_rank", Cell::I8(1)),
                    ("ordering_requirement", Cell::S("before")),
                ],
                vec![
                    ("entity_key", Cell::S("plan:structural")),
                    ("name", Cell::S("structural")),
                    ("accepted_input", Cell::S("file_set")),
                    ("produced_output", Cell::S("node_set")),
                    ("scope_assumption", Cell::S("whole_file")),
                    ("coordinate_preservation", Cell::S("preserved")),
                    ("recall_rank", Cell::I8(2)),
                    ("ordering_requirement", Cell::S("after")),
                ],
                vec![
                    ("entity_key", Cell::S("plan:capture")),
                    ("name", Cell::S("capture")),
                    ("accepted_input", Cell::S("node_set")),
                    ("produced_output", Cell::S("capture_set")),
                    ("scope_assumption", Cell::S("complete_enclosing_syntax")),
                    ("coordinate_preservation", Cell::S("preserved")),
                    ("recall_rank", Cell::I8(2)),
                    ("ordering_requirement", Cell::S("after")),
                ],
                vec![
                    ("entity_key", Cell::S("plan:rewrite")),
                    ("name", Cell::S("rewrite")),
                    ("accepted_input", Cell::S("file_set")),
                    ("produced_output", Cell::S("node_set")),
                    ("scope_assumption", Cell::S("none")),
                    // A rewrite invalidates every position established before it.
                    ("coordinate_preservation", Cell::S("lost")),
                    ("recall_rank", Cell::I8(0)),
                    ("ordering_requirement", Cell::S("after")),
                ],
                vec![
                    ("entity_key", Cell::S("plan:lines")),
                    ("name", Cell::S("lines")),
                    ("accepted_input", Cell::S("file_set")),
                    ("produced_output", Cell::S("line_set")),
                    ("scope_assumption", Cell::S("none")),
                    ("coordinate_preservation", Cell::S("preserved")),
                    ("recall_rank", Cell::I8(2)),
                    ("ordering_requirement", Cell::S("none")),
                ],
            ],
            definition: vec![vec![
                (
                    "entity_key",
                    Cell::S("def:grep-pcre2#grep_pcre2::matcher::RegexMatcher"),
                ),
                (
                    "canonical_path",
                    Cell::S("grep_pcre2::matcher::RegexMatcher"),
                ),
                ("item_kind", Cell::S("struct")),
                ("crate_name", Cell::S("grep-pcre2")),
                ("owner_path", Cell::S("grep_pcre2::matcher")),
            ]],
            // Two packages, because the dependency closure needs an edge with both ends present.
            package: vec![
                vec![
                    ("entity_key", Cell::S("pkg:crates.io/regex@1.13.0")),
                    ("name", Cell::S("regex")),
                    ("version", Cell::S("1.13.0")),
                    ("source", Cell::S("crates.io")),
                    ("in_subject_pins", Cell::B(true)),
                ],
                vec![
                    ("entity_key", Cell::S("pkg:crates.io/memchr@2.8.3")),
                    ("name", Cell::S("memchr")),
                    ("version", Cell::S("2.8.3")),
                    ("source", Cell::S("crates.io")),
                    ("in_subject_pins", Cell::B(true)),
                ],
            ],
            signature: vec![vec![
                (
                    "entity_key",
                    Cell::S("def:grep_pcre2::matcher::RegexMatcher#fn:new@inherent/0"),
                ),
                ("owner_path", Cell::S("grep_pcre2::matcher::RegexMatcher")),
                ("method_name", Cell::S("new")),
                ("signature_text", Cell::S("fn new() -> RegexMatcher")),
            ]],
            implementation: vec![vec![
                (
                    "entity_key",
                    Cell::S("impl:grep-pcre2#core::fmt::Debug~grep_pcre2::matcher::RegexMatcher"),
                ),
                ("trait_path", Cell::S("core::fmt::Debug")),
                (
                    "implementor_path",
                    Cell::S("grep_pcre2::matcher::RegexMatcher"),
                ),
                ("crate_name", Cell::S("grep-pcre2")),
            ]],
            export_path: vec![vec![
                ("entity_key", Cell::S("export:grep_pcre2::RegexMatcher")),
                ("access_path", Cell::S("grep_pcre2::RegexMatcher")),
                (
                    "canonical_path",
                    Cell::S("grep_pcre2::matcher::RegexMatcher"),
                ),
            ]],
            crate_unit: vec![vec![
                (
                    "entity_key",
                    Cell::S("crate:pkg:crates.io/regex@1.13.0#regex/lib"),
                ),
                ("package_key", Cell::S("pkg:crates.io/regex@1.13.0")),
                (
                    "package_id",
                    Cell::Owned(identity::entity_id("pkg:crates.io/regex@1.13.0", SNAPSHOT)),
                ),
                ("target_name", Cell::S("regex")),
                ("target_kind", Cell::S("lib")),
                ("src_path", Cell::S("src/lib.rs")),
                ("edition", Cell::S("2021")),
            ]],
            feature_declaration: vec![vec![
                (
                    "entity_key",
                    Cell::S("pkg:crates.io/regex@1.13.0#feature:std"),
                ),
                ("package_key", Cell::S("pkg:crates.io/regex@1.13.0")),
                (
                    "package_id",
                    Cell::Owned(identity::entity_id("pkg:crates.io/regex@1.13.0", SNAPSHOT)),
                ),
                ("feature_name", Cell::S("std")),
            ]],
            // A RESOLVED edge: both endpoints are package keys, and `resolve_present` is true,
            // which is what lets §7.3's closure traverse. The fixture's other package is the
            // target, so the closure has exactly one edge to walk.
            dependency_edge: vec![vec![
                (
                    "entity_key",
                    Cell::S("pkg:crates.io/regex@1.13.0#dep:normal/pkg:crates.io/memchr@2.8.3@any"),
                ),
                ("package_key", Cell::S("pkg:crates.io/regex@1.13.0")),
                (
                    "package_id",
                    Cell::Owned(identity::entity_id("pkg:crates.io/regex@1.13.0", SNAPSHOT)),
                ),
                ("dep_package_key", Cell::S("pkg:crates.io/memchr@2.8.3")),
                ("dep_name", Cell::S("memchr")),
                ("dep_req", Cell::S("^2.8")),
                ("dep_kind", Cell::S("normal")),
                ("dep_target_cfg", Cell::Null),
                ("resolve_present", Cell::B(true)),
            ]],
            native_binding: vec![vec![
                ("entity_key", Cell::S("native:skill.index.reexport/a -> b")),
                ("native_namespace", Cell::S("skill.index.reexport")),
                ("native_handle", Cell::S("a -> b")),
                ("mapping_basis", Cell::S("skill.index.symbols")),
            ]],
            // One run, which finished. `run_id` defaults to `run:test` on every other fixture
            // row, so this is the run that wrote all of them.
            extraction_run: vec![vec![
                ("entity_key", Cell::S("run:test")),
                ("family", Cell::S("catalog")),
                ("extractor_identity", Cell::S("codesearch-build@test")),
                ("context_id", Cell::S("ctx:test")),
            ]],
            bindings: String::new(),
            start_unit: "file_set".to_string(),
            max_depth: 4,
            include_partial: false,
            snapshot: String::new(),
            from_snapshot: String::new(),
            to_snapshot: String::new(),
            behavior_assertion: vec![vec![
                ("entity_key", Cell::S("assert:rg/A001")),
                (
                    "subject_entity_id",
                    Cell::Owned(identity::entity_id("topic:rg/search-space", SNAPSHOT)),
                ),
                ("subject_kind", Cell::S("topic")),
                ("predicate", Cell::S("does it?")),
                ("status", Cell::S("confirmed")),
            ]],
        }
    }

    async fn session(&self) -> SessionContext {
        let (ctx, _state) = session::build_session().expect("session builds");
        let tables: Vec<(&str, SchemaRef, &Vec<Row>)> = vec![
            (
                "capability",
                codesearch_model::schema::catalog_capability(),
                &self.capability,
            ),
            (
                "mechanism",
                codesearch_model::schema::catalog_mechanism(),
                &self.mechanism,
            ),
            (
                "parameter",
                codesearch_model::schema::catalog_parameter(),
                &self.parameter,
            ),
            (
                "surface_binding",
                codesearch_model::schema::catalog_surface_binding(),
                &self.surface_binding,
            ),
            (
                "surface_entry",
                codesearch_model::schema::catalog_surface_entry(),
                &self.surface_entry,
            ),
            (
                "behavior_assertion",
                codesearch_model::schema::evidence_behavior_assertion(),
                &self.behavior_assertion,
            ),
            (
                "result_contract",
                codesearch_model::schema::catalog_result_contract(),
                &self.result_contract,
            ),
            (
                "search_domain",
                codesearch_model::schema::catalog_search_domain(),
                &self.search_domain,
            ),
            (
                "negative_space",
                codesearch_model::schema::catalog_negative_space(),
                &self.negative_space,
            ),
            (
                "interaction",
                codesearch_model::schema::catalog_interaction(),
                &self.interaction,
            ),
            (
                "interaction_atom",
                codesearch_model::schema::catalog_interaction_atom(),
                &self.interaction_atom,
            ),
            (
                "plan_fragment",
                codesearch_model::schema::catalog_plan_fragment(),
                &self.plan_fragment,
            ),
            (
                "extraction_run",
                codesearch_model::schema::snapshot_extraction_run(),
                &self.extraction_run,
            ),
            (
                "definition",
                codesearch_model::schema::program_definition(),
                &self.definition,
            ),
            (
                "package",
                codesearch_model::schema::program_package(),
                &self.package,
            ),
            (
                "signature",
                codesearch_model::schema::program_signature(),
                &self.signature,
            ),
            (
                "implementation",
                codesearch_model::schema::program_implementation(),
                &self.implementation,
            ),
            (
                "export_path",
                codesearch_model::schema::program_export_path(),
                &self.export_path,
            ),
            (
                "crate_unit",
                codesearch_model::schema::program_crate_unit(),
                &self.crate_unit,
            ),
            (
                "feature_declaration",
                codesearch_model::schema::program_feature_declaration(),
                &self.feature_declaration,
            ),
            (
                "dependency_edge",
                codesearch_model::schema::program_dependency_edge(),
                &self.dependency_edge,
            ),
            (
                "native_binding",
                codesearch_model::schema::snapshot_native_binding(),
                &self.native_binding,
            ),
        ];
        // `install` REPLACES the default schema, so it goes first and everything else is
        // registered into the lazy one it puts there. The resolver is `Injected` because these
        // fixtures supply their tables directly -- which is the point: the views under test are
        // the same ones the query binary resolves from Delta, not a parallel registration path.
        let base: Vec<&str> = TABLE_NAMES.to_vec();
        projection::install(&ctx, &base, Arc::new(projection_lazy::Injected))
            .expect("the retrieval surface installs");
        for (name, schema, rows) in tables {
            let data = batch(&schema, rows);
            let table = MemTable::try_new(schema, vec![vec![data]]).expect("memtable");
            ctx.register_table(
                format!("{name}{}", projection::RAW_SUFFIX).as_str(),
                Arc::new(table),
            )
            .expect("register");
        }
        // The request is a one-row relation, exactly as §7.1 evaluates against.
        let request_schema = Arc::new(arrow_schema::Schema::new(vec![
            arrow_schema::Field::new("bindings", DataType::Utf8, false),
            arrow_schema::Field::new("start_unit", DataType::Utf8, false),
            arrow_schema::Field::new("max_depth", DataType::Int64, false),
            arrow_schema::Field::new("include_partial", DataType::Boolean, false),
            arrow_schema::Field::new("snapshot", DataType::Utf8, false),
            arrow_schema::Field::new("from_snapshot", DataType::Utf8, false),
            arrow_schema::Field::new("to_snapshot", DataType::Utf8, false),
        ]));
        let request = RecordBatch::try_new(
            Arc::clone(&request_schema),
            vec![
                Arc::new(StringArray::from(vec![self.bindings.clone()])),
                Arc::new(StringArray::from(vec![self.start_unit.clone()])),
                Arc::new(Int64Array::from(vec![self.max_depth])),
                Arc::new(BooleanArray::from(vec![self.include_partial])),
                Arc::new(StringArray::from(vec![self.snapshot.clone()])),
                Arc::new(StringArray::from(vec![self.from_snapshot.clone()])),
                Arc::new(StringArray::from(vec![self.to_snapshot.clone()])),
            ],
        )
        .expect("request batch");
        ctx.register_table(
            "request",
            Arc::new(MemTable::try_new(request_schema, vec![vec![request]]).expect("memtable")),
        )
        .expect("register");

        let reference = lattice::reference_batch().expect("lattice rows");
        let table = MemTable::try_new(reference.schema(), vec![vec![reference]]).expect("memtable");
        ctx.register_table(
            format!("lattice{}", projection::RAW_SUFFIX).as_str(),
            Arc::new(table),
        )
        .expect("register");
        ctx
    }
}

/// `validate` as a map from invariant name to (checked, violations).
async fn validate(catalog: &Catalog) -> BTreeMap<String, (i64, i64)> {
    let ctx = catalog.session().await;
    let batches = ctx
        .sql("SELECT invariant, checked, violations FROM projection.validate")
        .await
        .expect("validate plans")
        .collect()
        .await
        .expect("validate runs");

    let mut out = BTreeMap::new();
    for b in &batches {
        let names = b
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("invariant is a string");
        let checked = b
            .column(1)
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("checked is a count");
        let violations = b
            .column(2)
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("violations is a count");
        for i in 0..b.num_rows() {
            out.insert(
                names.value(i).to_string(),
                (
                    if checked.is_null(i) {
                        -1
                    } else {
                        checked.value(i)
                    },
                    violations.value(i),
                ),
            );
        }
    }
    out
}

/// The control. Without it, every test below would pass just as happily against a view that
/// returned rows unconditionally.
#[tokio::test]
async fn a_healthy_catalog_violates_nothing() {
    let report = validate(&Catalog::healthy()).await;
    assert_eq!(
        report.len(),
        9,
        "every invariant must be reported: {report:?}"
    );
    for (name, (checked, violations)) in &report {
        assert_eq!(*violations, 0, "{name} fired on a healthy catalog");
        assert!(
            *checked >= 0,
            "{name} appears in violations but is not declared as an invariant"
        );
    }
}

/// Every invariant that has rows to look at must actually look at some. An invariant reporting
/// `checked = 0` proved nothing, and the report says so rather than showing a clean bill of health.
#[tokio::test]
async fn an_invariant_that_inspected_nothing_reports_zero_checked() {
    let report = validate(&Catalog::healthy()).await;
    let (checked, violations) = report["behavior_assertion.subject resolves"];
    assert_eq!(
        (checked, violations),
        (0, 0),
        "the fixture's only assertion has a topic subject, so this invariant examines nothing -- \
         and that must be visible, not indistinguishable from passing"
    );
    let (checked, _) = report["entity_id is unique within its table"];
    assert!(checked > 0, "the identity invariants do have rows to check");
}

#[tokio::test]
async fn a_surface_entry_naming_no_mechanism_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.surface_entry[0].push(("catalog_key", Cell::S("mech:rg/cli/--does-not-exist")));
    let report = validate(&catalog).await;
    assert_eq!(report["surface_entry.catalog_key resolves"].1, 1);
}

#[tokio::test]
async fn a_binding_naming_a_key_nothing_holds_is_caught_on_either_side() {
    let mut catalog = Catalog::healthy();
    catalog.surface_binding[0].push(("subject_key", Cell::S("cap:invented")));
    catalog.surface_binding[0].push(("object_key", Cell::S("mech:invented")));
    let report = validate(&catalog).await;
    assert_eq!(report["surface_binding.subject_key resolves"].1, 1);
    assert_eq!(report["surface_binding.object_key resolves"].1, 1);
}

/// A binding into `program.*` is checked exactly as one into `catalog.*` is.
///
/// This is the test that makes §3.8's claim -- that `surface_binding` is how the two halves meet --
/// true of the code rather than only of the design. Before the `library-api` family the object side
/// could only be a mechanism, so "the two halves meet here" was a sentence with nothing behind it.
#[tokio::test]
async fn a_binding_into_the_program_model_resolves_like_any_other() {
    let mut catalog = Catalog::healthy();
    catalog.surface_binding.push(vec![
        (
            "entity_key",
            Cell::S("bind:mech:rg/cli/--pcre2/pkg:crates.io/regex@1.13.0"),
        ),
        ("subject_key", Cell::S("mech:rg/cli/--pcre2")),
        ("subject_kind", Cell::S("mechanism")),
        ("object_key", Cell::S("pkg:crates.io/regex@1.13.0")),
        ("object_kind", Cell::S("package")),
        ("role", Cell::S("implemented_by")),
    ]);
    let report = validate(&catalog).await;
    assert_eq!(
        report["surface_binding.object_key resolves"].1, 0,
        "a package the program model holds must resolve"
    );

    // The control: the same row pointing at a package nothing holds must NOT resolve, or the
    // assertion above would pass for a view that checks nothing.
    let mut broken = catalog;
    broken.surface_binding[1].push(("object_key", Cell::S("pkg:invented@9.9.9")));
    let report = validate(&broken).await;
    assert_eq!(report["surface_binding.object_key resolves"].1, 1);
}

/// The kind is checked, not just the key.
///
/// A definition key looked up in the package domain must fail even though the key itself is real,
/// because `subject_kind`/`object_kind` are facts the row asserts rather than conventions the key
/// prefix happens to keep.
#[tokio::test]
async fn a_binding_whose_kind_disagrees_with_its_key_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.surface_binding.push(vec![
        ("entity_key", Cell::S("bind:miskinded")),
        ("subject_key", Cell::S("mech:rg/cli/--pcre2")),
        ("subject_kind", Cell::S("mechanism")),
        // A real definition key, declared as a package.
        (
            "object_key",
            Cell::S("def:grep-pcre2#grep_pcre2::matcher::RegexMatcher"),
        ),
        ("object_kind", Cell::S("package")),
        ("role", Cell::S("implemented_by")),
    ]);
    let report = validate(&catalog).await;
    assert_eq!(report["surface_binding.object_key resolves"].1, 1);

    // The control: the same key with the honest kind resolves.
    let mut fixed = Catalog::healthy();
    fixed.surface_binding.push(vec![
        ("entity_key", Cell::S("bind:correctly-kinded")),
        ("subject_key", Cell::S("mech:rg/cli/--pcre2")),
        ("subject_kind", Cell::S("mechanism")),
        (
            "object_key",
            Cell::S("def:grep-pcre2#grep_pcre2::matcher::RegexMatcher"),
        ),
        ("object_kind", Cell::S("definition")),
        ("role", Cell::S("implemented_by")),
    ]);
    let report = validate(&fixed).await;
    assert_eq!(report["surface_binding.object_key resolves"].1, 0);
}

#[tokio::test]
async fn a_mechanism_in_no_surface_entry_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.mechanism.push(vec![
        ("entity_key", Cell::S("mech:rg/cli/--orphan")),
        ("tool", Cell::S("rg")),
        ("surface", Cell::S("cli")),
        ("natural_key", Cell::S("--orphan")),
        ("precision_rank", Cell::I8(2)),
        ("observed_rank", Cell::I8(2)),
    ]);
    let report = validate(&catalog).await;
    assert_eq!(
        report["every catalog record appears in surface_entry"].1, 1,
        "a mechanism outside the denominator is invisible to every coverage number"
    );
}

#[tokio::test]
async fn a_stale_denormalised_count_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.capability[0].push(("mechanism_count", Cell::I64(7)));
    let report = validate(&catalog).await;
    assert_eq!(
        report["capability.mechanism_count agrees with its bindings"].1,
        1
    );
}

#[tokio::test]
async fn an_assertion_claiming_a_mechanism_it_cannot_name_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.behavior_assertion[0].push(("subject_kind", Cell::S("mechanism")));
    let report = validate(&catalog).await;
    let (checked, violations) = report["behavior_assertion.subject resolves"];
    assert_eq!(
        (checked, violations),
        (1, 1),
        "changing the claimed kind is what brings the row into scope AND breaks it"
    );
}

#[tokio::test]
async fn a_duplicate_primary_key_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.mechanism.push(catalog.mechanism[0].clone());
    let report = validate(&catalog).await;
    assert_eq!(
        report["entity_id is unique within its table"].1, 1,
        "a duplicate makes the declared primary key false, and the optimiser acts on that \
         declaration"
    );
}

#[tokio::test]
async fn an_id_that_is_not_the_digest_of_its_key_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.mechanism[0].push(("entity_id", Cell::S("0123456789abcdef0123456789abcdef")));
    let report = validate(&catalog).await;
    assert_eq!(
        report["entity_id is the digest of its key and snapshot"].1, 1,
        "the derivation is a definition, so it is checkable rather than merely documented"
    );
}

/// A surface entry whose `catalog_kind` disagrees with the table its key is in.
///
/// The kind is denormalised so a reader need not search four tables for a record. Denormalised
/// data that disagrees with what it describes is worse than none, so the disagreement is checked.
#[tokio::test]
async fn a_surface_entry_labelled_with_the_wrong_kind_is_caught() {
    let mut catalog = Catalog::healthy();
    catalog.surface_entry.push(catalog.surface_entry[0].clone());
    let last = catalog.surface_entry.len() - 1;
    catalog.surface_entry[last].push(("entity_key", Cell::S("surf:rg/flags/--mislabelled")));
    catalog.surface_entry[last].push(("catalog_kind", Cell::S("search_domain")));
    let report = validate(&catalog).await;
    assert_eq!(
        report["surface_entry.catalog_kind names the right table"].1, 1,
        "the key is a mechanism, so calling it a search_domain must be caught"
    );
}

/// The whole interaction model, in the three states that make it worth having.
///
/// An unbound facet must answer `unknown` -- not `false`. That single distinction is the reason
/// for the `truth` lattice, for `atom_truth` returning a rank rather than a boolean, and for the
/// two `GROUP BY`s being `min` then `max`. A fold that collapsed `unknown` would report "this
/// interaction does not apply" to a caller who simply had not said which engine they were using.
#[tokio::test]
async fn an_interaction_is_unknown_until_its_facet_is_bound() {
    for (bindings, expected) in [
        ("", "unknown"),
        ("engine=pcre2", "true"),
        ("engine=default", "false"),
    ] {
        let mut catalog = Catalog::healthy();
        catalog.bindings = bindings.to_string();
        let ctx = catalog.session().await;
        let batches = ctx
            .sql("SELECT holds FROM projection.interaction_truth")
            .await
            .expect("the fold plans")
            .collect()
            .await
            .expect("the fold runs");
        let holds = batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("a label");
        assert_eq!(
            holds.value(0),
            expected,
            "with bindings {bindings:?} the interaction should hold `{expected}`"
        );
    }
}

/// A conjunct is `min`, so one unknown atom makes the whole term unknown even when its sibling is
/// satisfied. This is the half of Kleene logic a boolean encoding silently gets wrong.
#[tokio::test]
async fn one_unknown_atom_makes_the_conjunct_unknown() {
    let mut catalog = Catalog::healthy();
    catalog.interaction_atom.push(vec![
        ("entity_key", Cell::S("ix:engine/lookbehind#0.1")),
        ("interaction_key", Cell::S("ix:engine/lookbehind")),
        ("term_ord", Cell::I32(0)),
        ("atom_ord", Cell::I32(1)),
        ("facet", Cell::S("corpus")),
        ("op", Cell::S("eq")),
        ("value", Cell::S("utf8")),
        ("negated", Cell::B(false)),
    ]);
    // The first atom is satisfied; the second names a facet the request says nothing about.
    catalog.bindings = "engine=pcre2".to_string();
    let ctx = catalog.session().await;
    let batches = ctx
        .sql("SELECT holds FROM projection.interaction_truth")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let holds = batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("a label");
    assert_eq!(
        holds.value(0),
        "unknown",
        "true AND unknown is unknown, not true"
    );
}

/// And a disjunct is `max`, so a satisfied alternative outranks an unknown one.
#[tokio::test]
async fn a_satisfied_alternative_outranks_an_unknown_one() {
    let mut catalog = Catalog::healthy();
    catalog.interaction_atom.push(vec![
        ("entity_key", Cell::S("ix:engine/lookbehind#1.0")),
        ("interaction_key", Cell::S("ix:engine/lookbehind")),
        ("term_ord", Cell::I32(1)),
        ("atom_ord", Cell::I32(0)),
        ("facet", Cell::S("corpus")),
        ("op", Cell::S("absent")),
        ("negated", Cell::B(false)),
    ]);
    // The engine is unbound, so term 0 is unknown; term 1 asks whether `corpus` is unbound, which
    // is a definite yes. `max` takes the better of the two.
    let ctx = catalog.session().await;
    let batches = ctx
        .sql("SELECT holds FROM projection.interaction_truth")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let holds = batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("a label");
    assert_eq!(holds.value(0), "true", "unknown OR true is true");
}

/// The bound terminates a cyclic fragment graph, and it is the bound that does it.
///
/// PB09 measured the alternative: with `depth < :max` in an outer `WHERE`, this shape timed out at
/// 20 s where the bounded form returns in milliseconds. `plan:narrow` accepts and produces
/// `file_set`, so it composes with itself indefinitely -- the cycle is real, not contrived, and
/// the same self-loop exists in the shipped fragments (`rg -r` is `line_set` to `line_set`).
#[tokio::test]
async fn a_cyclic_fragment_graph_terminates_under_the_bound() {
    let mut counts = Vec::new();
    for depth in [1i64, 2, 3] {
        let mut catalog = Catalog::healthy();
        catalog.max_depth = depth;
        let ctx = catalog.session().await;
        let batches = ctx
            .sql("SELECT count(*) FROM projection.chain")
            .await
            .expect("the recursion plans")
            .collect()
            .await
            .expect("the recursion terminates");
        let n = batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("a count")
            .value(0);
        counts.push(n);
    }
    assert!(
        counts[0] < counts[1] && counts[1] < counts[2],
        "a deeper bound must admit more chains, or the bound is not what is limiting: {counts:?}"
    );
}

/// A chain is only as good as its weakest step, and `min` is what makes that automatic.
#[tokio::test]
async fn one_heuristic_step_makes_the_whole_chain_heuristic() {
    let ctx = Catalog::healthy().session().await;
    let batches = ctx
        .sql(
            "SELECT recall FROM projection.chain \
             WHERE path = 'narrow -> structural'",
        )
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let recall = batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("a label");
    assert_eq!(
        recall.value(0),
        "heuristic",
        "`structural` is preserving on its own, but it follows a heuristic narrowing step"
    );

    // The control: the preserving step reached directly is still preserving, so the value above
    // came from the fold rather than from the fragment being mislabelled.
    let direct = ctx
        .sql("SELECT recall FROM projection.chain WHERE path = 'structural'")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    assert_eq!(
        direct[0]
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("a label")
            .value(0),
        "preserving"
    );
}

/// A `line_set` producer does not compose with a stage needing whole files.
///
/// This is the proposal's "do not feed only matching lines into a structural stage" made
/// structural: the edge simply is not in the relation, so no chain can contain it.
#[tokio::test]
async fn lines_do_not_feed_a_stage_that_needs_whole_files() {
    let ctx = Catalog::healthy().session().await;
    let batches = ctx
        .sql(
            "SELECT count(*) FROM projection.fragment_edge \
             WHERE from_key = 'plan:lines' AND to_key = 'plan:structural'",
        )
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    assert_eq!(
        batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("a count")
            .value(0),
        0,
        "a line_set cannot satisfy a whole_file scope assumption"
    );

    // The control: the edge that should exist does, so the zero above is the guard and not an
    // empty relation.
    let allowed = ctx
        .sql(
            "SELECT count(*) FROM projection.fragment_edge \
             WHERE from_key = 'plan:narrow' AND to_key = 'plan:structural'",
        )
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    assert_eq!(
        allowed[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("a count")
            .value(0),
        1
    );
}

/// Losing coordinates breaks the join even when the unit types agree.
///
/// `plan:rewrite` and `plan:structural` both produce a `node_set`, so a rule that looked only at
/// unit types would let either feed the capture stage. Only one may: a capture needs the syntax
/// around a match, and a rewrite has invalidated every position established before it.
#[tokio::test]
async fn a_stage_that_loses_coordinates_does_not_feed_one_that_needs_them() {
    let ctx = Catalog::healthy().session().await;
    let count = |edge: &str| {
        let ctx = &ctx;
        let sql = format!(
            "SELECT count(*) FROM projection.fragment_edge \
             WHERE from_key = '{edge}' AND to_key = 'plan:capture'"
        );
        async move {
            let b = ctx
                .sql(&sql)
                .await
                .expect("plans")
                .collect()
                .await
                .expect("runs");
            b[0].column(0)
                .as_any()
                .downcast_ref::<Int64Array>()
                .expect("a count")
                .value(0)
        }
    };
    assert_eq!(
        count("plan:rewrite").await,
        0,
        "a rewrite has lost the coordinates a capture needs"
    );
    assert_eq!(
        count("plan:structural").await,
        1,
        "the same unit type with preserved coordinates does compose -- so the zero above is the \
         coordinate guard and not a missing row"
    );
}

/// Every canonical table is in the identity relation.
///
/// This exists because the gap it closes was real and silent. `plan_fragment`, `interaction` and
/// `interaction_atom` shipped outside every identity invariant: `validate` reported nine clean
/// invariants over 1,769 rows while the catalog held 1,842, and nothing failed. A table added to
/// the model and forgotten in `projection.identity` is a table whose ids nobody checks, so the
/// membership is asserted rather than remembered.
#[tokio::test]
async fn the_identity_relation_covers_every_canonical_table() {
    let ctx = Catalog::healthy().session().await;
    let batches = ctx
        .sql("SELECT DISTINCT source_table FROM projection.identity")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let listed: std::collections::BTreeSet<String> = batches
        .iter()
        .flat_map(|b| {
            let c = b
                .column(0)
                .as_any()
                .downcast_ref::<StringArray>()
                .expect("a name");
            (0..b.num_rows())
                .map(|i| c.value(i).to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    for (name, _) in codesearch_model::schema::all_tables() {
        // `catalog.lattice` is a reference table with no identity columns, and
        // `snapshot.source_anchor` belongs to a family this slice does not build.
        if name == "catalog.lattice" || name == "snapshot.source_anchor" {
            continue;
        }
        assert!(
            listed.contains(name),
            "`{name}` is a canonical table but is not in projection.identity, so nothing checks \
             that its ids are unique or derived"
        );
    }
    assert!(
        listed.len() >= 8,
        "only {} tables were listed",
        listed.len()
    );
}

/// The retrieval surface is enumerable, which is the reason for registering views at all.
#[tokio::test]
async fn every_projection_is_listed_in_information_schema() {
    let ctx = Catalog::healthy().session().await;
    let batches = ctx
        .sql("SELECT table_name FROM information_schema.views WHERE table_schema = 'projection'")
        .await
        .expect("information_schema is enabled")
        .collect()
        .await
        .expect("runs");
    let listed: Vec<String> = batches
        .iter()
        .flat_map(|b| {
            let c = b
                .column(0)
                .as_any()
                .downcast_ref::<StringArray>()
                .expect("string");
            (0..b.num_rows())
                .map(|i| c.value(i).to_string())
                .collect::<Vec<_>>()
        })
        .collect();
    for p in projection::PROJECTIONS {
        assert!(
            listed.contains(&p.name.to_string()),
            "{} is registered but information_schema does not list it",
            p.name
        );
    }
    assert_eq!(listed.len(), projection::PROJECTIONS.len());
}

/// Coverage answers two questions and never blends them. Asserted on the shape of the output
/// rather than on prose, so a future "overall score" column fails here.
#[tokio::test]
async fn coverage_reports_two_independent_relations() {
    let ctx = Catalog::healthy().session().await;
    for (view, expected) in [
        (
            "projection.coverage_surface",
            vec![
                "tool",
                "source_table",
                "surface_entries",
                "characterised",
                "uncharacterised",
                "characterised_as",
            ],
        ),
        (
            "projection.coverage_evidence",
            vec![
                "tool",
                "surface",
                "mechanisms",
                "confirmed",
                "weakest_rank",
                "weakest",
            ],
        ),
    ] {
        let frame = ctx
            .sql(&format!("SELECT * FROM {view}"))
            .await
            .unwrap_or_else(|e| panic!("{view} plans: {e}"));
        let columns: Vec<String> = frame
            .schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect();
        assert_eq!(columns, expected, "{view}");
    }
}

// ---------------------------------------------------------------------------------------------
// Run visibility
// ---------------------------------------------------------------------------------------------

/// Rows of one view, counted.
async fn count(ctx: &SessionContext, view: &str) -> i64 {
    let batches = ctx
        .sql(&format!("SELECT count(*) AS n FROM {view}"))
        .await
        .unwrap_or_else(|e| panic!("{view} plans: {e}"))
        .collect()
        .await
        .unwrap_or_else(|e| panic!("{view} runs: {e}"));
    batches
        .first()
        .map(|b| {
            b.column(0)
                .as_any()
                .downcast_ref::<Int64Array>()
                .expect("count is Int64")
                .value(0)
        })
        .unwrap_or(0)
}

/// A catalog whose only run is in `completion`.
fn catalog_at(completion: &str) -> Catalog {
    let mut c = Catalog::healthy();
    c.extraction_run = vec![vec![
        ("entity_key", Cell::S("run:test")),
        ("family", Cell::S("catalog")),
        ("extractor_identity", Cell::S("codesearch-build@test")),
        ("context_id", Cell::S("ctx:test")),
        (
            "completion_rank",
            Cell::I8(lattice::rank_of("completion", completion).expect("a completion value")),
        ),
    ]];
    c
}

/// The rule Phase 4 exists for: a run that did not finish serves nothing.
///
/// Three arms, because the one that matters is only meaningful against the other two. If
/// `discover` were empty for some unrelated reason, arm A alone would "pass" while measuring
/// nothing at all.
#[tokio::test]
async fn rows_written_by_a_run_that_did_not_complete_are_not_served() {
    // A CONTROL — the healthy catalog, whose run completed.
    let complete = count(&Catalog::healthy().session().await, "projection.discover").await;
    assert!(
        complete > 0,
        "the control must find rows, or the treatment proves nothing"
    );

    // B — the same rows, written by a run still marked `running`.
    for stalled in ["running", "partial", "abandoned"] {
        let ctx = catalog_at(stalled).session().await;
        assert_eq!(
            count(&ctx, "projection.discover").await,
            0,
            "rows from a `{stalled}` run must not be served"
        );
        // The same fixture reaches the same rows through a different view, so the filter is not
        // an accident of how `discover` happens to join.
        assert_eq!(
            count(&ctx, "projection.mechanism").await,
            0,
            "`{stalled}` must be filtered at the base table, not in one view"
        );
    }

    // C CONTROL — `--include-partial` brings them back, so arm B measured the flag and not a
    // fixture that was empty for some other reason.
    let mut c = catalog_at("running");
    c.include_partial = true;
    let ctx = c.session().await;
    assert_eq!(
        count(&ctx, "projection.discover").await,
        complete,
        "--include-partial must serve exactly what completing would have served"
    );
}

/// An abandoned run stays visible *as abandoned*.
///
/// This is the other half of the rule. Hiding the run row too would make "where did my rows go?"
/// unanswerable from inside the catalog, which is the failure the run table exists to prevent.
#[tokio::test]
async fn a_run_that_did_not_complete_is_still_visible_as_a_run() {
    let ctx = catalog_at("abandoned").session().await;
    assert_eq!(count(&ctx, "projection.discover").await, 0);
    assert_eq!(
        count(&ctx, "projection.extraction_run").await,
        1,
        "the run that wrote the hidden rows must itself be findable"
    );

    let batches = ctx
        .sql("SELECT completion FROM projection.extraction_run")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let label = batches[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("string")
        .value(0)
        .to_string();
    assert_eq!(label, "abandoned", "and it must say why");
}

/// The digest rule and the schema agree on what marks an entity reference.
///
/// `codesearch-bridge` cannot import the model's constant -- the dependency runs the other way --
/// so it carries its own copy and this asserts they are the same string. If they ever diverge,
/// every `entity_id` column silently re-enters the digest and every row reports as `changed`,
/// which is the failure the fixture below caught once already.
#[test]
fn the_digest_and_the_schema_agree_on_the_entity_id_extension() {
    assert_eq!(identity::EXT_ENTITY_ID, schema::EXT_ENTITY_ID);
}

/// A one-row batch for any authored schema, every column filled.
///
/// Deliberately not routed through [`batch`]: that helper fills only what a fixture mentions and
/// defaults the rest, which is right for the invariant tests and wrong here -- this needs a
/// complete row for *every* table, including ones no fixture models.
fn any_value(kind: &DataType) -> ArrayRef {
    match kind {
        DataType::Utf8 => Arc::new(StringArray::from(vec!["x"])),
        DataType::Int8 => Arc::new(Int8Array::from(vec![0i8])),
        DataType::Int32 => Arc::new(Int32Array::from(vec![0i32])),
        DataType::Int64 => Arc::new(Int64Array::from(vec![0i64])),
        DataType::Boolean => Arc::new(BooleanArray::from(vec![false])),
        DataType::Timestamp(_, zone) => {
            let a = arrow::array::TimestampMicrosecondArray::from(vec![0i64]);
            match zone {
                Some(z) => Arc::new(a.with_timezone(z.to_string())),
                None => Arc::new(a),
            }
        }
        DataType::List(child) => Arc::new(ListArray::new(
            Arc::clone(child),
            OffsetBuffer::new(vec![0i32, 0].into()),
            Arc::new(StringArray::from(Vec::<String>::new())),
            None,
        )),
        DataType::Struct(children) => Arc::new(arrow::array::StructArray::new(
            children.clone(),
            children.iter().map(|c| any_value(c.data_type())).collect(),
            None,
        )),
        other => panic!("the digest tests do not construct {other:?}"),
    }
}

fn filled(s: &SchemaRef) -> RecordBatch {
    let columns: Vec<ArrayRef> = s
        .fields()
        .iter()
        .map(|f| any_value(f.data_type()))
        .collect();
    identity::batch_with_digest(s, columns, 1).expect("a complete row builds")
}

/// The digest of a one-row batch.
fn digest_of(b: &RecordBatch) -> String {
    b.column_by_name(identity::PAYLOAD_DIGEST)
        .expect("this table carries the digest")
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("the digest is a string")
        .value(0)
        .to_string()
}

/// Every column typed as an entity reference is outside the digest, by type rather than by name.
///
/// Stated over the real schemas because the names differ per table -- `mechanism_id`,
/// `subject_entity_id`, `definition_id` -- so a by-name rule would have to be extended by whoever
/// adds the next family, which is exactly the kind of obligation nobody discharges.
///
/// This is the test the phase-7 fixture asked for. The first `compare` run reported all 165
/// `catalog.parameter` rows as `changed` when three things had changed, because `mechanism_id` is
/// an `entity_id` and therefore differs between snapshots by construction.
#[test]
fn no_entity_reference_column_reaches_the_digest() {
    let mut checked = 0;
    for (name, s) in schema::all_tables() {
        if s.index_of(identity::PAYLOAD_DIGEST).is_err() {
            continue;
        }
        let base = filled(&s);
        for field in s.fields() {
            if field
                .metadata()
                .get("ARROW:extension:name")
                .map(String::as_str)
                != Some(schema::EXT_ENTITY_ID)
            {
                continue;
            }
            checked += 1;
            let index = s.index_of(field.name()).expect("its own schema holds it");
            let mut columns: Vec<ArrayRef> = base.columns().to_vec();
            // Change ONLY the entity reference, and require the digest not to move.
            columns[index] = Arc::new(StringArray::from(vec!["a-different-id"]));
            let moved = identity::batch_with_digest(&s, columns, 1).expect("rebuilds");
            assert_eq!(
                digest_of(&base),
                digest_of(&moved),
                "{name}.{} is an entity reference and must not move the digest",
                field.name()
            );
        }
    }
    assert!(
        checked > 0,
        "this test inspected nothing, which proves nothing"
    );
}

/// The CONTROL: an ordinary column does move the digest.
///
/// Without it the test above passes just as well against a digest that covers nothing at all.
#[test]
fn an_ordinary_column_does_move_the_digest() {
    let s = schema::catalog_mechanism();
    let base = filled(&s);
    let index = s.index_of("summary").expect("mechanisms have a summary");
    let mut columns: Vec<ArrayRef> = base.columns().to_vec();
    columns[index] = Arc::new(StringArray::from(vec!["something else"]));
    let moved = identity::batch_with_digest(&s, columns, 1).expect("rebuilds");
    assert_ne!(digest_of(&base), digest_of(&moved));
}

/// A catalog holding two snapshots, planted so all three of `compare`'s kinds are exercised.
///
/// | key | snapshot A | snapshot B | expected |
/// |---|---|---|---|
/// | `mech:rg/cli/--stays` | present, summary "one" | present, summary "one" | **nothing** |
/// | `mech:rg/cli/--goes` | present | absent | `removed` |
/// | `mech:rg/cli/--comes` | absent | present | `added` |
/// | `mech:rg/cli/--moves` | present, summary "before" | present, summary "after" | `changed` |
///
/// The first row is the control, and it is the one that matters most: a `compare` that reported
/// everything would satisfy the other three and be useless. It also pins the digest's real job --
/// the same content under two snapshot ids has two different `entity_id`s and must still compare
/// equal.
fn two_snapshots() -> Catalog {
    const A: &str = "snap:aaaa";
    const B: &str = "snap:bbbb";

    fn mechanism(snapshot: &str, key: &'static str, summary: &'static str) -> Row {
        vec![
            ("entity_key", Cell::S(key)),
            ("entity_id", Cell::Owned(identity::entity_id(key, snapshot))),
            ("snapshot_id", Cell::Owned(snapshot.to_string())),
            ("run_id", Cell::Owned(format!("run:{snapshot}"))),
            ("tool", Cell::S("rg")),
            ("surface", Cell::S("cli")),
            ("natural_key", Cell::S(key)),
            ("summary", Cell::S(summary)),
        ]
    }

    fn run(snapshot: &str, started: i64) -> Row {
        vec![
            ("entity_key", Cell::Owned(format!("run:{snapshot}"))),
            (
                "entity_id",
                Cell::Owned(identity::entity_id(&format!("run:{snapshot}"), snapshot)),
            ),
            ("snapshot_id", Cell::Owned(snapshot.to_string())),
            ("run_id", Cell::Owned(format!("run:{snapshot}"))),
            ("family", Cell::S("catalog")),
            ("extractor_identity", Cell::S("codesearch-build@test")),
            ("context_id", Cell::S("ctx:test")),
            ("started_at", Cell::I64(started)),
            (
                "completion_rank",
                Cell::I8(lattice::rank_of("completion", "complete").expect("complete")),
            ),
        ]
    }

    let mut c = Catalog::healthy();
    c.capability = Vec::new();
    c.surface_binding = Vec::new();
    c.surface_entry = Vec::new();
    c.mechanism = vec![
        mechanism(A, "mech:rg/cli/--stays", "one"),
        mechanism(A, "mech:rg/cli/--goes", "one"),
        mechanism(A, "mech:rg/cli/--moves", "before"),
        mechanism(B, "mech:rg/cli/--stays", "one"),
        mechanism(B, "mech:rg/cli/--comes", "one"),
        mechanism(B, "mech:rg/cli/--moves", "after"),
    ];
    c.extraction_run = vec![run(A, 1_000), run(B, 2_000)];
    c.from_snapshot = A.to_string();
    c.to_snapshot = B.to_string();
    c
}

/// `compare` reports exactly the three differences, and says nothing about the row that did not
/// change.
#[tokio::test]
async fn compare_reports_added_removed_and_changed_and_nothing_else() {
    let ctx = two_snapshots().session().await;
    let batches = ctx
        .sql("SELECT entity_key, kind FROM projection.compare ORDER BY entity_key")
        .await
        .expect("compare plans")
        .collect()
        .await
        .expect("compare runs");

    let mut found: Vec<(String, String)> = Vec::new();
    for b in &batches {
        let keys = b.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let kinds = b.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..b.num_rows() {
            found.push((keys.value(i).to_string(), kinds.value(i).to_string()));
        }
    }

    assert_eq!(
        found,
        vec![
            ("mech:rg/cli/--comes".to_string(), "added".to_string()),
            ("mech:rg/cli/--goes".to_string(), "removed".to_string()),
            ("mech:rg/cli/--moves".to_string(), "changed".to_string()),
        ],
        "`--stays` must not appear: the same content under two snapshot ids has two different \
         entity_ids and still has to compare equal"
    );
}

/// Comparing a snapshot with itself reports nothing at all.
///
/// The other half of the control. A `compare` that reported every row would pass the test above by
/// accident if the digest were, say, derived from `entity_id`.
#[tokio::test]
async fn comparing_a_snapshot_with_itself_reports_nothing() {
    let mut c = two_snapshots();
    c.to_snapshot = c.from_snapshot.clone();
    let ctx = c.session().await;
    assert_eq!(count(&ctx, "projection.compare").await, 0);
}

/// An ordinary query answers about one snapshot, not the union of both.
///
/// Before phase 7 no view mentioned `snapshot_id`, so a second snapshot would have doubled every
/// count silently. This is the assertion that would have caught it.
#[tokio::test]
async fn an_ordinary_view_serves_one_snapshot_not_both() {
    let c = two_snapshots();
    let ctx = c.session().await;
    // Six mechanism rows exist across two snapshots; the newest run is B's, which has three.
    assert_eq!(count(&ctx, "mechanism__all").await, 6);
    assert_eq!(
        count(&ctx, "mechanism").await,
        3,
        "a base view must serve the selected snapshot alone"
    );

    let mut older = two_snapshots();
    older.snapshot = "snap:aaaa".to_string();
    let ctx = older.session().await;
    assert_eq!(
        count(&ctx, "mechanism").await,
        3,
        "--snapshot must pin the older one"
    );
    let rows = ctx
        .sql("SELECT entity_key FROM mechanism ORDER BY entity_key")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    let keys = rows[0]
        .column(0)
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap();
    assert_eq!(
        (0..rows[0].num_rows())
            .map(|i| keys.value(i))
            .collect::<Vec<_>>(),
        vec![
            "mech:rg/cli/--goes",
            "mech:rg/cli/--moves",
            "mech:rg/cli/--stays"
        ],
        "and it must be the older snapshot's rows, not the newer ones"
    );
}

/// The CONTROL for the dependency closure: with `resolve_present` false it must not traverse.
///
/// §7.3 puts this guard in the view, not the caller: *"§3.1's `resolve_present` must be `true`, or
/// the query returns no rows for a reason rather than an empty graph that reads as 'no
/// dependencies'."* Probe PL001 is why -- a null `resolve` from `cargo metadata --no-deps` is not
/// the same fact as a package having no dependencies, and one edge that ignored the flag would
/// turn "we did not ask" into "there is nothing there".
#[tokio::test]
async fn the_dependency_closure_refuses_to_traverse_an_unresolved_edge() {
    let mut c = Catalog::healthy();
    // The SAME edge, with the flag flipped. Nothing else changes, so a difference in the answer is
    // attributable to the flag and to nothing else.
    c.dependency_edge[0].push(("resolve_present", Cell::B(false)));
    let ctx = c.session().await;
    assert_eq!(
        count(&ctx, "projection.dependency_closure").await,
        0,
        "an edge whose resolve_present is false must not be walked"
    );

    // And the control on the control: unflipped, it does walk.
    let ctx = Catalog::healthy().session().await;
    assert_eq!(count(&ctx, "projection.dependency_closure").await, 1);
}

/// `lattice` and `extraction_run` are exempt from **both** base filters, and nothing else is.
///
/// Stated as a test rather than as a comment because the exemption list is the one place a new
/// table could be quietly added and stop being filtered. It covers two rules since phase 7 -- run
/// visibility and snapshot selection -- and both exemptions have to hold for the same two tables,
/// or `runs` stops being able to explain an empty answer in the case where the answer is "you are
/// looking at a different snapshot".
#[tokio::test]
async fn only_the_two_declared_tables_are_exempt_from_the_base_filters() {
    assert_eq!(projection::UNFILTERED, ["lattice", "extraction_run"]);

    // ---- the snapshot filter ----------------------------------------------------------------
    let mut elsewhere = two_snapshots();
    elsewhere.snapshot = "snap:nowhere".to_string();
    let ctx = elsewhere.session().await;
    assert!(
        count(&ctx, "lattice").await > 0,
        "a reference table belongs to no snapshot"
    );
    assert_eq!(
        count(&ctx, "extraction_run").await,
        2,
        "the run table must stay visible under a snapshot that holds nothing -- it is the only \
         thing that can say which snapshots exist"
    );
    assert_eq!(
        count(&ctx, "mechanism").await,
        0,
        "and everything else must be snapshot-filtered"
    );

    // ---- the run filter ---------------------------------------------------------------------
    let ctx = catalog_at("running").session().await;
    // A reference table has no run to belong to, so it is served whatever any run did.
    assert!(
        count(&ctx, "lattice").await > 0,
        "the lattice mapping must never depend on a run"
    );
    // And every other base table is empty under a stalled run.
    for t in ["mechanism", "capability", "surface_entry", "plan_fragment"] {
        assert_eq!(count(&ctx, t).await, 0, "{t} must be run-filtered");
        assert!(
            count(&ctx, &format!("{t}{}", projection::RAW_SUFFIX)).await > 0,
            "{t}{} must still hold the rows -- the filter is a view, not a deletion",
            projection::RAW_SUFFIX
        );
    }
}

/// A join equating an `entity_key` with an `entity_id` is refused at plan time.
///
/// The failure this guards is the worst shape a failure takes: `entity_id` is
/// `blake3(entity_key ‖ snapshot_id)`, so the two are equal for no entity in any snapshot, and the
/// join returns **zero rows without erroring**. A caller reads that as a fact about the data.
#[tokio::test]
async fn a_join_of_entity_key_to_entity_id_is_refused() {
    let ctx = Catalog::healthy().session().await;
    // `sql()` only builds the unoptimized plan; analyzer rules run when it is optimized, which is
    // what `collect()` does in the query binary. Asserting on `sql()` alone would have tested that
    // the parser accepts the query, which nobody doubted.
    let err = ctx
        .sql(
            "SELECT m.entity_key FROM mechanism m \
             JOIN surface_entry s ON s.catalog_key = m.entity_id",
        )
        .await
        .expect("it parses")
        .into_optimized_plan()
        .expect_err("a key-to-id join must not survive analysis");
    let message = err.to_string();
    assert!(
        message.contains("entity_id is") && message.contains("zero rows"),
        "the refusal must say WHY, not merely that it refused: {message}"
    );
}

/// The CONTROL: the same shape, joined key to key, still plans.
///
/// Without it the test above passes just as well against a rule that rejects every join, which is
/// the misfire `IdentityDiscipline` is deliberately narrow to avoid.
#[tokio::test]
async fn a_join_of_entity_key_to_entity_key_still_plans() {
    let ctx = Catalog::healthy().session().await;
    ctx.sql(
        "SELECT m.entity_key FROM mechanism m \
         JOIN surface_entry s ON s.catalog_key = m.entity_key",
    )
    .await
    .expect("it parses")
    .into_optimized_plan()
    .expect("key-to-key is the ordinary join and must be untouched");

    // And a join on columns carrying no identity extension type at all.
    ctx.sql("SELECT a.tool FROM mechanism a JOIN surface_entry b ON b.tool = a.tool")
        .await
        .expect("it parses")
        .into_optimized_plan()
        .expect("a join on ordinary columns must be untouched");
}

/// No projection folds a lattice column with an aggregate that is not `min` or `max`.
///
/// This is §5.3's `LatticePropagation` **as a test rather than as an analyzer rule**, and the
/// substitution is the point. The rule's trigger was "joins a parent to children without folding
/// each lattice", and parent-child is not decidable from a logical plan: a version conservative
/// enough not to block legitimate queries catches almost nothing. The population it was meant to
/// police is enumerable -- it is exactly `PROJECTIONS` -- so a red test gives the same guarantee
/// against the set that matters.
///
/// §5.2 fixes the algebra: composition is `min` (a chain is as good as its weakest link),
/// alternation is `max` (a choice is as good as its best option). `avg` over a rank is the one
/// that would look most reasonable and mean least, because the ranks are ordinal and their mean
/// is not a value of the lattice at all.
///
/// **It reads the planned tree, not the SQL text.** Two false positives are why: a recursive CTE's
/// column list (`WITH RECURSIVE chain(… recall_rank)`) scans as a call to `chain`, and
/// `sum(CASE WHEN observed_rank >= … THEN 1 ELSE 0 END)` scans as `sum` over a rank when it is a
/// count over a boolean. In the plan, an aggregate's argument is either the lattice column or it
/// is not, and the question stops being a guess.
#[tokio::test]
async fn no_projection_folds_a_lattice_with_the_wrong_aggregate() {
    const ALLOWED: &[&str] = &["min", "max"];

    let ctx = Catalog::healthy().session().await;
    let mut folds = 0;
    let mut planned = 0;

    for p in projection::PROJECTIONS {
        let plan = ctx
            .sql(&format!("SELECT * FROM projection.{} LIMIT 0", p.name))
            .await
            .unwrap_or_else(|e| panic!("projection `{}` plans: {e}", p.name))
            .into_unoptimized_plan();
        planned += 1;

        plan.apply(|node| {
            let LogicalPlan::Aggregate(agg) = node else {
                return Ok(TreeNodeRecursion::Continue);
            };
            let schema = agg.input.schema();
            for expr in &agg.aggr_expr {
                let Expr::AggregateFunction(call) = expr else {
                    continue;
                };
                // Only a fold applied DIRECTLY to a lattice column counts. An aggregate over a
                // `CASE` or an arithmetic expression is not folding the lattice, it is counting
                // something derived from it, and §5.2 says nothing about that.
                for arg in &call.params.args {
                    let Expr::Column(_) = arg else { continue };
                    let Ok((_, field)) = arg.to_field(schema) else {
                        continue;
                    };
                    if field
                        .metadata()
                        .get("ARROW:extension:name")
                        .map(String::as_str)
                        != Some(schema::EXT_LATTICE)
                    {
                        continue;
                    }
                    folds += 1;
                    let name = call.func.name().to_ascii_lowercase();
                    assert!(
                        ALLOWED.contains(&name.as_str()),
                        "projection `{}` applies `{name}` to the lattice column `{}`. §5.2 \
                         defines composition as `min` and alternation as `max`; a rank is \
                         ordinal, so any other aggregate over it is a semantics nothing states",
                        p.name,
                        field.name()
                    );
                }
            }
            Ok(TreeNodeRecursion::Continue)
        })
        .expect("walking a planned projection");
    }

    assert_eq!(
        planned,
        projection::PROJECTIONS.len(),
        "every projection was inspected"
    );
    assert!(
        folds > 0,
        "this test found no lattice fold at all, so it is checking nothing -- §5.2's algebra is \
         supposed to be exercised by the chain and interaction projections"
    );
}

/// The base views must not strip Arrow extension metadata.
///
/// Phase 8's `IdentityDiscipline` rule reads `ARROW:extension:name` off a plan's join keys to
/// reject `entity_key ⋈ entity_id`. Putting a view between the provider and every projection is
/// exactly the sort of change that could drop that metadata silently, so it is measured here
/// rather than assumed -- and the control is a column that never carried it.
#[tokio::test]
async fn the_run_filter_view_preserves_extension_metadata() {
    let ctx = Catalog::healthy().session().await;
    let frame = ctx.sql("SELECT * FROM mechanism").await.expect("plans");
    let schema = frame.schema();
    let field = schema
        .field_with_unqualified_name("entity_id")
        .expect("field");
    assert_eq!(
        field
            .metadata()
            .get("ARROW:extension:name")
            .map(String::as_str),
        Some(codesearch_model::schema::EXT_ENTITY_ID),
        "the run-filter view dropped the identity extension name"
    );
    let plain = schema
        .field_with_unqualified_name("snapshot_id")
        .expect("field");
    assert!(
        plain.metadata().get("ARROW:extension:name").is_none(),
        "the control column must NOT carry an extension name, or the assertion above is vacuous"
    );
}

/// Every projection still plans, and this is where that is now guaranteed.
///
/// `install` resolves a view only when a query names it, so the cost of planning all of them moved
/// off the per-invocation path. §7's argument for paying it eagerly -- that a projection which will
/// not plan is a broken retrieval surface -- is right, so the guarantee moved here rather than
/// being dropped. It runs once per gate instead of once per caller.
#[tokio::test]
async fn every_projection_plans() {
    let ctx = Catalog::healthy().session().await;
    let planned = projection::check_all(&ctx)
        .await
        .expect("every registered projection must plan");
    assert_eq!(
        planned,
        projection::PROJECTIONS.len(),
        "check_all must plan every projection, not a subset"
    );
}

/// A view resolved lazily still carries Arrow extension metadata.
///
/// Phase 8's `IdentityDiscipline` rule reads `ARROW:extension:name` off a plan's join keys to
/// reject `entity_key ⋈ entity_id`. That was measured through the eagerly-created views; putting a
/// `ViewTable` built on demand in the same place is exactly the sort of change that could drop it
/// silently, so it is measured again. The control is a column that never carried one.
#[tokio::test]
async fn a_lazily_resolved_view_preserves_extension_metadata() {
    let ctx = Catalog::healthy().session().await;
    let frame = ctx
        .sql("SELECT * FROM projection.mechanism")
        .await
        .expect("plans");
    let schema = frame.schema();
    let field = schema
        .field_with_unqualified_name("entity_id")
        .expect("field");
    assert_eq!(
        field
            .metadata()
            .get("ARROW:extension:name")
            .map(String::as_str),
        Some(codesearch_model::schema::EXT_ENTITY_ID),
        "a lazily-planned view dropped the identity extension name"
    );
    let plain = schema
        .field_with_unqualified_name("snapshot_id")
        .expect("field");
    assert!(
        plain.metadata().get("ARROW:extension:name").is_none(),
        "the control column must NOT carry an extension name, or the assertion above is vacuous"
    );
}

// ---------------------------------------------------------------------------------------------
// §7.3's closure queries
// ---------------------------------------------------------------------------------------------

/// A cyclic containment graph terminates under the bound.
///
/// The real index cannot exercise this -- it has **no** containment edges at all, because
/// `symbols.tsv` holds items and every item's owner is a module. So the property that matters is
/// tested where it can fail: a graph that is cyclic by construction. This is the PB09 shape, where
/// bounding `depth` inside the recursive term returns in milliseconds and bounding it outside did
/// not return at all.
#[tokio::test]
async fn a_cyclic_containment_graph_terminates_under_the_bound() {
    let mut catalog = Catalog::healthy();
    // A owns B and B owns A. Nothing in the schema forbids it, and a real extractor could produce
    // it from a re-export loop, so the query must survive it rather than assume it away.
    catalog.definition = vec![
        vec![
            ("entity_key", Cell::S("def:c#a")),
            ("canonical_path", Cell::S("a")),
            ("item_kind", Cell::S("struct")),
            ("crate_name", Cell::S("c")),
            ("owner_path", Cell::S("b")),
        ],
        vec![
            ("entity_key", Cell::S("def:c#b")),
            ("canonical_path", Cell::S("b")),
            ("item_kind", Cell::S("struct")),
            ("crate_name", Cell::S("c")),
            ("owner_path", Cell::S("a")),
        ],
    ];

    for (max_depth, expected) in [(2, 4), (4, 8), (8, 16)] {
        let mut c = catalog.clone();
        c.max_depth = max_depth;
        let ctx = c.session().await;
        let rows = count(&ctx, "projection.containment").await;
        assert_eq!(
            rows, expected,
            "at --max-depth {max_depth} a two-node cycle must yield exactly {expected} rows"
        );
    }
}

/// The path witness accumulates, and it is a real list rather than a string.
///
/// §7.3 requires a `List<Utf8>` -- *"a reachability answer without the path is exactly the opaque
/// convenience result the proposal's §12 forbids"*. A length that grows with depth is what proves
/// the accumulation works across the recursive term rather than being rebuilt each pass.
///
/// It also records something the `chain` view did not establish: `make_array` in the base term and
/// `array_append` in the recursive term **do** agree on type and field metadata, so the list shape
/// §7.3 asks for is achievable. `chain` concatenates strings, which was never shown to be forced.
#[tokio::test]
async fn the_path_witness_is_a_list_that_grows_with_depth() {
    let mut catalog = Catalog::healthy();
    catalog.definition = vec![
        vec![
            ("entity_key", Cell::S("def:c#a")),
            ("canonical_path", Cell::S("a")),
            ("item_kind", Cell::S("struct")),
            ("crate_name", Cell::S("c")),
            ("owner_path", Cell::S("b")),
        ],
        vec![
            ("entity_key", Cell::S("def:c#b")),
            ("canonical_path", Cell::S("b")),
            ("item_kind", Cell::S("struct")),
            ("crate_name", Cell::S("c")),
            ("owner_path", Cell::S("a")),
        ],
    ];
    catalog.max_depth = 3;
    let ctx = catalog.session().await;

    // Cast in SQL rather than guessing at the Rust side: `array_length` returns a width the engine
    // chooses, and the test is about the LENGTH, not about its storage type.
    let witness = |depth: i64| {
        let ctx = ctx.clone();
        async move {
            let batches = ctx
                .sql(&format!(
                    "SELECT CAST(max(array_length(path)) AS BIGINT) AS n \
                     FROM projection.containment WHERE depth = {depth}"
                ))
                .await
                .expect("plans")
                .collect()
                .await
                .expect("runs");
            batches[0]
                .column_by_name("n")
                .expect("n")
                .as_any()
                .downcast_ref::<Int64Array>()
                .expect("cast to BIGINT")
                .value(0)
        }
    };

    assert_eq!(
        witness(1).await,
        2,
        "a depth-1 witness holds the root and the member"
    );
    assert_eq!(
        witness(3).await,
        4,
        "and one more hop per level, so the accumulation crosses the recursive term"
    );
}

/// A closure that traversed nothing does not look like a closure that found nothing.
///
/// This is `validate`'s `checked` column applied to reachability, and it is the reason
/// `projection.closure` exists. An empty `containment` is the catalog's real state -- and an empty
/// result a caller reads as "this definition contains nothing" would be a confidently wrong answer
/// about 716 rows.
#[tokio::test]
async fn an_empty_closure_reports_its_edge_count_and_its_reason() {
    // A CONTROL -- the healthy fixture has one export_path row, so `reexport` has an edge.
    let ctx = Catalog::healthy().session().await;
    let batches = ctx
        .sql("SELECT closure, edges, limited_by FROM projection.closure ORDER BY closure")
        .await
        .expect("plans")
        .collect()
        .await
        .expect("runs");
    // BY NAME, not by position. The first version indexed `edges[1]` for `reexport` and broke the
    // moment a third closure sorted between them -- which is the kind of test that fails for a
    // reason unrelated to what it checks.
    let edges: std::collections::BTreeMap<String, i64> = {
        let names = batches[0]
            .column_by_name("closure")
            .expect("closure")
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("string");
        let a = batches[0]
            .column_by_name("edges")
            .expect("edges")
            .as_any()
            .downcast_ref::<Int64Array>()
            .expect("count is Int64");
        (0..batches[0].num_rows())
            .map(|i| (names.value(i).to_string(), a.value(i)))
            .collect()
    };
    assert_eq!(
        edges["containment"], 0,
        "the fixture has no containment edges"
    );
    assert_eq!(edges["reexport"], 1, "and exactly one re-export edge");
    // The dependency closure traverses, because the adapter resolves a pinned subject and the
    // fixture's edge carries `resolve_present = true`.
    assert_eq!(edges["dependency"], 1, "one resolved edge, one hop");

    // Every closure must carry a non-empty reason, or the zero above is a bare zero again.
    let reasons = batches[0]
        .column_by_name("limited_by")
        .expect("limited_by")
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("string");
    for i in 0..batches[0].num_rows() {
        assert!(
            reasons.value(i).len() > 40,
            "closure {i} must say what limits it, not merely report a count"
        );
    }
}
