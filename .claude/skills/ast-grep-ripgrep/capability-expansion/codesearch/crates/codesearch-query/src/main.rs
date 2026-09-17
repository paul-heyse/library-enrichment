//! `codesearch-query` -- one-shot retrieval over a built catalog.
//!
//! Opens the tables read-only, plans, executes, writes to stdout and exits. It never writes, so a
//! build rewriting tables cannot be observed half-done by a reader.
//!
//! # Why one-shot rather than a resident session
//!
//! A resident process would have to invalidate a cache against tables a build rewrites, and a
//! stale cache and a working one look identical from outside -- the same trap that made an earlier
//! probe's "no change" result meaningless. The catalog is small and Delta log replay is bounded,
//! so the process boundary is the isolation boundary.
//!
//! This is a decision with a falsifiable exit criterion, not a preference: `bench` records p50 and
//! p95 for `describe`, and **if p95 exceeds 400 ms the decision is revisited** in favour of a
//! resident mode over a socket.
//!
//! # This binary holds no SQL of its own
//!
//! Every projection is a view registered by `codesearch_bridge::projection`, so the join
//! discipline lives in one place and `information_schema.views` can enumerate the retrieval
//! surface. What is left here is a choice of view and a set of predicates.
//!
//! **Values reach the plan as `Expr::Placeholder`, never as interpolated text.** Which predicates
//! appear is a structural decision made from the arguments that were supplied; what they compare
//! against is bound. That keeps a caller's string out of the SQL entirely, and it keeps the filter
//! a plain comparison against a stored column, which is what prunes.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow::array::RecordBatch;
use clap::{Parser, Subcommand};
use codesearch_bridge::provider::CanonicalTable;
use datafusion::common::ScalarValue;
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::TableProviderBuilder;

/// Tables the query side reads, and the SQL name each is registered under.
const TABLES: &[(&str, &str)] = &[
    ("catalog.lattice", "lattice"),
    ("catalog.capability", "capability"),
    ("catalog.mechanism", "mechanism"),
    ("catalog.parameter", "parameter"),
    ("catalog.result_contract", "result_contract"),
    ("catalog.search_domain", "search_domain"),
    ("catalog.negative_space", "negative_space"),
    ("catalog.interaction", "interaction"),
    ("catalog.interaction_atom", "interaction_atom"),
    ("catalog.plan_fragment", "plan_fragment"),
    ("catalog.surface_binding", "surface_binding"),
    ("catalog.surface_entry", "surface_entry"),
    ("evidence.behavior_assertion", "behavior_assertion"),
    ("snapshot.extraction_run", "extraction_run"),
    ("snapshot.native_binding", "native_binding"),
    ("program.package", "package"),
    ("program.definition", "definition"),
    ("program.signature", "signature"),
    ("program.implementation", "implementation"),
    ("program.export_path", "export_path"),
    ("program.crate_unit", "crate_unit"),
    ("program.feature_declaration", "feature_declaration"),
    ("program.dependency_edge", "dependency_edge"),
];

#[derive(Parser, Debug)]
#[command(name = "codesearch-query", about, long_about = None)]
struct Args {
    /// Where the catalog tables live.
    #[arg(long, env = "CODESEARCH_HOME")]
    home: PathBuf,

    /// Emit Arrow-shaped JSON rather than a text table. Never touches a terminal renderer.
    #[arg(long, global = true)]
    json: bool,

    /// Bind a request facet: `--bind engine=pcre2`. Repeatable.
    ///
    /// A facet left unbound is not false, it is *unknown*, and every interaction that depends on
    /// it evaluates to `unknown` and is reported as a choice still open. That is the whole point
    /// of the three-valued treatment, so binding nothing is a legitimate request rather than an
    /// empty one.
    #[arg(long = "bind", global = true, value_name = "FACET=VALUE")]
    binds: Vec<String>,

    /// Where a chain starts: file_set, file_content, line_set, node_set or capture_set.
    #[arg(long, global = true, default_value = "file_set")]
    start_unit: String,

    /// Bound on chain length. PB09 measured why this is real and therefore visible: fragment
    /// composition is cyclic in general, and an unbounded recursion over a cyclic graph does not
    /// terminate.
    #[arg(long, global = true, default_value_t = 8)]
    max_depth: i64,

    /// Also serve rows written by runs that did not complete.
    ///
    /// Off by default, so a build killed halfway through cannot be mistaken for a catalog. On, it
    /// shows rows from `running`, `partial` and `abandoned` runs alike -- asking to see incomplete
    /// work means all of it, and `codesearch-query runs` says which run each came from.
    #[arg(long, global = true)]
    include_partial: bool,

    /// Which snapshot to answer about. Defaults to the newest one with a visible run.
    ///
    /// A default rather than a required argument, because the ordinary catalog holds exactly one
    /// snapshot and making every caller name it would be ceremony. It is never a *silent* default:
    /// `codesearch-query runs` prints which snapshot a request resolved to, so a catalog holding
    /// two and answering about one says which one out loud.
    #[arg(long, global = true, default_value = "")]
    snapshot: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Find mechanisms matching a surface and tool.
    Discover {
        /// What the capability acts on: `text`, `source-code`, `paths`.
        #[arg(long)]
        subject: Option<String>,
        /// The relationship it expresses: `containment`, `equality`, `ordering`, `identity`,
        /// `exclusion`.
        #[arg(long)]
        relationship: Option<String>,
        /// What it returns: `nodes`, `matching-text`, `paths`, `structured-records`,
        /// `rewritten-text`.
        #[arg(long)]
        result: Option<String>,
        /// Which layer it works at: `lexical`, `syntactic`.
        #[arg(long = "semantic-layer")]
        semantic_layer: Option<String>,
        /// Restrict to tools that can be pointed at this file type, e.g. `rust`.
        #[arg(long)]
        scope: Option<String>,
        /// Restrict to tools that can parse this language, e.g. `rust`.
        #[arg(long)]
        language: Option<String>,
        /// Token budget: how many CANDIDATES to return, never how many columns of one.
        ///
        /// §7.4 is specific -- "fewer candidates, never fewer columns of a candidate" -- and the
        /// `omitted` count comes from the same query as a window total, so the two cannot
        /// disagree about what was left out.
        #[arg(long)]
        budget: Option<usize>,
        #[arg(long)]
        tool: Option<String>,
        #[arg(long)]
        surface: Option<String>,
        /// Restrict to mechanisms that implement this capability.
        #[arg(long)]
        capability: Option<String>,
        /// Only mechanisms whose evidence is at least this strong.
        #[arg(long)]
        min_observed: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Show everything known about one mechanism, by key.
    Describe {
        /// One or more keys. §11 specifies `ENTITY_KEY...`: asking about three mechanisms is one
        /// question, and three invocations would pay the process cost three times.
        #[arg(required = true)]
        entity_key: Vec<String>,
        /// Show only these columns. Repeatable. §11's `--field`.
        ///
        /// A projection selector, not a filter: it narrows what is shown about each candidate,
        /// which is the opposite of what `--budget` does, and §7.4 keeps them apart for that
        /// reason -- "fewer candidates, never fewer columns of a candidate".
        #[arg(long)]
        field: Vec<String>,
    },
    /// List the abstract abilities, with how many mechanisms each actually binds.
    Capabilities,
    /// One of the two coverage numbers. They are never combined into a score.
    Coverage {
        /// `surface` -- how much of the tool is catalogued.
        /// `evidence` -- how much of what is catalogued was actually run.
        #[arg(long, default_value = "surface")]
        dimension: String,
        /// Report only this tool. §11's `--tool`.
        #[arg(long)]
        tool: Option<String>,
    },
    /// Run the cross-row and cross-table invariants, reporting what each inspected.
    Validate {
        /// List the offending rows rather than the per-invariant counts.
        #[arg(long)]
        detail: bool,
        /// Report only these invariants. Repeatable. §11's `--invariant`.
        ///
        /// Filtering still shows `checked` per invariant, so a narrowed run cannot look clean by
        /// inspecting nothing -- which is what `checked` exists to make visible.
        #[arg(long)]
        invariant: Vec<String>,
    },
    /// Capabilities known not to be reachable, with what to do instead.
    NegativeSpace {
        /// Restrict to one tool.
        #[arg(long)]
        tool: Option<String>,
    },
    /// A mechanism and the Rust code that implements it, with the evidence for the claim.
    Implements,
    /// What a definition contains, transitively.
    Containment {
        /// Restrict to one root, by canonical path.
        #[arg(long)]
        root: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Where an item can be named from, following re-exports to their origin.
    ReExports {
        /// Restrict to one access path.
        #[arg(long)]
        access_path: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    /// Each closure, its edge count and its deepest chain -- so an empty answer is not silence.
    Closures,
    /// What changed between two snapshots, joined on `entity_key` and nothing else.
    ///
    /// `ENTITY_KEY` is optional. Given one, this answers about a single entity; omitted, it
    /// reports every difference. The omitted form is what the acceptance criterion needs -- "these
    /// differences and nothing else" is not a statement a single-key query can make.
    Compare {
        /// Restrict to one entity. Omit to see every difference.
        entity_key: Option<String>,
        /// The snapshot to compare from.
        #[arg(long = "from")]
        from: String,
        /// The snapshot to compare to.
        #[arg(long = "to")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },
    /// Every extraction run, finished or not.
    Runs {
        /// Also show the interpretive scope: which skill was read, and how much of it.
        #[arg(long)]
        detail: bool,
    },
    /// What a command's exit status means, and the trap in reading it across tools.
    ResultContracts,
    /// What each tool can be pointed at, by name.
    Domains {
        /// `file_type` or `language`.
        #[arg(long)]
        kind: Option<String>,
    },
    /// Evaluate every interaction against the bound facets.
    Interactions {
        /// Only interactions affecting this mechanism.
        #[arg(long)]
        affecting: Option<String>,
        /// Only those that do not evaluate to `false`.
        #[arg(long)]
        relevant: bool,
    },
    /// The composable steps, with what each accepts, produces and owes.
    Fragments,
    /// Every chain from `--start-unit`, bounded by `--max-depth`, with recall folded.
    Chains {
        /// Only chains whose recall is at least this strong.
        #[arg(long)]
        min_recall: Option<String>,
    },
    /// The retrieval surface itself: every registered projection, from `information_schema`.
    Projections,
    /// Emit `bridge/CATALOG.md`: the inventory of everything added to the engine.
    CatalogMd,
}

/// Register the caller's request as a one-row relation.
///
/// §7.1 evaluates atoms `CROSS JOIN request`, and that is not merely a stylistic choice: the
/// caller's values reach the plan inside an Arrow array rather than anywhere near the SQL text,
/// and `atom_truth`'s six mixed-kind arguments give the planner nothing to infer a placeholder's
/// type from. A one-row table states the type by construction.
fn register_request(
    ctx: &SessionContext,
    binds: &[String],
    start_unit: &str,
    max_depth: i64,
    include_partial: bool,
    snapshot: &str,
    from_snapshot: &str,
    to_snapshot: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if max_depth < 1 {
        return Err("--max-depth must be at least 1; a chain of no steps is not a chain".into());
    }
    for bind in binds {
        if !bind.contains('=') {
            return Err(format!(
                "`{bind}` is not a binding. Write `--bind facet=value`; a facet left unbound \
                 evaluates to `unknown`, which is a legitimate request rather than an error."
            )
            .into());
        }
    }
    let bindings = binds.join(";");
    let schema = Arc::new(arrow_schema::Schema::new(vec![
        arrow_schema::Field::new("bindings", arrow_schema::DataType::Utf8, false),
        arrow_schema::Field::new("start_unit", arrow_schema::DataType::Utf8, false),
        arrow_schema::Field::new("max_depth", arrow_schema::DataType::Int64, false),
        // Reaches `projection.visible_run` the same way the other two reach their views: through
        // the relation, never through assembled SQL.
        arrow_schema::Field::new("include_partial", arrow_schema::DataType::Boolean, false),
        // Reaches `projection.selected_snapshot` the same way. Empty means "the newest", which
        // that view resolves -- a sentinel rather than a NULL because a one-row relation with a
        // null column gives the planner nothing to infer a type from, which is the same reason
        // this relation exists at all.
        arrow_schema::Field::new("snapshot", arrow_schema::DataType::Utf8, false),
        // `compare`'s two endpoints. They ride here rather than binding as `$n` placeholders for a
        // structural reason, not a stylistic one: `compare` is a registered view (§7.4), and a
        // view cannot reference a placeholder -- it is planned once, not once per caller.
        arrow_schema::Field::new("from_snapshot", arrow_schema::DataType::Utf8, false),
        arrow_schema::Field::new("to_snapshot", arrow_schema::DataType::Utf8, false),
    ]));
    let batch = arrow::array::RecordBatch::try_new(
        Arc::clone(&schema),
        vec![
            Arc::new(arrow::array::StringArray::from(vec![bindings])),
            Arc::new(arrow::array::StringArray::from(vec![
                start_unit.to_string(),
            ])),
            Arc::new(arrow::array::Int64Array::from(vec![max_depth])),
            Arc::new(arrow::array::BooleanArray::from(vec![include_partial])),
            Arc::new(arrow::array::StringArray::from(vec![snapshot.to_string()])),
            Arc::new(arrow::array::StringArray::from(vec![
                from_snapshot.to_string(),
            ])),
            Arc::new(arrow::array::StringArray::from(vec![
                to_snapshot.to_string(),
            ])),
        ],
    )?;
    let table = datafusion::datasource::MemTable::try_new(schema, vec![vec![batch]])?;
    ctx.register_table("request", Arc::new(table))?;
    Ok(())
}

/// Opens a canonical table from the Delta log, the first time a query names it.
///
/// Registration used to open all twenty up front -- 63 ms of metadata reads for a query that scans
/// four. This resolves one on demand instead; see `codesearch_bridge::lazy` for the measurement.
#[derive(Debug)]
struct DeltaTables {
    root: PathBuf,
    /// `<sql name>__all` -> canonical table name.
    names: BTreeMap<String, &'static str>,
}

impl DeltaTables {
    fn new(home: &Path) -> Self {
        Self {
            root: home.join("catalog"),
            names: TABLES
                .iter()
                .map(|(canonical, sql)| {
                    (
                        format!("{sql}{}", codesearch_bridge::projection::RAW_SUFFIX),
                        *canonical,
                    )
                })
                .collect(),
        }
    }
}

#[async_trait::async_trait]
impl codesearch_bridge::lazy::Resolver for DeltaTables {
    fn names(&self) -> Vec<String> {
        self.names.keys().cloned().collect()
    }

    async fn resolve(
        &self,
        name: &str,
    ) -> datafusion::common::Result<Option<Arc<dyn datafusion::catalog::TableProvider>>> {
        use datafusion::error::DataFusionError;

        let Some(table_name) = self.names.get(name) else {
            return Ok(None);
        };
        let location = self.root.join(table_name.replace('.', "__"));
        if !location.exists() {
            return Err(DataFusionError::Plan(format!(
                "no table at {} -- run `codesearch-build` first",
                location.display()
            )));
        }
        let url = url::Url::from_directory_path(&location).map_err(|_| {
            DataFusionError::Plan(format!("not an absolute path: {}", location.display()))
        })?;
        // The log store alone, and deliberately NOT `DeltaTableBuilder::load()`.
        //
        // `load()` runs `EagerSnapshot::try_new`, whose second phase replays every Add action in
        // the log into Arrow and parses its statistics. A provider needs the protocol, the metadata
        // and the schema, which is the FIRST phase -- and given a log store and no snapshot,
        // `TableProviderBuilder::build` does exactly that. delta-rs's own
        // `DeltaTable::table_provider()` takes this path too.
        //
        // The replay still happens, once, for the tables a query actually scans. Measured across
        // three runs of `just bench 20`: whole-process p95 fell from ~322 ms to ~255 ms at fourteen
        // tables, and the tail tightened more than the median moved.
        let log_store = DeltaTableBuilder::from_url(url)
            .map_err(|e| DataFusionError::External(Box::new(e)))?
            .build_storage()
            .map_err(|e| DataFusionError::External(Box::new(e)))?;
        let scan = TableProviderBuilder::default()
            .with_log_store(log_store)
            .build()
            .await
            .map_err(|e| DataFusionError::External(Box::new(e)))?;

        // Column 0 is `entity_id` on every canonical table. Declaring it eliminates an aggregation
        // from `SELECT DISTINCT entity_id` and turns a self-join into a semi-join -- measured, not
        // assumed. `catalog.lattice` is a reference table with no such column, so it declares
        // nothing rather than declaring something false.
        let key_columns = if *table_name == "catalog.lattice" {
            Vec::new()
        } else {
            vec![0]
        };
        Ok(Some(Arc::new(CanonicalTable::new(
            Arc::new(scan),
            key_columns,
        ))))
    }
}

fn render(batches: &[RecordBatch], json: bool) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        // The JSON path never constructs a terminal renderer. Rich's markup parser silently
        // deletes bracketed text -- `[a-z]`, and every ast-grep rule field name -- so machine
        // output must not pass through one.
        let mut writer = arrow::json::ArrayWriter::new(Vec::new());
        for batch in batches {
            writer.write(batch)?;
        }
        writer.finish()?;
        let bytes = writer.into_inner();
        println!("{}", String::from_utf8(bytes)?);
    } else {
        println!("{}", arrow::util::pretty::pretty_format_batches(batches)?);
    }
    Ok(())
}

/// A query: SQL with `$n` placeholders, and the values to bind to them.
#[derive(Debug)]
struct Query {
    sql: String,
    params: Vec<ScalarValue>,
}

impl Query {
    fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            params: Vec::new(),
        }
    }

    fn bind(&mut self, value: impl Into<String>) -> String {
        self.params.push(ScalarValue::Utf8(Some(value.into())));
        format!("${}", self.params.len())
    }

    fn bind_rank(&mut self, rank: i8) -> String {
        self.params.push(ScalarValue::Int8(Some(rank)));
        format!("${}", self.params.len())
    }
}

fn build_query(command: &Command) -> Result<Query, Box<dyn std::error::Error>> {
    Ok(match command {
        Command::Discover {
            subject,
            relationship,
            result,
            semantic_layer,
            scope,
            language,
            budget,
            tool,
            surface,
            capability,
            min_observed,
            limit,
        } => {
            // The view is chosen by whether an ability was asked for, because the two questions
            // have different shapes: `discover` joins through the binding relation, so a mechanism
            // the ability does not claim cannot appear. Filtering `mechanism_detail` afterwards
            // would leave unbound rows in the result.
            let mut q = Query::new(String::new());
            let mut predicates = Vec::new();
            // The four retrieval facets live on the capability, so asking for one means joining
            // through the binding -- which is what `projection.discover` is. Naming a facet
            // therefore selects the same view an ability does, and for the same reason: a
            // mechanism no capability claims cannot answer a question about capabilities.
            let facets = [
                ("facet_subject", subject),
                ("facet_relationship", relationship),
                ("facet_result", result),
                ("facet_semantic_layer", semantic_layer),
            ];
            let wants_capability = capability.is_some() || facets.iter().any(|(_, v)| v.is_some());
            let view = if wants_capability {
                if let Some(ability) = capability {
                    let p = q.bind(ability.clone());
                    predicates.push(format!("cand.ability = {p}"));
                }
                for (column, value) in facets {
                    if let Some(v) = value {
                        let p = q.bind(v.clone());
                        predicates.push(format!("{column} = {p}"));
                    }
                }
                "projection.discover"
            } else {
                "projection.mechanism_detail"
            };

            // `--scope` and `--language` are not facets of a capability; they are what a TOOL can
            // be pointed at, which `catalog.search_domain` records per tool. An `EXISTS` rather
            // than a join, so a tool supporting a language twice does not duplicate its
            // mechanisms.
            for (kind, value) in [("file_type", scope), ("language", language)] {
                if let Some(v) = value {
                    let p = q.bind(v.clone());
                    // `cand.tool`, QUALIFIED. Written as bare `tool` the correlated reference
                    // resolves to `sd.tool` inside the subquery -- `sd.tool = sd.tool` -- so the
                    // predicate is always true and every tool matches every language. It returned
                    // pcre2 mechanisms for `--language rust`, which is a wrong answer rather than
                    // an error, and is why this is qualified and tested.
                    predicates.push(format!(
                        "EXISTS (SELECT 1 FROM search_domain sd WHERE sd.tool = cand.tool \
                         AND sd.retracted = false AND sd.domain_kind = '{kind}' AND sd.name = {p})"
                    ));
                }
            }
            if let Some(t) = tool {
                let p = q.bind(t.clone());
                predicates.push(format!("cand.tool = {p}"));
            }
            if let Some(s) = surface {
                let p = q.bind(s.clone());
                predicates.push(format!("cand.surface = {p}"));
            }
            if let Some(label) = min_observed {
                let rank = codesearch_bridge::lattice::rank_of("observed", label)
                    .ok_or_else(|| format!("`{label}` is not a value of the observed lattice"))?;
                // A plain integer comparison on the stored rank. No function wraps the column, so
                // this prunes on Delta min/max statistics.
                let p = q.bind_rank(rank);
                predicates.push(format!("cand.observed_rank >= {p}"));
            }
            let where_clause = if predicates.is_empty() {
                String::new()
            } else {
                format!("WHERE {}", predicates.join(" AND "))
            };
            // The budget is a LIMIT on the candidate CTE, and `omitted` is a window total over
            // the same CTE -- so the number of rows shown and the number left out are computed
            // once and cannot disagree. Counting in a second query is exactly how they would.
            let budget = budget.unwrap_or(*limit);
            q.sql = format!(
                "WITH candidate AS (SELECT cand.entity_key, cand.tool, cand.surface, \
                     cand.observed, cand.precision, cand.parameter, cand.abilities, cand.summary \
                     FROM {view} cand {where_clause}), \
                 counted AS (SELECT c.*, count(*) OVER () AS matched FROM candidate c) \
                 SELECT entity_key, tool, surface, observed, precision, parameter, abilities, \
                        summary, matched - least(matched, {budget}) AS omitted \
                 FROM counted ORDER BY entity_key LIMIT {budget}"
            );
            q
        }
        Command::Describe { entity_key, field } => {
            const COLUMNS: &[&str] = &[
                "entity_key",
                "tool",
                "surface",
                "natural_key",
                "invocation_form",
                "observed",
                "precision",
                "parameter",
                "parameter_domain",
                "parameter_required",
                "abilities",
                "native_schema_ref",
                "summary",
            ];
            // `--field` is checked against the known set rather than interpolated. A caller value
            // never reaches the SQL text -- the test that feeds `' OR 1=1 --` through `describe`
            // asserts exactly that -- and an unknown field is an error naming what is available
            // rather than a column that silently does not appear.
            let selected: Vec<&str> = if field.is_empty() {
                COLUMNS.to_vec()
            } else {
                let mut out = Vec::new();
                for f in field {
                    match COLUMNS.iter().find(|c| *c == f) {
                        Some(c) => out.push(*c),
                        None => {
                            return Err(format!(
                                "`{f}` is not a field of `describe`. Available: {}",
                                COLUMNS.join(", ")
                            )
                            .into());
                        }
                    }
                }
                out
            };
            let mut q = Query::new(String::new());
            // One query for every key, because asking about three mechanisms is one question.
            let placeholders: Vec<String> = entity_key.iter().map(|k| q.bind(k.clone())).collect();
            q.sql = format!(
                "SELECT {} FROM projection.mechanism_detail WHERE entity_key IN ({}) \
                 ORDER BY entity_key",
                selected.join(", "),
                placeholders.join(", ")
            );
            q
        }
        Command::Capabilities => Query::new(
            "SELECT ability, facet_subject, facet_relationship, facet_result, \
             facet_semantic_layer, mechanism_count, summary \
             FROM projection.capability ORDER BY ability",
        ),
        Command::NegativeSpace { tool } => {
            let mut q = Query::new(String::new());
            let where_clause = match tool {
                Some(t) => {
                    let p = q.bind(t.clone());
                    format!("WHERE tool = {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT tool, capability_text, reachable, why, instead_text, instead_key \
                 FROM projection.negative_space {where_clause}ORDER BY tool, capability_text"
            );
            q
        }
        Command::Interactions {
            affecting,
            relevant,
        } => {
            let mut q = Query::new(String::new());
            let mut predicates = Vec::new();
            if let Some(key) = affecting {
                let p = q.bind(key.clone());
                predicates.push(format!("affected_key = {p}"));
            }
            if *relevant {
                // `unknown` is kept. An interaction whose facet the request did not bind is
                // exactly what the caller most needs to see: a choice they have not made yet.
                let rank = codesearch_bridge::lattice::rank_of("truth", "false")
                    .ok_or("the truth lattice has `false`")?;
                let p = q.bind_rank(rank);
                predicates.push(format!("truth_rank > {p}"));
            }
            let where_clause = if predicates.is_empty() {
                String::new()
            } else {
                format!("WHERE {} ", predicates.join(" AND "))
            };
            q.sql = format!(
                "SELECT kind, affected_key, holds, ordering_requirement, effect \
                 FROM projection.interaction_truth {where_clause}\
                 ORDER BY truth_rank DESC, affected_key"
            );
            q
        }
        Command::Fragments => Query::new(
            "SELECT name, accepted_input, produced_output, recall, scope_assumption, \
             coordinate_preservation, unresolved_obligations \
             FROM projection.plan_fragment ORDER BY accepted_input, name",
        ),
        Command::Chains { min_recall } => {
            let mut q = Query::new(String::new());
            let where_clause = match min_recall {
                Some(label) => {
                    let r = codesearch_bridge::lattice::rank_of("recall", label)
                        .ok_or_else(|| format!("`{label}` is not a value of the recall lattice"))?;
                    let p = q.bind_rank(r);
                    format!("WHERE recall_rank >= {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT depth, recall, produces, path FROM projection.chain {where_clause}\
                 ORDER BY depth, path"
            );
            q
        }
        // The default is the ledger: which runs exist, whether they finished, and where one
        // stopped. `--detail` adds the scope, which is a long absolute path and squeezes every
        // other column out of a terminal -- the renderer folds rather than truncates, on purpose,
        // so the fix is to ask for fewer columns rather than to lose characters from one.
        Command::Implements => Query::new(
            "SELECT mechanism_key, role, object_kind, object_key, precision, observed, \
                    evidence_note \
             FROM projection.implements",
        ),
        Command::Containment { root, limit } => {
            let mut q = Query::new(String::new());
            let where_clause = match root {
                Some(r) => {
                    let p = q.bind(r.clone());
                    format!("WHERE root_path = {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT root_path, member_path, depth, path \
                 FROM projection.containment {where_clause}\
                 ORDER BY depth, root_path, member_path LIMIT {limit}"
            );
            q
        }
        Command::ReExports { access_path, limit } => {
            let mut q = Query::new(String::new());
            let where_clause = match access_path {
                Some(a) => {
                    let p = q.bind(a.clone());
                    format!("WHERE access_path = {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT access_path, canonical_path, depth, path \
                 FROM projection.re_export_chain {where_clause}\
                 ORDER BY depth DESC, access_path LIMIT {limit}"
            );
            q
        }
        Command::Closures => Query::new(
            "SELECT closure, walks, nodes, edges, max_depth, limited_by FROM projection.closure",
        ),
        Command::Compare {
            entity_key,
            from,
            to,
            limit,
        } => {
            if from == to {
                // Not an error: comparing a snapshot with itself is the control a caller runs to
                // check that `compare` reports nothing when nothing changed, and refusing it would
                // remove the only cheap way to establish that.
                eprintln!("note: --from and --to name the same snapshot; expect no differences");
            }
            let mut q = Query::new(String::new());
            let where_clause = match entity_key {
                Some(k) => {
                    let p = q.bind(k.clone());
                    format!("WHERE entity_key = {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT source_table, entity_key, kind, before_id, after_id \
                 FROM projection.compare {where_clause}\
                 ORDER BY source_table, entity_key LIMIT {limit}"
            );
            q
        }
        // `selected` is why this command is the answer to "why is this empty?" in a catalog with
        // more than one snapshot. A default that picks one of two and does not say so is exactly
        // the silent choice the rest of this design spends its budget avoiding.
        Command::Runs { detail } => Query::new(if *detail {
            "SELECT r.run_id, r.snapshot_id, s.snapshot_id IS NOT NULL AS selected, r.family, \
                    r.completion, r.started_at, r.finished_at, r.failed_pass, \
                    r.index_files, r.index_rows, r.skill_root \
             FROM projection.extraction_run r \
             LEFT JOIN projection.selected_snapshot s ON s.snapshot_id = r.snapshot_id \
             ORDER BY r.started_at DESC"
        } else {
            "SELECT r.run_id, r.snapshot_id, s.snapshot_id IS NOT NULL AS selected, r.family, \
                    r.completion, r.started_at, r.finished_at, r.failed_pass \
             FROM projection.extraction_run r \
             LEFT JOIN projection.selected_snapshot s ON s.snapshot_id = r.snapshot_id \
             ORDER BY r.started_at DESC"
        }),
        Command::ResultContracts => Query::new(
            "SELECT tool, command, code, meaning, trap \
             FROM projection.result_contract ORDER BY tool, command, code",
        ),
        Command::Domains { kind } => {
            let mut q = Query::new(String::new());
            let where_clause = match kind {
                Some(k) => {
                    let p = q.bind(k.clone());
                    format!("WHERE domain_kind = {p} ")
                }
                None => String::new(),
            };
            q.sql = format!(
                "SELECT tool, domain_kind, name, supported, selector, grammar_kind_count, \
                 rule_schema_available FROM projection.search_domain {where_clause}\
                 ORDER BY tool, domain_kind, name"
            );
            q
        }
        Command::Coverage { dimension, tool } => match dimension.as_str() {
            "surface" => {
                let mut q = Query::new(String::new());
                let filter = match tool {
                    Some(t) => format!("WHERE tool = {} ", q.bind(t.clone())),
                    None => String::new(),
                };
                q.sql = format!(
                    "SELECT tool, source_table, surface_entries, characterised, uncharacterised, \
                     characterised_as \
                     FROM projection.coverage_surface {filter}ORDER BY tool, source_table"
                );
                q
            }
            "evidence" => {
                let mut q = Query::new(String::new());
                let filter = match tool {
                    Some(t) => format!("WHERE tool = {} ", q.bind(t.clone())),
                    None => String::new(),
                };
                q.sql = format!(
                    "SELECT tool, surface, mechanisms, confirmed, weakest \
                     FROM projection.coverage_evidence {filter}ORDER BY tool, surface"
                );
                q
            }
            "api" => Query::new(
                "SELECT object_kind, program_rows, mechanisms_bound \
                 FROM projection.coverage_api ORDER BY object_kind",
            ),
            other => {
                return Err(format!(
                    "`{other}` is not a coverage dimension. The three are `surface` (how much of \
                     the tool is catalogued), `evidence` (how much of that was actually run) and \
                     `api` (how much of the tool's own code a mechanism can point at). They are \
                     reported separately and never combined into a score."
                )
                .into());
            }
        },
        Command::Validate { detail, invariant } => {
            let mut q = Query::new(String::new());
            let filter = if invariant.is_empty() {
                String::new()
            } else {
                let placeholders: Vec<String> =
                    invariant.iter().map(|i| q.bind(i.clone())).collect();
                format!("WHERE invariant IN ({}) ", placeholders.join(", "))
            };
            q.sql = if *detail {
                format!(
                    "SELECT invariant, subject, detail FROM projection.violations {filter}\
                     ORDER BY invariant, subject"
                )
            } else {
                // `checked` stays in the narrowed output: an invariant that inspected nothing must
                // not read as clean just because a caller asked for it by name.
                format!(
                    "SELECT invariant, checked, violations FROM projection.validate {filter}\
                     ORDER BY violations DESC, invariant"
                )
            };
            q
        }
        // Reading the surface out of `information_schema` rather than out of the Rust constant is
        // the point of registering views at all: what is listed is what the session actually has.
        Command::Projections | Command::CatalogMd => Query::new(
            "SELECT table_name FROM information_schema.views \
             WHERE table_schema = 'projection' ORDER BY table_name",
        ),
    })
}

/// Render `bridge/CATALOG.md` from the live session.
///
/// The UDF rows come from each function's own `documentation()` and the view rows from
/// `information_schema`, so neither can drift from the code: a function or a projection that is
/// not actually on the session cannot appear here, and one that is cannot be left out. The two
/// sides are then cross-checked, because a view registered without a `PROJECTIONS` entry would
/// otherwise publish with an empty description rather than as a problem.
fn catalog_md(registered: &[String]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    out.push_str(
        "# `bridge` -- everything added to the engine\n\n\
         **Generated** by `just bridge-catalog` against a live session. Do not edit: fix the code \
         and regenerate.\n\n\
         This file exists so that \"what have we added to DataFusion?\" has an answer that cannot \
         go stale. Every row below was read out of a running session rather than transcribed.\n\n\
         ## Scalar functions\n\n\
         All are `Immutable`. PB06 measured why that is not a detail: a UDF that is not immutable \
         loses its filter entirely, with no diagnostic.\n\n\
         | function | arguments | what it is for |\n|---|---|---|\n",
    );
    for udf in codesearch_bridge::session::udfs() {
        let doc = udf.documentation();
        let arguments = doc
            .and_then(|d| d.arguments.as_ref())
            .map(|args| {
                args.iter()
                    .map(|(name, _)| format!("`{name}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        let description = doc
            .map(|d| d.description.replace('\n', " "))
            .unwrap_or_else(|| "UNDOCUMENTED".to_string());
        let syntax = doc.map(|d| d.syntax_example.clone()).unwrap_or_default();
        let _ = writeln!(out, "| `{syntax}` | {arguments} | {description} |");
    }

    let _ = write!(
        out,
        "\n## Projections\n\n\
         The retrieval surface, as registered views. A caller supplies predicates; the join \
         discipline lives in `bridge/src/projection.rs` and is written once.\n\n\
         | view | what it answers |\n|---|---|\n"
    );
    for name in registered {
        let about = codesearch_bridge::projection::PROJECTIONS
            .iter()
            .find(|p| p.name == *name)
            .map(|p| p.about.to_string())
            .unwrap_or_else(|| {
                "**registered but not declared in PROJECTIONS** -- fix this rather than \
                 documenting it"
                    .to_string()
            });
        let _ = writeln!(out, "| `projection.{name}` | {about} |");
    }
    let missing: Vec<&str> = codesearch_bridge::projection::PROJECTIONS
        .iter()
        .map(|p| p.name)
        .filter(|n| !registered.iter().any(|r| r == n))
        .collect();
    if !missing.is_empty() {
        let _ = write!(
            out,
            "\n**Declared but not registered:** {}. That is a bug, not a note.\n",
            missing.join(", ")
        );
    }

    let _ = write!(
        out,
        "\n## Provider wrapper\n\n\
         `CanonicalTable` declares `constraints()` -- the primary key on `entity_id` -- and \
         deliberately does **not** implement `statistics()`. PB02 measured the optimiser acting \
         on the declaration: the `Aggregate` disappears from `SELECT DISTINCT entity_id` and a \
         self-join becomes a `LeftSemi Join`. PB02b measured delta-rs already supplying accurate \
         statistics at the `ExecutionPlan` level, so adding them here would duplicate a working \
         channel.\n\n\
         ## Session\n\n\
         One session, built from delta-rs's `create_session()` and never from \
         `SessionContext::new()` -- PB07 measured that a bare DataFusion session cannot execute a \
         Delta write at all. Every delta-rs builder is given \
         `SessionFallbackPolicy::RequireSessionState`, because the default silently discards the \
         caller's session and everything registered on it (PB05).\n"
    );
    out
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (ctx, _session) = codesearch_bridge::session::build_session()?;
    // `install` REPLACES the default schema, so anything registered into the old one is discarded.
    // The request relation therefore goes in afterwards, not before.
    // `compare`'s endpoints reach its view through the request relation, so they are read from
    // the subcommand before the relation is built rather than bound per query.
    let (from_snapshot, to_snapshot) = match &args.command {
        Command::Compare { from, to, .. } => (from.clone(), to.clone()),
        _ => (String::new(), String::new()),
    };
    let base: Vec<&str> = TABLES.iter().map(|(_, sql)| *sql).collect();
    codesearch_bridge::projection::install(&ctx, &base, Arc::new(DeltaTables::new(&args.home)))?;
    register_request(
        &ctx,
        &args.binds,
        &args.start_unit,
        args.max_depth,
        args.include_partial,
        &args.snapshot,
        &from_snapshot,
        &to_snapshot,
    )?;

    let query = build_query(&args.command)?;
    let frame = ctx.sql(&query.sql).await?;
    let frame = if query.params.is_empty() {
        frame
    } else {
        frame.with_param_values(query.params)?
    };
    let batches = frame.collect().await?;

    if matches!(args.command, Command::CatalogMd) {
        let mut names = Vec::new();
        for b in &batches {
            let column = b
                .column(0)
                .as_any()
                .downcast_ref::<arrow::array::StringArray>()
                .ok_or("information_schema.views.table_name is not a string")?;
            for i in 0..b.num_rows() {
                names.push(column.value(i).to_string());
            }
        }
        print!("{}", catalog_md(&names));
        return Ok(());
    }

    render(&batches, args.json)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No caller-supplied value may reach the SQL text. This is the property that makes the
    /// placeholder discipline worth having, so it is asserted rather than trusted.
    /// `--language` and `--scope` correlate on the OUTER row, not on the subquery's own.
    ///
    /// Written as a bare `tool`, the reference resolves inside the subquery to `sd.tool`, so the
    /// predicate is `sd.tool = sd.tool` -- always true, every tool matches every language, and
    /// `--language rust` returns pcre2 mechanisms. That is a wrong answer rather than an error,
    /// and the only thing that catches it is asserting on the correlation itself.
    #[test]
    fn a_domain_filter_correlates_on_the_candidate_row() {
        for (flag, kind) in [("language", "language"), ("scope", "file_type")] {
            let mut command = Command::Discover {
                subject: None,
                relationship: None,
                result: None,
                semantic_layer: None,
                scope: None,
                language: None,
                budget: None,
                tool: None,
                surface: None,
                capability: None,
                min_observed: None,
                limit: 20,
            };
            if let Command::Discover {
                scope, language, ..
            } = &mut command
            {
                if flag == "language" {
                    *language = Some("rust".into());
                } else {
                    *scope = Some("rust".into());
                }
            }
            let q = build_query(&command).expect("plans");
            assert!(
                q.sql.contains("sd.tool = cand.tool"),
                "--{flag} must correlate on the candidate row, not on the subquery's own: {}",
                q.sql
            );
            assert!(
                q.sql.contains(&format!("sd.domain_kind = '{kind}'")),
                "--{flag} must ask about {kind} domains"
            );
        }
    }

    #[test]
    fn caller_values_are_bound_and_never_interpolated() {
        let hostile = "' OR 1=1 --";
        let q = build_query(&Command::Describe {
            entity_key: vec![hostile.to_string()],
            field: Vec::new(),
        })
        .expect("describe plans");
        assert!(
            !q.sql.contains(hostile),
            "the caller's string reached the SQL text: {}",
            q.sql
        );
        assert_eq!(q.params.len(), 1);
        assert_eq!(q.params[0], ScalarValue::Utf8(Some(hostile.to_string())));
    }

    /// Each supplied option adds exactly one placeholder, and an omitted one adds no predicate.
    /// Which predicates appear is structural; what they compare against is bound.
    #[test]
    fn discover_binds_one_placeholder_per_supplied_option() {
        let none = build_query(&Command::Discover {
            subject: None,
            relationship: None,
            result: None,
            semantic_layer: None,
            scope: None,
            language: None,
            budget: None,
            tool: None,
            surface: None,
            capability: None,
            min_observed: None,
            limit: 5,
        })
        .expect("plans");
        assert!(none.params.is_empty());
        assert!(!none.sql.contains("WHERE"), "{}", none.sql);
        assert!(none.sql.contains("projection.mechanism_detail"));

        let all = build_query(&Command::Discover {
            subject: None,
            relationship: None,
            result: None,
            semantic_layer: None,
            scope: None,
            language: None,
            budget: None,
            tool: Some("rg".into()),
            surface: Some("cli".into()),
            capability: Some("search-literal-text".into()),
            min_observed: Some("recorded".into()),
            limit: 5,
        })
        .expect("plans");
        assert_eq!(all.params.len(), 4);
        // An ability was asked for, so the join through the binding relation is what runs.
        assert!(all.sql.contains("projection.discover"), "{}", all.sql);
    }

    /// A lattice label that names no value must be refused, not silently dropped: a filter that
    /// quietly disappears returns more rows than asked for, which is the wrong direction to fail.
    #[test]
    fn an_unknown_lattice_label_is_refused() {
        let err = build_query(&Command::Discover {
            subject: None,
            relationship: None,
            result: None,
            semantic_layer: None,
            scope: None,
            language: None,
            budget: None,
            tool: None,
            surface: None,
            capability: None,
            min_observed: Some("definitely-not-a-value".into()),
            limit: 5,
        })
        .expect_err("an unknown label must be an error");
        assert!(err.to_string().contains("observed lattice"));
    }

    /// Coverage names a dimension, and an unrecognised one is an error rather than a default.
    /// Quietly answering the surface question when the evidence question was asked would be the
    /// composite-score failure by another route.
    #[test]
    fn coverage_refuses_a_dimension_it_does_not_have() {
        for good in ["surface", "evidence", "api"] {
            assert!(
                build_query(&Command::Coverage {
                    tool: None,
                    dimension: good.into()
                })
                .is_ok()
            );
        }
        assert!(
            build_query(&Command::Coverage {
                tool: None,
                dimension: "score".into()
            })
            .is_err()
        );
    }
}
