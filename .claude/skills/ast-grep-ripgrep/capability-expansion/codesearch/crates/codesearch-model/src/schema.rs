//! Arrow schemas for the canonical tables, and the type discipline they all obey.
//!
//! # Every column type is a fixed point of the Delta conversion
//!
//! Probe PB03 measured the decisive fact: **Delta does not reject types, it silently narrows
//! them.** Ten of eleven types that looked "unsupported" are accepted and mapped to something
//! else, with no error. So the schema is authored in the *narrowed* vocabulary, and
//! [`crate::ddl`] asserts on a round trip that what comes back equals what went in.
//!
//! | never author            | author instead | what the write path would otherwise do                |
//! |------------------------|----------------|-------------------------------------------------------|
//! | `UInt32`, `UInt64`     | `Int64`        | narrow -- **`UInt32` truncates above `i32::MAX`**      |
//! | `Dictionary(_, Utf8)`  | `Utf8`         | flatten; authoring a dictionary buys nothing           |
//! | `Utf8View`,`LargeUtf8` | `Utf8`         | narrow to `Utf8`                                       |
//! | `LargeList<T>`         | `List<T>`      | narrow to `List<T>`                                    |
//! | `FixedSizeBinary(n)`   | `Binary`       | narrow to `Binary`                                     |
//! | `Float16`              | `Float32`      | the one genuine rejection                              |
//!
//! `normalize_for_delta` is **not** a preview of persistence -- PB03 measured it returning all
//! twenty inputs unchanged, including every type the write path then narrows. The only reliable
//! pre-flight check is an actual round trip.
//!
//! Nested child fields are also **renamed** to Parquet conventions on write (`item` to `element`,
//! `entries/keys/values` to `key_value/key/value`), so those spellings are authored directly and
//! an authored schema compares equal to a read-back one without a normalisation step.

use std::sync::Arc;

use arrow_schema::{DataType, Field, Fields, Schema, SchemaRef, TimeUnit};
use codesearch_bridge::identity::PAYLOAD_DIGEST;
use codesearch_bridge::lattice;

/// Extension name stamped on identity columns. PB01 measured that Arrow extension metadata
/// survives a Delta write/read cycle byte-for-byte, so identity is a storage property and needs
/// no re-attachment step.
pub const EXT_ENTITY_ID: &str = "codesearch.entity_id";
pub const EXT_ENTITY_KEY: &str = "codesearch.entity_key";
pub const EXT_LATTICE: &str = "codesearch.lattice";
pub const EXT_BYTE_OFFSET: &str = "codesearch.byte_offset";

const ARROW_EXT_KEY: &str = "ARROW:extension:name";

fn with_ext(field: Field, ext: &str) -> Field {
    // One `with_metadata` call, not a loop: PB01's first run lost exactly one key per field
    // because `StructField::with_metadata` REPLACES rather than extends, so a per-entry loop
    // keeps only whichever entry came last.
    field.with_metadata([(ARROW_EXT_KEY.to_string(), ext.to_string())].into())
}

/// A canonical entity id: the per-snapshot join key, and the column a primary key is declared on.
pub fn entity_id_field(name: &str, nullable: bool) -> Field {
    with_ext(Field::new(name, DataType::Utf8, nullable), EXT_ENTITY_ID)
}

/// A stable, snapshot-independent key. `compare` joins on this and on nothing else.
pub fn entity_key_field(name: &str, nullable: bool) -> Field {
    with_ext(Field::new(name, DataType::Utf8, nullable), EXT_ENTITY_KEY)
}

/// A lattice column, stored as its `Int8` ordinal.
///
/// The name is always `<lattice>_rank`, so the stored type is visible at the call site and a
/// column holding a label instead of a rank is a naming error rather than a silent semantic one.
pub fn lattice_field(lattice_name: &str, nullable: bool) -> Field {
    with_ext(
        Field::new(format!("{lattice_name}_rank"), DataType::Int8, nullable),
        EXT_LATTICE,
    )
}

/// A byte offset. **Always `Int64`.** `UInt32` narrows to `Int32` on write and silently truncates
/// above `i32::MAX`, which corrupts offsets in files over 2 GiB (PB03).
pub fn byte_offset_field(name: &str, nullable: bool) -> Field {
    with_ext(Field::new(name, DataType::Int64, nullable), EXT_BYTE_OFFSET)
}

/// A `List<Utf8>` authored with the Parquet child spelling `element`, which is what Delta renames
/// `item` to on write.
pub fn string_list_field(name: &str, nullable: bool) -> Field {
    Field::new(
        name,
        DataType::List(Arc::new(Field::new("element", DataType::Utf8, true))),
        nullable,
    )
}

/// A UTC instant, stored as microseconds.
///
/// `Timestamp(Microsecond, Some("UTC"))` is Delta's own `timestamp`. Nanoseconds are **not** --
/// they narrow to microseconds on write, which is the PB03 failure mode applied to time, and a
/// timestamp that quietly loses three digits is worse than one that was never recorded. The
/// time zone is carried rather than left `None`, because `None` is `timestamp_ntz`, a different
/// Delta type meaning "no instant, just a reading on a clock somewhere".
pub fn instant_field(name: &str, nullable: bool) -> Field {
    Field::new(
        name,
        DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into())),
        nullable,
    )
}

/// The provenance and epistemics columns every canonical row carries.
///
/// Uniformity here is what lets one analyzer rule check every table and one fold serve every
/// dimension. `ord` is not decoration: a child sequence without it has silently asserted that its
/// order does not matter.
///
/// `payload_digest` is the newest and the only derived one. It exists because `compare` joins on
/// `entity_key` and on nothing else, which settles `added` and `removed` but leaves `changed`
/// undecidable: `entity_id` differs between snapshots for an unchanged entity by construction, so
/// it cannot serve. A digest over the row's own content can, uniformly, for every table at once --
/// the alternative was a comparison view per table, where a table added and forgotten would be
/// compared by nothing.
fn common_columns() -> Vec<Field> {
    vec![
        entity_id_field("entity_id", false),
        entity_key_field("entity_key", false),
        Field::new("snapshot_id", DataType::Utf8, false),
        Field::new("run_id", DataType::Utf8, false),
        Field::new("evidence_id", DataType::Utf8, true),
        lattice_field("precision", false),
        lattice_field("observed", false),
        Field::new("coverage_status", DataType::Utf8, false),
        Field::new("ord", DataType::Int32, true),
        Field::new("first_seen_version", DataType::Int64, false),
        Field::new("last_seen_version", DataType::Int64, true),
        Field::new("retracted", DataType::Boolean, false),
        Field::new(PAYLOAD_DIGEST, DataType::Utf8, false),
    ]
}

fn table(extra: Vec<Field>) -> SchemaRef {
    let mut fields = common_columns();
    fields.extend(extra);
    Arc::new(Schema::new(fields))
}

/// `catalog.lattice` -- the rank/label mapping, published as data so stored ordinals are
/// interpretable by anything that can query the catalog. This is the one table with no common
/// columns: it is a reference table, not a fact table.
pub fn catalog_lattice() -> SchemaRef {
    lattice::reference_schema()
}

/// `catalog.mechanism` -- a concrete way to exercise a capability.
pub fn catalog_mechanism() -> SchemaRef {
    table(vec![
        Field::new("tool", DataType::Utf8, false),
        Field::new("surface", DataType::Utf8, false),
        Field::new("natural_key", DataType::Utf8, false),
        Field::new("invocation_form", DataType::Utf8, true),
        Field::new("native_schema_ref", DataType::Utf8, true),
        Field::new("summary", DataType::Utf8, true),
        string_list_field("capability_ids", true),
    ])
}

/// `catalog.capability` -- the abstract querying ability, the axis above `Mechanism`.
///
/// Facets are stored as separate columns rather than a blob because the proposal's model is
/// explicitly *composable orthogonal fields*, not a hierarchy of prewritten use cases: an agent
/// filters on the facets that matter to its question and ignores the rest.
pub fn catalog_capability() -> SchemaRef {
    table(vec![
        Field::new("ability", DataType::Utf8, false),
        Field::new("topic", DataType::Utf8, true),
        Field::new("summary", DataType::Utf8, true),
        Field::new("facet_subject", DataType::Utf8, true),
        Field::new("facet_relationship", DataType::Utf8, true),
        Field::new("facet_result", DataType::Utf8, true),
        Field::new("facet_semantic_layer", DataType::Utf8, true),
        Field::new("mechanism_count", DataType::Int64, false),
    ])
}

/// `catalog.surface_binding` -- how a capability claims a mechanism.
///
/// A separate table rather than a list column on either side, because it is a many-to-many
/// relation and because the *reason* for a binding is itself a fact worth querying.
pub fn catalog_surface_binding() -> SchemaRef {
    table(vec![
        entity_key_field("subject_key", false),
        Field::new("subject_kind", DataType::Utf8, false),
        entity_key_field("object_key", false),
        Field::new("object_kind", DataType::Utf8, false),
        Field::new("role", DataType::Utf8, false),
        // Why the claim is believed, for an authored binding. Deliberately NOT `evidence_id`,
        // which is a foreign key into `evidence.behavior_assertion`: no probe establishes that a
        // flag is implemented by a crate, so pretending there is one to point at would be worse
        // than a sentence a reader can check.
        Field::new("evidence_note", DataType::Utf8, true),
    ])
}

/// `catalog.surface_entry` -- the upstream inventory item. This is the coverage DENOMINATOR, and
/// it is deliberately not derived from `catalog.mechanism`: two independent sides are what make
/// the `LEFT JOIN` between them undriftable.
///
/// # Why the numerator is not a mechanism
///
/// It was `mechanism_key` while a mechanism was the only canonical record a family could produce.
/// It is not: an exit code becomes a `result_contract`, a file type or language becomes a
/// `search_domain`, and an unreachable capability becomes a `negative_space` row. Keeping the
/// column named for one of the four would have left three quarters of the inventory permanently
/// `uncharacterised` -- not because nothing had been extracted from it, but because coverage was
/// asking the wrong question.
///
/// So the numerator is `catalog_key` plus the `catalog_kind` naming which table to find it in.
/// `projection.catalog_record` is the union coverage joins against.
pub fn catalog_surface_entry() -> SchemaRef {
    table(vec![
        Field::new("tool", DataType::Utf8, false),
        Field::new("surface", DataType::Utf8, false),
        Field::new("natural_key", DataType::Utf8, false),
        Field::new("source_table", DataType::Utf8, false),
        entity_key_field("catalog_key", true),
        Field::new("catalog_kind", DataType::Utf8, true),
    ])
}

/// `catalog.result_contract` -- what a command's exit status means, and the trap in reading it.
///
/// `trap` is a column rather than a sentence inside `meaning` because it is the fact that stops a
/// cross-tool mistake: ast-grep exits `8` for an unparseable pattern where a reader who knows
/// ripgrep would expect `2`. That is exactly the transfer this catalog exists to prevent, so it is
/// queryable rather than buried in prose.
pub fn catalog_result_contract() -> SchemaRef {
    table(vec![
        Field::new("tool", DataType::Utf8, false),
        Field::new("command", DataType::Utf8, false),
        Field::new("code", DataType::Int64, false),
        Field::new("meaning", DataType::Utf8, false),
        Field::new("trap", DataType::Utf8, true),
    ])
}

/// `catalog.search_domain` -- what a tool can be pointed at, by name.
///
/// Two domain kinds share one table because they answer one question from two sides: ripgrep's
/// `--type` names a set of files, ast-grep's language names a grammar it can parse. Both are "what
/// can this tool address".
///
/// `truth_rank` is the `truth` lattice and carries real measurement: the skill ran each language
/// against the binary, so `accepted` and `rejected` are verdicts rather than documentation. That
/// is the first data any lattice other than `precision` and `observed` has carried.
///
/// The grammar itself is deliberately not loaded (§3.8 -- `kinds.tsv` and `fields.tsv` churn on
/// every ast-grep grammar update). Its *size* is what the index records, so the counts are named
/// for what they are and are null for file types.
pub fn catalog_search_domain() -> SchemaRef {
    table(vec![
        Field::new("tool", DataType::Utf8, false),
        Field::new("domain_kind", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("selector", DataType::Utf8, true),
        lattice_field("truth", false),
        Field::new("grammar_kind_count", DataType::Int64, true),
        Field::new("grammar_field_count", DataType::Int64, true),
        Field::new("rule_schema_available", DataType::Boolean, true),
    ])
}

/// `catalog.negative_space` -- a capability that exists in the library and is not reachable.
///
/// # Why this is its own table
///
/// Nothing else in the model can say "known absent, for this reason, do this instead". A
/// `capability` row would assert the ability is one we model; a `mechanism` row would assert a way
/// to exercise it exists. Both are false, and both would be the confident wrong answer.
///
/// This is where `discover` gets a negative result worth returning. An agent asking for in-place
/// rewriting with ripgrep should be told that ripgrep never modifies a file **by design**, and
/// pointed at `ast-grep -U` -- not handed an empty result set it will read as "nothing found, try
/// harder".
///
/// `truth_rank` holds the proposition **"this capability is reachable through the surface"**:
/// `no` is `false`, `partial` is `unknown`. Those are different facts and the index keeps them
/// apart, so this does too. The column is named for its lattice rather than for its proposition
/// because that naming rule is what makes a column holding a label instead of a rank a visible
/// error -- the proposition belongs in this comment, where it cannot be mistaken for a value.
pub fn catalog_negative_space() -> SchemaRef {
    table(vec![
        Field::new("tool", DataType::Utf8, false),
        Field::new("capability_text", DataType::Utf8, false),
        lattice_field("truth", false),
        Field::new("why", DataType::Utf8, false),
        Field::new("instead_text", DataType::Utf8, true),
        entity_key_field("instead_key", true),
    ])
}

/// `catalog.parameter` -- three default columns, not one.
///
/// The declared library default, the application override and the effective behaviour on the
/// selected surface are three different facts. The worked example is `grep-pcre2`, whose builder
/// documents UTF and Unicode-property modes as *disabled* by default while ripgrep *enables* both
/// unless Unicode is disabled. An agent asking "what is the default" needs the third column;
/// collapsing them makes the catalog confidently wrong on the question it exists to answer.
pub fn catalog_parameter() -> SchemaRef {
    table(vec![
        entity_id_field("mechanism_id", false),
        Field::new("name", DataType::Utf8, false),
        Field::new("value_domain", DataType::Utf8, true),
        Field::new("declared_default", DataType::Utf8, true),
        Field::new("application_override", DataType::Utf8, true),
        Field::new("effective_default", DataType::Utf8, true),
        Field::new("required", DataType::Boolean, false),
    ])
}

/// `catalog.interaction` -- how one option changes what another does.
///
/// # Why a cross-reference by key rather than by id
///
/// `affected_key` is an `entity_key`, not an `entity_id`, and the rule behind that is worth
/// stating because both conventions are in use here. **Anything authored by a person references a
/// key; anything derived within a snapshot references an id.** A seed file names
/// `mech:rg/cli/--pcre2` because that is what a human can write and check; `catalog.parameter`
/// points at `mechanism_id` because the build minted both ends in the same pass. Interactions are
/// authored, so they use keys, and they survive a snapshot change for free.
pub fn catalog_interaction() -> SchemaRef {
    table(vec![
        Field::new("kind", DataType::Utf8, false),
        entity_key_field("affected_key", false),
        Field::new("effect", DataType::Utf8, false),
        Field::new("ordering_requirement", DataType::Utf8, false),
        string_list_field("assertion_keys", true),
    ])
}

/// `catalog.interaction_atom` -- one literal of a predicate in disjunctive normal form.
///
/// # Why DNF as rows rather than an expression tree
///
/// Arrow has no recursive type, and an expression interpreter is exactly the procedural code this
/// design exists to avoid. So a predicate is rows: **AND within a `term_ord`, OR across distinct
/// `term_ord`s**, and evaluation is two `GROUP BY`s -- `min` then `max` -- over the `truth`
/// lattice. Kleene `unknown` is not implemented anywhere; it is a consequence of the encoding.
///
/// If some interaction ever genuinely needs nesting, the answer is a new `interaction` row that
/// references another, not an interpreter. The atom table is append-only by design.
pub fn catalog_interaction_atom() -> SchemaRef {
    table(vec![
        entity_key_field("interaction_key", false),
        Field::new("term_ord", DataType::Int32, false),
        Field::new("atom_ord", DataType::Int32, false),
        Field::new("facet", DataType::Utf8, false),
        Field::new("op", DataType::Utf8, false),
        Field::new("value", DataType::Utf8, true),
        string_list_field("value_list", true),
        Field::new("negated", DataType::Boolean, false),
    ])
}

/// `catalog.plan_fragment` -- one composable step, with what it accepts and what it leaves behind.
///
/// # Composition is a join, not a planner
///
/// Fragment A may precede B exactly when `A.produced_output = B.accepted_input` and B's scope
/// assumption is satisfied by A's coordinate preservation. That is a join condition, so the
/// proposal's warning -- *"do not feed only matching lines into a structural stage that requires
/// complete enclosing syntax"* -- is enforced by the row not existing. A fragment producing
/// `line_set` simply does not join to one accepting `node_set`. The rule cannot be forgotten
/// because there is nowhere to forget it.
///
/// # `recall_rank` is evidence, not shape
///
/// `preserving` asserts `final_match(x) implies prefilter_selects(x)`, and that is a claim about
/// behaviour which no amount of staring at a fragment's input and output types can establish. So
/// it is carried by a `BehaviorAssertion` or it is not carried: a fragment claiming `preserving`
/// with no assertion behind it is a **build error**. When the implication is merely plausible the
/// honest value is `heuristic`, and the condition that would make it preserving goes in
/// `unresolved_obligations` where a reader can see what is still owed.
pub fn catalog_plan_fragment() -> SchemaRef {
    table(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("accepted_input", DataType::Utf8, false),
        Field::new("produced_output", DataType::Utf8, false),
        Field::new("scope_assumption", DataType::Utf8, false),
        Field::new("coordinate_preservation", DataType::Utf8, false),
        lattice_field("recall", false),
        Field::new("ordering_requirement", DataType::Utf8, false),
        string_list_field("mechanism_keys", true),
        string_list_field("assertion_keys", true),
        string_list_field("unresolved_obligations", true),
    ])
}

/// `evidence.behavior_assertion` -- carries supporting AND contradicting evidence.
///
/// A contradicted assertion is not deleted; it is one whose status the lattice will not let rise.
/// That is how "silence is not absence" is represented for behaviour rather than merely stated.
pub fn evidence_behavior_assertion() -> SchemaRef {
    table(vec![
        entity_id_field("subject_entity_id", false),
        Field::new("subject_kind", DataType::Utf8, false),
        Field::new("predicate", DataType::Utf8, false),
        Field::new("value_or_target", DataType::Utf8, true),
        Field::new("interpretation", DataType::Utf8, true),
        Field::new("status", DataType::Utf8, false),
        Field::new("probe_id", DataType::Utf8, true),
        Field::new("command", DataType::Utf8, true),
        Field::new("expected", DataType::Utf8, true),
        Field::new("control_command", DataType::Utf8, true),
        Field::new("control_exit", DataType::Int64, true),
        string_list_field("supporting_evidence", true),
        string_list_field("contradicting_evidence", true),
    ])
}

/// `snapshot.source_anchor` -- the table that exercises the byte-offset rule.
///
/// **Declared and empty, and that is a statement rather than an oversight.** No family fills it:
/// the shipped indexes carry no byte offsets, which is why `impl:` keys lost the anchor §1.2
/// specifies and why every `program.implementation` row is `inexact`. §3.3's `syntax` family is
/// what fills it — `ra_ap_syntax` parses a string with no database and no network, so it is the
/// cheapest of the unbuilt adapters and the one that retires that degradation.
///
/// It is declared ahead of its producer, alone among the tables here, because its CHECK constraint
/// (`start_byte <= end_byte`) and its `Int64` offsets are what `model/tests/fixed_point.rs`
/// exercises against real Delta storage. The rule elsewhere is the opposite — a table is declared
/// when a family writes it — and this exception is written down so it does not read as precedent.
pub fn snapshot_source_anchor() -> SchemaRef {
    table(vec![
        entity_key_field("file_key", false),
        byte_offset_field("start_byte", false),
        byte_offset_field("end_byte", false),
        Field::new("origin", DataType::Utf8, false),
        entity_key_field("expansion_parent", true),
    ])
}

/// `snapshot.extraction_run` -- one row per extraction pass, and the table every projection's
/// visibility is decided by.
///
/// # The run row is written before the rows it accounts for
///
/// A build writes this row with `completion` at `running` *first*, then the family's tables, then
/// rewrites it at `complete`. The order is the whole design: a process killed in the middle leaves
/// rows in the canonical tables **and** a row here saying the run that wrote them never finished,
/// so they are attributable rather than orphaned. Writing the run row last would make a crash
/// indistinguishable from a build that never started, while the half-written rows stayed visible.
///
/// A later build reaps any `running` row belonging to an earlier run to `abandoned`, because a run
/// that is not this process and is not finished is not going to finish.
///
/// # `scope` is a struct, not a JSON string
///
/// The interpretive context has to be *filterable*. `WHERE scope.index_rows > 0` is a predicate
/// the planner can push down and a reader can understand; `WHERE scope LIKE '%index_rows%'` is
/// neither. A JSON string would also make the schema a lie -- it would say `Utf8` where the
/// content is structured, and nothing would catch a field renamed inside it.
pub fn snapshot_extraction_run() -> SchemaRef {
    table(vec![
        Field::new("family", DataType::Utf8, false),
        Field::new("extractor_identity", DataType::Utf8, false),
        Field::new("context_id", DataType::Utf8, false),
        Field::new(
            "scope",
            DataType::Struct(Fields::from(vec![
                Field::new("skill_root", DataType::Utf8, false),
                Field::new("index_files", DataType::Int32, false),
                Field::new("index_rows", DataType::Int64, false),
            ])),
            false,
        ),
        lattice_field("completion", false),
        instant_field("started_at", false),
        // Null until the run stops. A finished_at on a `running` row would be a contradiction the
        // schema allowed, so the nullability carries the same fact as the lattice value.
        instant_field("finished_at", true),
        string_list_field("input_artifact_ids", true),
        // Which pass was in flight when a run stopped short. Null on a run that completed; set on
        // a `partial` one, because "partial" without saying partial in what is not a report.
        Field::new("failed_pass", DataType::Utf8, true),
    ])
}

// ================================================================================================
// `program` -- the typed program records (§4.2)
//
// Every table below is filled by the `library-api` family from the skill's own rustdoc-derived
// index. Several columns are present and left NULL by that family on purpose: they belong to the
// canonical model, the index cannot know them, and `catalog::Columns` refuses to let this family
// fill them (§3 part 4). A column absent from the schema would say the model has no such concept;
// a column present and null says this family did not observe it.
// ================================================================================================

/// `program.package` -- one crate at one version.
///
/// `source` is the §3.1 key segment this family cannot supply. `PROVENANCE.json` `pins.crates` is
/// a bare `name -> version` map, so the key degrades from `pkg:<source|path>/<name>@<version>` to
/// `pkg:<name>@<version>`. Writing `crates-io` in there would assert a fact nothing measured --
/// several of these crates are workspace members of the tools themselves.
pub fn program_package() -> SchemaRef {
    table(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("version", DataType::Utf8, false),
        // Filled now. It was never-filled while `library-api` minted these from `pins.crates`,
        // which records no source; the `cargo-metadata` family resolves a pinned subject and the
        // registry comes back with it.
        Field::new("source", DataType::Utf8, false),
        Field::new("repository", DataType::Utf8, true),
        Field::new("edition", DataType::Utf8, true),
        Field::new("rust_version", DataType::Utf8, true),
        Field::new("license", DataType::Utf8, true),
        // Whether the SKILL pinned this crate, as opposed to the resolve pulling it in: 23 of 155.
        // Coverage needs the distinction, because the skill's own set and the graph it implies are
        // two denominators and summing them would answer neither question.
        Field::new("in_subject_pins", DataType::Boolean, false),
    ])
}

/// `program.definition` -- a declared item: struct, enum, trait or function.
///
/// # `owner_id` is nullable because modules are not in the index
///
/// `symbols.tsv` holds items, not modules. So the owner of
/// `aho_corasick::ahocorasick::AhoCorasick` is `aho_corasick::ahocorasick`, which no row declares.
/// `owner_path` is therefore a derived *fact* -- the canonical path minus its last segment -- and
/// `owner_id` resolves only where that prefix is itself an indexed definition. §7.3's containment
/// closure is shallow for exactly this reason, and says so, rather than synthesising module rows
/// nobody extracted.
pub fn program_definition() -> SchemaRef {
    table(vec![
        Field::new("canonical_path", DataType::Utf8, false),
        Field::new("item_kind", DataType::Utf8, false),
        Field::new("crate_name", DataType::Utf8, false),
        Field::new("owner_path", DataType::Utf8, true),
        entity_id_field("owner_id", true),
        Field::new("api_page", DataType::Utf8, true),
        Field::new("summary", DataType::Utf8, true),
        Field::new("alias_count", DataType::Int64, true),
        Field::new("method_count", DataType::Int64, true),
        // NEVER FILLED by `library-api`. §3.5 makes `visibility_basis` an explicit column because
        // absence has three indistinguishable causes (not public, cfg-gated off, private module);
        // a TSV of public items records which of the three applies to nothing.
        Field::new("visibility", DataType::Utf8, true),
        Field::new("visibility_basis", DataType::Utf8, true),
        // NEVER FILLED: no byte offsets anywhere in the index.
        entity_id_field("source_anchor_id", true),
    ])
}

/// `program.signature` -- one method on one owner, by one route.
///
/// The key carries `via_trait` because one owner can have the same method name from several
/// traits: `clone` via `core::clone::Clone` and an inherent `clone` are two signatures, and a key
/// on `(owner, method)` alone would merge them.
pub fn program_signature() -> SchemaRef {
    table(vec![
        Field::new("owner_path", DataType::Utf8, false),
        entity_id_field("owner_id", true),
        Field::new("method_name", DataType::Utf8, false),
        // NULL for an inherent method. The index spells it `-`; storing that would make a sentinel
        // out of a value the column can hold honestly as null.
        Field::new("via_trait", DataType::Utf8, true),
        Field::new("signature_text", DataType::Utf8, false),
        Field::new("summary", DataType::Utf8, true),
    ])
}

/// `program.implementation` -- a trait implemented for a type.
///
/// `crate_name` is the **implementor's** crate, which is what the index carries: every
/// `core::fmt::Debug` row names the crate of the type implementing it, not `core`.
pub fn program_implementation() -> SchemaRef {
    table(vec![
        Field::new("trait_path", DataType::Utf8, false),
        Field::new("implementor_path", DataType::Utf8, false),
        Field::new("crate_name", DataType::Utf8, false),
        entity_id_field("implementor_id", true),
        // NEVER FILLED: §1.2 embeds a source anchor in an `impl:` key precisely so two impls for
        // similarly-printed types cannot collapse. `impls.tsv` has no offsets, so the key degrades
        // and the builder detects collisions instead.
        entity_id_field("anchor_id", true),
    ])
}

/// `program.export_path` -- where an item can be named from, as against where it is defined.
pub fn program_export_path() -> SchemaRef {
    table(vec![
        Field::new("access_path", DataType::Utf8, false),
        Field::new("canonical_path", DataType::Utf8, false),
        Field::new("item_kind", DataType::Utf8, true),
        entity_id_field("definition_id", true),
    ])
}

/// `program.crate_unit` -- a compilation target a package declares (§3.1).
///
/// # Why this is not called `program.target`
///
/// §3.1 lists `program.target` and `program.crate_unit` as two tables, and §1.2's grammar has one
/// key covering them: `crate:<pkg-key>#<target-name>/<target-kind>`. There is no `target:`
/// namespace. A first pass here did write a `program.target` with an invented `#target:` key --
/// a grammar violation nothing checked, because nothing checks key shapes. The table takes the
/// name its key has, and §3.1's two names for one thing is recorded in §12.1b.
///
/// `src_path` is **relative to the crate root**, because an absolute path is a fact about a
/// checkout rather than about the crate -- and the snapshot id digests these bytes, so an
/// absolute one would give identical inputs a different identity on every machine.
pub fn program_crate_unit() -> SchemaRef {
    table(vec![
        entity_key_field("package_key", false),
        entity_id_field("package_id", false),
        Field::new("target_name", DataType::Utf8, false),
        Field::new("target_kind", DataType::Utf8, false),
        Field::new("src_path", DataType::Utf8, false),
        Field::new("edition", DataType::Utf8, false),
    ])
}

/// `program.feature_declaration` -- a feature a package DECLARES, and what enabling it implies.
///
/// **Declared, never enabled.** Probe PL003 measured that `cargo metadata` reports what *could* be
/// enabled while rustdoc silently omits what was not (RD005). One boolean over both would assert
/// something neither family knows, so §3.1 has a `feature_declaration` table and there is no
/// `feature_enabled` column anywhere in this model.
///
/// `implies` is an empty list, never null, for a feature that implies nothing: "implies nothing"
/// and "nobody looked" are different facts and the adapter always knows which it is.
pub fn program_feature_declaration() -> SchemaRef {
    table(vec![
        entity_key_field("package_key", false),
        entity_id_field("package_id", false),
        Field::new("feature_name", DataType::Utf8, false),
        string_list_field("implies", true),
    ])
}

/// `program.dependency_edge` -- a resolved dependency between two packages (§3.1).
///
/// # `resolve_present` is a column, and it is what lets §7.3's closure traverse
///
/// `cargo metadata --no-deps` sets `resolve` to null, and **a null resolve is not "no
/// dependencies"** -- probe PL001. Without this column the transitive closure would return an
/// empty graph a caller reads as a fact about the crates rather than about the extraction. Every
/// row carries it, so the closure answers *no rows, for a reason* when it is false.
///
/// It is true here, because the adapter resolves a pinned subject manifest rather than reading
/// vendored checkouts with `--no-deps`. Both endpoints are package keys, so the closure walks keys
/// rather than matching names -- which it could not do before, and which is why this table's first
/// version had a permanently-null `dep_package_key`.
///
/// `dep_req` is the **declared requirement** (`^2.4.0`) beside the resolved edge. They are
/// different facts, and this design keeps declared and observed apart everywhere else.
pub fn program_dependency_edge() -> SchemaRef {
    table(vec![
        entity_key_field("package_key", false),
        entity_id_field("package_id", false),
        entity_key_field("dep_package_key", true),
        Field::new("dep_name", DataType::Utf8, false),
        Field::new("dep_req", DataType::Utf8, true),
        Field::new("dep_kind", DataType::Utf8, false),
        Field::new("dep_target_cfg", DataType::Utf8, true),
        Field::new("resolve_present", DataType::Boolean, false),
    ])
}

/// `snapshot.native_binding` -- the crosswalk/// `snapshot.native_binding` -- the crosswalk, and the only place native handles appear (§1.3).
///
/// `canonical_entity_id` is nullable **on purpose**. An unbound native handle is a recorded fact
/// -- it is how extraction coverage is measured -- not a failure to be cleaned up.
///
/// §1.3 gives this table a bespoke `binding_id` primary key. It uses the ordinary common columns
/// instead, so that every identity invariant, the `entity_id` derivation check and the duplicate
/// check all apply to it unchanged. A table with its own key shape would have been a table outside
/// the guarantees, which is the opposite of what a crosswalk should be.
pub fn snapshot_native_binding() -> SchemaRef {
    table(vec![
        Field::new("native_namespace", DataType::Utf8, false),
        Field::new("native_handle", DataType::Utf8, false),
        entity_id_field("canonical_entity_id", true),
        entity_key_field("canonical_entity_key", true),
        lattice_field("binding", false),
        Field::new("mapping_basis", DataType::Utf8, false),
    ])
}

/// Every table, by qualified name. The single registry the DDL, the fixed-point check and the
/// catalog assembly all read, so none of them can disagree about what exists.
pub fn all_tables() -> Vec<(&'static str, SchemaRef)> {
    vec![
        ("catalog.lattice", catalog_lattice()),
        ("catalog.capability", catalog_capability()),
        ("catalog.mechanism", catalog_mechanism()),
        ("catalog.surface_binding", catalog_surface_binding()),
        ("catalog.surface_entry", catalog_surface_entry()),
        ("catalog.parameter", catalog_parameter()),
        ("catalog.result_contract", catalog_result_contract()),
        ("catalog.search_domain", catalog_search_domain()),
        ("catalog.negative_space", catalog_negative_space()),
        ("catalog.interaction", catalog_interaction()),
        ("catalog.interaction_atom", catalog_interaction_atom()),
        ("catalog.plan_fragment", catalog_plan_fragment()),
        ("evidence.behavior_assertion", evidence_behavior_assertion()),
        ("snapshot.source_anchor", snapshot_source_anchor()),
        ("snapshot.extraction_run", snapshot_extraction_run()),
        ("snapshot.native_binding", snapshot_native_binding()),
        ("program.crate_unit", program_crate_unit()),
        ("program.feature_declaration", program_feature_declaration()),
        ("program.dependency_edge", program_dependency_edge()),
        ("program.package", program_package()),
        ("program.definition", program_definition()),
        ("program.signature", program_signature()),
        ("program.implementation", program_implementation()),
        ("program.export_path", program_export_path()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The PB03 rules, asserted over every authored column so a future table cannot quietly
    /// reintroduce a narrowing type.
    #[test]
    fn no_authored_column_uses_a_type_delta_would_narrow() {
        for (name, schema) in all_tables() {
            for field in schema.fields() {
                let banned = matches!(
                    field.data_type(),
                    DataType::UInt8
                        | DataType::UInt16
                        | DataType::UInt32
                        | DataType::UInt64
                        | DataType::Utf8View
                        | DataType::BinaryView
                        | DataType::LargeUtf8
                        | DataType::LargeBinary
                        | DataType::Float16
                        | DataType::Dictionary(_, _)
                        | DataType::LargeList(_)
                        | DataType::FixedSizeBinary(_)
                );
                assert!(
                    !banned,
                    "{name}.{} is {:?}, which Delta narrows on write",
                    field.name(),
                    field.data_type()
                );
            }
        }
    }

    /// Byte offsets must be `Int64`. `UInt32` truncates above `i32::MAX` with no error, which is
    /// a corrupt offset in any file over 2 GiB.
    #[test]
    fn every_byte_offset_is_int64() {
        for (name, schema) in all_tables() {
            for field in schema.fields() {
                if field.metadata().get(ARROW_EXT_KEY).map(String::as_str) == Some(EXT_BYTE_OFFSET)
                {
                    assert_eq!(
                        field.data_type(),
                        &DataType::Int64,
                        "{name}.{} is a byte offset and must be Int64",
                        field.name()
                    );
                }
            }
        }
    }

    /// Lattice columns are stored ranks, named `<lattice>_rank`, typed `Int8`.
    #[test]
    fn every_lattice_column_is_an_int8_rank_with_a_known_name() {
        let mut seen = 0;
        for (table_name, schema) in all_tables() {
            for field in schema.fields() {
                if field.metadata().get(ARROW_EXT_KEY).map(String::as_str) != Some(EXT_LATTICE) {
                    continue;
                }
                seen += 1;
                assert_eq!(field.data_type(), &DataType::Int8, "{table_name}");
                let stem = field
                    .name()
                    .strip_suffix("_rank")
                    .unwrap_or_else(|| panic!("{table_name}.{} must end in _rank", field.name()));
                assert!(
                    lattice::lattice(stem).is_some(),
                    "{table_name}.{} names no lattice in catalog.lattice",
                    field.name()
                );
            }
        }
        assert!(seen > 0, "the check must actually inspect some columns");
    }

    /// List children are authored with the Parquet spelling Delta renames to, so an authored
    /// schema and a read-back schema compare equal without a normalisation step.
    #[test]
    fn list_children_are_named_element() {
        for (name, schema) in all_tables() {
            for field in schema.fields() {
                if let DataType::List(child) = field.data_type() {
                    assert_eq!(
                        child.name(),
                        "element",
                        "{name}.{} child must use the Parquet spelling",
                        field.name()
                    );
                }
            }
        }
    }
}
