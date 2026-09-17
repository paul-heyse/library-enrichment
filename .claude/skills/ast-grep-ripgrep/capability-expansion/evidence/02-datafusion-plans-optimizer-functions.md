# Evidence 02 — DataFusion: logical plans, optimizer, expressions and the function surface

**Source:** the `datafusion` capability repository (60 crates, pinned), read 2026-09-16.
Covers design goals **(4)** make relational things programmatically relational, and **(5)** bridge
gaps with UDFs rather than stepping outside the framework.

---

## 1. `LogicalPlan` has a recursive-CTE node — the graph question is answered

```rust
enum LogicalPlan {
    Projection, Filter, Window, Aggregate, Sort, Join, Repartition, Union, TableScan,
    EmptyRelation, Subquery, SubqueryAlias, Limit, Statement, Values, Explain, Analyze,
    Extension, Distinct, Dml, Ddl, Copy, DescribeTable, Unnest, RecursiveQuery,
}

struct RecursiveQuery {
    fields: name, static_term, recursive_term, is_distinct, schema
    fn try_new(name: String, static_term: Arc<LogicalPlan>, recursive_term: Arc<LogicalPlan>, is_distinct: bool) -> Result<Self>
}
// "A variadic query operation, Recursive CTE."
```

**This is the single most important finding for the proposal's §11 claim that "graph semantics and
physical storage are separate decisions".** Every graph traversal the canonical model needs is a
recursive CTE over ordinary tables:

| Graph question in the proposal | Relational form |
|---|---|
| module / definition containment closure | `WITH RECURSIVE` over `Definition.owner_id` |
| call-graph reachability from a `CallSite` | `WITH RECURSIVE` over `CallSite → CallTarget → Definition` |
| dependency transitive closure (replaces Guppy, proposal §1 "What replaces Guppy") | `WITH RECURSIVE` over `DependencyEdge` |
| CFG reachability between `MirBlock`s | `WITH RECURSIVE` over `CfgEdge` |
| re-export / `ExportPath` chains | `WITH RECURSIVE` over alias edges |
| derivation provenance ("why is this fact here") | `WITH RECURSIVE` over `Derivation.input_fact_ids` |

`is_distinct` gives cycle-tolerant traversal (`UNION` vs `UNION ALL`) without a written visited-set.
No graph database, and no bespoke traversal code — which is exactly goal (4).

Other variants that remove hand-written steps: `Unnest` (explode a `List` column — the proposal's
"ordered child records for arguments, generic substitutions, projections, and syntax children"),
`Dml`/`Ddl`/`Copy` (writes are plans, so they are optimised and explainable like reads),
`Extension` (a custom logical node when nothing built-in fits — see §4).

---

## 2. `Expr` — 36 variants including lambdas

```rust
enum Expr {
    Alias, Column, ScalarVariable, Literal, BinaryExpr, Like, SimilarTo, Not,
    IsNotNull, IsNull, IsTrue, IsFalse, IsUnknown, IsNotTrue, IsNotFalse, IsNotUnknown,
    Negative, Between, Case, Cast, TryCast, ScalarFunction, AggregateFunction, WindowFunction,
    InList, Exists, InSubquery, SetComparison, ScalarSubquery, Wildcard, GroupingSet,
    Placeholder, OuterReferenceColumn, Unnest, HigherOrderFunction, Lambda, LambdaVariable,
}
```

`Lambda`, `LambdaVariable` and `HigherOrderFunction` mean predicates over nested `List<Struct>`
columns stay in SQL instead of forcing a normalising table. `Placeholder` is how the CLI's
prepared queries bind parameters without string interpolation.

`Expr` carries 87 methods. The ones that matter to the design:

```rust
fn alias_with_metadata(self, name: impl Into<String>, metadata: Option<FieldMetadata>) -> Expr
fn alias_qualified_with_metadata(self, relation: …, name: …, metadata: Option<FieldMetadata>) -> Expr
fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
fn column_refs(&self) -> HashSet<&Column>
fn contains_outer(&self) -> bool
fn apply_children<…>(&self, f: F) -> Result<TreeNodeRecursion>   // TreeNode API
```

**`alias_with_metadata` is the hook for goal (1)'s "metadata".** A projection can attach
provenance metadata (which extraction run, which precision) to the *output field*, so metadata
rides the schema through the plan instead of being carried in a parallel side-channel.

---

## 3. What the optimizer does for free

Built-in `OptimizerRule`s present in the index:

```
CommonSubexprEliminate  DecorrelateLateralJoin  DecorrelatePredicateSubquery
EliminateCrossJoin  EliminateDuplicatedExpr  EliminateFilter  EliminateGroupByConstant
EliminateJoin  EliminateLimit  EliminateOuterJoin  ExtractEquijoinPredicate
ExtractLeafExpressions  FilterNullJoinKeys  OptimizeProjections  OptimizeUnions
PropagateEmptyRelation  PullUpCorrelatedExpr  PushDownFilter  PushDownLeafProjections
PushDownLimit  ReplaceDistinctWithAggregate  RewriteSetComparison  ScalarSubqueryToJoin
SingleDistinctToGroupBy  UnionsToFilter
```

`AnalyzerRule`s (run before optimisation, may change types):

```
ApplyFunctionRewrites   ResolveGroupingFunction   TypeCoercion (+ TypeCoercionRewriter,
coerce_union_schema)
```

Consequences the plan relies on:

- `PushDownFilter` + `TableProvider::supports_filters_pushdown` is the whole prefilter story.
  A mechanism marked `Inexact` is re-checked above the scan **by the engine**, so a heuristic
  prefilter cannot silently become an exact answer (proposal §10 "Prefilter correctness").
- `DecorrelatePredicateSubquery` / `ScalarSubqueryToJoin` mean the decision-packet queries can be
  written as readable correlated subqueries and still execute as joins.
- `CommonSubexprEliminate` means a projection may name the same derived contract facet repeatedly
  without cost, so views can stay legible.

Custom rules attach via `SessionStateBuilder::with_analyzer_rule(s)`,
`with_optimizer_rule(s)`, `with_physical_optimizer_rule(s)`.

---

## 4. `ScalarUDFImpl` is a full optimizer participant, not a black box

This is the finding that satisfies goal **(5)**. A UDF is not an opaque call the planner steps
around — it has 24 methods, 20 of them provided/overridable:

```rust
// required
fn name(&self) -> &str
fn signature(&self) -> &Signature
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>

// optimizer participation
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
fn preimage(&self, _args: &[Expr], _lit_expr: &Expr, _info: &SimplifyContext) -> Result<PreimageResult>
fn evaluate_bounds(&self, _input: &[&Interval]) -> Result<Interval>
fn propagate_constraints(&self, _interval: &Interval, _inputs: &[&Interval]) -> Result<Option<Vec<Interval>>>
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
fn strictly_order_preserving(&self, _inputs: &[ExprProperties]) -> Result<bool>
fn short_circuits(&self) -> bool
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>

// typing and schema
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn coerce_types(&self, _arg_types: &[DataType]) -> Result<Vec<DataType>>
fn is_nullable(&self, _args: &[Expr], _schema: &dyn ExprSchema) -> bool
fn is_strict(&self) -> bool
fn struct_field_mapping(&self, _literal_args: &[Option<ScalarValue>]) -> Option<StructFieldMapping>

// presentation and config
fn aliases(&self) -> &[String]
fn display_name(&self, args: &[Expr]) -> Result<String>
fn schema_name(&self, args: &[Expr]) -> Result<String>
fn documentation(&self) -> Option<&Documentation>
fn placement(&self, _args: &[ExpressionPlacement]) -> ExpressionPlacement
fn with_updated_config(&self, _config: &ConfigOptions) -> Option<ScalarUDF>
```

**`preimage` is the decisive one.** It lets a UDF tell the optimizer how `f(col) = literal`
rewrites into a predicate on `col` — which means a domain UDF such as
`mechanism_supports(contract, request)` can still push a prunable predicate down to the Delta scan
rather than forcing a full read. `return_field_from_args` lets a UDF return a *field* (name,
nullability, **metadata**), not merely a type, so a UDF can stamp provenance onto its output.

`documentation()` returning `Documentation` matters for the CLI: UDFs are self-describing, so
`describe` (proposal §9) can list custom functions without a parallel registry.

Corresponding traits: `datafusion_expr::udaf::AggregateUDFImpl`,
`datafusion_expr::udwf::WindowUDFImpl`, `datafusion_session::table::TableFunctionImpl`,
plus `HigherOrderUDF` used by `Expr::HigherOrderFunction`.

Custom logical operators, where no function shape fits:
`datafusion_expr::logical_plan::extension::UserDefinedLogicalNode` +
`datafusion_session::planner::ExtensionPlanner` / `QueryPlanner`.

---

## 5. Built-in function families (what must *not* be re-implemented)

Counts are index rows under each module of `datafusion_functions` / `datafusion_functions_nested`.

| Family | Rows | Notable members |
|---|---:|---|
| `regex` | 23 | `regexp_like`, `regexp_match`, `regexp_replace`, `regexp_count`, `regexp_instr` |
| `string` | 69 | the byte-oriented string surface |
| `unicode` | 46 | the char-oriented surface |
| `core` | 72 | `coalesce`, `nullif`, `nvl`, casting, struct access |
| `datetime` | 60 | timestamp arithmetic for snapshot/run times |
| `map` | 3 | plus the nested `map_*` family below |

`datafusion_functions_nested` modules:

```
array_add array_any_match array_avg array_compact array_filter array_first array_has
array_normalize array_product array_scale array_subtract array_sum array_transform arrays_zip
cardinality concat cosine_distance dimension distance empty except extract flatten
inner_product length make_array map map_entries map_extract map_keys map_values min_max
position range remove repeat replace resize reverse set_ops sort string
all_default_higher_order_functions all_default_nested_functions
```

`array_filter`, `array_transform` and `array_any_match` are higher-order — they take a `Lambda`.
Combined with `Unnest` and `flatten`, nested child records (generic arguments, projections,
syntax children, `input_fact_ids`) can be queried in place. **This is what lets the schema stay
nested and typed rather than shredded into a dozen join tables**, which is goal (1)'s "minimising
repeats and discontinuities".

`set_ops` and `except` over arrays give set algebra on capability facets without a join.

## 6. Open questions for round 2

1. Whether `preimage` is honoured for Delta-backed scans specifically, or only for file formats
   with expression-level pruning. Determines how much predicate work can reach the log.
2. Whether recursive CTEs can push any predicate into the recursive term, which decides whether
   closure queries must be bounded by depth for large corpora.
3. `ExtractLeafExpressions` / `PushDownLeafProjections` are unfamiliar rules not present in older
   DataFusion; confirm what they do before relying on nested-column projection pruning.
