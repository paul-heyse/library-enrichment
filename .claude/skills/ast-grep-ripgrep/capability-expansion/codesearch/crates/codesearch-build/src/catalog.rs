//! Turning the skill's index rows into canonical catalog records.
//!
//! This is the catalog extraction family. It is the eighth family alongside the seven Rust ones,
//! and it obeys the same contract shape: it *produces* a named set of tables, mints keys by a
//! stated rule, emits a closed `mapping_basis` vocabulary, and **never fills** columns it cannot
//! know.
//!
//! # What it deliberately does not do
//!
//! It does not collapse the skill's epistemic vocabularies. `regex.tsv` carries an `observed`
//! column whose values are `confirmed | recorded | unknown | not-probed`, and those go straight
//! onto the `observed` lattice rather than becoming a boolean. A construct that was never probed
//! and a construct whose probe was inconclusive are different facts, and the catalog would be
//! worse than useless if it reported them the same way.
//!
//! # One construct can be two mechanisms
//!
//! A regex construct available in both engines yields **two** mechanism rows, not one. The two
//! engines are genuinely different: ast-grep's `regex` rule uses Rust regex syntax rather than
//! PCRE2, so a lookaround that works under `rg -P` is not authorised under the default engine
//! merely because both are called "regex". Collapsing them would license exactly the transfer the
//! catalog exists to prevent.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, BooleanArray, Int8Array, Int32Array, Int64Array, RecordBatch, StringArray,
};
use arrow_schema::SchemaRef;
use codesearch_bridge::identity;
use codesearch_bridge::lattice;

use crate::capabilities::{Seed, bind};
use crate::index::{self, Row};

/// The provenance vocabulary this family may emit. Closed, not free text.
pub const BASIS_FLAGS: &str = "skill.index.flags";
pub const BASIS_RULE_FIELDS: &str = "skill.index.rule-fields";
pub const BASIS_REGEX: &str = "skill.index.regex";

/// What kind of canonical record characterises a surface entry.
///
/// A mechanism is one of four, not the only one. An exit code becomes a result contract, a file
/// type or language becomes a search domain, and an unreachable capability becomes a negative-space
/// row -- so coverage asks "has this inventory item been turned into *something*", not "does it
/// have a mechanism".
pub const KIND_CAPABILITY: &str = "capability";
pub const KIND_MECHANISM: &str = "mechanism";
pub const KIND_RESULT_CONTRACT: &str = "result_contract";
pub const KIND_SEARCH_DOMAIN: &str = "search_domain";
pub const KIND_NEGATIVE_SPACE: &str = "negative_space";

/// One surface entry as an index row yields it, before the common columns are attached.
///
/// `characterised_by` is the canonical record this inventory item became, and the kind naming
/// which table holds it. `None` is the honest state for an item nothing was extracted from, and
/// it is what `coverage` reports as uncharacterised rather than omitting.
struct Entry {
    tool: String,
    natural_key: String,
    characterised_by: Option<(String, &'static str)>,
}

impl Entry {
    fn characterised(tool: &str, natural_key: String, key: String, kind: &'static str) -> Self {
        Self {
            tool: tool.to_string(),
            natural_key,
            characterised_by: Some((key, kind)),
        }
    }

    fn bare(tool: &str, natural_key: String) -> Self {
        Self {
            tool: tool.to_string(),
            natural_key,
            characterised_by: None,
        }
    }
}

/// Accumulates columns by name, then projects them onto a schema's field order.
///
/// Building by name and projecting last means a column added to the schema but not to the builder
/// is a loud error at build time rather than a silently misaligned array -- which, with columns of
/// the same type, is the kind of mistake that produces plausible wrong answers.
#[derive(Default)]
pub(crate) struct Columns {
    /// Columns this family may not know. See [`Columns::never_fills`].
    forbidden: &'static [&'static str],
    strings: BTreeMap<&'static str, Vec<Option<String>>>,
    ranks: BTreeMap<&'static str, Vec<i8>>,
    ints: BTreeMap<&'static str, Vec<Option<i64>>>,
    i32s: BTreeMap<&'static str, Vec<Option<i32>>>,
    bools: BTreeMap<&'static str, Vec<bool>>,
    lists: BTreeMap<&'static str, Vec<Vec<String>>>,
    len: usize,
}

impl Columns {
    /// Declare the columns this family must leave NULL, and make it an error to do otherwise.
    ///
    /// §3 part 4: *"a build-time assertion, not a comment. If an adapter writes a column its family
    /// cannot know, the build fails."* Evidence 06's warning is the reason -- four of seven families
    /// have blind spots that produce **plausible-looking nulls**, and a plausible-looking null is
    /// indistinguishable from an observed one unless something refuses to write it.
    ///
    /// "Never fills" means **null in every row**, not absent: `Columns::build` still requires every
    /// schema column to be supplied, because an omitted column would become null silently and this
    /// check would then be satisfied by the very mistake it exists to catch. A family says
    /// `str_col("visibility", None)` and means it.
    pub(crate) fn never_fills(forbidden: &'static [&'static str]) -> Self {
        Self {
            forbidden,
            ..Self::default()
        }
    }

    pub(crate) fn str_col(&mut self, name: &'static str, value: impl Into<Option<String>>) {
        self.strings.entry(name).or_default().push(value.into());
    }
    pub(crate) fn rank_col(&mut self, name: &'static str, value: i8) {
        self.ranks.entry(name).or_default().push(value);
    }
    pub(crate) fn int_col(&mut self, name: &'static str, value: Option<i64>) {
        self.ints.entry(name).or_default().push(value);
    }
    pub(crate) fn i32_col(&mut self, name: &'static str, value: Option<i32>) {
        self.i32s.entry(name).or_default().push(value);
    }
    pub(crate) fn bool_col(&mut self, name: &'static str, value: bool) {
        self.bools.entry(name).or_default().push(value);
    }
    /// A `List<Utf8>` cell. An empty vector is an empty list, which is a different fact from a
    /// null one -- "this interaction cites no assertion" rather than "nobody looked".
    pub(crate) fn list_col(&mut self, name: &'static str, values: Vec<String>) {
        self.lists.entry(name).or_default().push(values);
    }
    pub(crate) fn end_row(&mut self) {
        self.len += 1;
    }

    pub(crate) fn build(self, schema: &SchemaRef) -> Result<RecordBatch, CatalogError> {
        let mut arrays: Vec<ArrayRef> = Vec::with_capacity(schema.fields().len());
        for field in schema.fields() {
            let name = field.name().as_str();
            // The digest is DERIVED from the other columns, so it cannot be supplied by a family
            // and is filled once every other column exists. A placeholder holds its position so
            // the projection stays by field order rather than by insertion order.
            if name == codesearch_bridge::identity::PAYLOAD_DIGEST {
                arrays.push(Arc::new(StringArray::new_null(self.len)) as ArrayRef);
                continue;
            }
            let array: ArrayRef = if let Some(v) = self.strings.get(name) {
                Arc::new(StringArray::from(v.clone()))
            } else if let Some(v) = self.ranks.get(name) {
                Arc::new(Int8Array::from(v.clone()))
            } else if let Some(v) = self.ints.get(name) {
                Arc::new(Int64Array::from(v.clone()))
            } else if let Some(v) = self.i32s.get(name) {
                Arc::new(Int32Array::from(v.clone()))
            } else if let Some(v) = self.bools.get(name) {
                Arc::new(BooleanArray::from(v.clone()))
            } else if let arrow_schema::DataType::List(child) = field.data_type() {
                // A column the builder supplied, or -- for a family that does not populate this
                // list -- empty lists of the right length.
                let rows = self.lists.get(name);
                let mut values: Vec<String> = Vec::new();
                let mut offsets: Vec<i32> = Vec::with_capacity(self.len + 1);
                offsets.push(0);
                for i in 0..self.len {
                    if let Some(cell) = rows.and_then(|r| r.get(i)) {
                        values.extend(cell.iter().cloned());
                    }
                    offsets.push(values.len() as i32);
                }
                Arc::new(arrow::array::ListArray::new(
                    Arc::clone(child),
                    arrow::buffer::OffsetBuffer::new(offsets.into()),
                    Arc::new(StringArray::from(values)),
                    None,
                ))
            } else {
                return Err(CatalogError::MissingColumn(name.to_string()));
            };
            if array.len() != self.len {
                return Err(CatalogError::Ragged {
                    column: name.to_string(),
                    found: array.len(),
                    expected: self.len,
                });
            }
            // The never-fills check. A forbidden column must be null in every row -- one non-null
            // cell means this family wrote something it cannot know.
            if self.forbidden.contains(&name) && array.null_count() != array.len() {
                return Err(CatalogError::ForbiddenColumn {
                    column: name.to_string(),
                    filled: array.len() - array.null_count(),
                });
            }
            arrays.push(array);
        }
        // The digest is filled by the one implementation every producer shares, so the build and
        // the fixtures cannot drift into computing it two ways that agree only today.
        codesearch_bridge::identity::batch_with_digest(schema, arrays, self.len)
            .map_err(|e| CatalogError::Build(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error(
        "no data supplied for column `{0}`. Every schema column must be populated -- an omitted one would silently become null."
    )]
    MissingColumn(String),
    #[error(
        "this family filled `{column}` in {filled} row(s), but it is a column the family cannot know. A plausible-looking value here is indistinguishable from an observed one, which is the failure `never_fills` exists to prevent. Leave it NULL, or move the column to a family that can observe it."
    )]
    ForbiddenColumn { column: String, filled: usize },
    #[error(
        "two `{what}` rows mint the same key `{key}`. The index cannot tell them apart, so neither can this model. Merging them would silently discard one; that is a finding about the index, not a bug to route around."
    )]
    KeyCollision { what: String, key: String },
    #[error("column `{column}` has {found} values but the batch has {expected} rows")]
    Ragged {
        column: String,
        found: usize,
        expected: usize,
    },
    #[error("building batch: {0}")]
    Build(String),
    #[error("`{value}` is not a value of the `{lattice}` lattice")]
    UnknownLatticeValue { lattice: String, value: String },
}

/// `assert:<tool>/<probe>` -- shared by the assertion builder and by interaction validation, so
/// a citation and the row it cites cannot disagree about the spelling.
fn assertion_key(tool: &str, probe: &str) -> String {
    format!("assert:{}/{probe}", identity::tool_code(tool))
}

fn rank(lattice_name: &str, label: &str) -> Result<i8, CatalogError> {
    lattice::rank_of(lattice_name, label).ok_or_else(|| CatalogError::UnknownLatticeValue {
        lattice: lattice_name.to_string(),
        value: label.to_string(),
    })
}

/// The provenance and epistemics a row carries, gathered so the call sites read as named facts
/// rather than as a row of positional strings.
pub(crate) struct Common<'a> {
    pub(crate) entity_key: &'a str,
    pub(crate) precision: &'a str,
    pub(crate) observed: &'a str,
    pub(crate) coverage: &'a str,
    pub(crate) ord: Option<i32>,
}

/// Identity of the run producing these rows. Constant across a build, so it is threaded once
/// rather than repeated at every call site.
#[derive(Clone, Copy)]
pub struct RunContext<'a> {
    pub snapshot_id: &'a str,
    pub run_id: &'a str,
}

/// The common columns every canonical row carries.
pub(crate) fn push_common(
    cols: &mut Columns,
    run: RunContext<'_>,
    c: Common<'_>,
) -> Result<(), CatalogError> {
    let Common {
        entity_key,
        precision,
        observed,
        coverage,
        ord,
    } = c;
    cols.str_col(
        "entity_id",
        identity::entity_id(entity_key, run.snapshot_id),
    );
    cols.str_col("entity_key", entity_key.to_string());
    cols.str_col("snapshot_id", run.snapshot_id.to_string());
    cols.str_col("run_id", run.run_id.to_string());
    cols.str_col("evidence_id", None);
    cols.rank_col("precision_rank", rank("precision", precision)?);
    cols.rank_col("observed_rank", rank("observed", observed)?);
    cols.str_col("coverage_status", coverage.to_string());
    cols.i32_col("ord", ord);
    cols.int_col("first_seen_version", Some(0));
    cols.int_col("last_seen_version", None);
    cols.bool_col("retracted", false);
    Ok(())
}

/// Every batch this family produces.
pub struct CatalogBatches {
    pub lattice: RecordBatch,
    pub capability: RecordBatch,
    pub mechanism: RecordBatch,
    pub parameter: RecordBatch,
    pub plan_fragment: RecordBatch,
    pub interaction: RecordBatch,
    pub interaction_atom: RecordBatch,
    pub result_contract: RecordBatch,
    pub search_domain: RecordBatch,
    pub negative_space: RecordBatch,
    pub surface_binding: RecordBatch,
    pub surface_entry: RecordBatch,
    pub behavior_assertion: RecordBatch,
}

/// Build the catalog batches from parsed index rows.
pub fn build(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    seed: &Seed,
    interaction_seed: &crate::interactions::Seed,
    fragment_seed: &crate::plan_fragments::Seed,
    impl_bindings: &[crate::implementation_bindings::Authored],
    run: RunContext<'_>,
) -> Result<CatalogBatches, CatalogError> {
    let mechanism = build_mechanisms(indexes, run)?;

    // Capabilities bind to mechanisms that already exist, so binding happens after minting and a
    // capability claiming nothing fails the build rather than yielding an empty `discover`.
    let keys = mechanism
        .column_by_name("entity_key")
        .and_then(|c| c.as_any().downcast_ref::<StringArray>())
        .ok_or_else(|| CatalogError::Build("mechanism batch has no entity_key".into()))?;
    let mechanism_keys: Vec<String> = (0..keys.len()).map(|i| keys.value(i).to_string()).collect();
    let bound = bind(seed, &mechanism_keys).map_err(|e| CatalogError::Build(e.to_string()))?;
    let minted: BTreeSet<String> = mechanism_keys.iter().cloned().collect();

    // Interactions cite evidence, so the handles are checked against the assertions this build
    // actually produces. The keys are minted by the same rule `build_assertions` uses -- sharing
    // the rule is what keeps the citation and the cited row agreeing about what an assertion is
    // called.
    let assertion_keys: BTreeSet<String> = indexes
        .get("behaviors")
        .into_iter()
        .flatten()
        .map(|row| assertion_key(&row["tool"], &row["probe"]))
        .collect();
    let interactions =
        crate::interactions::collect(indexes, interaction_seed, &minted, &assertion_keys)
            .map_err(|e| CatalogError::Build(e.to_string()))?;

    // A `preserving` recall claim needs a CONFIRMED assertion, not merely an existing one, so the
    // two sets are kept apart. `recorded` is the tool's own word for it; that is enough to
    // catalogue a mechanism and not enough to guarantee a chain keeps what it should.
    let confirmed_assertions: BTreeSet<String> = indexes
        .get("behaviors")
        .into_iter()
        .flatten()
        .filter(|row| row["verdict"] == "confirmed")
        .map(|row| assertion_key(&row["tool"], &row["probe"]))
        .collect();
    let fragments = crate::plan_fragments::validate(
        fragment_seed,
        &minted,
        &assertion_keys,
        &confirmed_assertions,
    )
    .map_err(|e| CatalogError::Build(e.to_string()))?;

    Ok(CatalogBatches {
        lattice: lattice::reference_batch().map_err(|e| CatalogError::Build(e.to_string()))?,
        capability: build_capabilities(seed, &bound, run)?,
        mechanism,
        parameter: build_parameters(indexes, run)?,
        plan_fragment: build_plan_fragments(&fragments, run)?,
        interaction: build_interaction_rows(&interactions, run)?,
        interaction_atom: build_interaction_atoms(&interactions, run)?,
        result_contract: build_result_contracts(indexes, run)?,
        search_domain: build_search_domains(indexes, run)?,
        negative_space: build_negative_space(indexes, &minted, run)?,
        surface_binding: build_surface_bindings(&bound, impl_bindings, run)?,
        surface_entry: build_surface_entries(indexes, run)?,
        behavior_assertion: build_assertions(indexes, run)?,
    })
}

/// One row per authored capability, carrying how many mechanisms it actually binds.
fn build_capabilities(
    seed: &Seed,
    bound: &BTreeMap<&str, Vec<String>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_capability();
    let mut cols = Columns::default();
    for cap in &seed.capabilities {
        let key = identity::capability_key(&cap.ability);
        // A capability is an authored judgement, not an observation: `recorded` is the honest
        // rank. Claiming `confirmed` would assert a probe that does not exist.
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("ability", cap.ability.clone());
        cols.str_col("topic", Some(cap.topic.clone()));
        cols.str_col("summary", Some(cap.summary.clone()));
        for (column, facet) in [
            ("facet_subject", "subject"),
            ("facet_relationship", "required_relationship"),
            ("facet_result", "requested_result"),
            ("facet_semantic_layer", "semantic_requirement"),
        ] {
            cols.str_col(column, cap.facets.get(facet).cloned());
        }
        cols.int_col(
            "mechanism_count",
            Some(bound.get(cap.ability.as_str()).map(Vec::len).unwrap_or(0) as i64),
        );
        cols.end_row();
    }
    cols.build(&schema)
}

/// The capability-to-mechanism relation, as rows rather than as a list column on either side.
fn build_surface_bindings(
    bound: &BTreeMap<&str, Vec<String>>,
    authored: &[crate::implementation_bindings::Authored],
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_surface_binding();
    let mut cols = Columns::default();
    for (ability, mechanisms) in bound {
        let capability_key = identity::capability_key(ability);
        for (ord, mechanism_key) in mechanisms.iter().enumerate() {
            let key = format!("bind:{ability}/{mechanism_key}");
            push_common(
                &mut cols,
                run,
                Common {
                    entity_key: &key,
                    precision: "exact",
                    observed: "recorded",
                    coverage: "characterised",
                    ord: Some(ord as i32),
                },
            )?;
            cols.str_col("subject_key", capability_key.clone());
            cols.str_col("subject_kind", KIND_CAPABILITY.to_string());
            cols.str_col("object_key", mechanism_key.clone());
            cols.str_col("object_kind", KIND_MECHANISM.to_string());
            cols.str_col("role", "implemented_by".to_string());
            cols.str_col("evidence_note", None);
            cols.end_row();
        }
    }

    // The authored half: a mechanism and the Rust code that implements it. These rows live in the
    // same table as the derived capability->mechanism ones because they are the same relation --
    // §3.8's subject/object/role -- and keeping them apart would mean two tables to join and two
    // invariants to keep in step.
    //
    // They are written by the CATALOG family even though their object keys come from
    // `library-api`, because a table has one writer (`assert_single_writer`) and this table is
    // `catalog.*`. That is why the build runs `library-api` first and hands its keys over: the
    // seed is validated against rows that already exist rather than against rows it hopes for.
    for (ord, b) in authored.iter().enumerate() {
        let key = format!("bind:{}/{}", b.mechanism, b.object);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                // Authored, so the seed carries its own lattice placement rather than inheriting a
                // constant: a crate named by the skill's own prose is not the same kind of claim as
                // a builder type inferred from its name, and flattening the two would be the
                // "confidently wrong" failure this catalog exists to avoid.
                precision: &b.precision,
                observed: &b.observed,
                coverage: "characterised",
                ord: Some(ord as i32),
            },
        )?;
        cols.str_col("subject_key", b.mechanism.clone());
        cols.str_col("subject_kind", KIND_MECHANISM.to_string());
        cols.str_col("object_key", b.object.clone());
        cols.str_col("object_kind", b.object_kind.clone());
        cols.str_col("role", b.role.clone());
        cols.str_col("evidence_note", Some(b.evidence_note.clone()));
        cols.end_row();
    }
    cols.build(&schema)
}

fn build_mechanisms(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_mechanism();
    let mut cols = Columns::default();

    // CLI flags. Read from each binary's own `--help`, so `recorded` rather than `confirmed`:
    // the tool asserted it, but no probe exercised it.
    for row in indexes.get("flags").into_iter().flatten() {
        let (tool, command, long) = (&row["tool"], &row["command"], &row["long"]);
        if long.is_empty() {
            continue;
        }
        let natural = identity::flag_natural_key(tool, command, long);
        let key = identity::mechanism_key(tool, "cli", &natural);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", identity::tool_code(tool).to_string());
        cols.str_col("surface", "cli".to_string());
        cols.str_col("natural_key", natural);
        cols.str_col("invocation_form", Some(command.clone()));
        cols.str_col("native_schema_ref", Some(BASIS_FLAGS.to_string()));
        cols.str_col("summary", Some(row["summary"].clone()));
        cols.end_row();
    }

    // ast-grep rule fields, from the published rule schema.
    for row in indexes.get("rule-fields").into_iter().flatten() {
        // The natural key is SCOPE-QUALIFIED. 22 field names occur in more than one scope, and
        // `has` under `rule` is a different mechanism from `has` under `constraints`: they apply
        // in different places and take different values. Keying on the bare field name would
        // silently merge them, which is precisely the normalisation the proposal forbids.
        let natural = identity::rule_field_natural_key(&row["scope"], &row["field"]);
        let key = identity::mechanism_key("ast-grep", "rule", &natural);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", "sg".to_string());
        cols.str_col("surface", "rule".to_string());
        cols.str_col("natural_key", natural);
        cols.str_col("invocation_form", Some(row["scope"].clone()));
        cols.str_col("native_schema_ref", Some(BASIS_RULE_FIELDS.to_string()));
        cols.str_col("summary", Some(row["summary"].clone()));
        cols.end_row();
    }

    // Regex constructs. ONE ROW PER ENGINE -- see the module docs. `observed` comes straight from
    // the index, so a not-probed construct stays not-probed.
    for row in indexes.get("regex").into_iter().flatten() {
        let construct = &row["construct"];
        let observed = if row["observed"].is_empty() {
            "not-probed"
        } else {
            &row["observed"]
        };
        for (engine_column, tool) in [("rust_regex", "rg"), ("pcre2", "pcre2")] {
            if row[engine_column] != "yes" {
                continue;
            }
            let key = identity::mechanism_key(tool, "pattern", construct);
            push_common(
                &mut cols,
                run,
                Common {
                    entity_key: &key,
                    precision: "exact",
                    observed,
                    coverage: "characterised",
                    ord: None,
                },
            )?;
            cols.str_col("tool", tool.to_string());
            cols.str_col("surface", "pattern".to_string());
            cols.str_col("natural_key", construct.clone());
            cols.str_col("invocation_form", Some(row["syntax"].clone()));
            cols.str_col("native_schema_ref", Some(BASIS_REGEX.to_string()));
            cols.str_col("summary", Some(row["purpose"].clone()));
            cols.end_row();
        }
    }

    cols.build(&schema)
}

/// The argument a mechanism takes, where the index records one.
///
/// # What the index knows, and what it does not
///
/// `flags.tsv` carries `arg` (the placeholder from the tool's own `--help`) and `values` (the
/// enumerated domain, where the help prints one). `rule-fields.tsv` carries `type` and a genuine
/// `required`/`optional` column. That is the name, the domain and the obligation -- three of the
/// seven columns `catalog.parameter` holds.
///
/// **The three default columns stay null**, because the index records no default and this family
/// never fills a column it cannot know. §4.3's worked example is exactly why they are three
/// columns and not one: `grep-pcre2` documents UTF and Unicode-property modes as *disabled* by
/// default while ripgrep *enables* both unless Unicode is disabled, so the library default and the
/// effective default on the chosen surface are different facts and an agent asking "what is the
/// default" needs the third. Since that is the question the table exists to answer and it cannot
/// be answered from the index, every row here is `uncharacterised` -- not as a placeholder, but as
/// the accurate statement of what is missing.
fn build_parameters(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_parameter();
    let mut cols = Columns::default();

    let push = |cols: &mut Columns,
                mechanism_key: &str,
                name: String,
                value_domain: Option<String>,
                required: bool|
     -> Result<(), CatalogError> {
        let key = format!("param:{mechanism_key}/{name}");
        push_common(
            cols,
            run,
            Common {
                entity_key: &key,
                // The name and domain are verbatim from the tool's own output, so `exact`; the
                // tool asserted them and no probe exercised them, so `recorded`.
                precision: "exact",
                observed: "recorded",
                coverage: "uncharacterised",
                ord: None,
            },
        )?;
        cols.str_col(
            "mechanism_id",
            identity::entity_id(mechanism_key, run.snapshot_id),
        );
        cols.str_col("name", name);
        cols.str_col("value_domain", value_domain);
        cols.str_col("declared_default", None);
        cols.str_col("application_override", None);
        cols.str_col("effective_default", None);
        cols.bool_col("required", required);
        cols.end_row();
        Ok(())
    };

    for row in indexes.get("flags").into_iter().flatten() {
        let (tool, command, long, arg) = (&row["tool"], &row["command"], &row["long"], &row["arg"]);
        if long.is_empty() || arg.is_empty() {
            continue;
        }
        let natural = identity::flag_natural_key(tool, command, long);
        let mechanism_key = identity::mechanism_key(tool, "cli", &natural);
        // A leading `=` is how the index spells an *optional* argument: the help printed
        // `--json[=<STYLE>]`, so `--json` works bare and `--json=compact` works too. That reading
        // is a convention of the source, not an observation, which is why it is stated here rather
        // than left implicit in a strip call.
        let optional = arg.starts_with('=');
        let name = arg
            .trim_start_matches('=')
            .trim_matches(['<', '>'])
            .to_string();
        let domain = if row["values"].is_empty() {
            None
        } else {
            Some(row["values"].clone())
        };
        push(&mut cols, &mechanism_key, name, domain, !optional)?;
    }

    for row in indexes.get("rule-fields").into_iter().flatten() {
        let natural = identity::rule_field_natural_key(&row["scope"], &row["field"]);
        let mechanism_key = identity::mechanism_key("ast-grep", "rule", &natural);
        let domain = if row["type"].is_empty() {
            None
        } else {
            Some(row["type"].clone())
        };
        push(
            &mut cols,
            &mechanism_key,
            row["field"].clone(),
            domain,
            row["required"] == "required",
        )?;
    }

    cols.build(&schema)
}

/// One row per authored plan fragment.
fn build_plan_fragments(
    fragments: &[crate::plan_fragments::Authored],
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_plan_fragment();
    let mut cols = Columns::default();
    for f in fragments {
        // A fragment backed by a confirmed probe is `confirmed`; one resting on a reading of the
        // topic pages is `recorded`. The validator has already refused any `preserving` claim with
        // no confirmed assertion behind it, so this cannot overstate.
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &f.key,
                precision: "exact",
                observed: if f.assertions.is_empty() {
                    "recorded"
                } else {
                    "confirmed"
                },
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("name", f.name.clone());
        cols.str_col("accepted_input", f.accepted_input.clone());
        cols.str_col("produced_output", f.produced_output.clone());
        cols.str_col("scope_assumption", f.scope_assumption.clone());
        cols.str_col("coordinate_preservation", f.coordinate_preservation.clone());
        cols.rank_col("recall_rank", rank("recall", &f.recall)?);
        cols.str_col("ordering_requirement", f.ordering_requirement.clone());
        cols.list_col("mechanism_keys", f.mechanisms.clone());
        cols.list_col("assertion_keys", f.assertions.clone());
        cols.list_col("unresolved_obligations", f.unresolved_obligations.clone());
        cols.end_row();
    }
    cols.build(&schema)
}

/// One row per interaction, with its evidence handles as a list rather than a joined string.
fn build_interaction_rows(
    flats: &[crate::interactions::Flat],
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_interaction();
    let mut cols = Columns::default();
    for flat in flats {
        // An interaction is a reading of the surfaces, not an observation of them. `recorded` is
        // the honest rank; `confirmed` would assert a probe that in most cases does not exist.
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &flat.key,
                precision: "exact",
                observed: if flat.assertions.is_empty() {
                    "recorded"
                } else {
                    "confirmed"
                },
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("kind", flat.kind.clone());
        cols.str_col("affected_key", flat.affected.clone());
        cols.str_col("effect", flat.effect.clone());
        cols.str_col("ordering_requirement", flat.ordering.clone());
        cols.list_col("assertion_keys", flat.assertions.clone());
        cols.end_row();
    }
    cols.build(&schema)
}

/// One row per literal. `term_ord` is the disjunct, `atom_ord` the position within it.
fn build_interaction_atoms(
    flats: &[crate::interactions::Flat],
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_interaction_atom();
    let mut cols = Columns::default();
    for flat in flats {
        for (term_ord, term) in flat.terms.iter().enumerate() {
            for (atom_ord, atom) in term.iter().enumerate() {
                let key = format!("{}#{term_ord}.{atom_ord}", flat.key);
                push_common(
                    &mut cols,
                    run,
                    Common {
                        entity_key: &key,
                        precision: "exact",
                        observed: "recorded",
                        coverage: "characterised",
                        // `ord` is the atom's position, and it is not decoration: a conjunct
                        // whose atoms carry no order has asserted that their order does not
                        // matter, which for an ordered comparison is simply false.
                        ord: Some(atom_ord as i32),
                    },
                )?;
                cols.str_col("interaction_key", flat.key.clone());
                cols.i32_col("term_ord", Some(term_ord as i32));
                cols.i32_col("atom_ord", Some(atom_ord as i32));
                cols.str_col("facet", atom.facet.clone());
                cols.str_col("op", atom.op.clone());
                cols.str_col("value", atom.value.clone());
                cols.list_col("value_list", atom.value_list.clone());
                cols.bool_col("negated", atom.negated);
                cols.end_row();
            }
        }
    }
    cols.build(&schema)
}

/// What a command's exit status means, from `exit-codes.tsv`.
///
/// Nine rows, and the reason they matter is the `trap` column. ast-grep exits `8` for a pattern it
/// cannot parse where a reader who knows ripgrep expects `2`, and `outline` exits `0` even for a
/// path that does not exist. Both are recorded as traps rather than folded into `meaning`, because
/// a trap is the thing a caller gets wrong, not the thing it wants to know.
fn build_result_contracts(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_result_contract();
    let mut cols = Columns::default();

    for row in indexes.get("exit-codes").into_iter().flatten() {
        let (tool, command, code) = (&row["tool"], &row["command"], &row["code"]);
        let key = identity::result_contract_key(tool, command, code);
        // Probe-backed where the skill probed it: A007 established `outline`'s exit-0 behaviour.
        // The rest are read from documentation, so `recorded`.
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: if row["trap"].contains("Probe") {
                    "confirmed"
                } else {
                    "recorded"
                },
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", identity::tool_code(tool).to_string());
        cols.str_col("command", identity::subcommand_path(tool, command));
        cols.int_col("code", code.parse::<i64>().ok());
        cols.str_col("meaning", row["meaning"].clone());
        cols.str_col(
            "trap",
            if row["trap"].is_empty() {
                None
            } else {
                Some(row["trap"].clone())
            },
        );
        cols.end_row();
    }

    cols.build(&schema)
}

/// What each tool can be pointed at, from `file-types.tsv` and `languages.tsv`.
///
/// Two domain kinds in one table because they answer one question from two sides. The `truth`
/// lattice carries the part that was actually measured: `languages.tsv`'s `status` is the skill's
/// own adjudication, reached by running each language against the binary, so `rejected` is a
/// verdict and not an omission.
fn build_search_domains(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_search_domain();
    let mut cols = Columns::default();

    for row in indexes.get("file-types").into_iter().flatten() {
        let name = &row["type"];
        let key = identity::search_domain_key("rg", "file_type", name);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", "rg".to_string());
        cols.str_col("domain_kind", "file_type".to_string());
        cols.str_col("name", name.clone());
        cols.str_col("selector", Some(row["globs"].clone()));
        // A shipped type definition exists; whether any file matches it is a different question
        // this row does not answer.
        cols.rank_col("truth_rank", rank("truth", "true")?);
        cols.int_col("grammar_kind_count", None);
        cols.int_col("grammar_field_count", None);
        cols.bool_col("rule_schema_available", false);
        cols.end_row();
    }

    for row in indexes.get("languages").into_iter().flatten() {
        let name = &row["language"];
        let key = identity::search_domain_key("ast-grep", "language", name);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                // The adjudication was made by running the language against the binary, which is
                // what `confirmed` means everywhere else in this catalog.
                observed: "confirmed",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", "sg".to_string());
        cols.str_col("domain_kind", "language".to_string());
        cols.str_col("name", name.clone());
        cols.str_col("selector", None);
        cols.rank_col(
            "truth_rank",
            match row["status"].as_str() {
                "accepted" => rank("truth", "true")?,
                "rejected" => rank("truth", "false")?,
                _ => rank("truth", "unknown")?,
            },
        );
        cols.int_col("grammar_kind_count", row["kinds"].parse::<i64>().ok());
        cols.int_col("grammar_field_count", row["fields"].parse::<i64>().ok());
        // Eight accepted languages have no rule schema entry. Parseable and schema-listed are two
        // facts, and collapsing them would overstate what a rule can be written against.
        cols.bool_col("rule_schema_available", row["rule_schema"] == "yes");
        cols.end_row();
    }

    cols.build(&schema)
}

/// Capabilities that exist in the library and are not reachable, from `unreachable.tsv`.
///
/// This is the table that lets `discover` return a useful negative. Ten rows, each with a reason
/// and, in every case, something to do instead.
fn build_negative_space(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    mechanism_keys: &BTreeSet<String>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_negative_space();
    let mut cols = Columns::default();

    for row in indexes.get("unreachable").into_iter().flatten() {
        let (tool, capability) = (&row["exists_in"], &row["capability"]);
        let key = identity::negative_space_key(tool, capability);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("tool", identity::tool_code(tool).to_string());
        cols.str_col("capability_text", capability.clone());
        // `no` and `partial` are different facts. `partial` means the capability is reachable in
        // some configurations and not others -- which is `unknown` about any particular one, and
        // exactly the value the skill's own reference says must not be inferred away.
        cols.rank_col(
            "truth_rank",
            match row["cli_reachable"].as_str() {
                "no" => rank("truth", "false")?,
                "partial" => rank("truth", "unknown")?,
                "yes" => rank("truth", "true")?,
                other => {
                    return Err(CatalogError::UnknownLatticeValue {
                        lattice: "truth".to_string(),
                        value: other.to_string(),
                    });
                }
            },
        );
        cols.str_col("why", row["why"].clone());
        let instead = &row["instead"];
        cols.str_col(
            "instead_text",
            if instead.is_empty() {
                None
            } else {
                Some(instead.clone())
            },
        );
        // The alternative resolves to a real mechanism only when the prose names one this catalog
        // holds. Guessing would produce a dangling key that `validate` would then report, so an
        // unresolvable alternative stays text -- which is still useful to a reader.
        cols.str_col("instead_key", instead_mechanism(instead, mechanism_keys));
        cols.end_row();
    }

    cols.build(&schema)
}

/// Resolve an `instead` sentence to a mechanism key, when it names one unambiguously.
///
/// Deliberately conservative: it looks for a long flag spelled in the prose and accepts it only if
/// exactly one minted mechanism ends with it. A sentence naming two flags, or a flag this catalog
/// does not hold, resolves to nothing rather than to a guess.
fn instead_mechanism(instead: &str, mechanism_keys: &BTreeSet<String>) -> Option<String> {
    let mut matches: Vec<&String> = Vec::new();
    for token in instead.split_whitespace() {
        let flag = token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '-');
        if !flag.starts_with("--") || flag.len() < 4 {
            continue;
        }
        let hits: Vec<&String> = mechanism_keys
            .iter()
            .filter(|k| k.ends_with(&format!("/{flag}")))
            .collect();
        if hits.len() == 1 {
            matches.push(hits[0]);
        }
    }
    matches.dedup();
    if matches.len() == 1 {
        Some(matches[0].clone())
    } else {
        None
    }
}

/// Every row of every loaded index is a surface entry.
///
/// This is the coverage **denominator**, and it is built independently of `catalog.mechanism` on
/// purpose: two sides that are not derived from each other are what make the `LEFT JOIN` between
/// them unable to drift.
fn build_surface_entries(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::catalog_surface_entry();
    let mut cols = Columns::default();

    // Membership is read off the spec, not decided here. Previously this loop walked every spec
    // and let anything its `match` did not name fall through `_ => continue`, which meant a file
    // added to `SPECS` was silently outside the denominator -- indistinguishable from a file
    // deliberately excluded. `behaviors.tsv` was the deliberate one and said so in a comment; the
    // five `library-api` sources would have been five more, saying nothing. Now the file declares
    // which question it is part of, and this loop asks.
    for spec in index::specs_for(index::Denominator::ToolSurface) {
        let stem = spec.stem();
        for (ord, row) in indexes.get(stem).into_iter().flatten().enumerate() {
            // The natural key of a surface entry is whatever identifies it in its own table.
            //
            // One index row can be more than one entry. A regex construct available in both
            // engines is two inventory items, because "supported by the Rust engine" and
            // "supported by PCRE2" are the two separate columns the index carries and they are
            // separately true. Emitting one entry and picking an engine for it made `tool` say
            // `rg` while `mechanism_key` named a PCRE2 mechanism, and left the other engine's
            // mechanism in no denominator at all -- a catalog reporting less than it holds.
            let entries: Vec<Entry> = match stem {
                "flags" => {
                    if row["long"].is_empty() {
                        // No long form, so no mechanism was minted either. The entry still counts
                        // in the denominator; claiming a mechanism key here would dangle.
                        vec![Entry::bare(&row["tool"], row["short"].clone())]
                    } else {
                        let natural =
                            identity::flag_natural_key(&row["tool"], &row["command"], &row["long"]);
                        let mk = identity::mechanism_key(&row["tool"], "cli", &natural);
                        vec![Entry::characterised(
                            &row["tool"],
                            natural,
                            mk,
                            KIND_MECHANISM,
                        )]
                    }
                }
                "rule-fields" => {
                    let natural = identity::rule_field_natural_key(&row["scope"], &row["field"]);
                    let mk = identity::mechanism_key("ast-grep", "rule", &natural);
                    vec![Entry::characterised(
                        "ast-grep",
                        natural,
                        mk,
                        KIND_MECHANISM,
                    )]
                }
                "regex" => {
                    let construct = &row["construct"];
                    let mut out = Vec::new();
                    for (engine_column, tool) in [("rust_regex", "rg"), ("pcre2", "pcre2")] {
                        if row[engine_column] != "yes" {
                            continue;
                        }
                        let mk = identity::mechanism_key(tool, "pattern", construct);
                        out.push(Entry::characterised(
                            tool,
                            construct.clone(),
                            mk,
                            KIND_MECHANISM,
                        ));
                    }
                    if out.is_empty() {
                        // Reachable in neither engine is still a documented construct, and
                        // dropping it would shrink the denominator by exactly the rows that are
                        // least characterised. It is reported as uncharacterised instead.
                        out.push(Entry::bare("rg", construct.clone()));
                    }
                    out
                }
                "exit-codes" => {
                    let key =
                        identity::result_contract_key(&row["tool"], &row["command"], &row["code"]);
                    vec![Entry::characterised(
                        &row["tool"],
                        format!("{}#{}", row["command"], row["code"]),
                        key,
                        KIND_RESULT_CONTRACT,
                    )]
                }
                "file-types" => {
                    let key = identity::search_domain_key("rg", "file_type", &row["type"]);
                    vec![Entry::characterised(
                        "rg",
                        row["type"].clone(),
                        key,
                        KIND_SEARCH_DOMAIN,
                    )]
                }
                "languages" => {
                    let key = identity::search_domain_key("ast-grep", "language", &row["language"]);
                    vec![Entry::characterised(
                        "ast-grep",
                        row["language"].clone(),
                        key,
                        KIND_SEARCH_DOMAIN,
                    )]
                }
                "unreachable" => {
                    let key = identity::negative_space_key(&row["exists_in"], &row["capability"]);
                    vec![Entry::characterised(
                        &row["exists_in"],
                        row["capability"].clone(),
                        key,
                        KIND_NEGATIVE_SPACE,
                    )]
                }
                _ => continue,
            };

            for entry in entries {
                let Entry {
                    tool,
                    natural_key: natural,
                    characterised_by,
                } = entry;
                let entry_key = format!("surf:{}/{stem}/{natural}", identity::tool_code(&tool));
                // An entry characterised by nothing is uncharacterised -- the honest denominator
                // state, and the row `coverage` reports rather than omits.
                let coverage = if characterised_by.is_some() {
                    "characterised"
                } else {
                    "uncharacterised"
                };
                push_common(
                    &mut cols,
                    run,
                    Common {
                        entity_key: &entry_key,
                        precision: "exact",
                        observed: "recorded",
                        coverage,
                        ord: Some(ord as i32),
                    },
                )?;
                cols.str_col("tool", identity::tool_code(&tool).to_string());
                cols.str_col("surface", stem.to_string());
                cols.str_col("natural_key", natural);
                cols.str_col("source_table", stem.to_string());
                let (catalog_key, catalog_kind) = match characterised_by {
                    Some((key, kind)) => (Some(key), Some(kind.to_string())),
                    None => (None, None),
                };
                cols.str_col("catalog_key", catalog_key);
                cols.str_col("catalog_kind", catalog_kind);
                cols.end_row();
            }
        }
    }

    cols.build(&schema)
}

/// `behaviors.tsv` rows are already probe-and-control backed, so they are **re-typed, not
/// re-derived**. Each carries its command, its expected outcome, its control and the control's
/// exit code, which is exactly what a behaviour assertion needs.
fn build_assertions(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::evidence_behavior_assertion();
    let mut cols = Columns::default();

    for row in indexes.get("behaviors").into_iter().flatten() {
        let probe = &row["probe"];
        let key = assertion_key(&row["tool"], probe);
        // `inconclusive` is a first-class verdict in the skill and is NOT a failure. It maps to
        // the lattice's `unknown`: something was attempted and did not settle.
        let observed = match row["verdict"].as_str() {
            "inconclusive" => "unknown",
            other => other,
        };
        // A confirmed probe is exact; anything weaker is inexact, because the assertion holds
        // only under conditions the probe did not pin down.
        let precision = if observed == "confirmed" {
            "exact"
        } else {
            "inexact"
        };
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision,
                observed,
                coverage: "characterised",
                ord: None,
            },
        )?;
        // The subject is a TOPIC, and saying so matters. `behaviors.tsv` records the topic page a
        // probe belongs to -- `structural-patterns`, `pcre2-advanced-regex` -- which is not a
        // mechanism: no flag, rule field or regex construct has that name. Minting
        // `mech:<tool>/cli/<topic>` and calling the subject a mechanism produced 49 references to
        // mechanisms that do not exist, and `projection.violations_dangling_assertion_subject` is
        // what found it.
        //
        // A probe's command does mention the flags it exercises, so a mechanism-level link is
        // derivable -- but only by parsing command lines, which is a heuristic, and a heuristic
        // link in an evidence catalog is worse than an honest absence. It stays a topic until
        // there is a probe-backed way to do better.
        let subject = format!(
            "topic:{}/{}",
            identity::tool_code(&row["tool"]),
            row["topic"]
        );
        cols.str_col(
            "subject_entity_id",
            identity::entity_id(&subject, run.snapshot_id),
        );
        cols.str_col("subject_kind", "topic".to_string());
        cols.str_col("predicate", row["question"].clone());
        cols.str_col("value_or_target", Some(row["stdout"].clone()));
        cols.str_col("interpretation", Some(row["topic"].clone()));
        cols.str_col("status", row["verdict"].clone());
        cols.str_col("probe_id", Some(probe.clone()));
        cols.str_col("command", Some(row["command"].clone()));
        cols.str_col("expected", Some(row["exit"].clone()));
        cols.str_col("control_command", Some(row["control"].clone()));
        cols.int_col("control_exit", row["control_exit"].parse::<i64>().ok());
        cols.end_row();
    }

    cols.build(&schema)
}
