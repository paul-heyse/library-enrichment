# DataFusion relation contracts, reusable models, semantic discovery and configuration

**Scope:** dense functional and implementation reference for `TableProvider`, `ViewTable`/table functions, catalogs/`information_schema`, and `ConfigExtension`. Each capability is linked to its design value and the conditions under which that value is realized. Architectural choices are presented as options with reasoning; API correctness requirements are distinguished from recommendations.

**Baseline:** DataFusion **55.1.0**, Arrow Rust **59.3.0**, matching the inspected `pse-arrow` manifest and installed source on 2026-09-14. Library claims are **Interface-checked**; repository observations describe inspected implementations, not executed qualification. Context7 supplied discovery context; versioned source resolves differences from current/main documentation. The attachment is source material, not architectural authority. [Repository pins](</home/paul/pse-arrow/Cargo.toml>).

**Codebase context:** typed relations and their registry already supply authority in `pse-arrow`; the native-plan transition is governed by [blueprint D1, D10, §3.3.3, §4–§5](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>) and [Plan 05](</home/paul/pse-arrow/docs/plans/05-native-logical-plan-hard-pivot.md>). These DataFusion mechanisms can expose and execute that authority. Neither a catalog nor session settings automatically becomes a replacement source of domain truth.

**Reading map:** §1 relation contracts; §2 reusable relational definitions; §3 discovery and semantic registry; §4 configuration; §5 combined lifecycle and validation. One running heater-run diagnostic appears once per category, covering the relevant interfaces together. Its proposed derived relation/function is illustrative; existing code anchors are identified separately.

## 0. Design value across the four mechanisms

| Mechanism | Central capability | Design value in a simulator or other complex data system | Condition that makes the value real |
|---|---|---|---|
| `TableProvider` | Bind a relation's schema, established facts and physical access behind one object. | Source adapters, analysis, validation and analytics can use the same relation contract instead of independently reconstructing its shape and guarantees. | Declared schema, keys, filtering and source lifetime agree with actual data. |
| `ViewTable` | Retain a derived relation as a logical plan. | A diagnostic or preparation calculation can be composed, optimized and tested through the same representation as production queries. | The plan passes applicable analysis and binds the intended sources/functions. |
| Table function | Construct a provider from expression arguments and session context. | Parameterized relation families can expose variable subsets, layouts and calculations without a separate result-serving API. | Argument interpretation, output schema and context binding are explicit. |
| Catalog hierarchy | Resolve and enumerate named provider objects. | Agents, compilers and user interfaces can discover the same admitted environment that query planning uses. | Namespace membership and provider identities remain coherent during use. |
| `information_schema` plus domain relations | Query engine metadata and application semantics through relations. | Generic discovery can be joined to quantity, ownership, dependency and invariant data for targeted reasoning. | Engine metadata is supplemented where it does not encode domain meaning. |
| `ConfigExtension` | Typed options with string mutation, cloning and enumeration contracts. | One policy definition can serve Rust consumers, configuration inputs and diagnostics, reducing disagreement between interfaces. | Parsing, enumeration, clone behavior and phase-specific consumers agree. |

The mechanisms compose as `catalog → schema → provider → scan plan → batches`; a view supplies a retained logical definition, and a table function constructs a provider. Configuration participates at selected points in that path. Canonicality comes from binding these surfaces to the same authoritative declarations and context, not simply from implementing the traits.

## 1. TableProvider as a domain/data contract

### 1.1 Object model and complete trait surface

`TableProvider: Any + Debug + Send + Sync` represents a queryable, optionally writable relation. It is neither a `RecordBatch` nor an execution stream. In this release the defining source is `datafusion-session`, re-exported through `datafusion-catalog` and `datafusion::catalog`; newer examples may show different module paths or signatures.

| Method | Contract and default | Design value / implementation consideration |
|---|---|---|
| `schema() -> SchemaRef` | Required stable exposed Arrow schema. | One planning contract for names, storage, nullability and metadata, including nested fields. |
| `table_type() -> TableType` | Required; `Base`, `View` or `Temporary`. | Discoverable classification, independent of storage layout, write support or admission status. |
| `scan(state, projection, filters, limit)` | Required async physical-plan construction. | Source-specific access can participate in standard partitioned execution and optimization. |
| `scan_with_args(state, ScanArgs)` | Async structured alternative; default delegates projection/filter/limit to `scan`. | Carries additional statistics requests without changing the legacy signature. |
| `supports_filters_pushdown(&[&Expr])` | One positional classification per expression; default all `Unsupported`. | Declares exactly which filtering work can move below the source boundary. |
| `constraints() -> Option<&Constraints>` | Default None; supported-but-empty differs from unsupported. | Established keys can inform relational reasoning; declaration is not enforcement. |
| `statistics() -> Option<Statistics>` | Default None; trait documentation says this entry point is not presently used in mainline DataFusion. | Optional source facts for custom consumers; not a substitute for physical-plan statistics. |
| `get_column_default(name) -> Option<&Expr>` | Default None. | Defaults can remain logical expressions derived from the same authoring contract. |
| `get_logical_plan() -> Option<Cow<LogicalPlan>>` | Default None; borrowed or owned underlying plan. | Enables definition-aware inspection and optimizer expansion where consumed. |
| `get_table_definition() -> Option<&str>` | Default None. | Preserves creation text for explanation; text need not be executable or canonical identity. |
| `insert_into`, `delete_from`, `update`, `truncate`, `merge_into` | Async methods returning execution plans; defaults return unsupported errors. | Relation-specific effects can use native DML planning while the provider owns storage semantics. |

`dyn TableProvider` also offers `is::<T>()` and `downcast_ref::<T>()`. These identify the concrete provider implementation, including through `Arc` auto-dereferencing; they do not validate its data. `DefaultTableSource`/`provider_as_source` bridges providers into logical `TableScan` sources; `source_as_provider` recovers a provider for compatible sources. This distinction permits logical planning to depend on the lighter `TableSource` abstraction without giving every source arbitrary execution authority. [Provider and scan APIs](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html), [source bridge](https://docs.rs/datafusion-catalog/55.1.0/src/datafusion_catalog/default_table_source.rs.html).

### 1.2 Schema, admission and lifetime

A provider's exposed schema participates in expression resolution, coercion, result-field derivation and physical planning. The returned plan and batches have to agree with that schema after requested projection. Field metadata can carry semantic references, but ordinary Arrow type compatibility does not establish quantity algebra, domain membership, foreign-key validity or ownership. Non-nullability is a real field contract; a custom producer still has to emit conforming arrays.

| Source arrangement | Potential benefit | Relevant boundary |
|---|---|---|
| Immutable admitted snapshot | Reuses already established facts across repeated planning and scans. | Admission must refer to the actual buffers, registry and parent context being retained. |
| Mutable source behind a stable exposed schema | Isolates storage evolution from consumers through schema adaptation. | Data consistency, version selection and adaptations are source/provider responsibilities. |
| Unpublished candidate relation | Allows native validation queries over potentially invalid rows. | Claims such as uniqueness cannot be borrowed from the schema merely because accepted data will satisfy them. |
| Completed intermediate computation | Reuses owned native outputs without serializing/reloading them. | Completion alone does not establish persisted-relation admission, canonical keys or publication authority. |

These distinctions matter directly in `pse-arrow`: `RelationTable` retains an admitted `Snapshot` and `LoadedRelation`; `CandidateTable` exposes unpublished field-checked input without trusted keys; `ComputedTable` retains a completed computation and exposes a temporary relation without inventing persisted constraints. The common query interface does not erase their different evidence states. [Admitted provider](</home/paul/pse-arrow/crates/pse-catalog/src/provider/table.rs>), [candidate provider](</home/paul/pse-arrow/crates/pse-catalog/src/session/candidate.rs>), [computed provider](</home/paul/pse-arrow/crates/pse-catalog/src/session/computed.rs>).

### 1.3 Scan semantics: width, rows and work

The pinned signature is `async scan(&self, &dyn Session, Option<&Vec<usize>>, &[Expr], Option<usize>) -> Result<Arc<dyn ExecutionPlan>>`. Projection indexes refer to the provider schema, in requested output order. None means all columns; an empty projection can still represent rows, as needed by count-oriented queries. Columns used only by pushed predicates may be absent from the output projection, so internal evaluation can require a wider input than the emitted schema.

Filters are conjunctive. For exact filtering, only SQL-true rows qualify; false and NULL do not. The limit applies after required filtering and may be implemented as an early-stop opportunity. The `scan` contract allows more than the requested limit, but cannot discard needed qualifying rows to meet it; a finite exhausted source naturally returns fewer. Without an ordering requirement, a limit does not define a stable choice of rows. The documentation's logical ordering is filter → limit → projection; an implementation may combine/reorder physical work only while preserving that meaning.

**Documentation discrepancy:** `ScanArgs::with_limit` describes an at-most bound, whereas `TableProvider::scan` explicitly allows extra rows and the default structured path delegates to it. A provider that emits exactly the requested count when available, or all qualifying rows on exhaustion, fits both descriptions. Treating every provider as a strict limiting operator is not supported by the delegated contract.

`scan` can inspect cached metadata and choose partitions, but bulk reads, row filtering and expensive numerical work during preparation bypass much of the execution plan's scheduling, streaming, resource and cancellation behavior. Deferring that work to native operators or execution streams can keep schema discovery and EXPLAIN cheap. This is a performance/lifecycle reason, not a claim that the async trait forbids planning-time I/O.

### 1.4 Exact, inexact and unsupported pushdown

| Classification | Engine reliance | Correctness boundary | Value |
|---|---|---|---|
| `Exact` | May remove the residual filter. | Every matching row is retained, every nonmatching row removed, with correct values and multiplicities. | Avoids redundant filtering; can eliminate source I/O or use a native filter near the source. |
| `Inexact` | Keeps a residual filter. | May retain false positives, but cannot lose true matches. | Enables conservative partition, file or index pruning without pretending it is complete evaluation. |
| `Unsupported` | Evaluates the predicate outside the scan. | No provider filtering claim. | Preserves correctness for expressions outside the implemented vocabulary. |

The classification vector length equals the number of input expressions. Support can depend on expression shape, types, casts, NULL semantics and available source context; a blanket acceptance of an operator name is rarely enough to establish exactness. Inexact pruning must still be lossless: checking only returned rows cannot reveal missing matches. A limit cannot safely move below an unresolved inexact predicate because early false positives can consume the row budget.

Projection pushdown, exact native filtering and physical I/O pruning provide different savings. A memory provider can return an exact `FilterExec` without reducing bytes already loaded. A file provider may skip row groups or columns. A remote provider needs semantic agreement with its remote evaluator. Dynamic filters created during execution and physical ordering/partitioning declarations belong to additional physical/source interfaces; overriding this logical pushdown method does not automatically implement those capabilities.

In `pse-arrow`, the inspected provider recognizes a bounded key/enum/reference predicate vocabulary and constructs native memory/filter/limit operators rather than a second predicate interpreter. Its source projection retains filter dependencies and restores requested output columns. This concentrates exactness reasoning around the native engine and the support classifier. [Support classifier](</home/paul/pse-arrow/crates/pse-catalog/src/provider/pushdown.rs>), [native scan construction](</home/paul/pse-arrow/crates/pse-catalog/src/provider/table.rs>).

### 1.5 Constraints, statistics and optimizer evidence

`Constraint` has exactly two variants here: `PrimaryKey(Vec<usize>)` and `Unique(Vec<usize>)`. Primary keys describe joint uniqueness and non-nullness; unique keys have different nullable dependency semantics. SQL parser support for other constraint syntax does not make foreign keys or checks members of this provider constraint model. `Constraints::new_unverified` checks no rows. `project` remaps surviving complete keys and drops those whose determinant columns are lost; it does not prove the projected data valid.

These declarations can become functional dependencies in logical schemas, enabling simplification and grouping/dependency reasoning. A false declaration can therefore affect semantics, not just metadata displays. Domain foreign keys, closure, cardinality and scientific invariants remain application contracts or validation relations. Attaching only facts already established by admission can let the optimizer exploit them without adding repeated validation scans. [Constraint and dependency implementation](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/functional_dependencies.rs.html).

`Statistics` distinguishes `Exact`, `Inexact` and `Absent` for individual facts such as row count, bytes and column statistics. Unknown differs from zero. Filtering, projection, partition selection and snapshot changes can invalidate or weaken statistics; a whole-artifact extremum need not remain exact for a filtered subset. Optimizer estimates, numerical domain bounds and physical feasibility are different kinds of information.

`ScanArgs` contains only **projection, optional filters, limit and statistics requests** in 55.1.0. Its getters/builders expose these fields; there is **no `preferred_ordering` field**, despite forward-looking text on `scan_with_args`. `ScanResult::{new, plan, into_inner}` wraps one execution plan. Statistics request variants are `Min`, `Max`, `NullCount`, `DistinctCount`, `Sum`, `ByteSize`, `RowCount`, `TotalByteSize`. They let custom logical rules communicate useful metadata demands through `TableScan` to custom providers. Built-in providers ignore these requests; the default `scan_with_args` adapter also drops them. There is no statistics payload in `ScanResult`: useful answers require integration with the returned plan/statistics machinery. This can avoid expensive indiscriminate metadata collection when only a few facts matter. [ScanArgs](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html), [request vocabulary](https://docs.rs/datafusion-expr-common/55.1.0/src/datafusion_expr_common/statistics.rs.html).

### 1.6 Execution context and implementation granularity

`Session` supplies configuration, catalog access, runtime environment, execution properties, scalar/aggregate/window/higher-order function registries, extension-type registry, table options, task context and physical plan/expression construction. It also has optimizer/planner/statistics integration points. This permits a provider to build native operators under the same context as the surrounding query. It does not inject domain registries, authorize an external source, or make custom allocations automatically memory-accounted. [Session interface](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/session.rs.html).

Existing `MemTable`, `ListingTable`, streaming providers and `ViewTable` cover common storage/composition cases. A wrapper or provider that returns standard operators can preserve native execution behavior with limited custom code. A bespoke `ExecutionPlan` offers greater control but carries additional obligations: schema/properties, partition semantics, output ordering, boundedness/emission, streaming/error behavior, resource accounting and safe cancellation/reuse. Buffer retention can avoid copies; merely returning Arrow arrays does not establish zero-copy transport or bounded memory.

`TableProviderFactory::create(&dyn Session, &CreateExternalTable)` is an async constructor for external-table workflows, distinct from a SQL table function. It can interpret location, declared schema and options to construct a provider. Persistence, physical table creation and registration behavior depend on the chosen factory/session workflow, not on the constructor trait alone.

### 1.7 Defaults and mutation

| Operation | Inputs beyond session | Provider responsibility / effect boundary |
|---|---|---|
| `insert_into` | Input `ExecutionPlan`, `InsertOp` (`Append`, `Overwrite`, `Replace`). | Append preserves old rows; Overwrite replaces all rows; Replace replaces colliding rows under source-defined key semantics. Supported modes and validation/commit behavior are provider-owned. |
| `delete_from` | `Vec<Expr>` filters. | Empty filter list means all rows; implement predicate and affected-row semantics. |
| `update` | `(column_name, Expr)` assignments and filters. | Resolve simultaneous assignment/source semantics consistently with the planned operation. |
| `truncate` | No row predicate. | Remove all rows under the source's consistency rules. |
| `merge_into` | Source plan, combined target-then-source `DFSchemaRef`, ON expression and action clauses. | Evaluate match/action semantics, ambiguity and storage effects using preserved qualifiers. |

The return convention is a plan producing one UInt64 `count` row. A count does not prove transactionality, constraint enforcement, retry safety or publication. These are source/application responsibilities. Planning an effectful operation and executing it are distinct, but session SQL/DDL APIs may perform some work eagerly; an “inspection” path is only read-only if the chosen API and admitted operation actually are.

Defaults are expressions supplied for insertion planning, not retroactive fills for existing rows or proof of physical validity. Their volatility, conversions and dependencies can affect results. Deriving provider defaults from the existing authored declaration avoids a second definition while leaving the provider responsible for honoring the accepted input contract.

For `pse-arrow`, immutable snapshot reads and private authoring/import effects have different boundaries; [blueprint §5.4, §22.2](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>) describes this separation. Native DML eligibility does not imply writes to bound snapshots or automatic conditional publication.

### 1.8 Cross-cutting example: read a heater run without changing its model

**Illustrative workflow, grounded in existing provider machinery.** A heater diagnostic reads `runtime.residuals` for a selected run and joins equation/source context. Those relation families are specified in [blueprint §6.13 and §11.2](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>); availability of a particular populated run is not assumed.

| Contract element | Application to this one workflow | Resulting value |
|---|---|---|
| Snapshot/schema binding | Resolve the run's admitted artifact and generated residual schema through `RelationTable`. | A result column cannot silently come from a different model revision or registry context. |
| Projection/filtering | Select run identity and needed residual columns; retain run-ID input internally even if omitted from output. Advertise exactness only if the bound contract/classifier supports that predicate. | Narrows diagnostic work while preserving all selected residual rows. |
| Keys/statistics | Expose established keys and available snapshot-scoped facts. | Join/group reasoning can reuse admission evidence; missing metadata does not masquerade as an empty result. |
| Execution | Use native memory/filter/limit operators over retained owners. | Planning can describe the diagnostic without executing the residual scan; repeated queries share admitted buffers. |
| Effect boundary | Treat filtering as a report selection over results. | It does not remove equations from the coupled solver problem or mutate the heater model. |

This consolidates read shape, identity, optimization and evidence without requiring a new heater-specific storage service. Whether it materially reduces I/O depends on where data is already resident and which physical source is used.

## 2. ViewTable and table functions as reusable relational models

### 2.1 Retained logical definition versus relation factory

| Aspect | `ViewTable` | `TableFunctionImpl` |
|---|---|---|
| Construction | `ViewTable::new(LogicalPlan, Option<String>)`. | `call_with_args(TableFunctionArgs) -> Result<Arc<dyn TableProvider>>`. |
| Reusable object | Fixed logical definition and optional creation text. | Named constructor whose arguments/context select an instantiated relation. |
| Schema | Taken from the retained plan at construction. | Supplied by the returned provider; may depend on planning-time arguments. |
| Transparency | Exposes the retained plan. | Depends on returned provider; returning a view retains logical transparency. |
| Data production | Physical planning/execution of the definition when read. | Delegated to the instantiated provider/plan. |
| Natural design value | Common derived semantics shared by queries and tests. | Common relation-construction semantics shared by parameterized uses. |

Neither abstraction is an automatic cache or materialized view. Reusing a definition differs from reusing its computed batches. Native `Expr`/`LogicalPlan` composition can represent the same computation without naming it; catalog views/functions add discoverability and a stable invocation surface where those have value. [ViewTable source](https://docs.rs/datafusion-catalog/55.1.0/src/datafusion_catalog/view.rs.html), [table-function source](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/table.rs.html).

### 2.2 View implementation and optimizer visibility

`ViewTable` stores `logical_plan`, `table_schema`, `definition`; `logical_plan()` and `definition()` expose references. Its provider reports `TableType::View`, borrows the logical plan and returns optional definition text. `new` does **not** analyze or coerce the supplied plan. A stored plan therefore has the validity established by its construction/preparation route, not by the wrapper.

The view reports all offered filters `Exact` because its scan builds native filter expressions over the view output. It clones the definition, conjoins filters, adds a projection when it differs from the full identity projection, adds an optional limit, and calls `Session::create_physical_plan`. This promises output filtering, not that every predicate can move to the underlying source. Aggregation, outer joins, windows, limits, nondeterminism and error behavior still constrain optimizer movement.

Exposing `get_logical_plan` also permits engine rules to inline/optimize the definition where supported. This can remove redundant intermediate work and expose source pushdown across a reusable boundary. It does not guarantee inlining, shared evaluation across repeated references, or preservation of every application metadata key. Output names, qualifiers, nested semantic fields and metadata derivation remain part of the plan contract.

### 2.3 Dependency binding, materialization and provenance

A retained logical plan can contain provider and function objects already resolved during construction. Re-registering a name does not necessarily retarget those objects. Conversely, a retained provider may expose mutable underlying data. “Fixed plan” therefore guarantees neither immutable data nor dynamic name rebinding. A reproducible derived model can bind admitted snapshot/function identities; a deliberately live view needs an explicit refresh/reconstruction interpretation.

Logical plans expose dependencies for inspection, but semantic identity also depends on source versions, function implementation/captured state, argument interpretation and relevant configuration. A view name plus EXPLAIN text is not a complete canonical fingerprint. Optional SQL is useful provenance but can differ from the currently bound objects or be absent for programmatic plans.

Materialization can avoid repeated expensive work and give multiple consumers one owned result. It introduces lifetime, freshness, invalidation and memory/storage costs. A view retains optimization flexibility and avoids premature data production, but may recompute. In `pse-arrow`, completed computation roles already provide an explicit intermediate reuse boundary; they can complement reusable definitions without treating every intermediate as a new authoritative relation. [Computed roles](</home/paul/pse-arrow/crates/pse-catalog/src/session/computed.rs>).

### 2.4 Complete table-function construction contract

`TableFunctionImpl: Debug + Send + Sync + Any` has two methods: deprecated `call(&[Expr])` and `call_with_args(TableFunctionArgs)`. The latter is the current interface; its default delegates to legacy `call`, whose default errors. `TableFunctionArgs::{new, exprs, session}` supplies logical expressions and `&dyn Session`. The wrapper `TableFunction::{new, name, function, create_table_provider_with_args}` retains a name and implementation; `create_table_provider` is the deprecated legacy path.

The call is **synchronous provider construction**, even though subsequent provider scanning is async. Expressions are not evaluated batches. This trait has no scalar-style `Signature`, automatic `coerce_types`, volatility, declared parameter schema or return-field hook. The implementation/planner integration determines accepted arity, literal/expression forms, type compatibility, constants, output schema and diagnostics.

| Construction choice | Value | Condition / limitation |
|---|---|---|
| Interpret literal IDs/options | Early errors and stable schema/context selection. | Nonliteral arguments need supported resolution or a clear refusal, not an unchecked literal assumption. |
| Embed argument expressions into a returned plan | Retains inspectable computation and standard downstream optimization. | References and parameter bindings must be meaningful in that plan's scope. |
| Return a `ViewTable` | Parameterized native relation with visible definition. | Still needs the application's analysis, source-binding and field-contract path. |
| Return a custom provider | Specialized source/layout/scan behavior. | Logical lineage and optimization stop at whatever contract it exposes. |
| Retain immutable resolved context | Cheap synchronous construction with repeatable bindings. | Context/version identity and lifetime remain explicit dependencies. |

Passing `Expr` does not imply arbitrary correlated/lateral row-by-row invocation, relation-valued parameters, or table-function macros that can splice any SQL syntax. Those require matching planner/runtime support. A row-dependent expansion may instead fit native UNNEST/lateral facilities or another operator. Expensive solving or remote retrieval inside construction would make name resolution execute the workload; a returned execution plan allows normal scheduling and cancellation instead.

### 2.5 Registration, discovery and application admission

`SessionContext`/`SessionState::register_udtf(name, Arc<dyn TableFunctionImpl>)` stores the wrapper in the session table-function map; the same key replaces the previous entry. Deregistration and `table_functions()` expose lifecycle/lookup. This is a session function registry, not automatically a catalog-qualified family of overloaded semantic declarations. Case/quoting and SQL name normalization still affect lookup.

A function can return a provider with a schema specific to one invocation; the function name alone cannot describe every resulting relation. Versioned domain metadata can document argument meanings and output contracts where the generic UDTF API does not. Repeated construction is not promised exactly-once behavior, and registration does not confer transaction, cache or scientific validity semantics.

In `pse-arrow`, retained-source and function-object admission is stronger than name lookup. Adding a library-valid view/function is not sufficient evidence that every sealed platform entry point accepts its returned source graph. The inspected `admit_scan` requires retained provider-object membership and recognizes admitted, candidate and computed providers, plus scoped recursive work tables; a generic `ViewTable` is not accepted merely by registration. A derived definition composed from admitted sources, or a deliberately integrated provider path, can preserve those bindings. Integration value comes from participating in existing preparation and provenance boundaries. [Preparation](</home/paul/pse-arrow/crates/pse-catalog/src/session/preparation.rs>), [source admission](</home/paul/pse-arrow/crates/pse-catalog/src/session/admission.rs>).

### 2.6 Cross-cutting example: a reusable heater balance diagnostic

**Proposed relational composition.** Continue the heater-run workflow from §1. A logical definition joins the selected run's recorded residuals with compiled equation ownership and provenance, derives labels and acceptance metrics, and returns one diagnostic row per selected equation. It consumes recorded model equations rather than inventing a universal heater equation that ignores accumulation, phase or configured balance choices.

A fixed view exposes that diagnostic for its bound snapshot. A table function such as `heater_balance_diagnostics(run_id, unit_id, threshold)` can validate planning-time selectors and return a `ViewTable` built from the same definition. These names are illustrative, not claims of existing registration. A compact output could contain `run_id, unit_id, equation_id, residual, scaled_residual, accepted, source_id`.

The reusable relation makes three capabilities work together: output fields remain inspectable before execution; ordinary filters/projections can narrow downstream reporting; fixtures and production diagnostics can query the same calculation. A raw/scaled residual distinction prevents an unqualified tolerance from conflating physical units with dimensionless scaling. A changed reporting threshold changes the diagnostic result but does not change solver termination or retroactively validate the run. Binding it explicitly makes that dependency visible.

If several consumers repeatedly request the same expensive join, retaining its completed computation may be useful; if each requests a small different subset, the visible view may preserve more optimization opportunity. Source ownership and admissibility remain governed by the existing session preparation path. Context: [blueprint §6.13, §14.3.1, §15.6](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>).

## 3. Catalogs and information_schema as semantic discovery

### 3.1 What “canonical registry” can mean here

The catalog is a namespace over provider objects; `information_schema` is a queryable projection of engine metadata. Neither automatically stores the full semantic ontology. In `pse-arrow`, `RelationSpec` and `reference.schema_*` already describe semantic schema authority. A DataFusion catalog can expose those declarations together with their bound data, giving discovery and execution a common access surface while preserving that authority.

This is valuable beyond convenient SQL: an agent can discover actual available relations, join their declarations to quantity/role/invariant data, and construct a plan against the same provider inventory. The claim is bounded by what the session exposes; absence can mean unavailable in this snapshot, not nonexistent in the global schema registry. [Blueprint §4.1–§4.2 and §5.4](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>), [registry materialization](</home/paul/pse-arrow/crates/pse-schema/src/builder.rs>).

### 3.2 Complete hierarchy interfaces

All three synchronous hierarchy traits require `Any + Debug + Send + Sync`; each also supports concrete-type inspection through `is`/`downcast_ref` helpers.

| Trait | Required operations | Optional/default behavior | Design significance |
|---|---|---|---|
| `CatalogProviderList` | `register_catalog`, `catalog_names`, `catalog`. | Registration returns a previous provider or None, with no `Result` error channel; no general deregister method in this trait. | One replaceable root inventory; mutation refusal cannot be expressed as a typed error through this signature alone. |
| `CatalogProvider` | `schema_names`, `schema`. | `register_schema`, `deregister_schema(name, cascade)` default to unsupported errors. | A catalog can project immutable or external metadata; cascade behavior is implementation-owned. |
| `SchemaProvider` | `table_names`, async `table`, `table_exist`. | `owner_name` defaults None; async `table_type` resolves the provider by default; `register_table`/`deregister_table` default to unsupported errors. | A cheap classification override avoids constructing expensive providers for inventory queries. |

`SchemaProvider::table` returns `Result<Option<Arc<dyn TableProvider>>>`, distinguishing absent from failed lookup. Memory implementations provide concurrent in-process inventories; their mutation behavior is not a durable database transaction system. Table registration's documented behavior rejects existing names; root/catalog registration and explicit replace workflows have different semantics. Dropping a namespace/provider binding does not necessarily delete source files or invalidate retained plans.

`TableReference` supports bare, schema-qualified and catalog/schema-qualified names. Defaults come from `datafusion.catalog.default_catalog` and `default_schema`; parser identifier normalization/quoting also matters. Full qualification reduces accidental default-dependent binding, while human-readable aliases remain useful if resolved to the intended immutable identity. [Catalog traits](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/catalog.rs.html), [schema trait](https://docs.rs/datafusion-session/55.1.0/src/datafusion_session/schema.rs.html).

### 3.3 Snapshots, async resolution and remote metadata

A `SessionContext::state()` clone is not a deep snapshot of every catalog/provider/runtime object: shared `Arc`s can retain mutable implementations. Namespace consistency therefore depends on the catalog design. An immutable catalog/provider inventory can bind one planning/execution context; a mutable registry needs a defined refresh/version policy so names, schemas and data do not change independently.

`AsyncSchemaProvider`, `AsyncCatalogProvider`, and `AsyncCatalogProviderList` supply async lookup and `resolve` helpers over table references plus `SessionConfig`. Resolution caches found and missing entries into short-lived ordinary providers for one query. This avoids repeated lookup and makes remote failures occur in an explicit resolution phase. The default helpers do not establish transactional consistency across remote reads, a complete global inventory, automatic refresh, eviction or parallel batching. A query-specific cache contains the references resolved for that query; using it as a complete discovery registry would misstate coverage.

`SessionContext::refresh_catalogs` specifically refreshes `ListingSchemaProvider` instances in the inspected release; it is not a generic refresh callback for arbitrary catalog implementations. Custom snapshot/remote refresh remains an application integration. Immutable snapshots can improve reproducibility but retain metadata/buffer owners longer; mutable caches can lower retention costs but require explicit staleness semantics. [Async catalog helpers](https://docs.rs/datafusion-catalog/55.1.0/src/datafusion_catalog/async.rs.html), [context lifecycle](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/context/mod.rs.html).

### 3.4 Exact information_schema coverage and limits

The pinned implementation exposes **seven** virtual tables when `datafusion.catalog.information_schema` is enabled; default configuration disables it. Session metadata queries are executable plans and can enumerate providers at execution, so their cost and coherence depend on the backing inventories. Some surfaces construct a complete batch after enumeration; a selective SQL predicate does not guarantee cheap remote metadata access.

| Virtual table | Actual supplied information | Value and limits |
|---|---|---|
| `tables` | Catalog/schema/name/type; includes information-schema tables. Uses `SchemaProvider::table_type`. | Inventory and classification without data scans when lookup is cheap. Classification is not admission or persistence evidence. |
| `columns` | Top-level field names, ordinal position, storage-type rendering, nullability and selected size/precision fields. | Generic structural discovery. Does not flatten arbitrary field/schema metadata or nested semantic relationships into rows. |
| `views` | Catalog/schema/name and nullable `definition` from `get_table_definition`. | Creation-text provenance. In 55.1.0 the implementation enumerates all ordinary providers without filtering by `TableType::View`; membership alone does not classify a view. |
| `schemata` | Catalog/schema names, optional owner, SQL-shaped character-set/path fields. | Namespace inventory; owner text does not implement authorization. |
| `df_settings` | `name`, nullable `value`, `description`; combines `ConfigOptions::entries()` with `RuntimeEnv::config_entries()`. | Inspection of reported effective configuration, including runtime entries; not every service handle or model input. |
| `routines` | Scalar/aggregate/window signatures summarized as rows, documentation when present, plus registered UDTF rows. | Function discovery; UDTFs report TABLE and conservative deterministic=false without rich argument/output contracts. |
| `parameters` | Argument/return-type examples from scalar/aggregate/window signatures; optional documented names, variadic marker and signature grouping. | Helps invocation discovery; it does not enumerate UDTF parameters or every possible semantic signature. |

`columns.column_default` is currently emitted as NULL rather than fetched from `get_column_default`; merely having a column in the metadata schema does not imply its data is populated. `routines`/`parameters` derive example argument types and can call field-derivation hooks with synthetic nullable fields and no scalar constants. A field-aware domain UDF can therefore have an unknown displayed return type despite being valid with real semantic fields. Higher-order function registrations are not enumerated by these scalar/aggregate/window metadata loops.

Function catalog/schema fields use configured defaults for session registries; they do not establish independently catalog-owned function namespaces. `routines.is_deterministic` means Immutable in this implementation; false also covers Stable/unknown metadata and does not establish scientific nondeterminism. SHOW commands provide convenience views over related metadata, not stronger completeness guarantees. [Versioned information-schema implementation](https://docs.rs/datafusion-catalog/55.1.0/src/datafusion_catalog/information_schema.rs.html).

### 3.5 Domain metadata relations and value cases

| Design need | Engine discovery contribution | Domain relation/context contribution | Value of combining them |
|---|---|---|---|
| Find usable input data | Actual tables and output fields. | Relation identity/version, authority, snapshot membership and source port. | Avoids constructing a valid query against the wrong artifact stage. |
| Understand a numerical field | Storage type and nullability. | Quantity kind/basis/reference, canonical units, per-row typing and domain axes. | Distinguishes structurally similar but incompatible values before numerical use. |
| Select a calculation | Registered function names and approximate signatures. | Kernel/package identity, applicability, derivative/backend support and outcome policy. | Makes discovery relevant to simulator design, beyond listing callable names. |
| Explain a validation failure | Queryable finding/result relations. | Invariant definition, severity, source identities and derivation links. | Produces localized diagnostics using the same keys as the calculation. |
| Compare versions | Provider inventories and schemas. | Schema versions, migrations, semantic IDs and dependencies. | Separates storage changes from changes in meaning or ownership. |
| Reproduce a run | Bound sources and reported settings. | Model/case/run identities, actual implementation bindings and environment provenance. | Associates observed results with their computation context. |

`reference.schema_relations`, `schema_columns`, `schema_logical_types`, `schema_enums`, `schema_invariants` and `schema_migrations` provide an existing architectural vocabulary for these purposes. A registered declaration is not evidence that all corresponding relation instances are loaded. Catalog wrappers can project this authority rather than maintaining a second editable metadata store. Generic SQL discovery can reduce bespoke API surface, but compiled typed access may still be preferable on hot paths; both can derive from the same registry.

### 3.6 Cross-cutting example: discover what a heater diagnostic means

**The same heater-run workflow, through discovery.** The catalog first identifies which residual, equation and provenance providers are bound. `information_schema.columns` supplies their structural interface. Registry relations then identify which fields are quantities, references and keys; equation/provenance data identifies the heater ownership and generated law. The diagnostic view/function can be discovered separately from the data it consumes.

A compact reasoning path is:

`available providers → relation/version declarations → field meanings and keys → heater equation/source links → diagnostic invocation → findings with source IDs`.

This enables an agent to select joins and labels from declared semantics rather than infer meaning from `Float64` columns or helper names. It also reveals a missing artifact or unsupported function before attempting the complete diagnostic. A UDTF's presence in `routines` is only the first step; its argument semantics still come from domain documentation/metadata.

**Observed code anchor:** `SnapshotCatalog`, `SnapshotSchema` and `SnapshotCatalogList` retain immutable maps over the seven declared namespaces. Duplicate producer-port bindings are rejected rather than silently selecting one. The sealed root registration implementation leaves membership unchanged because the upstream signature has no error channel; platform entry points carry the explicit effect rejection. `read_back_settings` uses a narrowly scoped trusted metadata query; enabling information-schema support does not imply unrestricted platform query admission. [Catalog](</home/paul/pse-arrow/crates/pse-catalog/src/provider/catalog.rs>), [schema](</home/paul/pse-arrow/crates/pse-catalog/src/provider/schema.rs>), [root](</home/paul/pse-arrow/crates/pse-catalog/src/provider/list.rs>), [settings inspection](</home/paul/pse-arrow/crates/pse-catalog/src/session/snapshot_session.rs>).

## 4. ConfigExtension as a canonical configuration interface

### 4.1 Typed options, not automatic policy

`ConfigExtension: ExtensionOptions` adds `const PREFIX: &'static str`. `ExtensionOptions: Send + Sync + Debug + 'static` defines `as_any`, `as_any_mut`, `cloned`, `set(key, value) -> Result<()>`, and `entries() -> Vec<ConfigEntry>`. An entry contains `key: String`, `value: Option<String>`, and a static description. The trait can represent mutable user options, validated policies or read-only projections of another contract; mutability is implemented, not inherent.

This can centralize how a setting is named, parsed, documented and consumed. It does not make an option effective: an analyzer, provider, UDF or operator must read it at a defined phase. It also does not choose precedence, semantic identity, acceptable ranges, cross-option consistency or which users may change it. [Configuration implementation](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/config.rs.html).

### 4.2 Root, storage and namespace routing

`ConfigOptions` is non-exhaustive, Clone/Default, with `catalog`, `execution`, `optimizer`, `sql_parser`, `explain`, `format`, **`spark`**, and custom `extensions`. Core methods include `new`, `with_extensions`, fallible `set`, `from_env`, `from_string_hash_map`, `entries`, and static `generate_config_markdown`.

`Extensions::{new, insert, get, get_mut, iter}` combines prefix-based storage with typed downcasting. Its underlying map is keyed by **prefix**, not Rust TypeId: inserting another extension with the same prefix replaces the previous entry even if its Rust type differs. Subsequent typed lookup can then return None. `insert` panics for the reserved prefix exactly `datafusion`. This makes namespace ownership relevant to correctness as well as public key stability.

`ConfigOptions::set` splits on the first dot. `datafusion.*` routes to built-in sections; another first component selects a registered extension and receives the remainder. A dotted prefix such as `datafusion.pse` can be stored and retrieved by type, but ordinary `set("datafusion.pse.x", ...)` routes to built-in `pse`, not that extension. A registered `datafusion_ffi` bridge provides a special fallback for unknown external prefixes; it does not turn built-in `datafusion.*` routing into arbitrary nested extension lookup.

Thus typed storage, string routing and displayed entry names are separate surfaces. Matching them enables one usable public configuration vocabulary; a deliberately read-only projection may expose names that are never valid mutation routes.

### 4.3 Macro generation and manual implementation

`extensions_options!` generates a non-exhaustive struct with declared field types/defaults/docs, Clone/Debug/Default, `ExtensionOptions` and `ConfigField` traversal/mutation. Field types need the relevant configuration field behavior. A separate `ConfigExtension` implementation supplies the prefix. Primitive parsing handles representation conversion; semantic range checks, cross-field relationships and context-dependent validation may need custom types or manual methods.

**Pinned behavior:** the generated `entries()` visits fields without prepending `PREFIX`; `ConfigOptions::entries()` appends extension entries unchanged. A macro-defined extension can therefore be writable through `prefix.field` while reporting a bare `field` entry. A manual implementation/wrapper can expose full stable names when discovery and identity need them. This is why successful typed lookup alone is insufficient verification of public introspection.

`cloned()` explicitly requires a deep clone that does not share mutable configuration. A generated Clone is only as independent as the fields' Clone implementations; an `Arc<Mutex<_>>` field would still share mutation. Immutable shared references can be appropriate, but mutable runtime services belong to a different lifecycle. Returning errors from `set` is a valid read-only contract. Direct `get_mut`/public-field mutation can bypass checks implemented only in the string setter, so a canonical validation boundary matters if more than one construction path exists.

### 4.4 Loading, defaults, validation and reproducibility

| Path | Actual 55.1.0 behavior | Design implication |
|---|---|---|
| `new` / Default | Built-in defaults, empty custom extensions. | Extension defaults appear only after insertion. |
| `set` | Fallible namespaced dispatch and implementation-defined parsing. | Can report unknown keys/invalid values; a sequence of sets is not a transaction. |
| `from_env` | Enumerates built-in keys, uppercases and replaces dots with underscores; reads once. Calls `set(...)?`. | Unknown/custom keys are not loaded. The code propagates setting errors despite prose describing warning/default fallback. |
| `from_string_hash_map` | Extracts known built-in keys; ignores unrelated/custom entries. | Does not establish that every supplied option was accepted. |
| Registered-extension loading | Insert typed extension, then apply accepted keys via its chosen route. | Supports explicit key validation and precedence; custom file/environment mapping is additional integration. |
| `entries` | Built-in entries plus extension-provided entries, preserving optional absence. | Useful for diagnostics/identity if names are unique and actual consumers agree. |
| `generate_config_markdown` | Builds a fresh default root, with normalized display defaults. | Does not document extensions installed in some existing session. Custom documentation can be rendered from that session/extension's entries. |

A cloned candidate configuration can be fully parsed and cross-validated before becoming active, avoiding a half-applied set of options after an error. This adds a preparation step but makes failure behavior and effective values easier to explain. Parsing a positive integer, validating a memory budget and deciding an acceptable scientific tolerance are different checks.

Default → environment → file → explicit argument precedence is not supplied as one universal policy. Neither are config-file parsing, secrets management, migrations of renamed keys or full serialization/replay of arbitrary extensions. An effective settings record can capture resolved values, including absence, but may still need origin/version information to explain how defaults were selected. Enumerable settings are a poor place for unredacted credentials because diagnostics and metadata queries expose them.

### 4.5 Consumers, snapshots and refresh

| Consumer/phase | What can be read or captured | Why phase matters |
|---|---|---|
| Analyzer/logical optimizer | `ConfigOptions` while deriving/replacing plans. | A change can alter the analyzed plan or constant-folded result; later execution settings cannot undo that transformation. |
| Table function/provider planning | Session options while constructing providers/plans. | Selected sources/schema/layout may be captured in the resulting object. |
| Scalar UDF setup | `with_updated_config` may return a replacement instance. | Session builder and SQL SET/RESET paths refresh supporting scalar UDFs; arbitrary direct mutation is not a universal refresh signal. |
| Scalar invocation/task execution | Supplied configuration/task context. | Runtime reads may differ from values captured in old plans unless lifecycle is coherent. |
| Physical operators/runtime | Plan-time decisions and runtime resources. | Partitions, memory pools and service state need not all be mutable through `ConfigOptions`. |
| Metadata inspection | Reported configuration plus runtime entries. | A later inspection is not automatically the record of an earlier query. |

`SessionContext::state()` clones state and marks a query start time; changes to that returned configuration do not update the context. Correct extension cloning helps isolate query settings. Catalog/provider/runtime `Arc`s can still refer to shared objects, so this is not a full world snapshot. SQL runtime settings under `datafusion.runtime.*` have a separate context path. `RESET` built-in handling is not a general extension-reset protocol: `ExtensionOptions` has no reset method.

Configuration-sensitive function refresh changes registry bindings, not necessarily function objects retained by already-built plans. Capturing an immutable policy in a provider/UDF can simplify reproducibility; reading ambient mutable state can support live controls but makes semantics and cache dependencies harder to establish. The appropriate choice depends on whether an option controls operational behavior or calculation meaning. [Session/context behavior](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/context/mod.rs.html), [state construction](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/session_state.rs.html).

### 4.6 Config extensions, session objects and table options

| Mechanism | Suitable content and value | Boundary |
|---|---|---|
| `ConfigOptions.extensions` | Named typed options with parsing/enumeration. | No automatic effect, persistence or model versioning. |
| Opaque session extensions | Immutable registries, handles, caches, tracing/context objects. | Not automatically key-addressable, documented or visible in `df_settings`; shared-object lifecycle remains relevant. |
| `TableOptions.extensions` | Table/file handling configuration in external-table/format workflows. | Separate container and dispatch path from session options. |
| Explicit model/case/kernel relations | Component sets, property packages, parameter/reference definitions and other meaning-bearing inputs. | Can supply stable semantic dependencies rather than hidden session defaults. |

`register_table_options_extension` is available on context/state; table options also expose built-in CSV/JSON/Parquet options, format selection, `set`, `entries`, `with_extensions` and map-based updates. Registration in one container does not register the other. In 55.1.0, custom `TableOptions::set` forwards the **full key** to the extension, while `ConfigOptions::set` strips its prefix; a shared implementation may need to account for both routes. `combine_with_session_config` copies the relevant built-in Parquet settings, not all session extensions. These differences matter when one option schema is reused across ingestion and session APIs. [TableOptions source](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/config.rs.html).

### 4.7 Canonical configuration in pse-arrow

The inspected code already separates `ExecutionSettings`, thread budgeting, `PseOptions` and retained settings identity. `build` uses fallible `ConfigOptions::set`, enables information-schema support, and selects explicit catalog defaults. `PseOptions` is a manual read-only extension: its entries report `datafusion.pse.null_policy` and `datafusion.pse.kernel_outcome_policies`; its setter refuses overrides. Its dotted prefix is compatible with typed retrieval/enumeration, not a generic writable namespace under the upstream router.

`semantic_settings` currently retains the entire supplied inventory, including absence; `settings_hash` hashes ordered key/value entries with explicit absence tagging. `read_back_settings` queries actual `df_settings` and compares it to the sealed inventory. These source observations support the value of inspectable bound policy, but do not establish runtime qualification or that every future kernel policy is already implemented.

A proposal for arbitrary mutable `process.*` tolerance/package options would therefore need to consider existing contract ownership. Report-only choices, execution strategy and model-defining semantics can have different homes. Projecting a frozen kernel policy into settings can make it discoverable without creating a second writable authority. [Configuration implementation](</home/paul/pse-arrow/crates/pse-catalog/src/session/config.rs>), [sealed session](</home/paul/pse-arrow/crates/pse-catalog/src/session/snapshot_session.rs>).

### 4.8 Cross-cutting example: bind the heater diagnostic's policy and execution context

**Continue the same diagnostic.** The selected run retains its model/case/backend identity and actual solver options; those define the run being reported. The diagnostic's explicit threshold controls its reporting result. Session batch size, partition budget and bound semantic policies determine how its query is prepared/executed. These dependencies overlap, but are not interchangeable.

The existing typed settings builder can construct the execution context; the read-only `PseOptions` projection can expose bound semantics; `read_back_settings` can check what the engine reports. Capturing the effective inventory alongside the prepared diagnostic makes a later mismatch explainable. A changed threshold can be represented as a new diagnostic invocation without pretending the underlying solve used that threshold. A changed property package remains a model/kernel dependency rather than a hidden settings override.

This yields concrete design value: configuration validation happens before expensive diagnostics; different consumers share the same reported policy; provenance can distinguish a changed report from a changed physical model; and settings updates do not silently redefine an already-bound run. Whether a future option is writable, frozen per query or derived from another contract remains a codebase decision informed by its consumers and semantic consequences.

## 5. Combined lifecycle and evidence

### 5.1 Shared contracts across preparation, execution and discovery

| Stage | Relevant contract | Design payoff |
|---|---|---|
| Resolve context | Actual catalog/provider/function objects, snapshot identities and effective settings. | Stable interpretation before planning. |
| Construct relation | Native plan, view or table-function output with explicit schema and dependencies. | One inspectable definition for downstream composition. |
| Analyze/admit | Field/domain compatibility plus the application’s source/effect boundaries. | Early localized refusal; no inference that registration equals validity. |
| Optimize/scan | Truthful keys, pushdown classifications, statistics and source properties. | Reduced work without silently changing row coverage or computation meaning. |
| Execute/retain | Native streams, ownership, cancellation, result/error contracts. | Reusable outputs with an explicit completion boundary. |
| Validate/publish/report | Residual obligations, numerical acceptance, conditional publication and provenance. | Distinguishes a valid query from a valid artifact or scientifically acceptable run. |
| Discover | Engine metadata joined to domain registry relations. | Agents and tooling reason about the same environment used by execution. |

There need not be one registry object for all these facts. A useful unification is one authoritative declaration per meaning with derived DataFusion surfaces, so changes propagate consistently without merging unrelated lifecycles.

### 5.2 Focused validation and measurement opportunities

| Evidence | What it establishes / value |
|---|---|
| Full-result comparison of exact/inexact scans with an otherwise equivalent unsupported-pushdown baseline | Detects lost matches, duplicate changes and NULL/projection/limit defects; checking only returned rows misses over-pruning. |
| Schema/owner checks across scans and snapshot replacement | Detects stale fields, mismatched bindings and retention problems before higher-level numerical debugging. |
| View/function composition compared with direct logical construction | Exercises actual source/function bindings, metadata and optimizer interactions without duplicating the entire domain calculation in a helper. |
| Catalog enumeration/lookup checks, including misses and duplicate names | Establishes whether discovery matches what can be planned and whether a query cache is being mistaken for a full inventory. |
| Typed get, string set, entries and `df_settings` comparison | Reveals routing, prefix, absence and display inconsistencies that typed access alone cannot detect. |
| Clone/change/prepare/execute cases for relevant settings | Establishes which phase captures policy and whether old plans remain coherent after registry updates. |
| Workload measurements of bytes read, planning latency, allocations and repeated execution | Distinguishes available optimization hooks from realized benefit for the simulator's actual access patterns. |

These mechanisms can reduce the number of independent representations that tests must reproduce. They do not eliminate end-to-end tests, prove mathematical correctness, or turn declaration coverage into implementation completeness.

### 5.3 Companion references

The [logical-planning reference](</home/paul/Documents/Codex/2026-09-14/i-want-to-communicate-to-an/outputs/datafusion_logical_planning_capability_spec.md>) covers native operators and general plan-based validation. The [semantic-analysis, types and UDF reference](</home/paul/Documents/Codex/2026-09-14/i-want-to-communicate-to-an/outputs/datafusion_semantic_analysis_types_udfs_capability_spec.md>) covers field semantics, analyzers and optimizer-aware numerical functions. This document supplies their relation, namespace, reusable-definition and configuration integration boundaries.
