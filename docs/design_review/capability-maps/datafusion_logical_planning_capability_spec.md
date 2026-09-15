# DataFusion logical planning — compact agent specification

**Baseline:** Rust DataFusion **55.1.0**, checked 2026-09-14 against published Rust documentation and locally installed crate source. This is a capability reference, not verification of a particular application's configuration. The target repository's lockfile, enabled features, providers, registries, and planner rules determine which capabilities apply; comparing that context with this baseline can expose integration gaps before implementation. Python bindings and other releases may expose a different surface.

**Coverage:** all 25 `LogicalPlan` variants; all 37 `Expr` variants grouped by function; compilation and schema semantics; relational compositions; extension points; source/physical boundaries; inspection APIs; execution, optimization, and portability conditions; correctness validation, error diagnosis, and change testing. Function families are summarized rather than listing every built-in function name: the function registry is open-ended.

## 1. Capabilities and design considerations

A logical plan is a typed, inspectable description of relational computation: sources → operators and expressions → output relation. SQL, the DataFrame API, `LogicalPlanBuilder`, and custom language compilers can produce the same intermediate representation. It can describe multi-source, multi-stage computation without collecting intermediate rows. It also represents commands whose handling differs from relational queries. [LogicalPlan](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/logical_plan/enum.LogicalPlan.html)

**Design opportunity:** composing related relational work into a logical plan exposes more of the computation to pruning, pushdown and expression optimization, and can avoid intermediate data transfer. Existing operators usually offer the most optimizer visibility; functions, providers and extension nodes accommodate missing computations, sources and relational semantics respectively. Larger plans can also increase planning cost and complicate reuse or failure isolation. Materialization can therefore be useful for repeated consumption, bounded execution stages, diagnostics, persistence or external integration; its value depends on whether those benefits outweigh additional I/O and lost optimization across the boundary.

Recommendations below describe options and their rationale, leaving architecture and acceptance criteria to the consuming codebase. Statements about required types, execution support or operator semantics describe DataFusion constraints rather than prescribed application policy.

An operation is executable only when all applicable conditions hold:

1. **Representation:** its nodes and expressions can encode the intended semantics.
2. **Admission:** names resolve; schemas, types, nullability, arity, expression placement, and parameters are valid.
3. **Lowering:** analysis/optimization removes planner-only forms or the physical planner supports them directly.
4. **Implementation:** required kernels, registered functions, providers, extension planners, and command handlers exist.
5. **Runtime:** input access, boundedness, ordering, memory/spill, and sink requirements are satisfied.

Constructing a plan establishes neither successful execution nor equivalence to application semantics. SQL parser acceptance is also insufficient.

### Compilation model and frontend independence

The ordinary relational-query path can be understood as a compiler pipeline:

```text
SQL → sqlparser AST → SqlToRel ─┐
DataFrame / LogicalPlanBuilder ├→ initial LogicalPlan + Expr
custom DSL / plan translation ─┘          ↓ AnalyzerRule processing
                                analyzed LogicalPlan
                                         ↓ OptimizerRule rewrites
                                optimized LogicalPlan
                                         ↓ physical planning
                                ExecutionPlan + PhysicalExpr
                                         ↓ PhysicalOptimizerRule rewrites
                                optimized execution plan
                                         ↓ execution / stream polling
                                Arrow RecordBatch stream(s)
```

This separates intended computation (`LogicalPlan`/`Expr`) from executable algorithms and expression evaluation (`ExecutionPlan`/`PhysicalExpr`). Public APIs may combine several stages; DDL and session statements can branch into orchestration handlers instead of following this query path. External formats such as Substrait offer another frontend for supported translations (§6).

A common logical IR can let SQL, programmatic APIs and domain-specific languages share analysis, optimization, inspection and execution infrastructure. This can reduce duplicated implementation and make behavior comparable across frontends. A separate domain IR may still be valuable for concepts, provenance or invariants that DataFusion does not retain; lowering at a deliberate boundary can preserve those concerns while gaining relational optimization.

**Optimization visibility is a separate dimension of support.** A valid, executable UDF or extension can remain opaque to most optimizer rules, while a schema-only source can support logical rewrites without supporting execution. Availability of safe pruning/simplification and successful execution are therefore useful to assess independently rather than treating “optimizable” as another mandatory stage of capability maturity. [Session planning](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/session_state.rs.html), [physical planning/optimization](https://docs.rs/datafusion/55.1.0/src/datafusion/physical_planner.rs.html).

### Schema propagation and expression typing

`LogicalPlan::schema()` returns `&DFSchemaRef`: an Arc-backed logical schema containing Arrow fields and relation qualifiers. Field names, data types, nullability and metadata describe the result, while qualifiers distinguish references such as `orders.id` and `customers.id`. This semantic information makes a logical plan richer than a syntax tree, although a newly constructed plan may still need analysis/coercion before its expressions are fully resolved.

| Operator family | Schema behavior and design implication |
|---|---|
| Filter, sort, limit | Normally retain input fields; a row-selection change can alter values/cardinality without changing the interface. |
| Projection | Derives output fields from expressions, including aliases and casts; a rename or type change can invalidate downstream references. |
| Window | Adds computed fields while retaining input rows; expression types and names become part of the downstream interface. |
| Aggregate | Produces grouping fields followed by aggregate results; input fields outside that output are no longer directly available. |
| Join | Depends on join type: outer joins can introduce nullability, semi/anti joins expose one side, and mark joins add a marker. |
| Unnest / union | Unnest changes nested shape; union construction aligns compatible relation shapes. Both can require downstream field rebinding. |

`ExprSchemable::get_type`, `nullable`, `metadata` and `to_field` derive expression properties against a schema; `to_field` also provides the qualifier and Arrow field. These APIs can support lightweight expression validation or generated interface documentation without scanning rows. They are not substitutes for analyzer processing, which can insert coercions or otherwise rewrite expressions. `recompute_schema` and checked reconstruction can keep schema information synchronized after changes, while application metadata/units may need separate checks because storage compatibility does not establish domain equivalence. [DFSchema](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/dfschema.rs.html), [expression schema APIs](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/expr_schema.rs.html).

## 2. Complete logical operator surface

**N** = native relational capability, subject to types and physical support. **P** = provider/registry/runtime dependent. **L** = requires a supported lowering or contextual handling. **X** = custom implementation required. Labels can combine.

| Node | Operations expressible | Conditions / semantic boundaries |
|---|---|---|
| `TableScan` **N/P** | Read tables, files, in-memory Arrow batches, views, generated relations, remote or application-defined sources; carry projection, filters, fetch. | Logical `TableSource` needs an executable provider or custom planner mapping. Source format, snapshots, pruning, remote pushdown, and access are provider-specific. |
| `Values` **N** | Literal/constant relations, small lookup tables, seeds, parameterized rows. | Rectangular rows and compatible column types. Large embedded datasets increase plan size and processing overhead; provider-backed data may be more economical. |
| `EmptyRelation` **N** | Typed empty input; zero-column seed with zero or one row; constant queries without a table. | Zero rows suppress downstream row-wise computation; one empty row permits a constant projection to produce a row. |
| `Projection` **N** | Select, drop, reorder, rename, duplicate, derive, cast, assemble or extract fields. | Row-preserving scalar expressions; aggregation, windows, and row expansion need their corresponding operators. |
| `Filter` **N** | Predicates, validation subsets, conditional selection; WHERE/HAVING/post-window filtering by placement. | Boolean predicate; only TRUE survives. FALSE and NULL are discarded. |
| `Aggregate` **N/P** | Global/grouped reduction, multiple keys/measures, grouping sets, rollup, cube, conditional/ordered/distinct aggregation. | Groupable keys; registered aggregate implementations; nonaggregate outputs must satisfy grouping rules. |
| `Window` **N/P** | Ranking, offsets, running/moving aggregates, partition-relative statistics; multiple window expressions. | Partition/order/frame choices determine which rows contribute; explicit choices can prevent unintended default-frame behavior. ROWS/RANGE/GROUPS support depends on valid frames and function implementation. Retains input rows. |
| `Join` **N/L** | Inner, left/right/full outer, left/right semi, left/right anti, left/right mark; equality keys and additional Boolean conditions; cross/non-equi joins. | Aliased schemas and compatible keys; Cartesian/non-equi execution may be expensive. Cross join uses `Join`, not a separate variant. Mark joins are internal existence markers, not a universal implementation of three-valued ANY/ALL. |
| `Union` **N** | Concatenate compatible relations; basis of UNION ALL and UNION DISTINCT composition. | Schema/type alignment determines column correspondence; explicit alignment can prevent unintended positional matching, and by-name constructors exist. Plain union retains duplicates; distinctness needs deduplication. |
| `Distinct` **N/L** | Whole-row deduplication; `Distinct::On` selects one row per expression key with selection expressions/order. | Complete tie-breaking makes representative selection reproducible when tied rows differ in meaningful values. Optimizer can lower to aggregates. |
| `Sort` **N** | Multi-key ordering, direction, null placement; fetch-aware top-k planning. | Explicit order required for ordered results; equal sort keys do not define a unique order. Global sort generally needs finite input or usable ordering. |
| `Limit` **N** | Offset/skip and fetch; slicing and pagination. | Bounds must resolve to supported nonnegative integers. Stable pagination needs total ordering; limit alone selects arbitrary rows. |
| `Repartition` **N** | Round-robin batches, hash expressions, range partitions, `DistributeBy`. | Range requires valid ordered keys/split points. Distribution is not output ordering or a distributed cluster deployment. |
| `Subquery` **L** | Embedded relational queries with possible outer references. | Supported decorrelation or physical subquery handling required; arbitrary correlated plans are not guaranteed executable. |
| `SubqueryAlias` **N** | Relation naming, qualification, self-join disambiguation, derived-table scope. | Changes naming, not materialization or caching. |
| `Unnest` **N/L** | Expand list elements into rows; expand struct fields; multi-column and configurable recursive list expansion. | Null/empty-list preservation, depth and multi-list alignment/padding affect cardinality; multi-column expansion is not inherently a Cartesian product. |
| `RecursiveQuery` **N/L** | Seed plus recursive term using a working relation; iterative recursive CTE computation, with ALL or distinct mode. | Recursive CTE configuration and supported query shape required. Termination and resource analysis matter because cycles or expanding state can prevent completion; distinctness is over output rows, not an application-defined visited key. |
| `Extension` **X** | Arbitrary custom relational operators, including domain-specific transforms. | Integration includes schema, children, expressions, invariants, rewrite behavior and execution/lowering; these let the surrounding planner reason about and reconstruct the node. A node alone has no kernel. |
| `Dml` **P/L** | Insert append/overwrite/replace, delete, update, truncate, merge; CTAS operation representation. | Target must implement the particular write operation. CTAS normally routes through session DDL handling. See §5. |
| `Ddl` **P/L** | Create external/memory tables, views, schemas, catalogs, indexes, functions; drop tables/views/schemas/functions. | Session/catalog/factory handling required; enum presence does not establish an executable default, notably for indexes. |
| `Copy` **P** | Write a query result to files (`COPY TO`), including supported format/options/partitioning. | A writer-capable file format and writable destination are required; publication/transaction semantics belong to the sink/application. |
| `Statement` **P/L** | Transaction start/end; SET/RESET; PREPARE/EXECUTE/DEALLOCATE. | Session handles supported statements; transaction nodes do not provide a transaction manager. |
| `Explain` **N** | Inspect logical/optimized/physical planning stages and diagnostic representations. | Explains planning; it does not measure executed query behavior. |
| `Analyze` **N/P** | Execute input and report physical execution metrics (`EXPLAIN ANALYZE`). | Executes work; costs and effects follow the input. |
| `DescribeTable` **N/P** | Schema introspection. | Requires resolved table/schema context. |

Operator details and constructors: [versioned plan source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/plan.rs.html), [builder source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/builder.rs.html), [join types](https://docs.rs/datafusion-common/55.1.0/datafusion_common/enum.JoinType.html).

For joins, `join_on` accepts general conditions, while lower-level `join_detailed`/`join_detailed_with_options` expose explicit keys, residual predicates, `NullEquality` and null-aware options. Ordinary null equality, null-safe key matching and null-aware subquery semantics are distinct concepts; those choices can change which rows match even with identical column names. Their physical support remains plan-dependent. Expression-based limit construction similarly allows bounds to be represented before their executable values are resolved. [Builder contracts](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/builder.rs.html)

## 3. Complete expression surface

Expressions are typed computation inside plan nodes; their validity depends on position. A scalar projection cannot directly substitute for an aggregate, window operator, or relation-producing function.

| Family | Exact `Expr` variants | Capability / conditions |
|---|---|---|
| References and constants | `Column`, `Literal`, `Alias`, `ScalarVariable` | Qualified fields, scalar/typed nested literals, naming, session-provided variables. Field metadata can accompany literals and aliases; aliases also carry optional relation qualification. Variable types/values need a provider. |
| Parameters and correlation | `Placeholder`, `OuterReferenceColumn` | Reusable parameterized templates and correlated references. Type inference/checking and value binding make a template executable for specific inputs; outer references need valid enclosing scope and supported planning. |
| Operators | `BinaryExpr`, `Negative`, `Not` | Arithmetic, comparisons, Boolean logic, bitwise operations, concatenation and pattern operators supported by `Operator`; valid operand coercions/kernels required. NULL, overflow, decimal and timestamp behavior are part of the contract. |
| Null/truth tests | `IsNull`, `IsNotNull`, `IsTrue`, `IsFalse`, `IsUnknown`, `IsNotTrue`, `IsNotFalse`, `IsNotUnknown` | Explicit SQL three-valued logic tests. Null-safe equality/distinctness is also available through binary operators. |
| Predicates and branching | `Between`, `InList`, `Like`, `SimilarTo`, `Case` | Ranges, membership, patterns/escape rules, conditional values. CASE branches need compatible output types. |
| Conversion | `Cast`, `TryCast` | Explicit type conversion; TRY_CAST returns NULL for failed value conversions in supported casts. It does not make arbitrary type pairs valid. |
| Scalar calls | `ScalarFunction` | Built-in or custom scalar UDF; can compose arbitrarily with valid expressions. Signature, result field, null behavior, volatility and execution implementation required. |
| Reduction calls | `AggregateFunction`, `GroupingSet` | Aggregate arguments plus optional DISTINCT, FILTER, ORDER BY, null treatment; grouping sets/rollup/cube. Modifiers must be supported by the function. GroupingSet is valid in grouping context. |
| Window calls | `WindowFunction` | Arguments, partition/order keys, frame, filter/null treatment where supported; built-in, aggregate-as-window, or custom window UDF. |
| Subquery values/predicates | `Exists`, `InSubquery`, `SetComparison`, `ScalarSubquery` | EXISTS/NOT EXISTS, IN/NOT IN, comparisons with ANY/ALL, scalar lookups. Scalar subquery: one column, at most one row; zero rows yields NULL. Empty-set and NULL behavior can distinguish otherwise similar rewrites. |
| Nested expansion | `Unnest` | Planner expression for list/struct expansion, requiring appropriate relational lowering because it can change row count. |
| Higher-order computation | `HigherOrderFunction`, `Lambda`, `LambdaVariable` | Registered higher-order functions with bound lambda arguments, e.g. array transformation/filtering. Lambdas belong to supported higher-order calls, not arbitrary standalone runtime closures. |
| Wildcard | `Wildcard` | Deprecated planner form requiring expansion to concrete expressions before physical planning, where individual field computations are needed. |

`SortExpr` is a separate wrapper for an expression plus ascending/descending and null placement; it is not an `Expr::Sort` variant in this baseline. [Expr source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/expr.rs.html), [operators](https://docs.rs/datafusion-expr-common/55.1.0/datafusion_expr_common/operator/enum.Operator.html).

**Built-in function families:** numeric/math; strings/Unicode; regex; encoding/hashing/crypto; date/time/intervals; conditional/null/type utilities; structs/maps/lists; array construction/access/slicing/concatenation/set operations/sorting/flattening/reduction; vector distance/inner product; higher-order array operations; count/sum/min/max/average; exact/approximate distinctness, median/percentiles; variance/stddev/covariance/correlation/regression; Boolean/bitwise aggregation; array/string aggregation; first/last/nth; row number/rank/dense rank/distributions/tiles/lead/lag. Availability requires the relevant crate features and registration. Signature inspection identifies supported types and modifiers, since a function family does not imply support for every Arrow type or combination. [Scalar](https://docs.rs/datafusion-functions/55.1.0/datafusion_functions/), [aggregate](https://docs.rs/datafusion-functions-aggregate/55.1.0/datafusion_functions_aggregate/), [window](https://docs.rs/datafusion-functions-window/55.1.0/datafusion_functions_window/), [nested](https://docs.rs/datafusion-functions-nested/55.1.0/datafusion_functions_nested/).

## 4. Operations expressed by composition

| Desired operation | Logical formulation / condition |
|---|---|
| Normalize, cleanse, validate, classify, score | Projection + casts/CASE/functions + filters; a separate invalid-row relation can retain diagnostic evidence while valid rows continue through processing. Domain/unit validation requires authored expressions or custom functions. |
| Lookup, enrichment, existence, missing-reference detection | Inner/outer joins, semi/anti joins, or supported subqueries. Key uniqueness and join multiplicity determine whether enrichment preserves rows or multiplies them; inspecting these properties can reveal unintended many-to-many matches. |
| Reconcile datasets / detect changes | Full outer join + null-safe comparisons + CASE; aggregate/count for duplicate-sensitive comparison. |
| UNION DISTINCT, INTERSECT, EXCEPT | Union + distinct; intersection/difference via builder compositions. **55.1.0 caveat:** the checked `intersect/except(is_all=true)` builder uses semi/anti joins without occurrence matching. SQL bag semantics call for `min(left_count,right_count)` occurrences for INTERSECT ALL and `max(left_count-right_count,0)` for EXCEPT ALL. An explicit count/occurrence formulation can express those semantics; testing another entry point can establish whether it provides them. |
| Latest/best row per key, top-k per group | Window rank/row_number + filter, or DISTINCT ON; complete tie-breaking makes selection reproducible when tied rows differ. |
| Running totals, moving statistics, deltas, session labels | Window aggregates/lead/lag + projection; multi-stage windows for dependent expressions. Ordering, partition keys and frames determine the neighborhood and history used by each calculation. |
| Pivot / unpivot | Known pivot values → conditional aggregates; unpivot → projections + union or nested expansion. Discovering data-dependent output columns requires a separate schema-discovery step or fixed nested output type. |
| Nested/semi-structured transformations | Field extraction, list/map functions, lambdas, unnest, reaggregation. JSON/geospatial/specialized kernels may need extra registered functions/providers. |
| Generate rows, grids, combinations | Values or table functions such as series/range + cross join/unnest. Table functions produce providers; argument/correlation support is implementation-specific. |
| CTEs, views, reusable query fragments | Plan composition/reuse with aliases provides named query fragments. Repeated references do not promise once-only execution, cached results, or materialization. |
| Hierarchies / transitive traversal | RecursiveQuery or explicitly bounded joins. Cycle handling, shortest depth, visited-key state and termination depend on the authored traversal; full-row DISTINCT alone may not satisfy them because distinct paths or depths can keep producing new rows for the same vertex. |
| Parameterized/domain-specific query language | Typed inputs can compile to Expr + LogicalPlan, with parameter binding and analysis/optimization supplying executable query instances. This provides a common optimization surface without requiring SQL text. |
| Federation | Provider-backed sources can participate in a single join plan. Remote join/aggregate pushdown and cross-source consistency need provider/extension support. |

These are composition patterns, not additional plan variants. The duplicate-count caveat is a source-level finding, not a claim from an executed application test. [Builder](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/builder.rs.html), [recursive execution](https://docs.rs/datafusion-physical-plan/55.1.0/src/datafusion_physical_plan/recursive_query.rs.html), [table functions](https://docs.rs/datafusion-functions-table/55.1.0/datafusion_functions_table/).

## 5. Sources, writes, and extensions

| Need | Integration contract |
|---|---|
| New data source | `TableProvider`/`TableSource`; truthful schema, scan implementation and supported pushdown. Providers can expose memory, files, APIs, indexes or external stores as relations. |
| Filter pushdown | `Unsupported`: engine evaluates. `Inexact`: provider may prune but must retain every true match; engine retains residual filtering. `Exact`: provider completely enforces the predicate and the engine may remove its filter. False Exact claims are correctness defects. Inexact filtering constrains safe limit pushdown. |
| New per-row computation | `ScalarUDF`/`ScalarUDFImpl`; input signature/coercion, output field/nullability, volatility, and vectorized implementation. Simplification/planning hooks can expose useful properties of otherwise opaque custom internals, enabling optimizations that generic planning cannot infer. |
| New grouped computation | `AggregateUDF`/`AggregateUDFImpl`; accumulator update/state/merge/evaluate contract. Ordering, retraction, distinctness and specialized group support determine applicability to particular aggregate/window execution paths. |
| New window / lambda-aware computation | `WindowUDF` or `HigherOrderUDF` implementation with the required evaluator/signature and frame or lambda behavior. |
| New relation-producing function | A registered `TableFunction` returns a provider, fitting relation-producing computation; scalar UDFs model scalar results and do not supply arbitrary relational row-count changes. |
| New relational semantics | `UserDefinedLogicalNode` or `UserDefinedLogicalNodeCore`, exposed through `Extension`; lowering to existing nodes preserves their optimizer support, while an `ExtensionPlanner` can produce a specialized `ExecutionPlan`. Inputs/expressions, schema, reconstruction/equality/hash and pruning/pushdown contracts let planner transformations account for the custom operation. |
| New language / planning behavior | Custom compiler; SQL `ExprPlanner` for expressions/operators, `TypePlanner` for SQL types, `RelationPlanner` for table factors; analyzer/optimizer rules or custom query/physical planner. These hooks let recognized syntax lower into ordinary types/expressions/relations without necessarily introducing custom relational operators. They do not themselves extend the parser's grammar. |
| Write support | Provider hooks `insert_into`, `delete_from`, `update`, `truncate`, `merge_into`; insert mode is Append/Overwrite/Replace. Support is per hook and provider. Write expressions, predicates and merge clauses must meet that provider's contract. |
| Catalog / function commands | Session handlers plus catalog/table factories; CREATE FUNCTION needs a function factory. Index creation requires host support. BEGIN/COMMIT/ROLLBACK representations do not supply ACID, rollback, isolation, or multi-source atomicity. |

Custom nodes conservatively block predicate/limit pushdown unless their implementation permits it. Permitting pushdown can reduce work when it preserves cardinality, NULL behavior, errors, ordering requirements and effects; otherwise it can alter meaning. Function volatility declarations inform permissible evaluation/optimization behavior. Side-effecting UDFs therefore do not inherently provide once-only or row-ordered effects. [Provider source](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html), [extension contract](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/extension.rs.html), [UDF guide](https://datafusion.apache.org/library-user-guide/functions/adding-udfs.html), [physical planner](https://docs.rs/datafusion/55.1.0/src/datafusion/physical_planner.rs.html), [session handling](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/context/mod.rs.html).

### Source metadata, schema-only planning and scan construction

`TableSource` is the logical source interface; an executable `TableProvider` can be adapted through `provider_as_source`/`DefaultTableSource`. This separation supports planning against schema/capability metadata without reading the dataset. `LogicalTableSource` supplies a simple source for tests, documentation or custom planning backends. It can make schema-evolution experiments and compiler tests independent of storage availability, but it is not an executable provider for the ordinary physical planner.

**55.1.0 qualification:** `LogicalTableSource` advertises `Exact` for every filter. Consequently, a plan optimized against it can differ from one optimized against a production provider. A metadata-only source that mirrors production capabilities can improve optimizer-test fidelity; reconstructing from the original unoptimized plan against the executable source and rerunning analysis/optimization can avoid carrying unsupported pushdown assumptions into execution. [LogicalTableSource source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/builder.rs.html), [source adapters](https://docs.rs/datafusion/55.1.0/datafusion/datasource/default_table_source/index.html).

Provider `scan`/`scan_with_args` calls participate in physical planning by returning execution plans. Projection/filter/limit information describes the requested scan; provider capability declarations determine which semantics can be delegated. A projected output column list may omit fields still needed internally to evaluate a pushed predicate. This explains why output-column pruning and predicate evaluation have distinct provider responsibilities.

Deferring bulk I/O and transformation until execution can keep repeated planning, EXPLAIN and schema checks responsive and make runtime resource management more predictable. Metadata lookup, schema inference, remote preparation or file discovery may legitimately occur during planning, so the useful distinction is bulk execution versus the planning work required by the provider—not a promise of zero I/O. A pushed limit is also not by itself proof of physical early termination; the resulting operators determine how much work is avoided. [Provider scan contract](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html).

### Custom planner and node contracts in more detail

`TypePlanner::plan_type_field` returns a field, allowing type metadata such as extension-type annotations; recognizing such a type does not supply its domain validation or runtime kernels. `RelationPlanner::plan_relation` can handle an SQL table factor or return it for subsequent/default planning. These hooks are useful when a language extension maps naturally to existing DataFusion semantics. [SQL planner hooks](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/planner.rs.html)

For custom nodes, `with_exprs_and_inputs` reconstructs a node after rewrites; `fmt_for_explain` makes it diagnosable; `check_invariants` exposes custom validity checks. `prevent_predicate_push_down_columns`, `necessary_children_exprs` and `supports_limit_pushdown` describe which transformations are safe. These contracts can recover optimization opportunities around a specialized operator without requiring the optimizer to understand its entire implementation. Conversely, conservative behavior can be appropriate when a property is unknown, trading optimization for semantic isolation. [Extension APIs](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/extension.rs.html)

## 6. Planning, inspection, optimization, and reuse

- **Construct:** SQL → AST → logical plan; DataFrame methods; `LogicalPlanBuilder`; direct checked constructors; custom DSL compilation. Checked constructors and schema utilities can detect inconsistencies close to their origin and reduce the burden of manually maintaining planner invariants.
- **Analysis:** name/qualifier and function resolution, type inference/coercion, aggregate/window/subquery placement checks, parameter binding and invariant validation connect the authored query to the available schemas and implementations. Schema recomputation/checking after rewrites can expose stale field information before it reaches consumers. Arrow storage types and metadata do not automatically enforce business constraints or units.
- **Inspect/rewrite:** `TreeNode` visitors/transforms; node inputs/expressions/schema; parameter and outer-reference inspection; invariant checks. `*_with_subqueries` APIs include nested plans, making them useful for complete lineage or policy inspection that ordinary child traversal can miss.
- **Optimize:** expression simplification/constant folding; projection pruning; filter/limit pushdown; duplicate/common-expression elimination; union simplification; distinct lowering; empty-plan propagation; join-predicate extraction and redundant/cross/outer-join simplification; predicate/scalar/lateral-subquery decorrelation. Rule sets and applicability vary. These are not promises of a globally optimal plan or shared-subplan caching.
- **Lowering/execution:** physical planning selects implementations and enforces distribution/order; execution then streams Arrow record batches or collects/writes results. Partition count, join algorithm, spill behavior, scheduling and measured performance require physical evidence.
- **Observe:** logical/schema/Graphviz/other diagnostic displays; EXPLAIN for plan stages; ANALYZE and runtime metrics for execution evidence. Planning success cannot establish streaming feasibility or performance.
- **Serialize:** `datafusion-proto` supports many plans/expressions, with codecs/registries for custom providers, functions and nodes; Substrait support is a separate interoperability subset. Coverage is not universal: 55.1.0 protobuf rejects several DDL forms, most statement forms, and DistributeBy. Round-trip tests for the plan forms and extensions actually used can establish portability without assuming universal codec coverage.
- **Reuse:** retaining parameterized plans or compiled fragments can avoid repeated construction while context remains valid. Context comparison or rebinding across source/catalog identities, schemas, function versions, configuration and snapshots helps prevent reuse under changed semantics. Serialized plans do not embed all data, implementation code, credentials or session state. Diagnostic strings and protobuf bytes are not a canonical semantic identity.

[Optimizer source](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/optimizer.rs.html), [tree APIs](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/logical_plan/enum.LogicalPlan.html#method.apply_with_subqueries), [protobuf source](https://docs.rs/datafusion-proto/55.1.0/src/datafusion_proto/logical_plan/mod.rs.html), [Substrait](https://docs.rs/datafusion-substrait/55.1.0/datafusion_substrait/).

### Programmatic inspection and human diagnostics

| Surface | Information / possible use |
|---|---|
| `inputs()`, `expressions()`, `schema()` | Immediate structure and output interface; useful building blocks for inventories and transformation checks. They do not themselves constitute a recursive inspection. |
| `TreeNode::apply` / `transform` on plans and expressions | Structural visiting/rewriting: source and field references, functions, literals, casts, predicates and operator relationships. |
| `apply_expressions` / `map_expressions` | Access or replacement of a node's expressions; expression-tree traversal adds nested expression coverage. |
| Subquery-aware visit/transform APIs | Coverage of embedded plans and correlated references; useful when dependencies or rules must include nested queries. |
| `display()` / `display_indent()` | Current-node versus indented-tree diagnostics. |
| `display_indent_schema()` / `display_graphviz()` | Field propagation and graph views for understanding plan structure. |
| EXPLAIN / EXPLAIN VERBOSE / ANALYZE | Planning output, more detailed/intermediate stages where supported, and executed physical metrics respectively. |

Direct traversal retains typed objects and relationships that formatted EXPLAIN text may omit or reorganize, making it a stronger foundation for machine inspection. It can support dependency inventories, lineage derivation, function/type migration impact analysis, or application-authored governance checks. Derived column lineage requires following expressions through aliases and operators; opaque functions, views/providers and custom nodes can limit visibility. Plan inspection can support a policy implementation but does not automatically enforce authorization outside that path.

Analyzed plans are often closest to the resolved authored intent, while optimized logical plans show the transformed work presented to physical planning. A source-level operation may disappear as a separate node through simplification, decorrelation or scan pushdown without disappearing semantically. Comparing these stages can distinguish an incorrect frontend translation from an unexpected rewrite. Source spans or transformation identifiers maintained by the application can connect node/expression diagnostics back to authored input; arbitrary rewrites do not inherently preserve that provenance. [Plan/tree/display APIs](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/logical_plan/enum.LogicalPlan.html)

### What remains a physical decision

| Logical specification | Physical planning / optimization / runtime decisions |
|---|---|
| Join semantics and conditions | Hash, sort-merge or nested-loop strategy; build/probe side; exchanges and memory behavior. |
| Grouped/window computation | Partial/final aggregation, accumulator strategy, buffering, partition evaluators and supporting sorts. |
| Expressions and fields | Concrete kernels and `PhysicalExpr` evaluation against physical batch schemas/column positions. |
| Ordering/distribution intent | Concrete partition topology, additional exchanges, inserted/eliminated sorts, coalescing and scheduling. An explicit logical partition count or range still constrains the requested operation. |
| Source reads and resource needs | File/row-group selection, object-store requests, caching, spill and memory/concurrency behavior, subject to provider and runtime support. |

This distinction explains why a logical `Join` cannot identify its eventual algorithm, or a logical `Aggregate` its partial/final stages. Physical optimization can change implementation structure while preserving results. Physical-plan inspection and metrics are therefore useful when the question concerns skew, work distribution, memory or performance; even physical planning alone does not establish behavior on actual data. [Physical planner and optimizer integration](https://docs.rs/datafusion/55.1.0/src/datafusion/physical_planner.rs.html)

## 7. Semantic boundaries and architectural tradeoffs

- **Relational semantics:** bag multiplicity, null logic, empty inputs, outer-join unmatched rows, type/decimal/timezone rules and deterministic selection can distinguish superficially similar operations. These dimensions help assess whether a substitution preserves the application’s intended behavior. Constraints/statistics support optimization; their declaration is not general integrity enforcement.
- **Streaming:** batch-stream output does not imply bounded memory or incremental results for every operator. Global sort, distinct, aggregation, joins, windows and recursion need suitable finiteness, ordering, state/resource bounds or specialized implementations. A logical plan is not automatically a CDC/incremental-view-maintenance engine.
- **Dynamic schema:** output schema must be known during planning. Data-dependent column discovery or arbitrary schema-changing code needs staging or a fixed nested representation.
- **External algorithms:** numerical solvers, graph algorithms, model inference and procedural workflows can be exposed through functions/providers/extensions when their contracts fit; their algorithms are not supplied by logical planning itself. Representing external state/effects explicitly can clarify where reproducibility, retry and lifecycle guarantees depend on systems outside the planner.
- **Persistence and execution ownership:** a plan does not inherently provide durable checkpoints, result caching, snapshot isolation, multi-query orchestration, distributed execution, authorization, or transactional publication. Those capabilities can come from surrounding systems; their placement depends on the application’s consistency, recovery and execution requirements.

**Capability assessment:** node/expression composition, integration dependencies, semantic preconditions and physical execution route explain how an operation fits a codebase. Null/duplicate/order/empty-input examples can distinguish application-correct behavior from mere execution success; serialization evidence matters when plans cross persistence or process boundaries. The progression representable → analyzable → physically plannable → executable → application-correct provides a vocabulary for describing evidence without treating every level as an imposed acceptance gate.

## 8. Correctness validation, error diagnosis, and change testing

Logical planning provides **inspectable test subjects, validation hooks, and composable diagnostic queries**. These can support a layered testing strategy in which inexpensive structural checks catch representational defects, focused execution tests establish local semantics, and end-to-end tests cover integration. The appropriate balance depends on failure modes, execution cost and the independence of available reference results. The techniques below are design options; only the named DataFusion mechanisms are built-in behavior, and logical planning is not a general equivalence prover.

### Validation layers and their value

| Layer | Available mechanism / approach | Value and limits |
|---|---|---|
| Construction and analysis | Checked builders; parameter binding; `Analyzer::execute_and_check`; invalid-name/type/arity/placement cases. | Can reveal schema/type and analyzer errors before data execution, shortening feedback cycles. Despite its name, `execute_and_check` executes analyzer rules, not the data query; it does not establish value-level correctness. |
| Structural invariants | `check_invariants(InvariantLevel::Always)` and `Executable`; `apply_with_subqueries`; expression and extension inspection. | Can detect implemented invariants and application-defined structural defects. **55.1.0:** `Always` checks current-node field-name uniqueness without child recursion; explicit traversal extends local assertions across a plan. `Executable` adds subquery/extension checks but neither runs analysis nor guarantees physical execution. |
| Output contract | Comparison of ordered fields, types, relevant qualifiers, nullability and metadata before/after rewrites. | Can catch interface drift that leaves values apparently correct but breaks downstream interpretation. Built-in `assert_expected_schema` compares logical names/types and **ignores nullability and metadata**; application-specific comparisons cover those dimensions when consumers depend on them. |
| Structural intent | Source, join-key/type, grouping, filter-scope, frame/order, parameter and effect inspection, including subqueries. | Can detect a query that encodes the wrong requirement before execution. Assertions about semantic properties are often less brittle than exact tree shape because equivalent optimizer transformations can reorganize nodes. |
| Physical feasibility | Physical planning with the target provider/function registry and configuration. | Exposes missing lowering or implementations in the relevant integration context. Runtime data, resource and sink errors can remain. |
| Behavioral correctness | Execution on bounded fixtures against independently specified results or a trusted reference. | Establishes behavior on tested inputs, including expected failures. An independent oracle reduces the chance that candidate and reference share the same mistake; fixtures do not establish universal correctness. |
| Operational behavior | Physical-plan/metric inspection and variation of batches, partitions or resource limits. | Can expose execution-path and state-management defects or performance regressions. A desirable plan shape or faster execution alone says nothing about value correctness. |

Sources: [invariant implementation](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/invariants.rs.html), [analyzer](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/analyzer/mod.rs.html), [optimizer](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/optimizer.rs.html).

### Errors visible before scanning data, and errors requiring intent

Planning can often operate over plan structure, schemas, registries and metadata instead of the underlying rows, making it attractive for fast feedback on generated transformations. Detection belongs to different stages rather than one exhaustive validator:

| Potential defect | Where evidence can arise |
|---|---|
| Unknown/ambiguous columns, wrong qualifiers, unresolved functions or parameters | Frontend resolution, binding or analysis with the relevant context. |
| Incompatible types/shapes, invalid predicates or function arguments | Checked construction, coercion/analysis and sometimes physical expression planning; valid casts can still fail on particular values. |
| Invalid grouping, window invocation, subquery placement or correlation | Frontend/analyzer/invariant checks for supported forms; validity does not establish the intended group or frame. |
| Disappearing fields or changed result interface | Schema propagation and expected-schema comparison across transformation boundaries. |
| Valid but unintended join/group/window/filter choices | Application-authored intent comparisons or behavioral tests; these are not generally errors DataFusion can infer. |

For example, partitioning a window by `merchant_id` instead of `customer_id` can type-check and execute successfully. Comparing the intended partition with the `WindowFunction` parameters reveals a different class of defect from checking type validity. Similar comparisons involve `Join.on` and residual predicates, `Filter.predicate`, `Aggregate.group_expr`/`aggr_expr`, and expected output fields. Keeping the expectation independent of the plan generator helps avoid merely restating its mistake. For optimized plans, the corresponding semantics may be distributed across different nodes (§6).

### Diagnostic relations as local validation

Attaching validation plans to input/output relations or intermediate subplans can make local checks straightforward to compose using the same relational machinery as production work. When inputs are bounded or intermediate results are already available, these checks can be inexpensive and localize failures earlier than repeatedly executing an entire pipeline. This can reduce repeated local assertions in end-to-end tests and let those tests concentrate on integration, orchestration and external effects. Cost still follows the physical work: a small diagnostic plan may scan a large source, perform a global aggregation or repeat an expensive subplan. Reuse, sampling, targeted fixtures and materialization have different cost/coverage tradeoffs; sampled checks provide evidence about the sample rather than complete validation.

A violation relation containing offending keys, values, counts and a rule identifier can serve both as a test result and an explanation. Emptiness naturally expresses an exact invariant; counts or thresholds can express intentionally tolerated anomalies. The acceptance policy remains an application decision.

| Requirement | Candidate diagnostic formulation and reasoning |
|---|---|
| Required value / domain predicate | `Filter(IsNull(value))` identifies missing values; `Filter(IsNotTrue(predicate))` captures FALSE **and NULL** when both violate the rule. `NOT predicate` alone misses NULL failures because filtering discards UNKNOWN. |
| Unique key | Grouping by key with `COUNT(*) > 1` exposes duplicate groups. A separate null-key policy clarifies whether nulls are admissible keys, independently of duplicate counting. |
| Referential integrity / coverage | A left anti join reveals required keys absent from the available relation. Ordinary versus null-safe matching determines whether null keys count as missing. |
| Parsing/conversion | Retaining the original value alongside TRY_CAST makes non-null-to-null conversion failures observable and diagnosable. Unsupported cast pairs can still fail planning. |
| Join cardinality | Key multiplicities and output counts per input identity can reveal unintended row expansion or loss. Total counts alone may conceal compensating errors; lookup-side uniqueness explains the many-to-one case. |
| Conservation / reconciliation | Before/after grouped amounts or counts, compared with a full outer join, reveal changed totals and missing groups. Numerical tolerances distinguish acceptable arithmetic variation from domain-significant discrepancies. |
| Temporal/order consistency | LAG/LEAD under a defined order exposes gaps, overlaps or transitions. Complete tie-breaking matters when different arrangements of tied rows yield different diagnostics. |

Reading a shared snapshot strengthens the connection between validation evidence and production results: separate reads of changing data can invalidate that inference. CTE/plan reuse does not itself establish snapshot consistency. Periodic validation may legitimately observe a different snapshot, but then describes that observation rather than certifying a particular production execution. Similarly, pre-write checks alone do not provide atomic validation-and-write; provider/application coordination matters when concurrent changes could invalidate the check.

### Comparing proposed changes

| Technique | Reason to consider it / applicability |
|---|---|
| Controlled comparison context | Shared immutable inputs, schemas, parameters, function versions and relevant settings help attribute differences to the intended change. Controlled clocks/randomness support exact comparisons; intrinsically volatile behavior may call for distributional or invariant-based expectations instead. |
| Plan checkpoints | Original, analyzed, optimized and physical plans with schemas make transformations inspectable. Analyzer/optimizer observer callbacks add rule names, helping locate a regression. Formatted snapshots can be useful for intentional plan-shape tests; text changes across releases make them less suitable as the sole semantic oracle. |
| Baseline/candidate execution | Full-result and output-contract comparison tests a semantics-preserving change. Intended semantic changes instead benefit from new expected results, with baseline differences showing their effect. Isolating one optimizer rule can narrow causality while retaining necessary analysis/lowering; an arbitrary unoptimized plan may not execute. |
| Contract-sensitive equality | Multiset comparison captures duplicate and NULL differences for unordered relations; sequence comparison addresses ordered contracts. Typed Arrow comparison preserves distinctions that display strings can hide. Decimal, NaN, signed-zero and floating-point tolerance choices reflect application semantics. Totals and hashes offer cheap screening but can conceal differences; §4 describes a limitation of the checked EXCEPT ALL path. |
| Difference relations | For groupable types, full-row occurrence counts on both sides, null-safe full joining, and zero-filled missing counts expose unequal multiplicities without expanding occurrences. A typed external comparator can accommodate unsupported grouping types or approximate matching. This provides actionable witnesses beyond a Boolean mismatch. |
| Edge-case fixtures | Empty/all-null inputs, duplicate/unmatched keys, ties, scalar-subquery cardinalities, overflow boundaries, empty/null lists, window boundaries and recursive cycles exercise semantic discontinuities that typical data may miss. Relevance follows the changed operations; covering every family for every change can add little value. |
| Negative cases | Invalid inputs with expected failure stages/categories can establish rejection behavior, including regressions where a change silently accepts bad data. Category/context assertions generally tolerate harmless wording changes better than complete message snapshots. |
| Integration-specific comparisons | Pushdown versus unpruned engine filtering can reveal provider errors; varied batches/partitions can expose UDF or aggregate state bugs; reconstruction and codec round trips can exercise custom-node contracts. For Exact pushdown, rechecking returned rows detects false positives but cannot detect wrongly omitted rows, which motivates a full reference comparison. |

Property/metamorphic tests can extend coverage without enumerating every expected output. Examples include deduplication idempotence, preservation of a deterministic pure row-wise projection's multiset when input is split and re-unioned, and permutation invariance for order-independent operations. Their preconditions determine whether a mismatch indicates a defect or an invalid property. Deliberate mutations such as changing a join type or removing a predicate can test the sensitivity of the validation itself, especially when a green suite otherwise offers little evidence that it detects the targeted failure. These techniques provide empirical evidence rather than automatic DataFusion proofs.

### Error localization and evidence

The earliest failing stage, rule/node path, expression/schema, parameters and a minimal input witness can make a failure reproducible and distinguish construction defects from rewrite, kernel or provider errors. Observer checkpoints and controlled rule isolation can narrow a rewrite regression; intermediate results can identify the first transformation that changes values incorrectly. Some per-rule invariant checks are debug-build-only, so explicit test assertions can make selected guarantees independent of build mode. The useful evidence varies: an offending value often explains a cast failure, while source identity and pushed predicates are more informative for a provider mismatch.

Disposable targets can make DML/DDL/COPY and ANALYZE experiments repeatable without modifying shared state. Comparing resulting state as well as returned counts can reveal write errors that a successful response misses. Planning/EXPLAIN may also invoke provider metadata or planning hooks, so isolation needs depend on those integrations rather than the label “planning.” Separating performance observations from semantic results makes it easier to assess improvements without conflating speed with correctness. [Optimizer observers](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/optimizer.rs.html), [provider contract](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html).

**Possible evidence summary:** `change → intended contract → context/fixtures → structural checks → comparator → differences/errors → conclusion → untested boundaries`. This can communicate both the result and the strength of its support without dictating a codebase's test framework or acceptance gates. Different plans can be equivalent, and identical plan structures can behave differently under changed sources/functions; execution context and behavioral evidence therefore complement structural inspection.
