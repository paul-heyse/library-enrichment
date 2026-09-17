# Evidence 01 — DataFusion: catalog, session and provider framework

**Source:** the `datafusion` capability repository (60 crates, pinned). Every path below was read
from `content/index/symbols.tsv`, `content/index/methods.tsv` or `content/model/*.json`.
**Gathered:** 2026-09-16. **Nothing here is recalled; every signature is quoted from the index.**

This dossier covers design goal **(2)**: use the full catalog/schema/table-provider and
planning framework so that the codebase model is one integrated structure rather than a pile of
discrete methods.

---

## 1. The three-level catalog hierarchy

```
datafusion_session::catalog::CatalogProviderList   (trait)  — named catalogs
datafusion_session::catalog::CatalogProvider       (trait)  — named schemas
datafusion_session::schema::SchemaProvider         (trait)  — named tables
datafusion_session::table::TableProvider           (trait)  — one relation
```

`CatalogProvider`:

```rust
fn schema_names(&self) -> Vec<String>
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
fn deregister_schema(&self, _name: &str, _cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
```

`SchemaProvider` — note `table` is **async**, and `table_type` is separately askable without
materialising the provider:

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
async fn table_type(&self, name: &str) -> Result<Option<TableType>>
fn table_names(&self) -> Vec<String>
fn table_exist(&self, name: &str) -> bool
fn owner_name(&self) -> Option<&str>
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Ready-made implementations exist and can be subclassed by composition rather than written from
scratch: `datafusion_catalog::memory::catalog::MemoryCatalogProvider`,
`datafusion_catalog::memory::schema::MemorySchemaProvider`,
`datafusion_catalog::memory::table::MemTable`, `datafusion_catalog::view::ViewTable`
("an implementation of `TableProvider` that uses another logical plan"),
`datafusion_catalog::streaming::StreamingTable`.

### Why this matters for the design

The proposal's canonical model is a set of typed record categories (`Definition`, `MirBody`,
`Evidence`, `BehaviorAssertion`, …). Those become **schemas inside one catalog**, not separate
stores:

```
catalog  codesearch
  schema  snapshot     Artifact, ExtractionRun, Entity, NativeBinding, SourceAnchor
  schema  program      Package … FlowFact  (the typed program records)
  schema  catalog      Capability, Mechanism, Contract, Interaction, PlanFragment
  schema  evidence     Evidence, Derivation, BehaviorAssertion
  schema  projection   views (see §5) — no storage of their own
```

`ViewTable` is the mechanism that makes `projection` free: a projection is a `LogicalPlan`
registered as a table, so it is planned and optimised with everything else rather than being a
function someone remembers to call.

---

## 2. `TableProvider` carries far more than `scan`

This is the single most consequential finding for goal **(4)** — make relational actions
programmatically relational instead of writing bespoke code around them.

```rust
fn schema(&self) -> SchemaRef
fn table_type(&self) -> TableType
fn constraints(&self) -> Option<&Constraints>
fn get_column_default(&self, _column: &str) -> Option<&Expr>
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn get_table_definition(&self) -> Option<&str>
fn statistics(&self) -> Option<Statistics>
fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>

async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>

async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn update(&self, _state: &dyn Session, _assignments: Vec<(String, Expr)>, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
async fn delete_from(&self, _state: &dyn Session, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>
async fn merge_into(&self, _state: &dyn Session, _source: Arc<dyn ExecutionPlan>, _merge_schema: DFSchemaRef, _on: Expr, _clauses: Vec<MergeIntoClause>) -> Result<Arc<dyn ExecutionPlan>>
async fn truncate(&self, _state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

**`merge_into` on the provider reshapes the merge procedure — but see the correction below.**
The proposal's Passes A–F (§9 of the proposal) are described as a procedural reconciliation. The
trait makes each pass expressible as a `MERGE INTO … WHEN MATCHED … WHEN NOT MATCHED …`
statement against a provider, planned and executed by DataFusion.

> **Corrected by evidence 05 §1.** delta-rs's `DeltaScan` implements only `scan`, `schema`,
> `table_type`, `supports_filters_pushdown`, `insert_into`, `get_logical_plan` and
> `get_table_definition`. `merge_into`, `update`, `delete_from`, `truncate`, `constraints`,
> `statistics`, `get_column_default` and `scan_with_args` fall back to trait defaults. Merges
> therefore go through `deltalake_core::operations::merge::MergeBuilder`, whose *source* is a
> DataFusion `DataFrame` and which takes the shared `SessionState` — so the work stays
> relational and inside the engine, but the seam is a builder rather than a DML plan node.
> The gap in `constraints()`/`statistics()` is the argument for the wrapping provider described
> in evidence 05 §1.

`InsertOp` = `Append | Overwrite | Replace`.
`TableProviderFilterPushDown` = `Unsupported | Inexact | Exact` — the three-valued answer is the
hook for the proposal's recall vocabulary (`exact_under_stated_preconditions` vs
`candidate_generation_or_approximation`); an `Inexact` pushdown is precisely a prefilter that
DataFusion will re-check above the scan.

`ScanArgs` is the forward-compatible form of `scan` and adds a statistics channel:

```rust
fn projection(&self) -> Option<&'a [usize]>
fn filters(&self) -> Option<&'a [Expr]>
fn limit(&self) -> Option<usize>
fn statistics_requests(&self) -> &'a [StatisticsRequest]
fn with_projection(self, …) -> Self   // and with_filters / with_limit / with_statistics_requests
```

---

## 3. Constraints and functional dependencies are first-class

`datafusion_common::functional_dependencies`:

```rust
enum Constraint { PrimaryKey, Unique }          // "This object defines a constraint on a table."
enum Dependency { Single, Multi }               // "Describes functional dependency mode."

struct FunctionalDependence {
    fields: source_indices, target_indices, nullable, mode
    fn new(source_indices: Vec<usize>, target_indices: Vec<usize>, nullable: bool) -> Self
    fn with_mode(self, mode: Dependency) -> Self
}

struct Constraints {
    fn new_unverified(constraints: Vec<Constraint>) -> Self
    fn project(&self, proj_indices: &[usize]) -> Option<Self>
    fn extend(&mut self, other: Constraints)
}
```

### Why this matters

Goal (1) asks for predicates and relational capabilities that remove discrete validation steps.
Declaring `Constraint::PrimaryKey` on `entity_id` and `Constraint::Unique` on
`(run_id, native_namespace, native_handle)` does two things at once: it states the invariant the
proposal's identity model depends on, **and** it feeds the optimizer — DataFusion uses
functional dependencies to eliminate redundant aggregation and distinct operations. A uniqueness
rule that is also an optimisation hint does not need a separate validation pass.

`FunctionalDependence` lets the design state, e.g., that `entity_id → (kind, owner_id, snapshot_id)`
so a `GROUP BY entity_id` need not re-group by the dependent columns.

**Caution recorded:** the constructor is named `new_unverified`. The index does not show a
verifying constructor, so these are declarations DataFusion trusts, not checks it performs.
Verification, where wanted, must still be a query (see the plan's validation-as-SQL section).

---

## 4. `SessionStateBuilder` is the whole extension surface

Every extension point the plan needs is a builder call on
`datafusion::execution::session_state::SessionStateBuilder`:

| Builder method | What it registers |
|---|---|
| `with_catalog_list` | the `CatalogProviderList` — the root of §1 |
| `with_table_factories`, `with_table_factory` | `TableProviderFactory` for `CREATE EXTERNAL TABLE` |
| `with_table_functions`, `with_table_function_list` | table-valued functions |
| `with_scalar_functions`, `with_aggregate_functions`, `with_window_functions` | UDF/UDAF/UDWF |
| `with_higher_order_functions` | `HigherOrderUDF` (lambda-taking functions) |
| `with_function_factory` | `CREATE FUNCTION` handling — UDFs defined in SQL at runtime |
| `with_expr_planners` | `ExprPlanner` — custom SQL expression syntax |
| `with_relation_planners` | `RelationPlanner` — custom SQL table factors |
| `with_type_planner` | `TypePlanner` — custom SQL type names → Arrow types |
| `with_extension_type_registry` | Arrow extension types with registered semantics |
| `with_analyzer_rule(s)` | `AnalyzerRule` — runs before optimisation |
| `with_optimizer_rule(s)` | `OptimizerRule` — logical rewrites |
| `with_physical_optimizer_rule(s)` | `PhysicalOptimizerRule` |
| `with_query_planner` | `QueryPlanner` / `ExtensionPlanner` for custom logical nodes |
| `with_statistics_registry` | `StatisticsProvider` chain |
| `with_serializer_registry` | plan serialisation for custom nodes |
| `with_object_store`, `with_runtime_env`, `with_config` | storage and execution environment |
| `with_file_formats`, `with_table_options`, `with_execution_props`, `with_cache_factory` | the rest |

**This list is the answer to goal (2).** There is no part of the pipeline that must live outside
the session: parsing, typing, planning, optimisation, statistics and execution all have a
registered-extension seam.

### `FunctionFactory`

```rust
async fn create(&self, state: &SessionState, statement: CreateFunction) -> Result<RegisterFunction>
```

"Interface for handling `CREATE FUNCTION` statements and interacting with [SessionState] to
create and register functions (`ScalarUDF`, `AggregateUDF`, `WindowUDF`, and `TableFunctionImpl`)
dynamically."

This is how the capability catalog can let a **mechanism definition itself** register a callable
function — a mechanism row that carries a predicate becomes an invocable UDF without a code
change.

### `ExprPlanner`, `RelationPlanner`, `TypePlanner`

```rust
// ExprPlanner — 13 provided methods, all overridable
fn plan_binary_op(&self, expr: RawBinaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>>
fn plan_field_access(&self, expr: RawFieldAccessExpr, _schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>>
fn plan_compound_identifier(&self, _field: &Field, _qualifier: Option<&TableReference>, _nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>>
fn plan_struct_literal(&self, args: Vec<Expr>, _is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>>
// … plan_aggregate, plan_array_literal, plan_dictionary_literal, plan_extract, plan_make_map,
//     plan_overlay, plan_position, plan_substring, plan_window

// RelationPlanner
fn plan_relation(&self, relation: TableFactor, context: &mut dyn RelationPlannerContext) -> Result<RelationPlanning>

// TypePlanner
fn plan_type(&self, _sql_type: &sqlparser::ast::DataType) -> Result<Option<DataType>>
fn plan_type_field(&self, sql_type: &sqlparser::ast::DataType) -> Result<Option<FieldRef>>
```

`plan_compound_identifier` and `plan_field_access` are the hooks that make deeply nested typed
records ergonomic in SQL — the design uses nested `Struct`/`List` heavily (see evidence 03), and
these let `definition.signature.generics[0].bounds` resolve the way a reader expects.

### Arrow extension types with registered behaviour

```rust
trait ExtensionTypeRegistry {
    fn add_extension_type_registration(&self, extension_type: ExtensionTypeRegistrationRef) -> Result<Option<ExtensionTypeRegistrationRef>>
    fn extension_type_registration(&self, name: &str) -> Result<ExtensionTypeRegistrationRef>
    fn extension_type_registrations(&self) -> Vec<ExtensionTypeRegistrationRef>
    fn remove_extension_type_registration(&self, name: &str) -> Result<Option<ExtensionTypeRegistrationRef>>
    fn create_extension_type_for_field(&self, field: &Field) -> Result<Option<DFExtensionTypeRef>>
    fn extend(&self, extension_types: &[ExtensionTypeRegistrationRef]) -> Result<()>
}

struct ExtensionTypeRegistration {
    fn new_arc(name: impl Into<String>, factory: impl Fn(&DataType, Option<&str>) -> Result<DFExtensionTypeRef> + Send + Sync + 'static) -> ExtensionTypeRegistrationRef
    fn create_df_extension_type(&self, storage_type: &DataType, metadata: Option<&str>) -> Result<DFExtensionTypeRef>
    fn type_name(&self) -> &str
}
```

"Implementations of this trait are responsible for *creating* instances of `DFExtensionType` that
represent the entire semantics of an extension type."

**This is the mechanism for goal (1)'s "typed data" at its strongest.** `EntityId`,
`ContentHash`, `SourceAnchor` and `MechanismId` become Arrow extension types over
`Utf8`/`FixedSizeBinary` storage with registered semantics, rather than bare strings with a
naming convention. The type travels in the schema, survives IPC, and is visible to the planner.

---

## 5. What this replaces in the proposal

| Proposal text | Framework mechanism that subsumes it |
|---|---|
| §9 "Pass A–F" reconciliation procedure | six `MergeBuilder` invocations whose source is a DataFrame over this catalog (evidence 05 §2) |
| §10 "Replacing convenience queries with transparent projections" | `ViewTable` registered in a `projection` schema |
| §8.1 "`NativeBinding` is a crosswalk" | a table with `Constraint::Unique` on `(run_id, native_namespace, native_handle)` |
| §11 "normalized relation tables for graph edges" | ordinary tables with declared functional dependencies |
| §2 "Mechanism contract" fields | one `Struct` column per contract facet, not a JSON blob (evidence 03) |
| §9 retrieval `discover/describe/compare` | SQL over registered views, exposed by the CLI (evidence 08) |

## 6. Open questions for round 2

1. `Constraints::new_unverified` is the only constructor the index shows. Confirm whether any
   DataFusion path verifies constraints, or whether all verification must be an explicit query.
2. ~~Whether `TableProvider::merge_into` is implemented by delta-rs~~ — **answered** in
   evidence 05 §1: it is not. Open follow-up: whether the recommended wrapping provider should
   also synthesise `merge_into` by delegating to `MergeBuilder`, so that callers see one seam.
3. ~~Whether a Delta provider can answer statistics from the transaction log cheaply.~~
   **ANSWERED — it already does** (probe PB02b, evidence 11), at the `ExecutionPlan` level
   rather than through `TableProvider::statistics()`. No wrapper needed for this.
