---
title: DataFusion provider contracts
pins:
  datafusion: 55.1.0
  arrow: 59.3.0
  object_store: 0.13.2
  sqlparser: 0.62.0
regenerated: 2026-09-15
evidence: Interface-checked
---

# DataFusion Provider Contracts — v55.x

## How to read this document

Every claim carries a source class. Do not collapse them.

| Class | Means |
|---|---|
| **Source** | copied verbatim from the pinned crate source; the file is named |
| **Index** | an enumeration closed against the generated API index; the query is given |
| **Guide** | upstream prose or an upgrade guide — a lead, never proof of a signature |
| **Recommendation** | this document's architectural advice, not a library fact |

`⚠` marks a trap that silently produces wrong behaviour instead of an error.
`▲` marks an item whose shape changed inside the 49→55 window; §A is the consolidated list.

Paths are canonical (`datafusion_session::table::TableProvider`). **`datafusion::…` is an
access path, not a definition site**; it re-exports and says nothing about ownership.

Source references are relative to the vendored registry root, written `REG/` below:
`~/.cargo/registry/src/<registry-hash>/`. Index queries are relative to a pinned API index
whose layout is given in §C.

---

# 0. Mental model

## 0.1 The hierarchy

```text
CatalogProviderList                    // catalog namespace   — NO error channel
└── CatalogProvider                    // schema namespace    — synchronous, must not do I/O
    └── SchemaProvider                 // table namespace     — table() is async
        └── TableProvider              // relation + planning/execution contract
            └── ExecutionPlan          // physical execution
                └── DataSource         // reusable scan body
                    └── FileSource     // per-format opener + pushdown
                        └── ObjectStore // bytes
```

A reference `catalog.schema.table` resolves down the top three layers. Those are
**namespace/metadata registries**. `TableProvider` is the boundary at which the catalog model
becomes a **query-planning + execution contract**. `DataSource`/`FileSource` are the reusable
body most providers should assemble rather than author. DataFusion deliberately leaves
sophisticated catalog management to applications.

## 0.2 The full stack, including the paths the hierarchy diagram hides

```text
        SQL text                              DataFrame / LogicalPlanBuilder
            |                                              |
  resolve_table_references(stmt, normalize)                |
            |  (Vec<TableReference>, Vec<TableReference>)  |
            |   sorted, deduped, CTEs returned separately  |
            v                                              |
  ┌── batch resolution (optional) ──┐                      |
  │ AsyncCatalogProviderList        │                      |
  │   ::resolve(refs, config)       │ → immutable snapshot |
  └────────────┬────────────────────┘                      |
               v                                           |
   CatalogProviderList ── CatalogProvider ── SchemaProvider |
               |                                           |
               |  SchemaProvider::table(name).await        |
               v                                           v
      Arc<dyn TableProvider> ──provider_as_source──> Arc<dyn TableSource>
               |                                           |
               |                                 LogicalPlan::TableScan
               |                                 { projection, filters, fetch,
               |                                   statistics_requests }
               |                                           |
               |                   analyzer → optimizer — LOGICAL pushdown asks
               |                   TableSource::supports_filters_pushdown
               |                                           v
               └── scan_with_args(state: &dyn Session, ScanArgs) ──> ScanResult
                                                           |
                                                Arc<dyn ExecutionPlan>
                                                           |
                            physical optimizer — PHYSICAL pushdown, two phases:
                            gather_filters_for_pushdown / handle_child_pushdown_result
                            + DynamicFilterPhysicalExpr from TopK / joins / aggregates
                                                           |
                                                       execute()
```

Four things in that picture are absent from a catalog-trait-only account, and each is a
separate contract here:

1. **`TableSource`, not `TableProvider`, is what logical planning sees** (§7). The optimizer
   asks a `TableSource` about pushdown; execution asks the `TableProvider`. They are the same
   object only because `DefaultTableSource` forwards.
2. **There are two filter-pushdown systems** — logical (§9) and physical (§11). A provider
   that implements `supports_filters_pushdown` and stops uses only the first.
3. **There are two resolution paths.** The synchronous traits have nowhere to await, so
   remote metadata is resolved in a batch *before* planning and handed to the planner as an
   immutable snapshot (§19).
4. **There is a second route from a scan to a physical plan** that bypasses `TableProvider`
   entirely: a custom `TableSource` planned by `ExtensionPlanner::plan_table_scan` (§7.4).

## 0.3 Where the contracts live ▲

**Guide + Index.** 55.0.0 moved `CatalogProviderList`, `CatalogProvider`, `SchemaProvider`,
`TableProvider`, `TableProviderFactory`, `TableFunctionImpl` and the `TableFunction` struct
into **`datafusion-session`**, so they are reachable from `Session` without downcasting to
`SessionState`, including across FFI. `QueryPlanner`, `PhysicalPlanner`, `ExtensionPlanner`,
`PhysicalOptimizerRule` and `PhysicalOptimizerContext` moved with them, and their session
argument changed from `&SessionState` to `&dyn Session`.

`datafusion-catalog` re-exports every one of them as one-line `pub use`s, and
`datafusion::catalog` is `pub use datafusion_catalog::*`. `datafusion::catalog::TableProvider`,
`datafusion::datasource::TableProvider` and `datafusion_catalog::CatalogProvider` all still
compile. **Only the canonical module moved.**

⚠ **Canonical path ≠ usable import.** Several `datafusion-catalog` modules are private
(`mod catalog; mod schema; mod table; mod r#async; mod dynamic_file;`) with `pub use <mod>::*`
at the crate root. So `datafusion_catalog::UrlTableFactory` compiles while
`datafusion_catalog::dynamic_file::catalog::UrlTableFactory` — the *canonical* path rustdoc
reports, and the one this document uses for identity — does not. See §C.1: a generated alias
index does not record re-exports that glob through a private module, so **the absence of an
alias is not evidence that the import does not work.**

`Session` gained a **required** `catalog_list()` in the same release;
`datafusion_session::catalog::EmptyCatalogProviderList` exists so a custom `Session` with no
catalog can satisfy it.

## 0.4 Runtime type inspection ▲

**Source.** `as_any` was **removed** from `CatalogProviderList`, `CatalogProvider`,
`SchemaProvider`, `TableProvider`, `TableSource`, `FileSource`, `FileFormat`,
`FileFormatFactory`, `DataSource`, `DataSink`, `ExecutionPlan` and `PhysicalExpr` in 54.0.0.
Material written before 54 shows it as a required method. It is gone — do not implement it.

Those traits take `Any` as a **supertrait** and expose inherent methods on the `dyn` type:

```rust
// REG/datafusion-session-55.1.0/src/catalog.rs — identical blocks exist for
// dyn CatalogProvider, dyn SchemaProvider, dyn TableProvider, dyn TableSource,
// dyn DataSource, dyn FileSource, dyn DataSink
impl dyn CatalogProviderList {
    pub fn is<T: CatalogProviderList>(&self) -> bool { (self as &dyn Any).is::<T>() }
    pub fn downcast_ref<T: CatalogProviderList>(&self) -> Option<&T> { (self as &dyn Any).downcast_ref() }
}
```

⚠ These work through `Arc<dyn …>` auto-deref. `(&arc as &dyn Any).downcast_ref()` downcasts
the `Arc` itself and always fails. Write `provider.downcast_ref::<T>()`.

**`Session` is the exception** and keeps a required `fn as_any(&self) -> &dyn Any`. That is
why `ListingTableFactory` and `ListingTableConfigExt` can reach `SessionState` (§6.3).

## 0.5 The asymmetry that drives most of this document

```text
CatalogProviderList::register_catalog  -> Option<Arc<dyn CatalogProvider>>   no Result
CatalogProviderList::catalog           -> Option<...>                        no Result
CatalogProvider::schema                -> Option<...>                        no Result, sync
CatalogProvider::register_schema       -> Result<Option<...>>
SchemaProvider::table                  -> Result<Option<...>>                async
SchemaProvider::register_table         -> Result<Option<...>>
```

The top of the hierarchy has no way to report a failure and no way to await. Every decision
in §19 (remote catalogs) and §21.C (absence vs. error) follows from this.

---

# 1. Contract surface map

**Index** — regenerate the whole enumeration rather than trusting a hand-written list:

```bash
# every trait defined in a provider-owning crate
awk -F'\t' '$2=="trait" && $3 ~ /^datafusion-(session|catalog|catalog-listing|datasource|pruning|physical-expr-adapter|ffi)$/ {print $3"\t"$1}' \
  content/index/symbols.tsv | sort
# implementors of one trait
rg -P '^<canonical trait path>\t' content/index/impls.tsv | cut -f2
```

Counts below are **required / provided / in-tree implementors**, verified 2026-09-15.
Implementor counts are a *lower bound*: private and test-only types are not indexed
(`ArrowFileSink`, the `information_schema` tables and three test providers are all real and
all absent from the index).

## 1.1 Namespace and binding

| Trait | Canonical path | R/P/I | § |
|---|---|---:|---|
| `CatalogProviderList` | `datafusion_session::catalog::CatalogProviderList` | 3/0/5 | §2 |
| `CatalogProvider` | `datafusion_session::catalog::CatalogProvider` | 2/2/3 | §3 |
| `SchemaProvider` | `datafusion_session::schema::SchemaProvider` | 3/4/6 | §4 |
| `Session` | `datafusion_session::session::Session` | 16/6/2 | §6 |
| `TableSource` | `datafusion_expr::table_source::TableSource` | 1/5/2 | §7.1 |
| `ContextProvider` | `datafusion_expr::planner::ContextProvider` | 11/7/0 | §7.3 |
| `AsyncCatalogProviderList` | `datafusion_catalog::async::AsyncCatalogProviderList` | 1/1/0 | §19 |
| `AsyncCatalogProvider` | `datafusion_catalog::async::AsyncCatalogProvider` | 1/1/0 | §19 |
| `AsyncSchemaProvider` | `datafusion_catalog::async::AsyncSchemaProvider` | 1/1/0 | §19 |

## 1.2 Relation and construction

| Trait | Canonical path | R/P/I | § |
|---|---|---:|---|
| `TableProvider` | `datafusion_session::table::TableProvider` | 3/12/10 | §8 |
| `TableProviderFactory` | `datafusion_session::table::TableProviderFactory` | 1/0/5 | §16.1 |
| `TableFunctionImpl` | `datafusion_session::table::TableFunctionImpl` | 0/2/3 | §16.2 |
| `UrlTableFactory` | `datafusion_catalog::dynamic_file::catalog::UrlTableFactory` | 1/0/1 | §16.3 |
| `StreamProvider` | `datafusion_catalog::stream::StreamProvider` | 3/1/1 | §17.5 |
| `PartitionStream` | `datafusion_physical_plan::streaming::PartitionStream` | 2/0/1 | §17.4 |

## 1.3 Execution and the datasource fabric

| Trait | Canonical path | R/P/I | § |
|---|---|---:|---|
| `ExecutionPlan` | `datafusion_physical_plan::execution_plan::ExecutionPlan` | 6/29/47 | §13.1 |
| `DataSource` | `datafusion_datasource::source::DataSource` | 9/10/2 | §14.1 |
| `FileSource` | `datafusion_datasource::file::FileSource` | 6/14/5 | §14.4 |
| `FileFormat` | `datafusion_datasource::file_format::FileFormat` | 7/3/5 | §14.6 |
| `FileFormatFactory` | `datafusion_datasource::file_format::FileFormatFactory` | 2/0/5 | §14.6 |
| `FileOpener` | `datafusion_datasource::file_stream::FileOpener` | 1/0/4 | §14.5 |
| `Morselizer` / `MorselPlanner` / `Morsel` | `datafusion_datasource::morsel::*` | 1/0/0 each | §14.7 |
| `SchemaAdapterFactory` / `SchemaAdapter` / `SchemaMapper` | `datafusion_datasource::schema_adapter::*` | ▲ dead | §14.8 |
| `PhysicalExprAdapterFactory` / `PhysicalExprAdapter` | `datafusion_physical_expr_adapter::schema_rewriter::*` | 1/0/1 each | §14.8 |
| `PruningStatistics` | `datafusion_common::pruning::PruningStatistics` | 6/0/4 | §12.1 |
| `ParquetFileReaderFactory` | `datafusion_datasource_parquet::reader::ParquetFileReaderFactory` | 1/0/— | §12.4 |
| `ObjectStoreRegistry` | `datafusion_execution::object_store::ObjectStoreRegistry` | 2/1/1 | §15.1 |
| `MemoryPool` | `datafusion_execution::memory_pool::MemoryPool` | 5/3/— | §15.4 |
| `StatisticsProvider` | `datafusion_physical_plan::operator_statistics::StatisticsProvider` | 1/0/— | §13.4 |

## 1.4 Write and transport

| Trait | Canonical path | R/P/I | § |
|---|---|---:|---|
| `DataSink` | `datafusion_datasource::sink::DataSink` | 2/2/4 | §18.3 |
| `FileSink` | `datafusion_datasource::file_sink_config::FileSink` | 2/1/3 | §18.4 |
| `BatchSerializer` | `datafusion_datasource::write::BatchSerializer` | 1/0/2 | §18.4 |
| `Decoder` / `BatchDeserializer` | `datafusion_datasource::decoder::*` | 3/0/2, 3/0/1 | §14.5 |
| `LogicalExtensionCodec` | `datafusion_proto::logical_plan::LogicalExtensionCodec` | 4/10/7 | §20.2 |
| `PhysicalExtensionCodec` | `datafusion_proto::physical_plan::PhysicalExtensionCodec` | 2/10/3 | §20.2 |

## 1.5 Deliberately out of scope

Named so their absence is a decision, not an omission: `ScalarUDFImpl`, `AggregateUDFImpl`,
`WindowUDFImpl`, `Accumulator`, `GroupsAccumulator`, `PartitionEvaluator`, `AnalyzerRule`,
`OptimizerRule`, `ExprPlanner`, `TypePlanner`, `RelationPlanner`,
`UserDefinedLogicalNode(Core)`, `QueryPlanner`, `PhysicalPlanner`, `PhysicalOptimizerRule`,
`FunctionFactory`, `FileTypeExt`, `ExtensionOptionsFFIProvider`, `UnhandledPredicateHook`.
These are function, planning and optimizer extension points rather than relation contracts.
`ExtensionPlanner` is out of scope *except* for `plan_table_scan`, which is in §7.4 because
it is a provider-bypass route.

---
# 2. `CatalogProviderList`

**Source** — `REG/datafusion-session-55.1.0/src/catalog.rs`. Aliases:
`datafusion_session::CatalogProviderList`, `datafusion_catalog::CatalogProviderList`,
`datafusion::catalog::CatalogProviderList`.

```rust
pub trait CatalogProviderList: Any + Debug + Sync + Send {
    /// Adds a new catalog to this catalog list.
    /// If a catalog of the same name existed before, it is replaced in the list and returned.
    fn register_catalog(
        &self,
        name: String,
        catalog: Arc<dyn CatalogProvider>,
    ) -> Option<Arc<dyn CatalogProvider>>;

    fn catalog_names(&self) -> Vec<String>;

    fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>;
}
```

Three required methods, **zero provided**, no `as_any`.

| Fact | Consequence |
|---|---|
| **No error channel anywhere.** `register_catalog` returns the *displaced* catalog as `Option`, not `Result`; `catalog()` returns `Option` | "catalog absent" and "catalog lookup failed" are indistinguishable. A remote list that cannot reach its backend can only return `None` — the user then sees `failed to resolve catalog: {name}` from `SessionState::schema_for_ref` — or panic |
| All three take `&self` | Interior mutability is mandatory. `MemoryCatalogProviderList` uses `DashMap` |
| `catalog()` is synchronous | No remote I/O here. See §19 |
| Returning `None` from `register_catalog` without inserting | ⚠ Not a rejection protocol — it is indistinguishable from "there was nothing to displace". There is nowhere to put a refusal |

## 2.1 Implementors

**Index:** `rg -P '^datafusion_session::catalog::CatalogProviderList\t' content/index/impls.tsv`

| Implementor | Notes |
|---|---|
| `datafusion_catalog::memory::catalog::MemoryCatalogProviderList` | `pub catalogs: DashMap<String, Arc<dyn CatalogProvider>>` — a **public field**. ⚠ `catalog_names()` iterates a `DashMap`, so **enumeration order is nondeterministic**; anything rendering sorted metadata must sort itself |
| `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog` | §16.3 |
| `datafusion_session::catalog::EmptyCatalogProviderList` ▲ | New in 55.0.0. Unit struct; `register_catalog` returns `None` and **silently discards**, `catalog_names()` is empty, `catalog()` is `None`. Exists so a custom `Session` can satisfy the new required `catalog_list()` |
| `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList` | §20.1 |
| `datafusion_ffi::tests::catalog::FixedCatalogProviderList` | test double |
| *(private)* `ResolvedCatalogProviderList` | §19 — ⚠ `register_catalog` is `unimplemented!()` and **panics** |

## 2.2 Binding a list to a session

**Source** — `REG/datafusion-55.1.0/src/execution/session_state.rs`, `.../execution/context/mod.rs`.

```rust
SessionStateBuilder::with_catalog_list(Arc<dyn CatalogProviderList>) -> Self
SessionState::catalog_list(&self) -> &Arc<dyn CatalogProviderList>        // inherent, borrows
Session::catalog_list(&self) -> Arc<dyn CatalogProviderList>              // trait, owned Arc
SessionState::register_catalog_list(&mut self, Arc<dyn CatalogProviderList>)   // replaces the list
SessionContext::register_catalog_list(&self, Arc<dyn CatalogProviderList>)     // replaces the list
```

Two methods share the name `catalog_list` with different signatures. A provider holding
`&dyn Session` gets the owned-`Arc` trait method.

Default when none is supplied: `MemoryCatalogProviderList::new()`.

⚠ **The default-catalog bootstrap overwrites yours.** If
`datafusion.catalog.create_default_catalog_and_schema` is true (the default),
`SessionStateBuilder::build` constructs a `MemoryCatalogProvider` holding a
`MemorySchemaProvider` and calls `register_catalog(config.options().catalog.default_catalog, …)`.
It only logs `debug!("Overwrote the default catalog")`. If you supply a list that already
contains a catalog named `datafusion`, call `.with_create_default_catalog_and_schema(false)`.
`SessionStateBuilder::from(existing_state)` already flips that flag off when the default
catalog exists, so round-tripping a context does not clobber it.

### Recommendation

The root list has no error channel, so a mutation *policy* cannot be expressed through its
return type. Keep a conforming list as private assembly state and enforce admission through
fallible commands of your own before anything reaches `register_catalog`.

---

# 3. `CatalogProvider`

**Source** — `REG/datafusion-session-55.1.0/src/catalog.rs`.

```rust
pub trait CatalogProvider: Any + Debug + Send + Sync {
    fn schema_names(&self) -> Vec<String>;                                  // required
    fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>;        // required

    fn register_schema(                                                     // provided
        &self,
        name: &str,
        schema: Arc<dyn SchemaProvider>,
    ) -> Result<Option<Arc<dyn SchemaProvider>>> {
        not_impl_err!("Registering new schemas is not supported")
    }

    fn deregister_schema(                                                   // provided
        &self,
        name: &str,
        cascade: bool,
    ) -> Result<Option<Arc<dyn SchemaProvider>>> {
        not_impl_err!("Deregistering new schemas is not supported")
    }
}
```

Semantics:

- `schema_names()` = enumerable namespace snapshot.
- `schema(name)` = cheap synchronous lookup; `None` means absent.
- `register_schema` defaults to unsupported. If implemented, DataFusion's contract permits
  **replacement**: return the previously registered schema.
- `deregister_schema` defaults to unsupported. `cascade=false` should reject removal of a
  non-empty schema; `cascade=true` permits dependent tables to be removed. Missing schema →
  `Ok(None)`, **not** an error.
- There is deliberately no asynchronous failure path in `schema()`: a remote implementation
  **must not perform network I/O here**.

⚠ A provider that implements neither mutator still satisfies `CREATE SCHEMA` at the type
level and fails at runtime with `NotImplemented`.

**Source — the reference cascade semantics.** `MemoryCatalogProvider::deregister_schema`
implements PostgreSQL `CASCADE`: non-empty and `cascade == false` →
`exec_err!("Cannot drop schema {name} because other tables depend on it: {…}")`; a missing
schema → `Ok(None)`. The trait doc instructs custom implementors to match this.

**Index — implementors (3):** `MemoryCatalogProvider`, `ForeignCatalogProvider` (FFI),
`FixedCatalogProvider` (test), plus the private `ResolvedCatalogProvider` (§19) and the
private `DynamicFileCatalogProvider` (§16.3).

### Standardizable implementation

Treat a catalog as:

```text
name policy
+ concurrent Registry<Arc<dyn SchemaProvider>>
+ mutation policy
+ snapshot/cache policy
```

The actual backing system — memory, filesystem, metastore, REST API, Iceberg/Delta catalog —
should not leak into callers.

---

# 4. `SchemaProvider`

**Source** — `REG/datafusion-session-55.1.0/src/schema.rs`.

```rust
pub trait SchemaProvider: Any + Debug + Send + Sync {
    fn table_names(&self) -> Vec<String>;                                          // required
    async fn table(&self, name: &str)                                              // required
        -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>;
    fn table_exist(&self, name: &str) -> bool;                                     // required

    fn owner_name(&self) -> Option<&str> { None }                                  // provided
    async fn table_type(&self, name: &str) -> Result<Option<TableType>> {          // provided
        // default: materializes the whole TableProvider
        Ok(self.table(name).await?.map(|t| t.table_type()))
    }
    fn register_table(&self, name: String, table: Arc<dyn TableProvider>)          // provided
        -> Result<Option<Arc<dyn TableProvider>>>;                                 //   default: exec_err!
    fn deregister_table(&self, name: &str)                                         // provided
        -> Result<Option<Arc<dyn TableProvider>>>;                                 //   default: exec_err!
}
```

Important semantics:

- `table_names()` and `table_exist()` should be **metadata-only and cheap**.
- `table()` is async because constructing or resolving a table can require metadata access.
- `table_type()` exists so metadata consumers such as `information_schema.tables` need not
  instantiate an expensive `TableProvider`. **Override it whenever type metadata is cheaper
  than `table()`** — for a remote catalog this is the difference between one metadata call and
  N table opens. ⚠ A table whose `table_type` returns `Ok(None)` is **omitted from
  `information_schema.tables` entirely**.
- `owner_name()` feeds the `schema_owner` column of `information_schema.schemata` and nothing
  else. Left at the default it is NULL.
- `register_table` differs subtly from catalog registration: **duplicate table registration is
  specified as an error**, not replacement. The default body is
  `exec_err!("schema provider does not support registering tables")`.
- `deregister_table` returns the removed provider, or `Ok(None)` if it did not exist.

That replacement-vs-error difference matters when building a common registry abstraction:
**share mechanics, not semantics**.

⚠ `SessionContext::register_table` resolves the reference through `schema_for_ref` and then
passes only `table_ref.table()` — the bare name — to `SchemaProvider::register_table`. **A
`SchemaProvider` never sees a qualified name.** `SessionContext` performs no duplicate check
of its own; whether a duplicate is an error is entirely your implementation's decision.
`MemorySchemaProvider` enforces it; `ListingSchemaProvider` does not.

## 4.1 Implementors

**Index** — 6 indexed, all real:

| Implementor | What it demonstrates |
|---|---|
| `datafusion_catalog::memory::schema::MemorySchemaProvider` | the reference registry; duplicate `register_table` is an error |
| `datafusion_catalog::information_schema::InformationSchemaProvider` | §5.5 — metadata-as-relations, never registered in a catalog |
| `datafusion_catalog::listing_schema::ListingSchemaProvider` | §16.4 — directory-as-schema. ⚠ note the crate: `datafusion-catalog`, **not** `datafusion-catalog-listing` |
| `datafusion_catalog::dynamic_file::catalog::DynamicFileSchemaProvider` | §16.3 — URL-as-table-name |
| `datafusion_ffi::schema_provider::ForeignSchemaProvider` | §20.1 |
| `datafusion_ffi::tests::catalog::FixedSchemaProvider` | test double |

plus the private `ResolvedSchemaProvider` (§19), whose `owner_name()` returns the **catalog
name** rather than a real owner.

---

# 5. Name resolution and binding

The layer between a SQL identifier and a `SchemaProvider` lookup. Getting it wrong produces
"table not found" for a table that exists.

## 5.1 `TableReference` and `ResolvedTableReference`

**Source** — `REG/datafusion-common-55.1.0/src/table_reference.rs`. Aliases:
`datafusion_common::TableReference`, `datafusion::common::TableReference`.

```rust
pub struct ResolvedTableReference { pub catalog: Arc<str>, pub schema: Arc<str>, pub table: Arc<str> }
// Display => "{catalog}.{schema}.{table}"; Hash + Ord, so it can key a resolution map

pub enum TableReference {
    Bare    { table: Arc<str> },
    Partial { schema: Arc<str>, table: Arc<str> },
    Full    { catalog: Arc<str>, schema: Arc<str>, table: Arc<str> },
}
```

All fields are `Arc<str>` — cloning is cheap.

```rust
pub fn bare(table: impl Into<Arc<str>>) -> TableReference           // NO normalization
pub fn partial(schema: …, table: …) -> TableReference               // NO normalization
pub fn full(catalog: …, schema: …, table: …) -> TableReference      // NO normalization
pub fn parse_str(s: &str) -> Self
pub fn parse_str_normalized(s: &str, ignore_case: bool) -> Self
pub fn resolve(self, default_catalog: &str, default_schema: &str) -> ResolvedTableReference  // consumes self
pub fn resolved_eq(&self, other: &Self) -> bool
pub fn table(&self) -> &str
pub fn schema(&self) -> Option<&str>     // None for Bare
pub fn catalog(&self) -> Option<&str>    // None for Bare and Partial
pub fn to_quoted_string(&self) -> String
pub fn to_vec(&self) -> Vec<String>
```

⚠ **Normalization is a trap with two opposite halves.**

```text
TableReference::bare("MySchema.MyTable")  => Bare{ table: "MySchema.MyTable" }   one table, dots and all
TableReference::from("MySchema.MyTable")  => Partial{ "myschema", "mytable" }    parsed AND lowercased
```

`bare`/`partial`/`full` do **no** normalization at all. Every `From<&str>`/`From<String>`
impl routes to `parse_str`, which is `parse_str_normalized(s, /* ignore_case */ false)` —
and `ignore_case: false` means **do lowercase** unquoted identifiers. Quoted identifiers keep
their case. If parsing fails entirely it falls back to `Bare { table: <the whole string> }`.

`resolved_eq` compares across variants, ignoring any field the *other* side lacks:
`Bare("t").resolved_eq(&Full("c","s","t")) == true`.

`to_quoted_string` quotes only when needed: `partial("myschema","mytable")` →
`myschema.mytable`; `partial("MySchema","MyTable")` → `"MySchema"."MyTable"`.

## 5.2 Session-level resolution

**Source** — `REG/datafusion-55.1.0/src/execution/session_state.rs`.

```rust
SessionState::resolve_table_ref(&self, impl Into<TableReference>) -> ResolvedTableReference  // pub(crate)
SessionState::schema_for_ref(&self, impl Into<TableReference>) -> Result<Arc<dyn SchemaProvider>>  // public
```

`schema_for_ref` special-cases `information_schema` **before** consulting the catalog (§5.5),
then walks `catalog_list.catalog(…)` → `plan_datafusion_err!("failed to resolve catalog: {}")`
and `.schema(…)` → `plan_datafusion_err!("failed to resolve schema: {}")`. Both are
`DataFusionError::Plan`.

**Batch reference discovery** — this is what makes §19 possible:

```rust
// REG/datafusion-sql-55.1.0/src/resolve.rs
pub fn resolve_table_references(
    statement: &crate::parser::Statement,
    enable_ident_normalization: bool,
) -> Result<(Vec<TableReference>, Vec<TableReference>)>   // (relations, ctes)
```

CTE names are **excluded** from `relations` and returned separately. Both vectors come out of
`BTreeSet`s, so they are sorted and deduplicated. Wrapped as
`SessionState::resolve_table_references(&self, &Statement)` under `#[cfg(feature = "sql")]`.

## 5.3 Identifier and catalog configuration

**Source** — `REG/datafusion-common-55.1.0/src/config.rs`.

`datafusion.sql_parser.*`:

| Setting | Default | Why a provider cares |
|---|---|---|
| `enable_ident_normalization` | `true` | unquoted identifiers are lowercased before they reach you |
| `map_string_types_to_utf8view` | **`true`** | VARCHAR/CHAR/Text/String plan as `Utf8View`. ⚠ your declared schema must match or coercion appears in every plan |
| `dialect` | `Generic` | |
| `parse_float_as_decimal` | `false` | |
| `support_varchar_with_length` | `true` | |
| `collect_spans` | `false` | must be `true` for `Diagnostic` spans to point at SQL text (§5.7) |
| `recursion_limit` | `50` | |
| `default_null_ordering` | `nulls_max` | |
| `enable_options_value_normalization` | `false` | deprecated, warns, ignored |

`datafusion.catalog.*`:

| Setting | Default | Effect |
|---|---|---|
| `create_default_catalog_and_schema` | `true` | bootstraps `datafusion.public`; ⚠ see §2.2 |
| `default_catalog` | `datafusion` | fills `Bare`/`Partial` references |
| `default_schema` | `public` | fills `Bare` references |
| `information_schema` | **`false`** | ⚠ off by default — `SHOW TABLES` fails out of the box |
| `location` | `None` | with `format`, registers a `ListingSchemaProvider` |
| `format` | `None` | `TableProviderFactory` key for `location` |
| `has_header` | `true` | default for `CREATE EXTERNAL TABLE` |
| `newlines_in_values` | `false` | default for `CREATE EXTERNAL TABLE` |

Builders: `SessionConfig::with_default_catalog_and_schema(catalog, schema)`,
`with_create_default_catalog_and_schema(bool)`, `with_information_schema(bool)`.
Reusable helper: `datafusion_sql::planner::IdentNormalizer::new(normalize).normalize(Ident)`.

⚠ `catalog.location` + `catalog.format` together call
`SessionStateDefaults::register_default_schema`, which registers a `ListingSchemaProvider`
under the **hardcoded name `"default"`** — not `catalog.default_schema`. It returns silently
if the object store or the factory is missing, but **panics** on an unparseable URL.

## 5.4 DDL routing — which statement reaches which trait method

**Source** — `REG/datafusion-55.1.0/src/execution/context/mod.rs`, dispatching
`LogicalPlan::Ddl(DdlStatement)`. `DdlStatement` has **eleven** variants;
`CreateExternalTable` and `CreateFunction` are **`Box`ed** ▲ (55.0.0).

| SQL | Handler | Reaches |
|---|---|---|
| `CREATE EXTERNAL TABLE` | `create_external_table` → `create_custom_table` | `TableProviderFactory::create`, then `SchemaProvider::register_table` |
| `CREATE TABLE … AS` | `create_memory_table` | `MemTable::try_new(…).with_constraints(…).with_column_defaults(…)` → `register_table` |
| `CREATE VIEW` | `create_view` | `apply_type_coercion` → `ViewTable::new` → `register_table` |
| `CREATE SCHEMA` | `create_catalog_schema` | `CatalogProvider::register_schema(name, MemorySchemaProvider::new())` |
| `CREATE DATABASE` | `create_catalog` | `CatalogProviderList::register_catalog(name, MemoryCatalogProvider::new())` |
| `DROP TABLE` | `drop_table` | `find_and_deregister(name, TableType::Base)` |
| `DROP VIEW` | `drop_view` | `find_and_deregister(name, TableType::View)` |
| `DROP SCHEMA` | `drop_schema` | `CatalogProvider::deregister_schema(name, cascade)` |
| `CREATE/DROP FUNCTION` | `create_function` / `drop_function` | the `FunctionFactory`; a `Table` function routes to `register_udtf` |
| `CREATE INDEX`, anything else | fall-through | ⚠ **silently becomes a no-op `DataFrame`** |

Traps worth internalising:

- ⚠ **`CREATE SCHEMA` always installs `MemorySchemaProvider`.** There is no hook to substitute
  your own type. The only route to a custom `SchemaProvider` is
  `CatalogProvider::register_schema` called from Rust.
- ⚠ **`find_and_deregister` is type-checked**: it resolves the reference, awaits
  `schema.table(&name)`, and deregisters **only if `provider.table_type() == table_type`**.
  `DROP TABLE` on a view yields `exec_err!("Table '{name}' doesn't exist.")`. So
  `TableProvider::table_type()` is load-bearing for DDL, not only for metadata.
- ⚠ **`CREATE TEMPORARY TABLE` / `CREATE TEMPORARY VIEW` are
  `not_impl_err!("Temporary tables not supported")`** despite `TableType::Temporary` existing.
- `CREATE SCHEMA` cannot name a catalog via SQL parameters; the handler splits the schema name
  on `.` (1 token → default catalog, 2 → `catalog.schema`, 3+ → `exec_err!`).
- `DROP SCHEMA` uses `datafusion_common::schema_reference::SchemaReference`
  (`Bare { schema } | Full { schema, catalog }`), not `TableReference`.
- **Factory keys are matched UPPERCASE** — see §16.1.

## 5.5 `information_schema`

**Source** — `REG/datafusion-catalog-55.1.0/src/information_schema.rs`.

```rust
pub const INFORMATION_SCHEMA: &str = "information_schema";
pub const INFORMATION_SCHEMA_TABLES: &[&str] =
    &[TABLES, VIEWS, COLUMNS, DF_SETTINGS, SCHEMATA, ROUTINES, PARAMETERS];
```

**Exactly seven views in 55.1.0.** Provided by
`datafusion_catalog::information_schema::InformationSchemaProvider`, a `SchemaProvider`:

```rust
pub fn new(catalog_list: Arc<dyn CatalogProviderList>) -> Self
pub fn with_table_functions(mut self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self
```

⚠ **It is never registered in any catalog.** `SessionState::schema_for_ref` intercepts
`resolved_ref.schema == "information_schema"` when `datafusion.catalog.information_schema` is
true and constructs a **fresh provider per lookup**. Consequences: it never appears in
`CatalogProvider::schema_names()`; it does not exist at all unless the setting is on; and
because the match is on the *resolved* name, `mycatalog.information_schema.tables` works.

⚠ **The seven views are not `TableProvider`s.** Each is a private struct implementing
`PartitionStream`, wrapped by `StreamingTable::try_new(schema, vec![table])`. That is why they
never appear in any `TableProvider` implementor list.

| View | Columns |
|---|---|
| `tables` | `table_catalog`, `table_schema`, `table_name`, `table_type` (all Utf8 NOT NULL) |
| `views` | `table_catalog`, `table_schema`, `table_name` NOT NULL; `definition` Utf8 NULL |
| `columns` | the four identity columns + `ordinal_position` UInt64, `column_default` Utf8, `is_nullable` Utf8, `data_type` Utf8, `character_maximum_length`, `character_octet_length`, `numeric_precision`, `numeric_precision_radix`, `numeric_scale`, `datetime_precision` (UInt64 NULL), `interval_type` Utf8 NULL |
| `schemata` | `catalog_name`, `schema_name` NOT NULL; `schema_owner`, `default_character_set_catalog`/`_schema`/`_name`, `sql_path` (Utf8 NULL) |
| `routines` | `specific_catalog`/`_schema`/`_name`, `routine_catalog`/`_schema`/`_name`, `routine_type` NOT NULL; `is_deterministic` Boolean NULL; `data_type`, `function_type`, `description`, `syntax_example` Utf8 NULL |
| `parameters` | `specific_catalog`/`_schema`/`_name` NOT NULL, `ordinal_position` UInt64, `parameter_mode` Utf8, `parameter_name` Utf8 NULL, `data_type` Utf8, `parameter_default` Utf8 NULL, `is_variadic` Boolean, `rid` UInt8 (groups overloaded signatures for `SHOW FUNCTIONS`) |
| `df_settings` | setting name/value shape; the exact field list is **not verified here** |

`TableType` renders as `Base => "BASE TABLE"`, `View => "VIEW"`,
`Temporary => "LOCAL TEMPORARY"`. The seven views themselves are reported per catalog as
`VIEW`.

⚠ **`information_schema.views` is O(all tables).** `make_views` calls
`schema.table(&name).await?` for **every table in every schema** to read
`get_table_definition()`; there is no `table_type` shortcut on that path. `make_tables` does
use `table_type()`. Both, and `make_columns`, call `catalog_list.catalog(&name).unwrap()` — ⚠
**a `CatalogProviderList` whose `catalog_names()` and `catalog()` disagree will panic.**

**Two public helpers for async catalogs**, unusual enough to be worth knowing:
`information_schema::schemata_schema() -> SchemaRef` and
`information_schema::InformationSchemataBuilder`. Their doc states they exist for downstream
catalogs that must emit `schemata` rows from their own metadata source, because
`InformationSchemaProvider` enumerates schemas synchronously and is therefore unsuitable for
asynchronously-resolving backends.

## 5.6 `SQLOptions` — the query-time policy surface

**Source** — `REG/datafusion-55.1.0/src/execution/context/mod.rs`.

```rust
pub struct SQLOptions { allow_ddl: bool, allow_dml: bool, allow_statements: bool }  // private fields
impl Default for SQLOptions { /* all three true */ }
pub fn new() -> Self
pub fn with_allow_ddl(mut self, allow: bool) -> Self
pub fn with_allow_dml(mut self, allow: bool) -> Self
pub fn with_allow_statements(mut self, allow: bool) -> Self
pub fn verify_plan(&self, plan: &LogicalPlan) -> Result<()>
```

⚠ **`SQLOptions::new()` permits everything** — all three default to `true`.

`verify_plan` visits the plan with subqueries and flags exactly four node kinds, all as
`DataFusionError::Plan`:

```text
LogicalPlan::Ddl(ddl)        if !allow_ddl        -> "DDL not supported: {ddl.name()}"
LogicalPlan::Dml(dml)        if !allow_dml        -> "DML not supported: {dml.op}"
LogicalPlan::Copy(_)         if !allow_dml        -> "DML not supported: COPY"
LogicalPlan::Statement(stmt) if !allow_statements -> "Statement not supported: {stmt.name()}"
```

⚠ **Enforced only by `SessionContext::sql_with_options`.** `sql()` calls it with
`SQLOptions::new()` (everything allowed), and `execute_logical_plan` and the DataFrame API
bypass it entirely. `verify_plan` is a public method, so it can be applied to any plan.

### Recommendation

`SQLOptions` is a *query-time* admission gate; §21.D's `MutationPolicy` is a
*registration-time* one. They are independent, and a provider that is read-only needs both:
`SQLOptions` stops the statement from planning, the provider's `not_impl_err!` stops any other
route. Neither alone closes the DataFrame path.

## 5.7 Error vocabulary

**Source** — `REG/datafusion-common-55.1.0/src/error.rs`. 19 variants, three feature-gated.
The ones a provider must choose between correctly:

| Variant | Doc-stated meaning |
|---|---|
| `Plan(String)` | the user's query is wrong |
| `Execution(String)` | malformed input encountered at runtime |
| `NotImplemented(String)` | a real feature gap |
| `Configuration(String)` | invalid configuration |
| `Internal(String)` | **a bug in DataFusion.** "A user should not be able to trigger internal errors under normal circumstances." I/O and external-system failures explicitly do **not** belong here |
| `IoError` / `External(GenericError)` / `ObjectStore` | where infrastructure failure belongs |
| `SchemaError(Box<SchemaError>, …)` | `AmbiguousReference`, `DuplicateQualifiedField`, `DuplicateUnqualifiedField`, `FieldNotFound { field, valid_fields }` |
| `ResourcesExhausted(String)` | memory/disk limits |
| `Context(String, Box<DataFusionError>)` | wraps, does not replace |
| `Collection(Vec<DataFusionError>)` | report several failures at once |
| `Shared(Arc<DataFusionError>)` | one error fanned out to many consumers |
| `Diagnostic(Box<Diagnostic>, Box<DataFusionError>)` | rustc-style annotation |
| `Ffi(String)` | stringified across the FFI boundary |

Every constructor macro comes in two forms — `X_err!` returns `Err(…)`,
`X_datafusion_err!` returns the bare error — and both accept a trailing
`; diagnostic = <expr>`:

```text
plan_err! / plan_datafusion_err!            Plan
internal_err! / internal_datafusion_err!    Internal
not_impl_err! / not_impl_datafusion_err!    NotImplemented
exec_err! / exec_datafusion_err!            Execution
config_err! / config_datafusion_err!        Configuration
resources_err! / resources_datafusion_err!  ResourcesExhausted
substrait_err! · ffi_err! · sql_err! · arrow_err! · schema_err!
```

```rust
pub fn context(self, description: impl Into<String>) -> Self   // wraps; does not replace
pub fn find_root(&self) -> &Self                               // unwraps Context/Shared chains
pub fn with_diagnostic(self, diagnostic: Diagnostic) -> Self
pub fn with_diagnostic_fn<F: FnOnce(&DataFusionError) -> Diagnostic>(self, f: F) -> Self
pub fn diagnostic(&self) -> Option<&Diagnostic>
pub fn strip_backtrace(&self) -> String
pub fn message(&self) -> Cow<'_, str>
```

Because `context` wraps, `find_root()` is how you recover the original variant for matching.

`Diagnostic { kind: DiagnosticKind, message, span: Option<Span>, notes, helps }` models one
rustc error block. ⚠ **`Span`s are only populated when
`datafusion.sql_parser.collect_spans = true`, which defaults to `false`.**

---
# 6. `Session` — the context a provider receives

Every `TableProvider` method takes `state: &dyn Session`. This is the whole of what a provider
can reach, and a document that does not describe it leaves the reader believing `scan` has
access to nothing.

**Source** — `REG/datafusion-session-55.1.0/src/session.rs`. Supertraits: `Send + Sync` only
(no `Debug`, no `Any` supertrait — it has an explicit `as_any`). `#[async_trait]`.
Aliases: `datafusion::catalog::Session`, `datafusion_catalog::Session`.

## 6.1 The complete surface

| Method | Signature | R/P | Default |
|---|---|---|---|
| `session_id` | `fn session_id(&self) -> &str` | **R** | — |
| `config` | `fn config(&self) -> &SessionConfig` | **R** | — |
| `catalog_list` ▲ | `fn catalog_list(&self) -> Arc<dyn CatalogProviderList>` | **R** | — (new in 55.0.0) |
| `config_options` | `fn config_options(&self) -> &ConfigOptions` | P | `self.config().options()` |
| `query_planner` | `fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>` | P | ⚠ `UnsupportedQueryPlanner` |
| `optimize` | `fn optimize(&self, plan: &LogicalPlan) -> Result<LogicalPlan>` | P | ⚠ `Ok(plan.clone())` — **no optimization** |
| `physical_optimizers` | `fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]` | P | ⚠ `&[]` — **no rules** |
| `statistics_registry` | `fn statistics_registry(&self) -> Option<&StatisticsRegistry>` | P | `None` |
| `create_physical_plan` | `async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> Result<Arc<dyn ExecutionPlan>>` | **R** | — |
| `create_physical_expr` | `fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>` | **R** | — |
| `scalar_functions` | `fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>` | **R** | — |
| `higher_order_functions` | `fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>` | **R** | — |
| `aggregate_functions` | `fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>` | **R** | — |
| `window_functions` | `fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>` | **R** | — |
| `extension_type_registry` | `fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef` | **R** | — |
| `runtime_env` | `fn runtime_env(&self) -> &Arc<RuntimeEnv>` | **R** | — |
| `execution_props` | `fn execution_props(&self) -> &ExecutionProps` | **R** | — |
| `as_any` | `fn as_any(&self) -> &dyn Any` | **R** | — |
| `table_options` | `fn table_options(&self) -> &TableOptions` | **R** | — |
| `default_table_options` | `fn default_table_options(&self) -> TableOptions` | P | `table_options().combine_with_session_config(config_options())` |
| `table_options_mut` | `fn table_options_mut(&mut self) -> &mut TableOptions` | **R** | — |
| `task_ctx` | `fn task_ctx(&self) -> Arc<TaskContext>` | **R** | — |

## 6.2 What a provider actually gets from it

| Need | Reach for |
|---|---|
| batch size, target partitions, `collect_statistics` | `config()` / `config_options()` |
| an `ObjectStore` for a URL | `runtime_env().object_store(&url)` |
| turn a logical `Expr` filter into a `PhysicalExpr` | `create_physical_expr(expr, &df_schema)` |
| plan a stored `LogicalPlan` (what `ViewTable` does) | `create_physical_plan(&plan)` |
| a `TaskContext` for a stream you build eagerly | `task_ctx()` |
| resolve a UDF named in a pushed-down filter | `scalar_functions()` and friends |
| format-specific options (`TableParquetOptions`, extensions) | `default_table_options()` |
| enumerate catalogs from inside a provider ▲ | `catalog_list()` |
| inject custom statistics into planning | `statistics_registry()` — §13.4 |

⚠ **Three provided methods are conservative to the point of being wrong** if a wrapper
`Session` forgets to forward them: `query_planner` returns `UnsupportedQueryPlanner` (whose
`create_physical_plan` is `not_impl_err!("This session does not expose its query planner")`),
`optimize` returns the plan unchanged, and `physical_optimizers` returns an empty slice. They
default that way only so `datafusion-session` need not depend on the optimizer crate.

⚠ **`create_physical_plan` errors on DDL.** Its doc: plans such as `CREATE TABLE` "do not have
corresponding physical plans and must be handled by another layer, typically the
`SessionContext`."

⚠ **`create_physical_expr` applies type coercion and function rewrites but does not simplify.**
`a = 1 + 2` stays `a = 1 + 2`. A provider that wants constant-folded predicates must run
`ExprSimplifier` itself.

⚠ **`table_options_mut` takes `&mut self`**, so it is unreachable from the `&dyn Session` a
provider is handed.

`impl From<&dyn Session> for TaskContext` builds a context with `task_id: None` from the
session id, config, all four function registries and the runtime env.

## 6.3 Downcasting to `SessionState`

There is **no `as_session_state` helper in 55.1.0**. The blessed pattern is in the trait's own
doc:

```rust
fn session_state_from_session(session: &dyn Session) -> Result<&SessionState> {
    session.as_any().downcast_ref::<SessionState>()
        .ok_or_else(|| exec_datafusion_err!("Failed to downcast Session to SessionState"))
}
```

with the caveat "this may stop working in future versions" and a request to file an issue for
the missing trait method instead.

⚠ Several in-tree components do this and therefore **only work inside a real `SessionContext`**:
`ListingTableFactory::create` errors with `"ListingTableFactory requires SessionState"`;
`ListingTableConfigExt::infer_options` downcasts and `.unwrap()`s, i.e. **panics** on a custom
`Session`; `DynamicListTableFactory::try_new` downcasts and turns failure into
`plan_datafusion_err!("get current SessionStore error")`.

Implementors of `Session`: `datafusion::execution::session_state::SessionState` and
`datafusion_ffi::session::ForeignSession`.

## 6.4 `SessionStore` — the back-reference

**Source** — `REG/datafusion-session-55.1.0/src/session.rs`.

```rust
type SessionRefLock = Arc<Mutex<Option<Weak<RwLock<dyn Session>>>>>;
pub struct SessionStore { session: SessionRefLock }
pub fn new() -> Self
pub fn with_state(&self, state: Weak<RwLock<dyn Session>>)
pub fn get_session(&self) -> Weak<RwLock<dyn Session>>
```

This is how a *factory* that receives no session — a `UrlTableFactory` gets only a URL string
— reaches back for one. ⚠ **`get_session` `.unwrap()`s and panics if `with_state` was never
called.** `SessionContext::enable_url_table` must therefore build the factory, build the
context, and *then* back-fill the store; any component using this pattern inherits that
ordering requirement.

---

# 7. The logical face of a provider

## 7.1 `TableSource`

**Source** — `REG/datafusion-expr-55.1.0/src/table_source.rs`. Aliases:
`datafusion_expr::TableSource`, `datafusion::logical_expr::TableSource`.

```rust
pub trait TableSource: Any + Sync + Send {
    fn schema(&self) -> SchemaRef;                                              // the ONLY required method

    fn constraints(&self) -> Option<&Constraints> { None }
    fn table_type(&self) -> TableType { TableType::Base }
    fn supports_filters_pushdown(&self, filters: &[&Expr])
        -> Result<Vec<TableProviderFilterPushDown>> {
        Ok((0..filters.len()).map(|_| TableProviderFilterPushDown::Unsupported).collect())
    }
    fn get_logical_plan(&'_ self) -> Option<Cow<'_, LogicalPlan>> { None }
    fn get_column_default(&self, _column: &str) -> Option<&Expr> { None }
}
```

One required method, five provided. No `Debug` supertrait (unlike `TableProvider`), no
`as_any`, and an `impl dyn TableSource { is / downcast_ref }` block.

**This — not `TableProvider` — is what the analyzer and logical optimizer see.** It is a
deliberate subset: planning-time information only, no `scan`, no DML, no statistics, no
`get_table_definition`.

⚠ Only **non-volatile** expressions are passed to `TableSource::supports_filters_pushdown`.

⚠ If you install a custom `TableSource` that is not a `DefaultTableSource`, the optimizer uses
*its* pushdown answer while execution uses the `TableProvider`'s. The two can diverge silently.

## 7.2 `DefaultTableSource` and the conversion functions

**Source** — `REG/datafusion-catalog-55.1.0/src/default_table_source.rs`. Aliases:
`datafusion::datasource::{DefaultTableSource, provider_as_source, source_as_provider}`.

```rust
pub struct DefaultTableSource { pub table_provider: Arc<dyn TableProvider> }   // public field
impl DefaultTableSource { pub fn new(table_provider: Arc<dyn TableProvider>) -> Self }

pub fn provider_as_source(table_provider: Arc<dyn TableProvider>) -> Arc<dyn TableSource>;
pub fn source_as_provider(source: &Arc<dyn TableSource>) -> Result<Arc<dyn TableProvider>>;
```

`DefaultTableSource` forwards all six methods 1:1. It does **not** forward
`get_table_definition`, which is `TableProvider`-only and feeds `information_schema.views`.

⚠ `source_as_provider` downcasts to `DefaultTableSource` and otherwise returns
`internal_err!("TableSource was not DefaultTableSource")` — `DataFusionError::Internal`, the
"this is a bug, file a report" variant, shown to the user. Note it takes
`&Arc<dyn TableSource>`, not `&dyn TableSource`.

**`LogicalTableSource`** (`datafusion_expr::logical_plan::builder::LogicalTableSource`) is a
provider-free `TableSource` for schema-only logical planning: `new(table_schema)`,
`with_constraints(…)`. ⚠ Its `supports_filters_pushdown` returns **`Exact` for every filter**,
which is correct for planning experiments and catastrophic if the plan is ever executed.

## 7.3 `LogicalPlan::TableScan`

**Source** — `REG/datafusion-expr-55.1.0/src/logical_plan/plan.rs`.

```rust
pub struct TableScan {
    pub table_name: TableReference,
    pub source: Arc<dyn TableSource>,
    pub projection: Option<Vec<usize>>,
    pub projected_schema: DFSchemaRef,
    pub filters: Vec<Expr>,
    pub fetch: Option<usize>,
    /// Statistics the planner would like the provider to answer for this scan, typically
    /// attached by a custom optimizer rule. A `BTreeSet`, not a `Vec`, to keep the plan
    /// deterministic.
    pub statistics_requests: BTreeSet<StatisticsRequest>,
}
```

⚠ **`statistics_requests` is a seventh public field** that appears in **no upgrade guide
between 46.0.0 and 55.0.0**. Because `TableScan` has public fields, struct-literal
construction and exhaustive destructuring both break against older code. Its introduction
release is **not established** by the available evidence.

⚠ **`TableScan::try_new` is `#[deprecated(since = "54.0.0")]`.** ▲ Use the builder, which the
doc says is "resilient to new fields being added to `TableScan`":

```rust
TableScanBuilder::new(table_name: impl Into<TableReference>, source: Arc<dyn TableSource>)
    .with_projection(Option<Vec<usize>>)
    .with_filters(Vec<Expr>)
    .with_fetch(Option<usize>)
    .with_statistics_requests(BTreeSet<StatisticsRequest>)
    .build() -> Result<TableScan>
TableScanBuilder::from(scan)   // round-trips an existing scan
```

`build()` derives `projected_schema` from `source.schema()` + projection, computes
`FunctionalDependencies::new_from_constraints(source.constraints(), …)`, and errors with
`plan_err!("table_name cannot be empty")`.

⚠ **`PartialEq for TableScan` ignores `source` and `statistics_requests`.** Two scans of
different providers with the same name, projection, filters and fetch compare equal.

`UNNAMED_TABLE` (`"?table?"`) is the scan name produced by `SessionContext::read_table`,
`read_batch` and DataFrame-first flows. A provider that keys off `table_name` must handle it.

## 7.4 `ContextProvider` — the SQL planner's view of the hierarchy

**Source** — `REG/datafusion-expr-55.1.0/src/planner.rs`. ⚠ `datafusion_sql::planner::ContextProvider`
is a **re-export**, not a definition. No supertraits at all — not even `Send`/`Sync`/`Debug`.

| Method | Signature | R/P | Default |
|---|---|---|---|
| `get_table_source` | `fn get_table_source(&self, name: TableReference) -> Result<Arc<dyn TableSource>>` | **R** | — |
| `get_function_meta` | `fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>>` | **R** | — |
| `get_higher_order_meta` ▲ | `fn get_higher_order_meta(&self, name: &str) -> Option<Arc<HigherOrderUDF>>` | **R** | — (new required in 54.0.0) |
| `get_aggregate_meta` | `fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>>` | **R** | — |
| `get_window_meta` | `fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>>` | **R** | — |
| `get_variable_type` | `fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType>` | **R** | — |
| `options` | `fn options(&self) -> &ConfigOptions` | **R** | — |
| `udf_names` / `udaf_names` / `udwf_names` / `higher_order_function_names` | `fn …(&self) -> Vec<String>` | **R** | — |
| `get_variable_field` | `fn get_variable_field(&self, variable_names: &[String]) -> Option<FieldRef>` | P | wraps `get_variable_type` in a nullable `Field`, no metadata |
| `get_table_function_source` | `fn get_table_function_source(&self, _name: &str, _args: Vec<Expr>) -> Result<Arc<dyn TableSource>>` | P | `not_impl_err!("Table Functions are not supported")` |
| `create_cte_work_table` | `fn create_cte_work_table(&self, _name: &str, _schema: SchemaRef) -> Result<Arc<dyn TableSource>>` | P | `not_impl_err!("Recursive CTE is not implemented")` |
| `get_file_type` | `fn get_file_type(&self, _ext: &str) -> Result<Arc<dyn FileType>>` | P | `not_impl_err!("Registered file types are not supported")` |
| `get_expr_planners` | `fn get_expr_planners(&self) -> &[Arc<dyn ExprPlanner>]` | P | `&[]` |
| `get_relation_planners` | `#[cfg(feature = "sql")] fn get_relation_planners(&self) -> &[Arc<dyn RelationPlanner>]` | P | `&[]` |
| `get_type_planner` | `#[cfg(feature = "sql")] fn get_type_planner(&self) -> Option<Arc<dyn TypePlanner>>` | P | `None` |

⚠ `get_relation_planners` and `get_type_planner` are `#[cfg]`-gated **on the trait method
itself**, so they vanish from the vtable when `datafusion-expr`'s `sql` feature is off.

### ⚠ The single most important fact about `ContextProvider`

**It never touches the catalog.** The only real implementation is the private
`SessionContextProvider<'a> { state: &'a SessionState, tables: HashMap<ResolvedTableReference, Arc<dyn TableSource>> }`,
whose `get_table_source` is a pure map lookup returning
`plan_datafusion_err!("table '{name}' not found")` on a miss.

The catalog is consulted **once, earlier**, in `SessionState::statement_to_plan`: resolve all
references from the AST, then for each one `schema_for_ref(resolved)` followed by
`schema.table(&resolved.table).await?`, inserting `provider_as_source(table)` into the map.

⚠ That loop is written `if let Ok(schema) = …` — **a failed schema resolution is silently
swallowed** and resurfaces only as "table not found" from `get_table_source`.

This is why a remote catalog must batch its lookups at the resolution phase (§19), and why
`ContextProvider::get_table_source` returning `Result` does not give a remote backend a place
to report failure.

`SessionContextProvider::get_table_function_source` coerces and simplifies the argument
expressions through `ExprSimplifier` before calling
`TableFunction::create_table_provider_with_args` (§16.2).

## 7.5 `ExtensionPlanner::plan_table_scan` — the provider-bypass route

**Source** — `REG/datafusion-session-55.1.0/src/planner.rs`.

```rust
async fn plan_table_scan(
    &self,
    _planner: &dyn PhysicalPlanner,
    _scan: &TableScan,
    _session: &dyn Session,
    _planning_ctx: &PhysicalPlanningContext,
) -> Result<Option<Arc<dyn ExecutionPlan>>> { Ok(None) }
```

A whole second route from a scan to a physical plan: implement `TableSource` (**not**
`TableProvider`) and plan it yourself. The physical planner falls through to the registered
extension planners when `source_as_provider(source)` fails, and errors with
`"No installed planner was able to plan TableScan for custom TableSource"` if none returns
`Some`. The dispatch idiom from the doc is `if scan.source.is::<MyCustomTableSource>() { … }`.

### Recommendation

Use this only when the relation genuinely has no `TableProvider` form — a federated pushdown
target, or a source whose physical plan depends on planning context a provider cannot see.
Otherwise the `DefaultTableSource` path keeps one object answering both layers, which is the
only way to guarantee the logical and physical pushdown answers agree.

---
# 8. `TableProvider`

**Source** — `REG/datafusion-session-55.1.0/src/table.rs`. Aliases:
`datafusion::datasource::TableProvider`, `datafusion::catalog::TableProvider`,
`datafusion_catalog::TableProvider`, `datafusion_session::TableProvider`.

```rust
#[async_trait]
pub trait TableProvider: Any + Debug + Sync + Send {
```

Three required methods against twelve provided ones. **Every provided method you leave at its
default is a planning decision made blind.**

## 8.1 Required

```rust
fn schema(&self) -> SchemaRef;
fn table_type(&self) -> TableType;
async fn scan(
    &self,
    state: &dyn Session,
    projection: Option<&Vec<usize>>,     // NOT a slice — see §10
    filters: &[Expr],
    limit: Option<usize>,
) -> Result<Arc<dyn ExecutionPlan>>;
```

## 8.2 Provided, with exact defaults

| Method | Signature | Default | Cost of leaving it |
|---|---|---|---|
| `supports_filters_pushdown` | `fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>` | `Unsupported` × `filters.len()` | every filter runs after a full scan, and no `LIMIT` is pushed |
| `scan_with_args` | `async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>` | delegates to `scan` | you never see `statistics_requests`; you are not forward-compatible |
| `statistics` | `fn statistics(&self) -> Option<Statistics>` | `None` | see §8.5 — this hook is not consumed by mainline |
| `constraints` | `fn constraints(&self) -> Option<&Constraints>` | `None` | no functional dependencies; `DISTINCT` and join eliminations do not fire |
| `get_column_default` | `fn get_column_default(&self, _column: &str) -> Option<&Expr>` | `None` | SQL `INSERT` with omitted columns inserts NULL |
| `get_logical_plan` | `fn get_logical_plan(&'_ self) -> Option<Cow<'_, LogicalPlan>>` | `None` | the table cannot participate in view inlining |
| `get_table_definition` | `fn get_table_definition(&self) -> Option<&str>` | `None` | `information_schema.views.definition` is NULL |
| `insert_into` | `async fn insert_into(&self, _state: &dyn Session, _input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>` | `not_impl_err!("Insert into not implemented for this table")` | |
| `delete_from` | `async fn delete_from(&self, _state: &dyn Session, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>` | `not_impl_err!("DELETE not supported for {} table", self.table_type())` | |
| `update` | `async fn update(&self, _state: &dyn Session, _assignments: Vec<(String, Expr)>, _filters: Vec<Expr>) -> Result<Arc<dyn ExecutionPlan>>` | `not_impl_err!("UPDATE not supported for {} table", …)` | |
| `truncate` | `async fn truncate(&self, _state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>` | `not_impl_err!("TRUNCATE not supported for {} table", …)` | |
| `merge_into` | `async fn merge_into(&self, _state: &dyn Session, _source: Arc<dyn ExecutionPlan>, _merge_schema: DFSchemaRef, _on: Expr, _clauses: Vec<MergeIntoClause>) -> Result<Arc<dyn ExecutionPlan>>` | `not_impl_err!("MERGE INTO not supported for {} table", …)` | |

Note that the DML default messages interpolate `self.table_type()`, which is one more reason
`table_type()` is load-bearing beyond metadata (§5.4).

## 8.3 `TableType`

**Source** — `REG/datafusion-expr-55.1.0/src/table_source.rs`.

```rust
pub enum TableType { Base, View, Temporary }   // Copy; Display => "Base" / "View" / "Temporary"
```

| Value | Meaning | Where it bites |
|---|---|---|
| `Base` | physical table | `DROP TABLE` matches only this |
| `View` | non-materialized relation backed by a query | `DROP VIEW` matches only this |
| `Temporary` | transient relation | ⚠ `CREATE TEMPORARY TABLE`/`VIEW` are `not_impl_err!`; the variant exists but SQL cannot produce it |

⚠ Classification is a declaration, not a derivation: `StreamingTable` returns `View` although
it is a physical source, and `CteWorkTable` returns `Temporary`.

## 8.4 Metadata contract

**Recommendation.** Standardize these into a single immutable metadata object:

```rust
struct TableMetadata {
    schema: SchemaRef,
    table_type: TableType,
    constraints: Option<Constraints>,
    definition: Option<Arc<str>>,
    logical_plan: Option<Arc<LogicalPlan>>,
    column_defaults: BTreeMap<String, Expr>,
    statistics: Option<Statistics>,
}
```

Then make the `TableProvider` methods trivial projections over this state. This prevents
metadata inconsistencies between `schema()`, catalog introspection, DDL, optimizer metadata and
the physical scan.

### `get_logical_plan` and the `Cow`

The `Cow` exists so a provider that **stores** a plan returns `Cow::Borrowed` (zero-copy, what
`ViewTable` does) while a provider that **synthesizes** one per call returns `Cow::Owned`.
`CteWorkTable` deliberately returns `None` even though it is plan-backed. Returning `Some` is
what makes a table participate in view inlining and plan rewriting.

### `get_column_default` — where defaults are actually applied

**Source.** There is exactly **one** consumption site in the pinned set: SQL `INSERT`
planning, reached through `DefaultTableSource::get_column_default`. Defaults apply to **SQL
`INSERT` with omitted columns and nowhere else** — not on scan, not in the DataFrame write
API, and not inside `insert_into`.

## 8.5 `Constraints`

**Source** — `REG/datafusion-common-55.1.0/src/functional_dependencies.rs`.

```rust
pub enum Constraint { PrimaryKey(Vec<usize>), Unique(Vec<usize>) }
pub struct Constraints { inner: Vec<Constraint> }
pub fn new_unverified(constraints: Vec<Constraint>) -> Self   // doc: "does not check whether the argument is valid"
pub fn extend(&mut self, other: Constraints)
pub fn project(&self, proj_indices: &[usize]) -> Option<Self>
impl Deref for Constraints { type Target = [Constraint]; }    // iter/len/is_empty come free
impl IntoIterator for Constraints
impl Display for Constraints                                   // "constraints=[PrimaryKey([0]), …]"
```

The three-state distinction the trait intends:

```text
None                         = constraints unsupported / unknown
Some(<supported, empty>)     = constraints supported, but none exist
Some(<non-empty>)            = declared keys
```

⚠ **`Constraints::empty()` does not exist in 55.1.0**, despite being named in
`TableProvider::constraints`'s own doc comment. Use `Constraints::default()` — which is what
`MemTable`, `StreamConfig` and `ListingTable` all do.

**What DataFusion actually does with them — exactly one thing.** `TableScanBuilder::build`
converts `source.constraints()` into `FunctionalDependencies` (`PrimaryKey` →
`nullable: false`, `Unique` → `nullable: true`). Those are read by three optimizer rules:
`replace_distinct_aggregate` (drop a `DISTINCT` a key already guarantees), `eliminate_join`,
and `optimize_projections`.

⚠ **Constraints are never enforced.** Nothing validates uniqueness on insert.
`InsertOp::Replace`'s "existing rows are replaced" semantics are entirely the provider's job.
**Declaring a false `PrimaryKey` yields wrong results, not an error.** `Constraints::project`
drops any constraint whose columns are not fully preserved.

## 8.6 `Statistics`, `ColumnStatistics`, `Precision`

**Source** — `REG/datafusion-common-55.1.0/src/stats.rs`.

```rust
pub enum Precision<T: Debug + Clone + PartialEq + Eq + PartialOrd> {
    Exact(T),
    Inexact(T),
    #[default] Absent,
}
```

⚠ **`Absent` is an absorbing element.** Any arithmetic involving it yields `Absent`, and
`Absent.to_inexact() == Absent` — the doc calls it "a fundamentally different state". Treating
`Absent` as "zero" or as "inexact unknown" is the most common implementor error.

```rust
// generic
pub fn get_value(&self) -> Option<&T>
pub fn map<U, F>(self, f: F) -> Precision<U>
pub fn is_exact(&self) -> Option<bool>
pub fn max(&self, other: &Precision<T>) -> Precision<T>
pub fn min(&self, other: &Precision<T>) -> Precision<T>
pub fn to_inexact(self) -> Self
// Precision<usize>
pub fn add / sub / multiply (&self, other: &Precision<usize>) -> Precision<usize>
pub fn with_estimated_selectivity(self, selectivity: f64) -> Self
// Precision<ScalarValue>
pub fn add / sub / multiply
pub fn cast_to_sum_type(&self) -> Precision<ScalarValue>
pub fn add_for_sum(&self, other: &Precision<ScalarValue>) -> Precision<ScalarValue>
pub fn cast_to(&self, data_type: &DataType) -> Result<Precision<ScalarValue>>
```

```rust
pub struct Statistics {
    pub num_rows: Precision<usize>,
    pub total_byte_size: Precision<usize>,
    pub column_statistics: Vec<ColumnStatistics>,   // MUST be one per schema field
}
pub fn new_unknown(schema: &Schema) -> Self
pub fn unknown_column(schema: &Schema) -> Vec<ColumnStatistics>
pub fn with_num_rows(mut self, Precision<usize>) -> Self
pub fn with_total_byte_size(mut self, Precision<usize>) -> Self
pub fn add_column_statistics(mut self, ColumnStatistics) -> Self
pub fn calculate_total_byte_size(&mut self, schema: &Schema)
pub fn to_inexact(mut self) -> Self
pub fn project(self, projection: Option<&impl AsRef<[usize]>>) -> Self
pub fn with_fetch(mut self, fetch: Option<usize>, skip: usize, n_partitions: usize) -> Result<Self>
pub fn try_merge_iter<'a, I>(items: I, schema: &Schema) -> Result<Statistics>
pub fn try_merge_iter_with_ndv_fallback<'a, I>(…) -> Result<Statistics>   // NdvFallback { Max, Sum }
```

⚠ **`Statistics::default()` has zero column statistics** and violates the one-per-field
invariant. Use `Statistics::new_unknown(&schema)`.
`with_fetch` is per-partition; `n_partitions` scales it to a global estimate.

```rust
pub struct ColumnStatistics {
    pub null_count: Precision<usize>,
    pub max_value: Precision<ScalarValue>,
    pub min_value: Precision<ScalarValue>,
    pub sum_value: Precision<ScalarValue>,
    pub distinct_count: Precision<usize>,
    pub byte_size: Precision<usize>,     // sixth field; frequently omitted from write-ups
}
pub fn new_unknown() -> Self
pub fn is_singleton(&self) -> bool
pub fn with_null_count / with_max_value / with_min_value / with_sum_value
       / with_distinct_count / with_byte_size
pub fn to_inexact(mut self) -> Self
```

`sum_value` should hold widened integral types (`Int8/16/32 → Int64`,
`UInt8/16/32 → UInt64`); prefer `with_sum_value` plus `cast_to_sum_type`/`add_for_sum`.

### ⚠ `TableProvider::statistics()` is the hook that does not work

Its own doc says the entry point "is not presently used in mainline DataFusion". The
statistics that **are** consumed travel a different road entirely:

```text
PartitionedFile::statistics  ->  FileGroup::statistics  ->  FileScanConfig::statistics()
      ->  DataSource::partition_statistics  ->  ExecutionPlan::statistics_from_inputs
      ->  StatisticsContext::compute
```

§13.4 documents that path. Implement `statistics()` if you like — but a provider whose row
counts must reach the optimizer populates `PartitionedFile::statistics` (file-backed) or
overrides `statistics_from_inputs` on the plan it returns (everything else), and a provider
with genuinely custom statistics registers a `StatisticsProvider`.

---

# 9. The scan contract and logical pushdown

## 9.1 Evaluation order

The effective logical operation of `scan(projection, filters, limit)` is:

```text
projection(
    limit(
        filter(
            source
        )
    )
)
```

i.e. **filter → limit → projection**. It must be equivalent to
`PROJECTION a (LIMIT 3 (SCAN WHERE b > 5))`.

## 9.2 Projection

`projection: Option<&Vec<usize>>` (`scan`) / `Option<&[usize]>` (`ScanArgs`).

Indices reference `TableProvider::schema()` and specify both the included columns and their
output ordering.

Do not assume filter columns occur in the projection:

```sql
SELECT a FROM t WHERE b > 5
```

may legitimately call the provider with roughly:

```text
projection = [a]
filters    = [b > 5]
```

The datasource must internally read `b` if needed for predicate evaluation while exposing only
projected output columns.

## 9.3 Filter pushdown classification

For every proposed expression return exactly one:

```rust
pub enum TableProviderFilterPushDown { Unsupported, Inexact, Exact }
```

```text
Unsupported -> provider cannot exploit the predicate; DataFusion does not even pass it to scan
Inexact     -> provider uses it but may retain false positives;
               DataFusion applies a residual Filter above the scan
Exact       -> provider guarantees the predicate's semantics;
               no residual Filter is planned
```

⚠ The returned vector's length **and order must exactly match** the input filters; a length
mismatch is an error. ⚠ A wrong `Exact` **silently returns wrong rows** — there is no residual
filter to catch it.

**Recommendation.** Centralize the classification:

```rust
trait PredicatePushdown {
    fn classify(&self, expr: &Expr) -> Result<TableProviderFilterPushDown>;
}
```

and derive both capability reporting and actual scan predicate translation from the same
implementation. Otherwise a dangerous class of bugs appears where `supports_filters_pushdown()`
promises `Exact` but `scan()` implements different semantics.

`Inexact` is almost always better than the `Unsupported` default: DataFusion re-applies the
filter, so an approximate answer is safe.

## 9.4 Limit

A pushed limit is an optimization hint whose semantics are **after filtering**. A scan may
produce more than the requested number; upper plan nodes enforce final SQL semantics.

⚠ DataFusion does **not** push `LIMIT` through an **inexact** pushed predicate, because false
positives could consume the source-side limit before enough valid rows are available. So
leaving `supports_filters_pushdown` at `Unsupported` costs you the limit as well as the filter.

## 9.5 This is only half the pushdown story

Everything above is the **logical** system, decided during logical optimization against
`Expr`s and answered by `TableSource`. A second, independent **physical** system runs after
physical planning against `PhysicalExpr`s and is answered by `ExecutionPlan` / `DataSource` /
`FileSource`. §11 documents it. The two are not redundant — the canonical Parquet
implementation reports `PushedDown::No` to its parent while still absorbing the predicate for
container pruning.

---

# 10. `scan_with_args` — the preferred extension surface

**Source** — `REG/datafusion-session-55.1.0/src/table.rs`.

```rust
#[derive(Debug, Clone, Default)]
pub struct ScanArgs<'a> {
    filters: Option<&'a [Expr]>,
    projection: Option<&'a [usize]>,
    limit: Option<usize>,
    statistics_requests: &'a [StatisticsRequest],
}
```

Exactly eight methods — **no more**:

```rust
pub fn with_projection(mut self, projection: Option<&'a [usize]>) -> Self
pub fn projection(&self) -> Option<&'a [usize]>
pub fn with_filters(mut self, filters: Option<&'a [Expr]>) -> Self
pub fn filters(&self) -> Option<&'a [Expr]>
pub fn with_limit(mut self, limit: Option<usize>) -> Self
pub fn limit(&self) -> Option<usize>
pub fn with_statistics_requests(mut self, statistics_requests: &'a [StatisticsRequest]) -> Self
pub fn statistics_requests(&self) -> &'a [StatisticsRequest]
```

```rust
#[derive(Debug, Clone)]
pub struct ScanResult { plan: Arc<dyn ExecutionPlan> }   // ONE private field
pub fn new(plan: Arc<dyn ExecutionPlan>) -> Self
pub fn plan(&self) -> &Arc<dyn ExecutionPlan>
pub fn into_inner(self) -> Arc<dyn ExecutionPlan>
impl From<Arc<dyn ExecutionPlan>> for ScanResult
```

⚠ **`ScanArgs::projection_vec()` does not exist in 55.1.0.** Use
`args.projection().map(|p| p.to_vec())`.
⚠ **`ScanResult::with_statistics()` does not exist.** `ScanResult` is a newtype over the plan,
designed so the return type can grow later. There is no statistics channel on the scan result;
statistics travel through the returned `ExecutionPlan` (§13.4).

## 10.1 The default delegation

```rust
async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult> {
    let filters = args.filters().unwrap_or(&[]);
    let projection = args.projection().map(|p| p.to_vec());
    let limit = args.limit();
    let plan = self.scan(state, projection.as_ref(), filters, limit).await?;
    Ok(plan.into())
}
```

## 10.2 ⚠ The physical planner calls `scan_with_args`, never `scan`

**Source** — `REG/datafusion-55.1.0/src/physical_planner.rs`:

```rust
let opts = ScanArgs::default()
    .with_projection(projection.as_deref())
    .with_filters(Some(&filters_vec))
    .with_limit(*fetch)
    .with_statistics_requests(&stats_requests);
let res = source.scan_with_args(session_state, opts).await?;
```

`scan` remains **required**, so you must still write it. The in-tree pattern (`ListingTable`,
`CteWorkTable`) is to put the real logic in `scan_with_args` and make `scan` a thin adapter:

```rust
// REG/datafusion-catalog-listing-55.1.0/src/table.rs
let options = ScanArgs::default()
    .with_projection(projection.map(|p| p.as_slice()))
    .with_filters(Some(filters))
    .with_limit(limit);
Ok(self.scan_with_args(state, options).await?.into_inner())
```

**Recommendation.** Normalize both entry points into one internal request and plan once:

```text
scan(...)  ─┐
            ├──> InternalScanRequest { projection, filters, limit,
scan_with_args ┘                        statistics requests, future extensions }
                                                   |
                                                   v
                                          ScanPlanner::plan()
                                                   |
                                                   v
                                            ExecutionPlan
```

Do not maintain independent scan implementations.

## 10.3 `StatisticsRequest`

**Source** — `REG/datafusion-expr-common-55.1.0/src/statistics.rs`, re-exported as
`datafusion_expr::statistics::StatisticsRequest`.

```rust
pub enum StatisticsRequest {
    Min(Arc<Column>), Max(Arc<Column>), NullCount(Arc<Column>), DistinctCount(Arc<Column>),
    Sum(Arc<Column>), ByteSize(Arc<Column>),
    RowCount, TotalByteSize,
}
```

Each variant maps onto a field of `Statistics` / `ColumnStatistics`, so a provider that
already populates one can answer the request trivially. `Arc<Column>` keeps plan clones cheap;
`BTreeSet` in `TableScan` keeps plans deterministic.

⚠ **DataFusion itself neither populates nor consumes this.** Its own doc: "It exists so a
request can be threaded from a `TableScan` … through `ScanArgs::statistics_requests` to a
`TableProvider`", and `with_statistics_requests`'s doc adds "DataFusion's own `TableProvider`s
ignore this field." It is a pure extension channel: a **custom optimizer rule** sets
`TableScan::statistics_requests`, the planner forwards it, and a **custom provider** answers.
Nothing happens without both halves.

---
# 11. Physical filter pushdown and dynamic filters

The second, independent pushdown system. It runs **after** physical planning, operates on
`Arc<dyn PhysicalExpr>` rather than `Expr`, and is answered by `ExecutionPlan` / `DataSource` /
`FileSource` rather than by `TableSource`. A provider that implements only §9 never
participates in it — which means no TopK threshold, no join-derived predicate and no
aggregate-derived predicate ever reaches its scan.

**Source** — `REG/datafusion-physical-plan-55.1.0/src/filter_pushdown.rs` and
`REG/datafusion-physical-optimizer-55.1.0/src/filter_pushdown.rs`.

## 11.1 The two phases

```rust
pub enum FilterPushdownPhase { Pre, Post }
```

| Phase | When | What may be pushed | May the plan be restructured? |
|---|---|---|---|
| `Pre` | before most optimizations | only filters that do **not** reference an `ExecutionPlan` | **yes** — e.g. a `FilterExec` absorbed into a `DataSourceExec` |
| `Post` | after most optimizations | filters that reference an `ExecutionPlan` are now safe, because later rules only replace children | no |

`Post` is what links a `SortExec`'s TopK threshold, or a `HashJoinExec`'s build-side values,
to a `DataSourceExec`.

## 11.2 The vocabulary

```rust
pub enum PushedDown { Yes, No }                       // + and() / or() / wrap_expression(expr)
pub struct PushedDownPredicate { pub discriminant: PushedDown, pub predicate: Arc<dyn PhysicalExpr> }
    ::supported(expr)  ::unsupported(expr)  .into_inner()

pub struct ChildFilterPushdownResult { pub filter: Arc<dyn PhysicalExpr>, pub child_results: Vec<PushedDown> }
    ::any() -> PushedDown    // OR-fold; No when there are zero children
    ::all() -> PushedDown    // AND-fold; No when there are zero children

pub struct ChildPushdownResult {
    pub parent_filters: Vec<ChildFilterPushdownResult>,
    pub self_filters: Vec<Vec<PushedDownPredicate>>,   // outer index = child
}

pub struct FilterPushdownPropagation<T> { pub filters: Vec<PushedDown>, pub updated_node: Option<T> }
    ::if_all(ChildPushdownResult)  ::if_any(..)  ::all_unsupported(..)
    ::with_parent_pushdown_result(Vec<PushedDown>)  .with_updated_node(T)

pub struct ChildFilterDescription { /* private */ }
    ::from_child(&[Arc<dyn PhysicalExpr>], &Arc<dyn ExecutionPlan>) -> Result<Self>
    ::from_child_with_allowed_indices(&[…], HashSet<usize>, &Arc<dyn ExecutionPlan>) -> Result<Self>
    ::all_unsupported(&[Arc<dyn PhysicalExpr>]) -> Self
    .with_self_filter(Arc<dyn PhysicalExpr>)  .with_self_filters(Vec<…>)

pub struct FilterDescription { /* private */ }
    ::new()  Default  .with_child(ChildFilterDescription)
    ::from_children(Vec<Arc<dyn PhysicalExpr>>, &[&Arc<dyn ExecutionPlan>]) -> Result<Self>
    ::all_unsupported(&[Arc<dyn PhysicalExpr>], &[&Arc<dyn ExecutionPlan>]) -> Self
    .parent_filters() -> Vec<Vec<PushedDownPredicate>>
    .self_filters() -> Vec<Vec<Arc<dyn PhysicalExpr>>>
```

`from_child` runs column analysis: a filter is `supported` only if every referenced column
exists in the child schema *and* its index is within `allowed_indices` (default the whole child
schema); indices are then remapped to the child schema.
`from_child_with_allowed_indices` is the join-safe variant that prevents pushing `k@2` into a
left input that also has a differently-indexed `k`.

## 11.3 The two `ExecutionPlan` hooks

```rust
fn gather_filters_for_pushdown(
    &self,
    _phase: FilterPushdownPhase,
    parent_filters: Vec<Arc<dyn PhysicalExpr>>,
    _config: &ConfigOptions,
) -> Result<FilterDescription> {
    FilterDescription::all_unsupported(&parent_filters, &self.children())   // default
}

fn handle_child_pushdown_result(
    &self,
    _phase: FilterPushdownPhase,
    child_pushdown_result: ChildPushdownResult,
    _config: &ConfigOptions,
) -> Result<FilterPushdownPropagation<Arc<dyn ExecutionPlan>>> {
    FilterPushdownPropagation::if_all(child_pushdown_result)                // default
}
```

⚠ **Hard ordering contract**, quoted from the trait doc: "Implementations must preserve the
order of `parent_filters` in the returned child `FilterDescription`: each child parent-filter
result is matched back to the corresponding input parent filter by position."

Protocol, per node: ask the parent via `gather_filters_for_pushdown` → recurse into children →
collect each `FilterPushdownPropagation` → call `handle_child_pushdown_result` on the parent.
The rule refuses to push volatile predicates.

## 11.4 Where the rule sits

**Source** — `REG/datafusion-physical-optimizer-55.1.0/src/optimizer.rs`. The default pipeline,
in order:

```text
OutputRequirements(add) → AggregateStatistics → JoinSelection → LimitedDistinctAggregation
→ FilterPushdown::new()                      [Pre]
→ WindowTopN → EnsureRequirements → CombinePartialFinalAggregate → OptimizeAggregateOrder
→ ProjectionPushdown → OutputRequirements(remove) → TopKAggregation → LimitPushPastWindows
→ HashJoinBuffering → LimitPushdown → TopKRepartition → ProjectionPushdown → PushdownSort
→ EnsureCooperative
→ FilterPushdown::new_post_optimization()    [Post]
→ SanityCheckPlan
```

▲ `EnforceDistribution` and `EnforceSorting` **no longer exist as separate rules**; they were
merged into the single idempotent `EnsureRequirements`. Documentation naming those two rules is
stale for 55.1.0.

## 11.5 `DynamicFilterPhysicalExpr`

**Source** — `REG/datafusion-physical-expr-55.1.0/src/expressions/dynamic_filters/mod.rs`.

```rust
pub fn new(children: Vec<Arc<dyn PhysicalExpr>>, inner: Arc<dyn PhysicalExpr>) -> Self
pub fn current(&self) -> Result<Arc<dyn PhysicalExpr>>
pub fn update(&self, new_expr: Arc<dyn PhysicalExpr>) -> Result<()>
pub fn mark_complete(&self)
pub fn is_used(self: &Arc<Self>) -> bool
```

Generation-counted, broadcast over a `watch` channel, with a per-generation cache for the
remap walk. `update` must supply an expression whose children are a subset of the constructor
children.

⚠ A plan holding a dynamic filter **should also override `reset_state`**, or a recursive CTE or
a reused plan will observe a stale threshold.

Producers: TopK (`SortExec` with a fetch), hash joins, aggregates. Consumers: any
`DataSource`/`FileSource` that accepts the predicate — which is how a `LIMIT 10 ORDER BY x`
ends up pruning Parquet row groups.

## 11.6 ⚠ How this interacts with `supports_filters_pushdown` — the pattern to copy

The two systems answer different questions, and the canonical Parquet implementation shows why
conflating them loses rows:

```rust
// REG/datafusion-datasource-parquet-55.1.0/src/source.rs — FileSource::try_pushdown_filters
let config_pushdown_enabled = config.execution.parquet.pushdown_filters;
let table_pushdown_enabled  = self.pushdown_filters();
let pushdown_filters = table_pushdown_enabled || config_pushdown_enabled;
…
if !pushdown_filters {
    return Ok(FilterPushdownPropagation::with_parent_pushdown_result(
                  vec![PushedDown::No; filters.len()])
              .with_updated_node(source));      // updated node, but "No" to the parent
}
```

It **absorbs the predicate regardless** — it needs it for row-group, page and bloom-filter
pruning — but reports `PushedDown::Yes`, which lets the parent drop its `FilterExec`, **only
when row-level late materialization is actually enabled**.

**A source that can prune only at container granularity must do exactly this.** Reporting
`Yes` when you filter whole containers but not individual rows deletes the `FilterExec` and
silently returns rows that should have been excluded.

Parquet additionally refuses to mark as pushed-down anything referencing a **virtual** column
(`TableSchema::schema_without_virtual_columns`), because virtual columns are synthesized by the
reader and cannot appear in a `RowFilter`.

### Recommendation

One predicate classifier, three consumers: `supports_filters_pushdown` (logical),
`try_pushdown_filters` (physical), and the actual scan-time evaluation. Whether a predicate is
*absorbed* and whether it is *reported as fully handled* are two separate booleans; model them
separately or you will eventually conflate them.

---

# 12. Pruning

## 12.1 `PruningStatistics`

**Source** — `REG/datafusion-common-55.1.0/src/pruning.rs` (⚠ **not** `datafusion-pruning`,
which only re-exports it alongside `PruningPredicate`).

```rust
pub trait PruningStatistics {
    fn min_values(&self, column: &Column) -> Option<ArrayRef>;
    fn max_values(&self, column: &Column) -> Option<ArrayRef>;
    fn num_containers(&self) -> usize;
    fn null_counts(&self, column: &Column) -> Option<ArrayRef>;
    fn row_counts(&self) -> Option<ArrayRef>;                                   // ▲ no column parameter
    fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>;
}
```

**All six are required; there are no provided methods.** ⚠ `contained`'s own doc says "return
`None` (the default)", but there is no default body — the phrase describes the recommended
return value, not a provided method. ▲ `row_counts` lost its `column` parameter in 54.0.0; it
is container-level.

Contract: every returned array must have exactly `num_containers()` rows. A `null` entry means
"unknown for that container"; `None` means "unknown for all containers". `contained`
semantics: `true` = the column's values are **only** from `values`; `false` = **none of**
`values`; `null` = unknown.

The interface is deliberately vectorized so one `RecordBatch` evaluation prunes thousands of
containers at once:

```text
file1: column a  min=5   max=10
file2: column a  no stats
file3: column a  min=20  max=30

min_values("a") -> Some([5, Null, 20])
max_values("a") -> Some([10, Null, 30])
min_values("X") -> None
```

**Index — implementors (4):** `CompositePruningStatistics`, `PrunableStatistics`,
`PartitionPruningStatistics` (▲ **deprecated since 52.0.0**; use
`replace_columns_with_literals` from `datafusion-physical-expr-adapter` to substitute partition
values before pruning — removal announced for 58.0.0), and
`datafusion_datasource_parquet::bloom_filter::BloomFilterStatistics`, which is the reference
implementation of `contained`.

## 12.2 `PruningPredicate`

**Source** — `REG/datafusion-pruning-55.1.0/src/pruning_predicate.rs`.

```rust
#[deprecated(since = "55.0.0", note = "Use PruningPredicateBuilder instead")]   // ▲
pub fn PruningPredicate::try_new(expr: Arc<dyn PhysicalExpr>, schema: SchemaRef) -> Result<Self>

pub fn prune<S: PruningStatistics + ?Sized>(&self, statistics: &S) -> Result<Vec<bool>>
pub fn schema() · orig_expr() · predicate_expr() · literal_guarantees() -> &[LiteralGuarantee]
pub fn always_true() -> bool · required_columns() -> &RequiredColumns · literal_columns() -> Vec<String>
```

`prune` returns one `bool` per container: `true` = **may** match, `false` = definitely does
not.

```rust
pub struct PruningPredicateBuilder<'a>
    ::new()                                   // max_in_list_size = MAX_IN_LIST_SIZE
    .with_file_schema(SchemaRef)              // REQUIRED, else internal_datafusion_err
    .with_error_counter(&'a Count)            // consulted by build(), not try_build()
    .with_max_in_list_size(usize)             // wire to datafusion.execution.parquet.max_in_list_size
    .build(predicate) -> Option<Arc<PruningPredicate>>   // None if always-true or construction failed
    .try_build(predicate) -> Result<PruningPredicate>
```

⚠ `try_build` internally calls `snapshot_physical_expr_opt`, which **unravels a
`DynamicFilterPhysicalExpr` into a static expression**, then runs `PhysicalExprSimplifier` if
the snapshot changed anything. This is the mechanism by which a TopK threshold from §11.5
becomes usable for row-group pruning — a provider that builds its pruning predicate by hand
from `expr` loses it.

## 12.3 `FilePruner` — file-level and mid-scan pruning

**Source** — `REG/datafusion-pruning-55.1.0/src/file_pruner.rs`.

```rust
pub fn FilePruner::new(…)
pub fn FilePruner::try_new(…)            // decides whether a pruner is worthwhile at all
pub fn is_watching(&self) -> bool
pub fn should_prune(&mut self) -> Result<bool>
```

▲ 55.0.0: `try_new` no longer builds a pruner for static predicates without statistics.

Used by the Parquet opener's early-stop wrapper, which re-checks the (possibly updated)
dynamic filter **mid-file** and abandons the rest of the file once it can no longer match.

## 12.4 The Parquet pruning ladder

**Source** — `REG/datafusion-datasource-parquet-55.1.0/src/opener/mod.rs`. Observed order:

| # | Step | Gate |
|---|---|---|
| 1 | `build_pruning_predicates` | — |
| 2 | `prune_by_range` (byte-range split) | always |
| 3 | `prune_by_statistics` (row-group min/max) | `datafusion.execution.parquet.pruning` |
| 4 | `prune_by_bloom_filters` | `…parquet.bloom_filter_on_read` |
| 5 | `prune_by_limit` | a pushed limit |
| 6 | `prune_plan_with_page_index_and_metrics` | `…parquet.enable_page_index` |
| 7 | `FilePruner::try_new` (dynamic, while streaming) | a live dynamic filter |

Types: `RowGroupAccessPlanFilter` (`prune_by_limit`/`_range`/`_statistics`/`_bloom_filters`,
`build()`, `remaining_row_group_count()`, `row_group_indexes()`, `is_fully_matched()`),
`PagePruningAccessPlanFilter`, `BloomFilterStatistics`.

### `ParquetAccessPlan` — a provider-supplied access plan

```rust
pub enum RowGroupAccess { … }   // .should_scan() -> bool
pub struct ParquetAccessPlan
    ::new_all(row_group_count) · ::new_none(row_group_count) · ::new(Vec<RowGroupAccess>)
    ::try_new_from_overall_row_selection(…)
    .set(idx, RowGroupAccess) · .skip(idx) · .scan(idx) · .should_scan(idx) -> bool
    .scan_selection(idx, RowSelection) · .into_overall_row_selection(…)
    .row_group_index_iter() · .row_group_indexes() · .len() · .is_empty()
    .inner() -> &[RowGroupAccess] · .into_inner() -> Vec<RowGroupAccess>
```

Attach one per file with `PartitionedFile::with_extension(access_plan)` — this is the
documented route for an **external index**: your provider consults its own index during
`scan`, produces a per-file `ParquetAccessPlan`, and DataFusion reads only those row groups.
(The consumption site inside the opener was not traced here; the type is public and
`with_extension`'s doc names it explicitly.)

### `ParquetSource` knobs

```rust
ParquetSource::new(table_schema: impl Into<TableSchema>) -> Self
    .with_table_parquet_options(TableParquetOptions)
    .with_metadata_size_hint(usize)
    .with_predicate(&self, Arc<dyn PhysicalExpr>) -> Self     // takes &self, returns a clone
    .with_parquet_file_reader_factory(Arc<dyn ParquetFileReaderFactory>)
    .with_encryption_factory(Arc<dyn EncryptionFactory>)      // cfg(parquet_encryption)
    .with_pushdown_filters(bool)       // default false
    .with_reorder_filters(bool)        // default false
    .with_enable_page_index(bool)      // default true
    .with_bloom_filter_on_read(bool)   // default true
    .with_bloom_filter_on_write(bool)  // default false
    .max_predicate_cache_size() -> Option<usize> · .max_in_list_size() -> usize
```

```rust
pub trait ParquetFileReaderFactory: Debug + Send + Sync + 'static {
    fn create_reader(&self, partition_index: usize, partitioned_file: PartitionedFile,
                     metadata_size_hint: Option<usize>, metrics: &ExecutionPlanMetricsSet)
        -> Result<Box<dyn AsyncFileReader + Send>>;
}
```

This is the hook for a custom metadata cache, an embedded index in the file footer, or a
bespoke byte-range fetch strategy.

---

# 13. Declared physical capability

Everything a provider asserts about its output so the optimizer can *remove* work. These are
declarations, not derivations: DataFusion trusts them, and a wrong one produces wrong results.

## 13.1 `ExecutionPlan` — required vs. provided

**Source** — `REG/datafusion-physical-plan-55.1.0/src/execution_plan.rs`. 6 required, 29
provided, 47 in-tree implementors.

```rust
pub trait ExecutionPlan: Any + Debug + DisplayAs + Send + Sync {
    fn name(&self) -> &str;
    fn properties(&self) -> &Arc<PlanProperties>;                          // ▲ &Arc, not &
    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>;                    // ▲ Vec<&Arc<…>>
    fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>)
        -> Result<TreeNodeRecursion>;                                      // ▲ required since 55.0.0
    fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>;
    #[deprecated(since = "55.0.0", note = "Use `ExecutionPlan::replace_children`")]  // ▲ deprecated AND required
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>)
        -> Result<Arc<dyn ExecutionPlan>>;
}
```

The provided methods that matter to a provider-authored plan:

| Method | Default | Why override |
|---|---|---|
| `schema` | `properties().schema()` clone | rarely |
| `statistics_from_inputs` | delegates to deprecated `partition_statistics` | §13.4 |
| `child_stats_requests` | `ChildStats::Skip` per child | ⚠ required if you *read* `input_stats` |
| `partition_statistics` ▲ | deprecated; asserts index then `Statistics::new_unknown` | legacy |
| `repartitioned` | `Ok(None)` | byte-range or file splitting |
| `required_input_distribution` ▲ | deprecated; `UnspecifiedDistribution` per child | use `input_distribution_requirements` |
| `input_distribution_requirements` | wraps the deprecated one | |
| `required_input_ordering` | `None` per child | |
| `maintains_input_order` | `false` per child | preserves sort across the node |
| `benefits_from_input_partitioning` | `!SinglePartition` per child | |
| `replace_children` | dispatches on `ChildrenPropertiesMode` | the modern child-swap |
| `reset_state` | `replace_children(same, Keep)` | ⚠ override if you hold dynamic filters or mutable state |
| `metrics` | `None` | `EXPLAIN ANALYZE` |
| `supports_limit_pushdown` | `false` | |
| `with_fetch` / `fetch` | `None` | limit absorption |
| `cardinality_effect` | `Unknown` | `Equal` / `LowerEqual` / `GreaterEqual` |
| `try_swapping_with_projection` | `Ok(None)` | projection absorption |
| `gather_filters_for_pushdown` / `handle_child_pushdown_result` | all-unsupported / if-all | §11 |
| `try_pushdown_sort` | `Unsupported` | ordering absorption |
| `with_preserve_order` | `None` | |
| `dynamic_expressions_produced` | `Vec::new()` | declare produced dynamic filters |
| `downcast_delegate` | `None` | wrapper nodes |
| `with_new_state` | `None` | |
| `check_invariants` | `check_default_invariants` | |
| `try_to_proto` | `Ok(None)` | §20.2 |
| `static_name` | last segment of `type_name::<Self>()` | |

```rust
pub struct ReplaceChildrenOptions { pub children_properties: ChildrenPropertiesMode }
pub enum ChildrenPropertiesMode { Keep, Recompute }
pub enum CardinalityEffect { Unknown, Equal, LowerEqual, GreaterEqual }
```

▲ `with_new_children_and_same_properties` and `with_new_children_if_necessary` are also
deprecated, in favour of `replace_children` and `replace_children_if_necessary`.

## 13.2 `PlanProperties`

```rust
pub struct PlanProperties {
    pub eq_properties: EquivalenceProperties,
    pub partitioning: Partitioning,
    pub emission_type: EmissionType,
    pub boundedness: Boundedness,
    pub evaluation_type: EvaluationType,
    pub scheduling_type: SchedulingType,
    output_ordering: Option<LexOrdering>,   // private; derived from eq_properties
}
pub fn new(eq_properties, partitioning, emission_type, boundedness) -> Self
    // evaluation_type = Lazy, scheduling_type = NonCooperative,
    // output_ordering  = eq_properties.output_ordering()
.with_partitioning · .set_eq_properties · .with_eq_properties · .with_boundedness
.with_emission_type · .with_scheduling_type · .with_evaluation_type
.set_constraints · .with_constraints
.equivalence_properties() · .output_partitioning() · .output_ordering()
```

⚠ `set_eq_properties` recomputes `output_ordering` — you cannot declare an ordering that the
equivalence properties do not imply.

```rust
pub enum EmissionType { Incremental, Final, Both }
pub enum Boundedness  { Bounded, Unbounded { requires_infinite_memory: bool } }
pub enum SchedulingType { NonCooperative, Cooperative }
pub enum EvaluationType { Lazy, Eager }
```

`ExecutionPlanProperties` (`output_partitioning`, `output_ordering`, `boundedness`,
`pipeline_behavior`, `equivalence_properties`) is blanket-implemented for
`Arc<dyn ExecutionPlan>` and `&dyn ExecutionPlan`.

## 13.3 Ordering and partitioning

```rust
pub struct PhysicalSortExpr { pub expr: Arc<dyn PhysicalExpr>, pub options: SortOptions }
pub struct LexOrdering { … }
    ::new(exprs: impl IntoIterator<Item = PhysicalSortExpr>) -> Option<Self>   // ⚠ Option — empty is not representable
pub enum OrderingRequirements { Hard(Vec<LexRequirement>), Soft(Vec<LexRequirement>) }
    ::new_alternatives(alternatives: impl IntoIterator<Item = LexRequirement>)

pub fn EquivalenceProperties::new(schema: SchemaRef) -> Self
pub fn EquivalenceProperties::new_with_orderings(
    schema: SchemaRef,
    orderings: impl IntoIterator<Item = impl IntoIterator<Item = PhysicalSortExpr>>) -> Self
pub fn with_constraints(mut self, constraints: Constraints) -> Self
pub fn output_ordering(&self) -> Option<LexOrdering>
pub fn add_orderings(…) · pub fn add_constants(…)

pub enum Partitioning {
    RoundRobinBatch(usize),
    Hash(Vec<Arc<dyn PhysicalExpr>>, usize),
    Range(RangePartitioning),          // ordered key space with split points
    UnknownPartitioning(usize),
}
pub enum Distribution {
    UnspecifiedDistribution,
    SinglePartition,
    #[deprecated(since = "55.0.0", note = "Use Distribution::KeyPartitioned")] ▲
    HashPartitioned(Vec<Arc<dyn PhysicalExpr>>),
    KeyPartitioned(Vec<Arc<dyn PhysicalExpr>>),
}
pub enum PartitioningSatisfaction { NotSatisfied, Exact, Subset }   // .is_satisfied()
pub struct InputDistributionRequirements { ::new(Vec<Distribution>) · .per_child_distributions()
                                           · .child_satisfaction(idx, plan, opts) }
```

**How a file-backed provider declares all of this without touching `PlanProperties`:** set
`FileScanConfigBuilder::with_output_ordering(Vec<LexOrdering>)` — expressed against the **full
table schema, pre-projection, pre-filter** — and optionally
`with_output_partitioning(Some(Partitioning::Hash(...)))`. `FileScanConfig::eq_properties()`
composes ordering + constraints + filter equivalences + projection.

⚠ `FileScanConfig::validated_output_ordering()` **silently drops a declared ordering** when
file-group statistics show that files overlap. A declared ordering only survives if
`PartitionedFile::statistics` are populated.

## 13.4 Statistics propagation ▲

**Source** — `REG/datafusion-physical-plan-55.1.0/src/statistics.rs`.

```rust
pub struct StatisticsArgs { ::new() · .with_partition(Option<usize>) · .partition() -> Option<usize> }
pub enum ChildStats { At(..), Skip }
pub struct StatisticsContext { ::new() · .reset_cache() · .compute(…) }
```

55.0.0 split statistics computation in two:

- `StatisticsContext` owns the bottom-up tree walk and a per-walk memoization cache. Call
  `StatisticsContext::compute` to obtain statistics for a plan.
- `ExecutionPlan::statistics_from_inputs` computes one node's statistics from its children's
  **already-resolved** statistics. The node does not traverse the tree itself.

⚠ **The delegation is one-way.** The default `statistics_from_inputs` calls the deprecated
`partition_statistics`, but the default `partition_statistics` does **not** call
`statistics_from_inputs` — it returns `Statistics::new_unknown`. A node that overrides only
`statistics_from_inputs` silently returns unknown statistics to any caller still using
`partition_statistics`.

⚠ The default `child_stats_requests` is `Skip` for every child, which means the walker never
fetches child statistics. If your `statistics_from_inputs` reads `input_stats`, you must also
override `child_stats_requests` to return `ChildStats::At(..)`.

⚠ Never call `StatisticsContext::compute` from inside `statistics_from_inputs`.

▲ Related removals: `ExecutionPlan::statistics` was removed in 53.0.0; `partition_statistics`
began returning `Arc<Statistics>` in 54.0.0 and was deprecated in 55.0.0.

### `StatisticsProvider` — the supported statistics injection point

**Source** — `REG/datafusion-physical-plan-55.1.0/src/operator_statistics/mod.rs`, reached via
`Session::statistics_registry()`.

```rust
pub trait StatisticsProvider: Debug + Send + Sync {
    fn compute_statistics(&self, plan: &dyn ExecutionPlan, child_stats: &[ExtendedStatistics])
        -> Result<StatisticsResult>;
}
// StatisticsResult::{ Computed(stats), Delegate }  — providers form a chain
// ExtendedStatistics: a base Statistics plus typed extensions
//   get_extension::<T>() · set_extension · has_extension · merge_extensions
// StatisticsRegistry::{ new, with_providers, default_with_builtin_providers,
//                       register, providers, compute, compute_base }
```

`DefaultStatisticsProvider` delegates to each node's `partition_statistics()`. Gated by
`datafusion.optimizer.use_statistics_registry` (default `false`).

**Recommendation.** This — not `TableProvider::statistics()` — is where NDV estimates,
histograms and any other custom statistic belong. It is the only channel that carries typed
extensions and composes across the whole plan.

## 13.5 Cancellation, yielding and spilling

From the `execute` doc:

- The returned `Stream` must free all resources on drop.
- ⚠ **`tokio::task::spawn` is disallowed.** Use `datafusion_common_runtime::SpawnedTask`, a
  `JoinSet`, or `RecordBatchReceiverStreamBuilder`.
- The stream must not occupy the CPU indefinitely; yield via `datafusion_physical_plan::coop`,
  a manual `Poll::Pending` plus waker, or `tokio::task::yield_now()`.

```rust
pub fn cooperative<T>(stream: T) -> CooperativeStream<T>
pub fn make_cooperative(stream: SendableRecordBatchStream) -> SendableRecordBatchStream
```

The `EnsureCooperative` physical optimizer rule wraps any leaf or eager-evaluation root that
reports `NonCooperative` in a `CooperativeExec`. Declaring `SchedulingType::Cooperative` and
wrapping your own stream — which `FileScanConfig` does — avoids that extra node.

`EvaluationType::Eager` declares that your stream produces ahead of downstream demand (spawned
tasks, buffering).

---
# 14. Below `scan` — the reusable datasource fabric

**The point of this section:** a file-backed provider should not author an `ExecutionPlan`. It
should assemble a `FileScanConfig` and hand it to `DataSourceExec`, which supplies partitioning,
equivalence properties, batch splitting, cooperative scheduling, metrics, projection swapping,
filter pushdown forwarding and repartitioning for free.

```text
TableProvider::scan_with_args
        |
        v
FileScanConfigBuilder::new(object_store_url, file_source)
    .with_file_groups(...)  .with_projection_indices(...)  .with_limit(...)
    .with_output_ordering(...)  .with_expr_adapter(...)  .build()
        |
        v
DataSourceExec::from_data_source(file_scan_config)      // FileScanConfig: DataSource
        |
        v   at execute(partition, ctx)
FileSource::create_morselizer  ->  Morselizer::plan_file(PartitionedFile)
        ->  MorselPlanner::plan()  ->  Morsel::into_stream()
        ->  FileStream  ->  BatchSplitStream(batch_size)  ->  cooperative(...)
```

## 14.1 `DataSource`

**Source** — `REG/datafusion-datasource-55.1.0/src/source.rs`.

```rust
pub trait DataSource: Any + Send + Sync + Debug {
    // required (9)
    fn open(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>;
    fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter) -> fmt::Result;
    fn output_partitioning(&self) -> Partitioning;
    fn eq_properties(&self) -> EquivalenceProperties;
    fn partition_statistics(&self, partition: Option<usize>) -> Result<Arc<Statistics>>;
    fn with_fetch(&self, _limit: Option<usize>) -> Option<Arc<dyn DataSource>>;
    fn fetch(&self) -> Option<usize>;
    fn try_swapping_with_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn DataSource>>>;
    fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>)
        -> Result<TreeNodeRecursion>;

    // provided (10)
    fn repartitioned(&self, _target_partitions: usize, _repartition_file_min_size: usize,
                     _output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>> { Ok(None) }
    fn scheduling_type(&self) -> SchedulingType { SchedulingType::NonCooperative }
    fn metrics(&self) -> ExecutionPlanMetricsSet { ExecutionPlanMetricsSet::new() }
    fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions)
        -> Result<FilterPushdownPropagation<Arc<dyn DataSource>>>;              // default: all PushedDown::No
    fn try_pushdown_sort(&self, _order: &[PhysicalSortExpr])
        -> Result<SortOrderPushdownResult<Arc<dyn DataSource>>> { Ok(Unsupported) }
    fn with_preserve_order(&self, _preserve_order: bool) -> Option<Arc<dyn DataSource>> { None }
    fn with_new_state(&self, _state: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn DataSource>> { None }
    fn create_sibling_state(&self, _config: &ConfigOptions) -> Option<Arc<dyn Any + Send + Sync>> { None }
    fn open_with_args(&self, args: OpenArgs) -> Result<SendableRecordBatchStream> {
        self.open(args.partition, args.context)
    }
    #[cfg(feature = "proto")]
    fn try_to_proto(&self, _ctx: &ExecutionPlanEncodeCtx<'_>) -> Result<Option<PhysicalPlanNode>> { Ok(None) }
}

pub struct OpenArgs { pub partition: usize, pub context: Arc<TaskContext>,
                      pub sibling_state: Option<Arc<dyn Any + Send + Sync>> }
    ::new(partition, context) · .with_shared_state(…)
```

`create_sibling_state` is the work-stealing hook: state shared across the partitions of one
plan, initialised lazily once.

## 14.2 `DataSourceExec`

```rust
pub fn from_data_source(data_source: impl DataSource + 'static) -> Arc<Self>
pub fn new(data_source: Arc<dyn DataSource>) -> Self
pub fn data_source(&self) -> &Arc<dyn DataSource>
pub fn with_data_source(mut self, data_source: Arc<dyn DataSource>) -> Self
pub fn with_constraints(mut self, constraints: Constraints) -> Self
pub fn with_partitioning(mut self, partitioning: Partitioning) -> Self
pub fn downcast_to_file_source<T: FileSource>(&self) -> Option<(&FileScanConfig, &T)>
impl<S: DataSource + 'static> From<S> for DataSourceExec
```

What it computes for you:

```rust
PlanProperties::new(data_source.eq_properties(), data_source.output_partitioning(),
                    EmissionType::Incremental, Boundedness::Bounded)
    .with_scheduling_type(data_source.scheduling_type())
```

⚠ **`DataSourceExec` is always `Incremental` + `Bounded`.** An unbounded source cannot use it
unmodified — use `StreamingTable`/`StreamingTableExec` instead (§17.4).

`execute` wraps the source's stream in
`BatchSplitStream::new(stream, context.session_config().batch_size(), SplitMetrics::…)`, so
splitting to `datafusion.execution.batch_size` is automatic and your `open` may return
oversized batches. It lazily initialises `create_sibling_state` once per plan via a
`OnceLock`. `statistics_from_inputs` delegates to `partition_statistics`;
`handle_child_pushdown_result` forwards all parent filters to `DataSource::try_pushdown_filters`
and recomputes `PlanProperties` if the source changed.

## 14.3 `FileScanConfig` and its builder

**Source** — `REG/datafusion-datasource-55.1.0/src/file_scan_config/mod.rs`.

```rust
pub struct FileScanConfig {
    pub object_store_url: ObjectStoreUrl,
    pub file_groups: Vec<FileGroup>,
    pub constraints: Constraints,
    pub limit: Option<usize>,
    pub preserve_order: bool,
    pub output_ordering: Vec<LexOrdering>,
    pub file_compression_type: FileCompressionType,
    pub file_source: Arc<dyn FileSource>,
    pub batch_size: Option<usize>,
    pub expr_adapter_factory: Option<Arc<dyn PhysicalExprAdapterFactory>>,
    pub(crate) statistics: Statistics,          // private on purpose — use .statistics()
    pub output_partitioning: Option<Partitioning>,
}
```

```rust
FileScanConfigBuilder::new(object_store_url: ObjectStoreUrl, file_source: Arc<dyn FileSource>)
  .with_limit(Option<usize>)
  .with_preserve_order(bool)
  .with_source(Arc<dyn FileSource>)
  .with_projection(Option<Vec<usize>>)                       // ▲ DEPRECATED since 51.0.0
  .with_projection_indices(Option<Vec<usize>>) -> Result<Self>   // returns Result
  .with_constraints(Constraints)
  .with_statistics(Statistics)
  .with_file_groups(Vec<FileGroup>) / .with_file_group(FileGroup) / .with_file(PartitionedFile)
  .with_output_ordering(Vec<LexOrdering>)
  .with_output_partitioning(Option<Partitioning>)
  .with_file_compression_type(FileCompressionType)
  .with_batch_size(Option<usize>)
  .with_expr_adapter(Option<Arc<dyn PhysicalExprAdapterFactory>>)
  .build(self) -> FileScanConfig                              // infallible
```

Defaults applied in `build()`: `Constraints::default()`,
`Statistics::new_unknown(table_schema)`, `UNCOMPRESSED`, and
⚠ `preserve_order = preserve_order || !output_ordering.is_empty()` — **declaring an output
ordering silently turns on order preservation**, which disables file-stream work stealing
(§14.9).

⚠ `with_projection_indices` returns `internal_err` if the `FileSource` returns `Ok(None)` from
`try_pushdown_projection`. A custom `FileSource` that does not implement
`try_pushdown_projection` cannot be used with a projection through the builder.

Accessors and helpers:

```rust
.file_schema() · .table_partition_cols() -> &Fields · .statistics() -> Statistics
.projected_schema() -> Result<Arc<Schema>> · .newlines_in_values() · .projected_constraints()
.file_column_projection_indices() · .file_source() -> &Arc<dyn FileSource>

pub fn split_groups_by_statistics_with_target_partitions(
    table_schema: &SchemaRef, file_groups: &[FileGroup],
    sort_order: &LexOrdering, target_partitions: usize) -> Result<Vec<FileGroup>>
pub fn split_groups_by_statistics(
    table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering) -> Result<Vec<FileGroup>>

pub fn output_partitioning_from_partition_fields(
    schema: &Schema, partition_cols: &Fields, partition_count: usize) -> Option<Partitioning>
```

The two `split_groups_*` helpers bin-pack files so that within each group files are
non-overlapping on min/max — the mechanism behind
`datafusion.execution.split_file_groups_by_statistics`, and the way a sorted dataset reaches the
optimizer as genuinely sorted partitions.
`output_partitioning_from_partition_fields` builds `Partitioning::Hash(cols, n)` so a
Hive-partitioned scan can declare hash partitioning and elide a `RepartitionExec`.

`impl DataSource for FileScanConfig` behaviour worth knowing:

| Method | Behaviour |
|---|---|
| `output_partitioning` | `UnknownPartitioning(file_groups.len())` unless `output_partitioning` is set **and** its count equals `file_groups.len()`; otherwise it `warn!`s and falls back |
| `eq_properties` | orderings (validated) + constraints + filter equivalences, then projected |
| `scheduling_type` | `Cooperative` |
| `repartitioned` | ⚠ returns `Ok(None)` outright if `output_partitioning.is_some()` — declaring partitioning forfeits byte-range splitting |
| `create_sibling_state` | `None` (no work stealing) when `preserve_order`, or `output_partitioning.is_some()`, or `!config.execution.enable_file_stream_work_stealing` |
| `try_pushdown_filters` | unprojects filters back to the **table schema** before handing them to the `FileSource` |

## 14.4 `FileSource`

**Source** — `REG/datafusion-datasource-55.1.0/src/file.rs`. ⚠ Note: **no `Debug` bound**.

```rust
pub trait FileSource: Any + Send + Sync {
    // required (6)
    fn create_file_opener(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig,
                          partition: usize) -> Result<Arc<dyn FileOpener>>;
    fn table_schema(&self) -> &TableSchema;
    fn with_batch_size(&self, batch_size: usize) -> Arc<dyn FileSource>;
    fn metrics(&self) -> &ExecutionPlanMetricsSet;
    fn file_type(&self) -> &str;
    fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>)
        -> Result<TreeNodeRecursion>;

    // provided (14)
    fn create_morselizer(&self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig,
                         partition: usize) -> Result<Box<dyn Morselizer>>;   // default wraps create_file_opener
    fn filter(&self) -> Option<Arc<dyn PhysicalExpr>> { None }
    fn projection(&self) -> Option<&ProjectionExprs> { None }
    fn fmt_extra(&self, _t: DisplayFormatType, _f: &mut Formatter) -> fmt::Result { Ok(()) }
    fn supports_repartitioning(&self) -> bool { true }
    fn repartitioned(&self, target_partitions: usize, repartition_file_min_size: usize,
                     output_ordering: Option<LexOrdering>, config: &FileScanConfig)
        -> Result<Option<FileScanConfig>>;                                   // default: FileGroupPartitioner
    fn try_pushdown_filters(&self, filters: Vec<Arc<dyn PhysicalExpr>>, _config: &ConfigOptions)
        -> Result<FilterPushdownPropagation<Arc<dyn FileSource>>>;           // default: all No
    fn try_pushdown_sort(&self, order: &[PhysicalSortExpr], eq_properties: &EquivalenceProperties)
        -> Result<SortOrderPushdownResult<Arc<dyn FileSource>>>;
    #[deprecated(since = "53.0.0")] fn try_reverse_output(…);                // ▲
    fn reorder_files(&self, files: Vec<PartitionedFile>) -> Vec<PartitionedFile> { files }
    fn try_pushdown_projection(&self, _projection: &ProjectionExprs) -> Result<Option<Arc<dyn FileSource>>> { Ok(None) }
    #[deprecated(since = "53.0.0")] fn with_schema_adapter_factory(…) -> Result<…>;  // ▲ not_impl_err!
    #[deprecated(since = "53.0.0")] fn schema_adapter_factory(&self) -> Option<…> { None }  // ▲
    #[cfg(feature = "proto")] fn try_to_proto(…) -> Result<Option<PhysicalPlanNode>> { Ok(None) }
}

pub fn as_file_source<T: FileSource + 'static>(source: T) -> Arc<dyn FileSource>
```

Two genuinely unusual capabilities that an index-backed provider should exploit:

- **`reorder_files`** — the source chooses the order in which its files are read, e.g. most
  selective first according to an external index.
- **`try_pushdown_sort` / `try_reverse_output`** — absorb an ordering requirement, including
  reading a sorted file backwards to satisfy a `DESC` sort without a `SortExec`.

The default `repartitioned` bails if `config.file_compression_type.is_compressed()` or
`!self.supports_repartitioning()`.

Helpers for partial projection pushdown: `ProjectionOpener::try_new(…)`,
`SplitProjection::unprojected(&TableSchema)`, `SplitProjection::new(&Schema, &ProjectionExprs)`
in `REG/datafusion-datasource-55.1.0/src/projection.rs`.

## 14.5 `PartitionedFile`, `FileGroup`, `FileGroupPartitioner`, `FileOpener`

```rust
pub struct PartitionedFile {
    pub object_meta: ObjectMeta,
    pub partition_values: Vec<ScalarValue>,
    pub range: Option<FileRange>,                  // FileRange { start: i64, end: i64 }
    pub statistics: Option<Arc<Statistics>>,
    pub ordering: Option<LexOrdering>,
    pub extensions: FileExtensions,                // ▲ type-keyed since 54.0.0
    pub metadata_size_hint: Option<usize>,
    pub table_reference: Option<TableReference>,
    pub arrow_schema: Option<SchemaRef>,
}
::new(path, size) · ::new_from_meta(ObjectMeta) · ::new_with_range(path, size, start, end)
::from_path(String) -> Result<Self> · From<ObjectMeta>
.with_arrow_schema · .with_partition_values · .with_table_reference · .with_metadata_size_hint
.with_range(start, end) · .with_statistics(Arc<Statistics>) · .has_statistics()
.with_ordering(Option<LexOrdering>) · .path() -> &Path · .effective_size() · .range()
.with_extension<T: Any + Send + Sync>(value) · .extension::<T>() -> Option<&T>
#[deprecated(since = "54.0.0")] .with_extensions(Arc<dyn Any + Send + Sync>)   // ▲
```

⚠ **`with_statistics` takes statistics for the *file columns only*** and then appends exact
partition-column statistics derived from `partition_values` (`null_count = Exact(0)`,
`min = max = Exact(value)`, `distinct_count = Exact(1)`, `sum = Absent`, `byte_size` computed
when the type has a fixed width). The stored field therefore covers the **full table schema**.
The struct doc warns: "DataFusion relies on these statistics for planning (in particular to sort
file groups), so if they are incorrect, incorrect answers may result."

▲ `extensions` is now a **type-keyed map**, so several independent components can attach
without conflict. This is the supported channel for handing a `ParquetAccessPlan` or a custom
index entry to your opener.

`arrow_schema` is the user-supplied *physical file* schema (no partition columns); only the
Parquet reader uses it, to skip parsing embedded `ARROW:schema`.

```rust
pub struct FileGroup { files: Vec<PartitionedFile>, statistics: Option<Arc<Statistics>> }
::new(Vec<PartitionedFile>) · .len() · .with_statistics(Arc<Statistics>) · .files() -> &[PartitionedFile]
.iter() · .into_inner() · .is_empty() · .pop() · .push() · .file_statistics(Option<usize>)
.statistics_mut() · .split_files(n) -> Vec<FileGroup> · .group_by_partition_values(…)

pub struct FileGroupPartitioner
::new()   // target_partitions = 1, repartition_file_min_size = 10 MiB, preserve_order_within_groups = false
.with_target_partitions(usize) · .with_repartition_file_min_size(usize)
.with_preserve_order_within_groups(bool)
.repartition_file_groups(&self, file_groups: &[FileGroup]) -> Option<Vec<FileGroup>>
```

⚠ The partitioner's own default `repartition_file_min_size` is **10 MiB**, while the session
default `datafusion.optimizer.repartition_file_min_size` is **1 MiB**. `FileSource::repartitioned`
is always handed the session value, so the struct default only bites if you construct the
partitioner yourself.

```rust
pub type FileOpenFuture = BoxFuture<'static, Result<BoxStream<'static, Result<RecordBatch>>>>;
pub trait FileOpener: Unpin + Send + Sync {
    fn open(&self, partitioned_file: PartitionedFile) -> Result<FileOpenFuture>;   // ▲ takes PartitionedFile directly
}
pub enum OnError { #[default] Fail, Skip }

FileStreamBuilder::new(config: &'a FileScanConfig)
  .with_partition(usize)                    // required
  .with_file_opener(Arc<dyn FileOpener>)    // wraps into a FileOpenerMorselizer
  .with_morselizer(Box<dyn Morselizer>)     // one of these two is required
  .with_metrics(&'a ExecutionPlanMetricsSet)// required
  .with_on_error(OnError)                   // default Fail
  .build(self) -> Result<FileStream>
#[deprecated(since = "54.0.0", note = "Use FileStreamBuilder instead")] FileStream::new(…)   // ▲
```

Going through `FileStream` gives you these `EXPLAIN ANALYZE` metrics for free:
`time_opening`, `time_scanning_until_data`, `time_scanning_total`, `time_processing`,
`file_open_errors`, `file_scan_errors`, `files_opened`, `files_processed`.

Row-oriented formats also have `datafusion_datasource::decoder::{Decoder, BatchDeserializer}`
for incremental parsing, and `datafusion_datasource::write::BatchSerializer` on the write side.

## 14.6 `FileFormat` and `FileFormatFactory`

The route to a **new file format** that `ListingTable` can drive, rather than a whole new
`TableProvider`.

```rust
#[non_exhaustive] pub struct FileMeta { pub statistics: Statistics, pub ordering: Option<LexOrdering> }

pub trait FileFormat: Any + Send + Sync + fmt::Debug {
    // required (7)
    fn get_ext(&self) -> String;
    fn get_ext_with_compression(&self, _fct: &FileCompressionType) -> Result<String>;
    fn compression_type(&self) -> Option<FileCompressionType>;
    async fn infer_schema(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>,
                          objects: &[ObjectMeta]) -> Result<SchemaRef>;
    async fn infer_stats(&self, state: &dyn Session, store: &Arc<dyn ObjectStore>,
                         table_schema: SchemaRef, object: &ObjectMeta) -> Result<Statistics>;
    async fn create_physical_plan(&self, state: &dyn Session, conf: FileScanConfig)
        -> Result<Arc<dyn ExecutionPlan>>;
    fn file_source(&self, table_schema: TableSchema) -> Arc<dyn FileSource>;

    // provided (3)
    async fn infer_ordering(&self, …) -> Result<Option<LexOrdering>> { Ok(None) }
    async fn infer_stats_and_ordering(&self, …) -> Result<FileMeta>;   // default = infer_stats + infer_ordering
    async fn create_writer_physical_plan(&self, _input, _state, _conf: FileSinkConfig,
                                         _order_requirements: Option<LexRequirement>)
        -> Result<Arc<dyn ExecutionPlan>> { not_impl_err!("Writer not implemented for this format") }
}

pub trait FileFormatFactory: Any + Sync + Send + GetExt + fmt::Debug {
    fn create(&self, state: &dyn Session, format_options: &HashMap<String, String>)
        -> Result<Arc<dyn FileFormat>>;
    fn default(&self) -> Arc<dyn FileFormat>;
}

pub struct DefaultFileType { ::new(Arc<dyn FileFormatFactory>) · .as_format_factory() }
pub fn format_as_file_type(Arc<dyn FileFormatFactory>) -> Arc<dyn FileType>
pub fn file_type_to_format(&Arc<dyn FileType>) -> Result<Arc<dyn FileFormatFactory>>
pub const DEFAULT_SCHEMA_INFER_MAX_RECORD: usize = 1000;
```

`infer_ordering` / `infer_stats_and_ordering` let a format advertise per-file sortedness (for
Parquet, `sorting_columns`) at listing time. That flows into `PartitionedFile::ordering` and
hence into sort elision — a capability most write-ups omit.

**Registration:**

```rust
SessionState::register_file_format(&mut self, Arc<dyn FileFormatFactory>, overwrite: bool) -> Result<()>
SessionState::get_file_format_factory(&self, ext: &str) -> Option<Arc<dyn FileFormatFactory>>
SessionStateBuilder::with_file_formats(self, Vec<Arc<dyn FileFormatFactory>>) -> Self
SessionState::register_table_options_extension<T: ConfigExtension>(&mut self, extension: T)
```

Keyed by `get_ext().to_lowercase()`; registering an existing extension with `overwrite = false`
is a `config_err!`. Lookup lowercases, so extensions are case-insensitive.

```rust
pub struct TableOptions {
    pub csv: CsvOptions, pub parquet: TableParquetOptions, pub json: JsonOptions,
    pub current_format: Option<ConfigFileType>, pub extensions: Extensions,
}
```

Your format's options live in `extensions` via `register_table_options_extension` and become
reachable from SQL `OPTIONS (...)`. `ConfigField::visit` visits only `current_format` under the
key prefix `format` when one is selected, otherwise all three built-ins under
`csv`/`parquet`/`json`.

Related: `datafusion_common::file_options::file_type::{GetExt, FileType}`, with
`DEFAULT_{ARROW,AVRO,CSV,JSON,PARQUET}_EXTENSION` constants.

## 14.7 The morsel API ▲ (experimental)

**Source** — `REG/datafusion-datasource-55.1.0/src/morsel/mod.rs`. Introduced in 54.0.0; the
module doc says explicitly that these are "experimental APIs that may change substantially".

```rust
pub trait Morselizer: Send + Sync + Debug {
    fn plan_file(&self, file: PartitionedFile) -> Result<Box<dyn MorselPlanner>>;
}
pub trait MorselPlanner: Send + Debug {
    fn plan(self: Box<Self>) -> Result<Option<MorselPlan>>;
}
pub trait Morsel: Send + Debug {
    fn into_stream(self: Box<Self>) -> BoxStream<'static, Result<RecordBatch>>;
}
pub struct MorselPlan
    ::new() · .with_morsels(Vec<Box<dyn Morsel>>) · .with_planners(Vec<Box<dyn MorselPlanner>>)
    · .with_pending_planner<F>(io_future) · .set_pending_planner · .take_morsels
    · .take_ready_planners · .take_pending_planner · .has_io_future
pub struct PendingMorselPlanner { ::new<F>(future) · .into_future() }   // also impl Future
```

The contract: **`plan()` is not async and must do CPU work only**; any I/O goes into
`MorselPlan::with_pending_planner`. One outstanding I/O per planner; DataFusion drives planners
in parallel. `plan()` returning `None` means "no batches from this file" — e.g. fully pruned.

This is sub-file work splitting: it is what lets one file become many independently schedulable
units of work. ⚠ `ParquetSource` implements `create_morselizer` natively and its
`create_file_opener` returns `internal_err!` — for a format with meaningful intra-file
structure, `FileOpener` is now the legacy adapter path.

## 14.8 Schema evolution ▲ — which adapter 55.1.0 actually uses

**Unambiguous answer: `PhysicalExprAdapterFactory`. `SchemaAdapter` is dead code kept only for
compile compatibility.**

**Source** — `REG/datafusion-datasource-55.1.0/src/schema_adapter.rs`, module doc:
"Deprecated: `SchemaAdapter` and `SchemaAdapterFactory` have been removed." All of
`CastColumnFn`, `SchemaAdapterFactory`, `SchemaAdapter`, `SchemaMapper`,
`DefaultSchemaAdapterFactory` and `SchemaMapping` carry `#[deprecated(since = "52.0.0")]`, and
the surviving bodies are:

```text
DeprecatedSchemaAdapter::map_column_index   -> None
DeprecatedSchemaAdapter::map_schema         -> not_impl_err!("SchemaAdapter has been removed…")
SchemaMapping::map_batch / map_column_statistics -> not_impl_err!
FileSource::with_schema_adapter_factory (default) -> not_impl_err!
```

⚠ Code calling these **compiles and then fails at runtime**. This is the single most dangerous
stale-knowledge trap in the datasource layer, because the types still exist in the index.

The live API:

```rust
// REG/datafusion-physical-expr-adapter-55.1.0/src/schema_rewriter.rs
pub trait PhysicalExprAdapter: Send + Sync + Debug {
    fn rewrite(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>;   // only method
}
pub trait PhysicalExprAdapterFactory: Send + Sync + Debug {
    fn create(&self, logical_file_schema: SchemaRef, physical_file_schema: SchemaRef)
        -> Result<Arc<dyn PhysicalExprAdapter>>;
}
pub struct DefaultPhysicalExprAdapterFactory;
pub struct DefaultPhysicalExprAdapter { ::new(logical_file_schema, physical_file_schema) }
pub struct BatchAdapterFactory { ::new(target_schema) · .with_adapter_factory(…) · .make_adapter(&SchemaRef) }
pub struct BatchAdapter { .adapt_batch(&self, batch: &RecordBatch) -> Result<RecordBatch> }
pub fn replace_columns_with_literals(…)
```

`DefaultPhysicalExprAdapter` handles type casting (wrapping in `CastExpr`, without
simplification), missing columns as null literals, missing struct fields as null, and
partition-column substitution via `replace_columns_with_literals`.

Wiring: `FileScanConfigBuilder::with_expr_adapter(Some(factory))`, or
`ListingTableConfig::with_expr_adapter_factory(factory)`. When `None`, sources fall back to
`DefaultPhysicalExprAdapterFactory`. `BatchAdapter` replaces any manual
`SchemaMapper::map_batch` call.

## 14.9 `TableSchema` and virtual columns

**Source** — `REG/datafusion-datasource-55.1.0/src/table_schema.rs`. Introduced in 51.0.0 ▲.

```rust
pub struct TableSchema { /* file_schema, table_partition_cols: Fields, virtual_columns: Fields */ }
pub struct TableSchemaBuilder { … }
.schema_without_virtual_columns()
```

Three-way split: columns in the file, columns derived from the directory path (Hive
partitioning), and **virtual columns** — reader-generated columns that are in neither, such as
a row number. Virtual columns are why Parquet refuses to report a predicate referencing one as
pushed-down (§11.6).

---

# 15. Storage binding

## 15.1 Object stores

**Source** — `REG/datafusion-execution-55.1.0/src/object_store.rs`.

```rust
pub struct ObjectStoreUrl { … }
    ::parse(s: impl AsRef<str>) -> Result<Self>
    ::local_filesystem() -> Self
    .as_str(&self) -> &str

pub trait ObjectStoreRegistry: Send + Sync + Debug + 'static {
    fn register_store(&self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>;
    fn get_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>;
    fn deregister_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>> { not_impl_err!(…) }
}
pub struct DefaultObjectStoreRegistry { ::new() }
```

Only `register_store` and `get_store` are required. ⚠ `get_store`'s doc explicitly permits
**lazy ad-hoc discovery and registration** — this is the hook for resolving credentials or
endpoints on first use rather than up front.

⚠ `FileScanConfig::object_store_url` must be the **prefix** of the absolute URL
(`scheme://authority`), never the path to the file itself, and that prefix must have been
registered.

`datafusion_execution::object_store::ObjectStoreRegistry` and
`object_store::registry::ObjectStoreRegistry` are **different traits with the same name**;
resolve the alias before implementing.

## 15.2 `RuntimeEnv`

```rust
pub struct RuntimeEnv {
    pub memory_pool: Arc<dyn MemoryPool>,
    pub disk_manager: Arc<DiskManager>,
    pub cache_manager: Arc<CacheManager>,
    pub object_store_registry: Arc<dyn ObjectStoreRegistry>,
    pub parquet_encryption_factory_registry: Arc<EncryptionFactoryRegistry>,
}
pub fn register_object_store(&self, …)
pub fn deregister_object_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
pub fn object_store(&self, url: impl AsRef<Url>) -> Result<Arc<dyn ObjectStore>>
pub fn spilling_progress(&self) -> SpillingProgress
pub fn register_parquet_encryption_factory(…) · parquet_encryption_factory(…) · config_entries()

RuntimeEnvBuilder::new()
  .with_disk_manager_builder(DiskManagerBuilder)     // ▲ replaces with_disk_manager(DiskManagerConfig)
  .with_memory_pool(Arc<dyn MemoryPool>)
  .with_cache_manager(CacheManagerConfig)
  .with_object_store_registry(…)
  .with_memory_limit(max_memory: usize, memory_fraction: f64)
  .with_temp_file_path(impl Into<PathBuf>) · .with_max_temp_directory_size(u64)
  .with_max_spill_merge_fan_in(usize)
  .with_metadata_cache_limit(usize)
  .with_object_list_cache_limit(usize) · .with_object_list_cache_ttl(Option<Duration>)
  .with_file_statistics_cache_limit(usize)
  .build() -> Result<RuntimeEnv> · .build_arc() -> Result<Arc<RuntimeEnv>>
::from_runtime_env(&RuntimeEnv) -> Self · .entries() · ::generate_config_markdown()
```

## 15.3 Caches ▲

**Source** — `REG/datafusion-execution-55.1.0/src/cache/cache_manager.rs`. ▲ 55.0.0 unified
these into one generic LRU implementation and made them memory-limited.

```rust
pub type FileStatisticsCache = dyn Cache<TableScopedPath, CachedFileMetadata>;
pub type ListFilesCache      = dyn Cache<TableScopedPath, CachedFileList>;
pub type FileMetadataCache   = dyn Cache<Path, CachedFileMetadataEntry>;

pub struct CachedFileMetadata { ::new(…) · .is_valid_for(…) }     // ▲ validates the file schema
pub struct CachedFileList { ::new(Vec<ObjectMeta>) · .files_matching_prefix(&Option<Path>) }
pub trait FileMetadata: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn memory_size(&self) -> usize;
    fn extra_info(&self) -> HashMap<String, String>;
}
pub struct CacheManager
    ::try_new(&CacheManagerConfig) -> Result<Arc<Self>>
    .get_file_statistic_cache() -> Option<Arc<FileStatisticsCache>>
    .get_list_files_cache() -> Option<Arc<ListFilesCache>>
    .get_list_files_cache_ttl() -> Option<Duration>
    .get_file_metadata_cache() -> Arc<FileMetadataCache>            // NOT Option
    .get_file_statistic_cache_limit() · .get_list_files_cache_limit() · .get_metadata_cache_limit()
pub struct CacheManagerConfig { with_file_statistics_cache · with_file_statistics_cache_limit
    · with_list_files_cache · with_list_files_cache_limit · with_list_files_cache_ttl
    · with_file_metadata_cache · with_metadata_cache_limit }
```

The statistics and list-files caches are keyed by `TableScopedPath`, **not** a bare `Path`, so
two tables over the same path do not collide. Both are `Option` — off unless configured. The
metadata cache is always present. `FileMetadata` is the trait to implement if your format has
its own footer/index metadata worth caching.

## 15.4 Execution-time resources

```rust
// TaskContext — REG/datafusion-execution-55.1.0/src/task.rs
.session_config() -> &SessionConfig      // .batch_size(), .options()
.session_id() · .task_id() -> Option<String>
.memory_pool() -> &Arc<dyn MemoryPool>
.runtime_env() -> Arc<RuntimeEnv>        // .object_store(&url)
.scalar_functions() · .higher_order_functions() · .aggregate_functions() · .window_functions()
.with_session_config(…) · .with_runtime(…) · .with_task_id(…)

// Streams
pub trait RecordBatchStream: Stream<Item = Result<RecordBatch>> { fn schema(&self) -> SchemaRef; }
pub type SendableRecordBatchStream = Pin<Box<dyn RecordBatchStream + Send>>;
RecordBatchStreamAdapter::new(schema, stream)
RecordBatchReceiverStreamBuilder::new(schema, capacity)
MemoryStream::try_new(…) · .with_reservation(MemoryReservation) · .with_fetch(Option<usize>)
BatchSplitStream::new(…)
```

⚠ Stream error contract: after yielding an error the stream should not be polled again, and
`Ready(None)` is the recommended subsequent behaviour.

```rust
pub trait MemoryPool: Any + Send + Sync + Debug + Display {
    fn name(&self) -> &str;
    fn register(&self, _consumer: &MemoryConsumer) {}
    fn unregister(&self, _consumer: &MemoryConsumer) {}
    fn grow(&self, reservation: &MemoryReservation, additional: usize);
    fn shrink(&self, reservation: &MemoryReservation, shrink: usize);
    fn try_grow(&self, reservation: &MemoryReservation, additional: usize) -> Result<()>;
    fn reserved(&self) -> usize;
    fn memory_limit(&self) -> MemoryLimit { … }
}
MemoryConsumer::new(name) · .with_can_spill(bool) · .register(&Arc<dyn MemoryPool>) -> MemoryReservation
MemoryReservation:: size · free · shrink · try_shrink · resize · try_resize · grow · try_grow
                    · split(capacity) · new_empty() · take()
```

`MemoryReservation::drop` releases automatically — leak-safe by construction. A provider that
buffers must register a `MemoryConsumer` and reserve, or it is invisible to the memory limit.

`DiskManagerBuilder` / `DiskManager` provide the spill path
(`create_tmp_file`, `used_disk_space`, `max_temp_directory_size`, `max_spill_merge_fan_in`,
`spilling_progress`).

## 15.5 Metrics

`datafusion_physical_plan::metrics` is a **pure re-export** of
`datafusion_physical_expr_common::metrics::*`.

```rust
pub struct ExecutionPlanMetricsSet { ::new() · .register(Arc<Metric>) · .clone_inner() -> MetricsSet }
impl From<MetricsSet> for ExecutionPlanMetricsSet
// every clone() shares the same underlying set — that is how per-partition streams aggregate

pub struct BaselineMetrics
    ::new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self
    .intermediate() · .elapsed_compute() -> &Time · .output_rows() -> &Count · .output_batches() -> &Count
    ::output_rows_skew_metric(metrics: &MetricsSet) -> Option<Arc<Metric>>
    .done() · .record_output(num_rows) · .try_done() · .record_poll(…)

pub enum MetricValue {
    OutputRows(Count), ElapsedCompute(Time), SpillCount(Count), SpilledBytes(Count),
    OutputBytes(Count), OutputBatches(Count), SpilledRows(Count), CurrentMemoryUsage(Gauge),
    Count { name, count }, Gauge { name, gauge }, PeakMemoryUsage { name, gauge },
    Time { name, time }, StartTimestamp(Timestamp), EndTimestamp(Timestamp),
    PruningMetrics { name, pruning_metrics }, Ratio { name, ratio_metrics },
    Custom { name, value: Arc<dyn CustomMetricValue> },
}
```

Calling `record_poll` from your `Stream::poll_next` is the entire integration.
`PruningMetrics` is the idiomatic way to report row-group / file / page pruning in
`EXPLAIN ANALYZE`; `Custom` carries anything else.

---
# 16. Provider construction

Registration puts an existing provider into a namespace. **Construction** is the separate
question of who builds one, from what, and when. There are three distinct factory traits, and a
document that covers only registration misses all of them.

| Trait | Input | Triggered by |
|---|---|---|
| `TableProviderFactory` | `&CreateExternalTable` | `CREATE EXTERNAL TABLE`, `ListingSchemaProvider::refresh`, `catalog.location`+`catalog.format` |
| `TableFunctionImpl` | `&[Expr]` + `&dyn Session` | `SELECT * FROM my_func(1, 2)` |
| `UrlTableFactory` | `&str` (a URL) | `SELECT * FROM 'data.parquet'` |

## 16.1 `TableProviderFactory`

**Source** — `REG/datafusion-session-55.1.0/src/table.rs`.

```rust
#[async_trait]
pub trait TableProviderFactory: Debug + Sync + Send {
    async fn create(&self, state: &dyn Session, cmd: &CreateExternalTable)
        -> Result<Arc<dyn TableProvider>>;
}
```

One required method, nothing else.

```rust
// REG/datafusion-expr-55.1.0/src/logical_plan/ddl.rs
pub struct CreateExternalTable {
    pub schema: DFSchemaRef,
    pub name: TableReference,
    pub locations: Vec<String>,          // ▲ PLURAL since 55.0.0
    pub file_type: String,
    pub table_partition_cols: Vec<String>,
    pub if_not_exists: bool,
    pub or_replace: bool,
    pub temporary: bool,
    pub definition: Option<String>,
    pub order_exprs: Vec<Vec<Sort>>,
    pub unbounded: bool,
    pub options: HashMap<String, String>,
    pub constraints: Constraints,
    pub column_defaults: HashMap<String, Expr>,
}
CreateExternalTable::builder(name, location, file_type, schema) -> CreateExternalTableBuilder
    .with_locations(Vec<String>)   // for more than one
```

⚠ **`locations` is a `Vec`** — older material shows a singular `location: String`.
`CREATE EXTERNAL TABLE … LOCATION ('a.parquet','b.parquet')` is legal; all listed locations
must resolve to the same schema and live on the same object store. `StreamTableFactory` shows
the single-location idiom:

```rust
match cmd.locations.as_slice() {
    [single] => …,
    _ => config_err!("Stream tables support exactly one location; use a listing table to read multiple files"),
}
```

**Registration:**

```rust
SessionStateBuilder::with_table_factory(key: String, Arc<dyn TableProviderFactory>) -> Self
SessionStateBuilder::with_table_factories(HashMap<String, Arc<dyn TableProviderFactory>>) -> Self
SessionState::table_factories(&self) -> &HashMap<String, Arc<dyn TableProviderFactory>>
SessionState::table_factories_mut(&mut self) -> &mut HashMap<…>          // mutate a live state
```

⚠ **The key is the `STORED AS` string, UPPERCASED.** `create_custom_table` does
`state.table_factories().get(cmd.file_type.to_uppercase().as_str())` and otherwise
`exec_datafusion_err!("Unable to find factory for {}", cmd.file_type)`. The defaults are
`PARQUET`, `CSV`, `JSON`, `NDJSON`, `AVRO`, `ARROW`.
⚠ `with_table_factory` **replaces** the whole map unless `with_default_features()` is also
called, which extends it with the built-ins.

**The three in-tree factories:**

| Factory | Behaviour |
|---|---|
| `datafusion::datasource::provider::DefaultTableFactory` | dispatches on `cmd.unbounded` or an `options` key `unbounded == "true"`; holds a `StreamTableFactory` and a `ListingTableFactory` |
| `datafusion::datasource::listing_table_factory::ListingTableFactory` | ⚠ downcasts to `SessionState`, erroring `"ListingTableFactory requires SessionState"`; pre-warms the file-statistics cache when `collect_statistics` is on |
| `datafusion_catalog::stream::StreamTableFactory` | reads `cmd.file_type` as a `StreamEncoding`, builds `FileStreamProvider` → `StreamConfig` → `StreamTable` |

plus `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory` (§20.1) and
`datafusion::test_util::TestTableFactory`.

## 16.2 Table functions (UDTFs)

**Source** — `REG/datafusion-session-55.1.0/src/table.rs`.

```rust
pub struct TableFunctionArgs<'e, 's> { exprs: &'e [Expr], session: &'s dyn Session }
    ::new(exprs, session) · .exprs() -> &'e [Expr] · .session() -> &'s dyn Session

pub trait TableFunctionImpl: Debug + Sync + Send + Any {
    #[deprecated(since = "53.0.0", note = "Implement `TableFunctionImpl::call_with_args` instead")]  // ▲
    fn call(&self, _exprs: &[Expr]) -> Result<Arc<dyn TableProvider>> { internal_err!(…) }

    fn call_with_args(&self, args: TableFunctionArgs) -> Result<Arc<dyn TableProvider>> {
        #[expect(deprecated)] self.call(args.exprs)
    }
}

pub struct TableFunction { name: String, fun: Arc<dyn TableFunctionImpl> }
    ::new(name: String, fun: Arc<dyn TableFunctionImpl>) -> Self
    .name() · .function() -> &Arc<dyn TableFunctionImpl>
    #[deprecated(since = "53.0.0")] .create_table_provider(&[Expr])                      // ▲
    .create_table_provider_with_args(TableFunctionArgs) -> Result<Arc<dyn TableProvider>>
```

⚠ **Both methods are provided and they default into each other.** Implementing neither
compiles and fails at runtime with `internal_err!`. Implement `call_with_args` — it is the only
one that receives the `Session`.

⚠ **What has already happened to the arguments.** Before your `call_with_args` runs, each
argument is passed through `ExprSimplifier::coerce` then `simplify` **against an empty
`DFSchema`**. So `range(1, 2+3)` arrives as `Expr::Literal(Int64(5), _)` — literal folding is
done for you — but **column references cannot appear**. Match on
`Expr::Literal(ScalarValue::…, _)`.

**Registration:**

```rust
SessionContext::register_udtf(&self, name: &str, fun: Arc<dyn TableFunctionImpl>)   // no Result
SessionContext::deregister_udtf(&self, name: &str)                                  // swallows the error
SessionContext::table_function(&self, name: &str) -> Result<Arc<TableFunction>>
SessionState::register_udtf(&mut self, name: &str, fun: Arc<dyn TableFunctionImpl>)
SessionState::deregister_udtf(&mut self, name: &str) -> Result<Option<Arc<dyn TableFunctionImpl>>>
SessionState::table_functions(&self) -> &HashMap<String, Arc<TableFunction>>
```

`CREATE FUNCTION` via a `FunctionFactory` can also produce a UDTF
(`RegisterFunction::Table(name, f)` routes to `register_udtf`), and `DROP FUNCTION`
deregisters it.

**Built-ins** — `datafusion-functions-table` ships exactly **two**:

```rust
pub fn all_default_table_functions() -> Vec<Arc<TableFunction>> { vec![generate_series(), range()] }
#[macro_export] macro_rules! create_udtf_function { … }   // exported for downstream use
```

`generate_series` (inclusive end) and `range` (exclusive end), both delegating to a private
`GenerateSeriesFuncImpl { name, include_end }`, accepting 1–3 arguments whose first must be
Int64/NULL, Timestamp or Date32. `GenerateSeriesTable` is the resulting `TableProvider`. The
`create_udtf_function!` macro gives you a `LazyLock` singleton and is exported for your own
functions.

Table functions also reach the SQL planner through
`ContextProvider::get_table_function_source` (§7.4).

## 16.3 `UrlTableFactory` and the dynamic file catalog

**Source** — `REG/datafusion-catalog-55.1.0/src/dynamic_file/catalog.rs`.
⚠ The file is `dynamic_file/catalog.rs`, **not** `dynamic_file.rs`, and `mod dynamic_file` is
**private**: import these as `datafusion_catalog::{UrlTableFactory, DynamicFileCatalog,
DynamicFileSchemaProvider}` or `datafusion::catalog::…`, never by the canonical path (§0.3).

```rust
#[async_trait]
pub trait UrlTableFactory: Debug + Sync + Send {
    async fn try_new(&self, url: &str) -> Result<Option<Arc<dyn TableProvider>>>;
}

pub struct DynamicFileCatalog { … }                     // CatalogProviderList
    ::new(inner: Arc<dyn CatalogProviderList>, factory: Arc<dyn UrlTableFactory>) -> Self
pub struct DynamicFileSchemaProvider { … }              // SchemaProvider
    ::new(inner: Arc<dyn SchemaProvider>, factory: Arc<dyn UrlTableFactory>) -> Self
struct DynamicFileCatalogProvider { … }                 // ⚠ PRIVATE — not nameable outside the crate
```

The entire trick is three lines of `DynamicFileSchemaProvider::table`:

```rust
if let Some(table) = self.inner.table(name).await? { return Ok(Some(table)); }
self.factory.try_new(name).await
```

Registered tables win; otherwise the **table name is handed to the factory as a URL**.
`Ok(None)` means "not my URL" and is not an error.

⚠ `DynamicFileSchemaProvider::table_names()` and `table_exist()` delegate to `inner` **only**.
A dynamic file table is therefore invisible to `information_schema.tables`, to
`SessionContext::table_exist`, and to `find_and_deregister`. Wrapping is per-call: every
`catalog()`/`schema()` allocates a fresh wrapper `Arc`.

**The in-tree implementation lives in the `datafusion` core crate**, not `datafusion-catalog`:

```rust
// REG/datafusion-55.1.0/src/datasource/dynamic_file.rs
pub struct DynamicListTableFactory { session_store: SessionStore }
    ::new(session_store: SessionStore) -> Self · .session_store() -> &SessionStore
```

Its `try_new` parses a `ListingTableUrl`, reaches the live `SessionState` through the
`SessionStore` (§6.4), and runs
`infer_options → infer_partitions_from_path → infer_schema → ListingTable::try_new`.

⚠ **Two silent-`None` paths**: an unparseable URL, and any error from `infer_options`. Both
surface to the user as a bare "table not found" with no diagnostic about why the file could not
be read.

**Wiring:** `SessionContext::enable_url_table(self) -> Self` — consumes `self`, wraps the
current catalog list in `DynamicFileCatalog`, rebuilds through `into_state_builder()`, then
back-fills the `SessionStore`. Its doc: "This feature is security sensitive and should only be
enabled for systems that wish to permit direct access to the file system from SQL."

## 16.4 `ListingSchemaProvider` — directory-as-schema

⚠ **Canonical path is `datafusion_catalog::listing_schema::ListingSchemaProvider`** — in
`datafusion-catalog`, **not** `datafusion-catalog-listing`, which exports only
`ListingTableConfig`, `SchemaSource`, `ListingOptions`, `ListingTable`, `ListFilesResult` and
`helpers`.

```rust
pub struct ListingSchemaProvider { /* authority, path, factory, store, tables, format */ }
pub fn new(authority: String, path: object_store::path::Path,
           factory: Arc<dyn TableProviderFactory>, store: Arc<dyn ObjectStore>,
           format: String) -> Self
pub async fn refresh(&self, state: &dyn Session) -> Result<()>
```

One subfolder = one table. `refresh` lists the store, derives
`table_name = file_name.split('.').next()`, and for each new name calls
`factory.create(state, &CreateExternalTable::builder(TableReference::bare(name), url, format,
Arc::new(DFSchema::empty())).build())`. ⚠ **The schema handed to the factory is empty** — the
factory must infer.

⚠ `refresh` is **not** automatic. Drive it with
`SessionContext::refresh_catalogs(&self) -> Result<()>`, which walks every catalog and schema
and `downcast_ref::<ListingSchemaProvider>()`s.

⚠ Two divergences from convention: `register_table` returns `Ok(Some(table))` — **the table you
just inserted, not the displaced one** — and all five `SchemaProvider` methods `.expect()` on a
`std::sync::Mutex`, so a poisoned lock panics. `table()` never hits the store; it reads only
the cache that `refresh` populated.

## 16.5 `SessionContext` registration surface

**Source** — `REG/datafusion-55.1.0/src/execution/context/mod.rs` and submodules.

```rust
// catalog
register_catalog(&self, name: impl Into<String>, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
catalog_names(&self) -> Vec<String> · catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
register_catalog_list(&self, Arc<dyn CatalogProviderList>)
async refresh_catalogs(&self) -> Result<()>

// table
register_table(&self, impl Into<TableReference>, Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
deregister_table(&self, impl Into<TableReference>) -> Result<Option<Arc<dyn TableProvider>>>
table_exist(&self, impl Into<TableReference>) -> Result<bool>
async table(&self, impl Into<TableReference>) -> Result<DataFrame>
async table_provider(&self, impl Into<TableReference>) -> Result<Arc<dyn TableProvider>>
read_table(&self, Arc<dyn TableProvider>) -> Result<DataFrame>
register_batch(…) · read_batch(…) · read_batches(…) · read_empty()

// listing / formats
async register_listing_table(&self, table_ref, table_path, options: ListingOptions,
                             provided_schema: Option<SchemaRef>, sql_definition: Option<String>) -> Result<()>
async register_csv / register_parquet / register_json / register_avro / register_arrow

// functions, stores, SQL
register_udtf(&self, name: &str, Arc<dyn TableFunctionImpl>) · deregister_udtf(&self, name: &str)
table_function(&self, name: &str) -> Result<Arc<TableFunction>>
register_variable(&self, VarType, Arc<dyn VarProvider + Send + Sync>)
register_object_store(…) · deregister_object_store(&self, &Url) -> Result<Arc<dyn ObjectStore>>
register_table_options_extension<T: ConfigExtension>(&self, extension: T)
async sql(&self, &str) -> Result<DataFrame>
async sql_with_options(&self, &str, SQLOptions) -> Result<DataFrame>
async execute_logical_plan(&self, LogicalPlan) -> Result<DataFrame>

// session plumbing
state(&self) -> SessionState              // ⚠ CLONES the whole state and marks a new execution start
state_ref(&self) -> Arc<RwLock<SessionState>> · state_weak_ref(&self) -> Weak<RwLock<SessionState>>
into_state_builder(self) -> SessionStateBuilder · enable_url_table(self) -> Self
copied_config(&self) -> SessionConfig · copied_table_options(&self) -> TableOptions
```

Semantics that surprise:

- ⚠ `register_table` / `deregister_table` / `table_exist` / `table_provider` all go through
  `schema_for_ref`, and the `Result` the user sees is the one **your `SchemaProvider` returned**.
  `SessionContext` performs no duplicate check of its own.
- `deregister_table` calls `invalidate_caches(&table_ref, table_provider.table_type())` on
  success.
- `read_table(provider)` builds a scan named `UNNAMED_TABLE` (`"?table?"`), registered nowhere.
- `register_csv`/`parquet`/`json`/`avro` are thin wrappers over `register_listing_table` with a
  **file-extension check** that errors
  `exec_err!("File path '{path}' does not match the expected extension '{ext}'")` unless the
  path is a collection. ⚠ `register_arrow` **skips** that check.
- `register_listing_table` infers the schema when `provided_schema` is `None` and attaches the
  runtime's file-statistics cache.
- ⚠ `state()` clones the entire `SessionState` on every call. It is not free in a loop.

---

# 17. The in-tree implementor catalogue

**Read one before writing one.** The overwhelmingly common mistake is authoring a
`TableProvider` for something that `MemTable`, `ViewTable`, `StreamingTable` or a
`PartitionStream` already does.

**Index:** `rg -P '^datafusion_session::table::TableProvider\t' content/index/impls.tsv | cut -f2`
— 10 rows; three more exist in-tree as private or test-only types.

| Implementor | Reach for it when | Demonstrates |
|---|---|---|
| `datafusion_catalog::empty::EmptyTable` | you need a schema and no rows | the minimum viable provider; still honours projection |
| `datafusion_catalog::memory::table::MemTable` | the data is already in memory | partitions, constraints, column defaults, declared sort order, insert/delete/update |
| `datafusion_catalog::view::ViewTable` | the relation is a stored `LogicalPlan` | `get_logical_plan` + `Cow::Borrowed`, `Exact` pushdown by plan grafting |
| `datafusion_catalog_listing::table::ListingTable` | the data is files in an object store | listing, partition pruning, `scan_with_args`, `insert_into` via `FileSink` |
| `datafusion_catalog::streaming::StreamingTable` | rows are produced on demand, possibly unbounded | `PartitionStream`, declared ordering and partitioning |
| `datafusion_catalog::stream::StreamTable` | append-in-place to one file | read + write over a `StreamProvider` |
| `datafusion_catalog::cte_worktable::CteWorkTable` | — (internal) | recursive-CTE work table, `TableType::Temporary` |
| `datafusion_functions_table::generate_series::GenerateSeriesTable` | — | a provider produced by a UDTF |
| `datafusion_ffi::table_provider::ForeignTableProvider` | — | out-of-process provider (§20.1) |
| `datafusion::test_util::TestTableProvider` | tests | |

## 17.1 `MemTable`

```rust
pub struct MemTable {
    schema: SchemaRef,
    pub batches: Vec<PartitionData>,                 // PartitionData = Arc<RwLock<Vec<RecordBatch>>>
    constraints: Constraints,
    column_defaults: HashMap<String, Expr>,
    pub sort_order: Arc<Mutex<Vec<Vec<SortExpr>>>>,
}
pub fn try_new(schema: SchemaRef, partitions: Vec<Vec<RecordBatch>>) -> Result<Self>
pub fn with_constraints(mut self, constraints: Constraints) -> Self
pub fn with_column_defaults(mut self, column_defaults: HashMap<String, Expr>) -> Self
pub fn with_sort_order(self, mut sort_order: Vec<Vec<SortExpr>>) -> Self    // takes `self`, swaps into the Mutex
pub async fn load(t: Arc<dyn TableProvider>, output_partitions: Option<usize>, state: &dyn Session) -> Result<Self>
```

⚠ `try_new` **rejects an empty `partitions` vector** with "No partitions provided, expected at
least one partition". For an empty table pass `vec![vec![]]`.
⚠ `with_sort_order`'s doc: "If the data is not sorted by this order, DataFusion may produce
**incorrect** results." The outer `Vec` is a list of equivalent orderings.
`load` runs `scan(state, None, &[], None)` + `collect_partitioned`, optionally repartitions
round-robin, and propagates the source's **constraints but not its sort order or column
defaults**.

Implements `schema`, `constraints`, `table_type` (`Base`), `scan`, `insert_into`,
`get_column_default`, `delete_from`, `update`. **Does not implement `truncate` or
`merge_into`.** ⚠ `insert_into` **resets the declared sort order to empty** before doing
anything, and errors `not_impl_err!("{insert_op} not implemented for MemoryTable yet")` for
anything but `Append`. `delete_from`/`update` also clear sort order.

## 17.2 `ViewTable`

```rust
pub fn new(logical_plan: LogicalPlan, definition: Option<String>) -> Self
pub fn definition(&self) -> Option<&String>
pub fn logical_plan(&self) -> &LogicalPlan
```

⚠ `new`'s doc: "the `LogicalPlan` is not validated or type coerced. If this is needed it should
be done after calling this function." `SessionContext::create_view` runs a one-rule
`TypeCoercion` analyzer first — with a **fresh `ConfigOptions::default()`, not the session's**.

`TableType::View`; `get_logical_plan` returns `Cow::Borrowed`; `get_table_definition` returns
the stored SQL. **`supports_filters_pushdown` returns `Exact` for every filter**, which is
sound because it grafts the filter onto the view's plan with `LogicalPlanBuilder::filter`, then
projection, then `limit`, and calls `state.create_physical_plan(&plan)`.

## 17.3 `ListingTable`

```rust
pub fn try_new(config: ListingTableConfig) -> Result<Self>   // "No schema provided." / "No ListingOptions provided"
pub fn with_constraints(…) · with_column_defaults(…) · with_definition(Option<String>)
pub fn with_cache(mut self, cache: Option<Arc<FileStatisticsCache>>) -> Self
pub fn table_paths(&self) -> &Vec<ListingTableUrl> · options() -> &ListingOptions · schema_source() -> SchemaSource
pub fn try_create_output_ordering(&self, …)
pub async fn list_files_for_scan(&self, ctx: &dyn Session, filters: &[Expr], limit: Option<usize>)
    -> Result<ListFilesResult>

pub struct ListingTableConfig {
    pub table_paths: Vec<ListingTableUrl>,
    pub file_schema: Option<SchemaRef>,
    pub options: Option<ListingOptions>,
    pub(crate) schema_source: SchemaSource,
    pub(crate) expr_adapter_factory: Option<Arc<dyn PhysicalExprAdapterFactory>>,
}
::new(table_path) · ::new_with_multi_paths(Vec<ListingTableUrl>)
.with_schema(SchemaRef)            // the FILE schema, WITHOUT partition columns
.with_listing_options(ListingOptions)
.with_expr_adapter_factory(Arc<dyn PhysicalExprAdapterFactory>)
async .infer_schema(state) · async .infer_partitions_from_path(state)

pub struct ListingOptions {
    pub file_extension: String,
    pub format: Arc<dyn FileFormat>,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub file_sort_order: Vec<Vec<SortExpr>>,
    pub output_partitioning: Option<Partitioning>,
}
::new(format) · .with_file_extension(…) · .with_file_extension_opt(…)
.with_table_partition_cols(…) · .with_file_sort_order(…) · .with_output_partitioning(…)
```

⚠ Call `with_listing_options()` **before** `with_schema()`; there is a `debug_assert!`
otherwise.

⚠ **`ListingOptions::{collect_stat, target_partitions}` were REMOVED in 55.0.0** ▲, along with
`with_collect_stat`, `with_target_partitions` and `with_session_config_options`. `ListingTable`
now reads both from the **live `SessionConfig` at scan time**
(`ctx.config().collect_statistics()`, `ctx.config().target_partitions()`, erroring
"ListingTable requires target_partitions to be greater than zero"). Migrate to
`SessionConfig::with_target_partitions(..)` / `.with_collect_statistics(..)`.

⚠ **`ListingTable::with_schema_adapter_factory` is deprecated AND a literal no-op** ▲ — its
body returns `self` unchanged, and `schema_adapter_factory()` always returns `None`. The
replacement is `ListingTableConfig::with_expr_adapter_factory`.

⚠ **`infer_options` and `infer` are not inherent methods.** They live on the extension trait
`datafusion::datasource::listing::ListingTableConfigExt`:

```rust
#[async_trait]
pub trait ListingTableConfigExt {
    async fn infer_options(self, state: &dyn Session) -> Result<ListingTableConfig>;
    async fn infer(self, state: &dyn Session) -> Result<ListingTableConfig>;  // = infer_options + infer_schema
}
```

⚠ It downcasts `state` to `SessionState` and **`.unwrap()`s** — a custom `Session` panics. The
format is inferred from the **first** `table_path` only.

Pushdown: **`Exact`** for filters evaluable purely on partition columns, **`Inexact`** for
everything else. `insert_into` refuses a single-file table path and builds a `FileSinkConfig`
with `FileOutputMode::Automatic`. It does **not** implement `statistics()`.

`output_partitioning` on `ListingOptions` declares N output partitions and yields one file
group per declared partition. ⚠ Its doc is explicit: "DataFusion does not route files by
partition values or validate row placement, so callers must ensure file group `i` contains rows
for partition `i`."

`ListingTableUrl`: `parse`, `try_new(Url, Option<Pattern>)`, `scheme`, `prefix`,
`contains(path, ignore_subdirectory)`, `is_collection`, `is_folder`, `file_extension`,
`strip_prefix`, `list_prefixed_files`, `list_all_files`, `as_str`, `object_store`, `get_url`,
`get_glob`, `with_glob`, `with_table_ref`, `get_table_ref`. The last two scope the file-listing
cache key.

## 17.4 `StreamingTable` and `PartitionStream`

```rust
pub fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>) -> Result<Self>
pub fn with_infinite_table(mut self, infinite: bool) -> Self
pub fn with_sort_order(mut self, sort_order: Vec<SortExpr>) -> Self      // ⚠ singular ordering
pub fn with_output_partitioning(mut self, output_partitioning: Partitioning) -> Self

pub trait PartitionStream: Debug + Send + Sync {
    fn schema(&self) -> &SchemaRef;
    fn execute(&self, ctx: Arc<TaskContext>) -> SendableRecordBatchStream;
}
```

**`PartitionStream` + `StreamingTable` is the lowest-boilerplate way to build a custom source.**
Two synchronous methods, no `ExecutionPlan`, no `DataSource`. It is how the seven
`information_schema` views are implemented.

⚠ `StreamingTable::table_type()` is **`View`**, not `Base`. It ignores `filters`. Its
`with_sort_order` takes `Vec<SortExpr>` (one ordering), unlike `MemTable`'s `Vec<Vec<SortExpr>>`.
It projects the declared ordering through the projection so a projected-away sort key does not
produce a bogus claim. `with_output_partitioning` expressions refer to the **pre-projection**
schema.

## 17.5 `StreamTable`, `StreamProvider`, `FileStreamProvider`

```rust
pub trait StreamProvider: Debug + Send + Sync {
    fn schema(&self) -> &SchemaRef;
    fn reader(&self) -> Result<Box<dyn RecordBatchReader>>;
    fn writer(&self) -> Result<Box<dyn RecordBatchWriter>> { unimplemented!() }   // ⚠ PANICS, not an Err
    fn stream_write_display(&self, t: DisplayFormatType, f: &mut Formatter) -> std::fmt::Result;
}
pub enum StreamEncoding { Csv, Json }                       // + FromStr
pub struct FileStreamProvider { … }
    ::new_file(schema, location: PathBuf)                   // batch_size 1024, Csv, header false
    .with_batch_size(usize) · .with_header(bool) · .with_encoding(StreamEncoding)
pub struct StreamConfig { ::new(Arc<dyn StreamProvider>) · .with_order(Vec<Vec<SortExpr>>) · .with_constraints(…) }
pub struct StreamTable(Arc<StreamConfig>);  ::new(Arc<StreamConfig>)
```

`StreamTable` is `TableType::Base`, always builds `StreamingTableExec` with `infinite = true`,
and ⚠ its `insert_into` **ignores `insert_op`** entirely.

---

# 18. DML and sinks

## 18.1 The method set

```text
insert_into(state, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp)
delete_from(state, filters: Vec<Expr>)
update(state, assignments: Vec<(String, Expr)>, filters: Vec<Expr>)
truncate(state)
merge_into(state, source: Arc<dyn ExecutionPlan>, merge_schema: DFSchemaRef,
           on: Expr, clauses: Vec<MergeIntoClause>)
```

All default to `not_impl_err!`. Additional semantics:

- DELETE with empty filters → delete all rows.
- UPDATE with empty filters → update all rows.
- TRUNCATE → remove all rows.
- INSERT receives an **already-planned** input plus `InsertOp`.
- MERGE receives the source plan, the combined target/source logical schema, the `ON`
  expression, and the matched/not-matched clauses.

```rust
pub enum InsertOp { Append, Overwrite, Replace }   // name(): "Insert Into" / "Insert Overwrite" / "Replace Into"
#[non_exhaustive]
pub enum WriteOp { Insert(InsertOp), Delete, Update, Ctas, Truncate, MergeInto(Box<MergeIntoOp>) }
```

⚠ `InsertOp::Replace` semantics ("existing rows are replaced") are **entirely the provider's
job** — declared constraints are never enforced (§8.5).

## 18.2 MERGE types

**Source** — `REG/datafusion-expr-55.1.0/src/logical_plan/dml.rs`.

```rust
pub struct MergeIntoOp { pub on: Expr, pub clauses: Vec<MergeIntoClause> }
pub struct MergeIntoClause {
    pub kind: MergeIntoClauseKind,
    pub predicate: Option<Expr>,
    pub action: MergeIntoAction,
}
pub enum MergeIntoClauseKind { Matched, NotMatched, NotMatchedByTarget, NotMatchedBySource }
pub enum MergeIntoAction {
    Update(Vec<(String, Expr)>),
    Insert { columns: Vec<String>, values: Vec<Expr> },
    Delete,
}
```

⚠ **`NotMatched` and `NotMatchedByTarget` are semantically identical.** The type-level doc:
"Downstream consumers (planners, table providers, optimizers) **MUST** treat the two variants
identically." Use the provided helpers instead of a hand-written `matches!`:

```rust
pub fn is_not_matched_by_target(&self) -> bool   // true for both spellings
pub fn canonical(self) -> Self                   // collapses NotMatched -> NotMatchedByTarget
```

`MergeIntoOp::exprs() -> Vec<&Expr>` and `with_new_exprs(Vec<Expr>) -> Result<Self>` give a
stable flat ordering (`on`, then each clause's predicate then its action values) for optimizer
rewriting. `merge_schema` is "the target columns followed by the source columns, preserving
their logical qualifiers."

## 18.3 The affected-rows convention

All successful mutation plans follow one convention: return an `ExecutionPlan` whose result is
a **single `UInt64` `count` row**.

⚠ **There is no public helper.** `make_count_schema` exists twice and is **private in both
places** (`datafusion-expr`'s `logical_plan/dml.rs` returning `DFSchemaRef`, and
`datafusion-datasource`'s `sink.rs` returning `SchemaRef`). `make_count_batch` and
`DmlResultExec` are private too. Reproduce it by hand:

```rust
Arc::new(Schema::new(vec![Field::new("count", DataType::UInt64, false)]))
```

`DmlStatement::new` and `CopyTo::new` hard-wire `output_schema: make_count_schema()`, so a
mismatch fails at plan verification rather than at runtime.

**The practical route is to return `DataSinkExec`,** which builds the count schema and the
count batch for you.

## 18.4 `DataSink`, `DataSinkExec`, `FileSink`

```rust
pub trait DataSink: Any + DisplayAs + Debug + Send + Sync {
    fn schema(&self) -> &SchemaRef;
    async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>;
    fn metrics(&self) -> Option<MetricsSet> { None }
    #[cfg(feature = "proto")] fn try_to_proto(&self, _exec: &DataSinkExec, _ctx: &…)
        -> Result<Option<PhysicalPlanNode>> { Ok(None) }
}
```

⚠ **`write_all` is called exactly once per DML statement**, and its doc requires that "prior to
return, the sink should do any commit or rollback required." It returns the row count as `u64`.
Same `Any` supertrait + `impl dyn DataSink { is / downcast_ref }` pattern as `TableProvider`.

```rust
pub fn DataSinkExec::new(input: Arc<dyn ExecutionPlan>, sink: Arc<dyn DataSink>,
                         sort_order: Option<LexRequirement>) -> Self
    .input() · .sink() -> &dyn DataSink · .sort_order() -> &Option<LexRequirement>
    #[cfg(feature = "proto")] .encode_sort_order(ctx) · .decode_sort_order(…)
```

⚠ **`DataSinkExec` requires a single-partition input**; `execute()` asserts `partition == 0`.
The physical optimizer inserts a merge when you go through it, but a hand-built plan must do so
itself.

```rust
#[async_trait]
pub trait FileSink: DataSink {
    fn config(&self) -> &FileSinkConfig;
    async fn spawn_writer_tasks_and_join(&self, context: &Arc<TaskContext>,
        demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver,
        object_store: Arc<dyn ObjectStore>) -> Result<u64>;
    async fn write_all(&self, …) -> Result<u64> { /* resolves the store, demuxes, delegates */ }
}

pub struct FileSinkConfig {
    pub original_url: String,
    pub object_store_url: ObjectStoreUrl,
    pub file_group: FileGroup,
    pub table_paths: Vec<ListingTableUrl>,
    pub output_schema: SchemaRef,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub insert_op: InsertOp,
    pub keep_partition_by_columns: bool,
    pub file_extension: String,
    pub file_output_mode: FileOutputMode,      // ▲ added in 53.0.0
}
pub enum FileOutputMode { #[default] Automatic, SingleFile, Directory }
    pub fn single_file_output(self, base_output_path: &ListingTableUrl) -> bool
impl From<Option<bool>> for FileOutputMode
```

`Automatic` means "a single file iff the URL is not a collection and has a file extension".
Only `config` and `spawn_writer_tasks_and_join` are yours to implement.

**In-tree sinks:** `MemSink`, `CsvSink`, `JsonSink`, `ParquetSink`, and `ArrowFileSink` —
⚠ the last is real but **absent from the generated index**, which is a reminder that
implementor enumerations are lower bounds.

Row serialization for text formats goes through
`datafusion_datasource::write::BatchSerializer`.

---

# 19. Remote catalogs and async resolution

## 19.1 The rule

**Do not make ordinary provider lookup methods disguise arbitrary RPC traffic.**

DataFusion's design explicitly discourages asynchronous remote lookups while walking the
logical catalog hierarchy during planning, because that produces many serialized RPCs. The
intended model is: discover the required table references first, resolve metadata in a batch,
and expose a **synchronous cached catalog snapshot** to the planner.

§7.4 shows why this is not optional: the SQL planner's `ContextProvider` is a pure map lookup
that never touches the catalog, and the one place the catalog *is* consulted swallows schema
resolution failures.

## 19.2 The async traits

**Source** — `REG/datafusion-catalog-55.1.0/src/async.rs` (module `r#async`, re-exported at the
crate root). All three are `Send + Sync`, `#[async_trait]`, **one required method and one
provided `resolve`**.

```rust
pub trait AsyncSchemaProvider: Send + Sync {
    async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>;      // REQUIRED
    async fn resolve(&self, references: &[TableReference], config: &SessionConfig,
                     catalog_name: &str, schema_name: &str) -> Result<Arc<dyn SchemaProvider>>;
}
pub trait AsyncCatalogProvider: Send + Sync {
    async fn schema(&self, name: &str) -> Result<Option<Arc<dyn AsyncSchemaProvider>>>;  // REQUIRED
    async fn resolve(&self, references: &[TableReference], config: &SessionConfig,
                     catalog_name: &str) -> Result<Arc<dyn CatalogProvider>>;
}
pub trait AsyncCatalogProviderList: Send + Sync {
    async fn catalog(&self, name: &str) -> Result<Option<Arc<dyn AsyncCatalogProvider>>>;  // REQUIRED
    async fn resolve(&self, references: &[TableReference], config: &SessionConfig)
        -> Result<Arc<dyn CatalogProviderList>>;
}
```

Note the **descending arity**: the list level takes no name, the catalog level takes
`catalog_name`, the schema level takes both. You supply the names the async object does not
know about itself.

## 19.3 Canonical architecture

```text
remote metadata service
        |
        v
Async* providers
        |
        | resolve(required TableReference[], config)
        v
query-scoped immutable snapshot
        |
        v
CatalogProviderList
  -> CatalogProvider
      -> SchemaProvider
          -> resolved TableProvider
        |
        v
     planner
```

Feed `resolve` from `datafusion_sql::resolve::resolve_table_references` (§5.2), which already
returns a sorted, deduplicated list with CTE names separated out.

## 19.4 ⚠ What `resolve` actually does — and does not

| Fact | Consequence |
|---|---|
| Each level **inlines** the defaulting (`reference.catalog().unwrap_or(&config…default_catalog)`) rather than calling `TableReference::resolve` | behaviour matches, but the defaults come from the `SessionConfig` you pass |
| A reference belonging to another catalog/schema is **`continue`d, not an error** — "a cache-miss is not an error at this point … a not-found error will be raised during planning" | a typo produces "table not found" later, never a resolution error |
| Lookups are **sequential `await`s in a loop**, not concurrent | the `CatalogProvider` docs describe resolving "in parallel"; **this code does not**. Bounded parallelism is your responsibility |
| Negative results are cached, then filtered away | absence within one query is stable |
| The cache is per-`resolve` call and **has "no mechanism for refresh or eviction of stale entries"** | it is one query's snapshot, by design |

The three resulting synchronous types are **all private**:

| Private type | Behaviour |
|---|---|
| `ResolvedSchemaProvider` | a `HashMap`; ⚠ `owner_name()` returns the **catalog name**, not a real owner; `register_table`/`deregister_table` → `not_impl_err!("Attempt to register table '{name}' with ResolvedSchemaProvider which is not supported")` |
| `ResolvedCatalogProvider` | cache only; mutators left at the trait defaults → `not_impl_err!` |
| `ResolvedCatalogProviderList` | ⚠⚠ **`register_catalog` is `unimplemented!("resolved providers cannot handle registration APIs")` — it PANICS.** Because `CatalogProviderList` has no error channel (§0.5), there was nowhere to put a refusal |

⚠ **Once a resolved list is installed on a session, any `CREATE DATABASE` or
`SessionContext::register_catalog` aborts the process.** This is the sharpest edge in the entire
layer.

## 19.5 What resolution does not give you

A coherent remote generation requires a backend revision/snapshot contract or explicit input
capture. Caching supplies **none** of: a remote transaction, full namespace coverage, bounded
parallelism, or refresh. Applications must implement those, and must retain negative-read
coverage.

For an async catalog that must also answer `information_schema.schemata`, the sanctioned escape
hatch is `information_schema::schemata_schema()` plus `InformationSchemataBuilder` (§5.5) —
`InformationSchemaProvider` enumerates schemas synchronously and is unsuitable.

---

# 20. Crossing a process boundary

Async resolution is only one of three remote shapes. The other two are what make a provider
usable in a distributed or polyglot fabric, and neither is reachable from the catalog traits.

## 20.1 FFI providers

**Source** — `REG/datafusion-ffi-55.1.0/src/`. Modules: `arrow_wrappers`, `catalog_provider`,
`catalog_provider_list`, `config`, `execution`, `execution_plan`, `expr`, `ffi_option`,
`insert_op`, `physical_expr`, `physical_optimizer`, `placement`, `plan_properties`, `proto`,
`query_planner`, `record_batch_stream`, `schema_provider`, `session`, `statistics`,
`table_provider`, `table_provider_factory`, `table_source`, `udaf`, `udf`, `udtf`, `udwf`,
`util`, `volatility`.

Every layer of the hierarchy has an FFI pair — a `#[repr(C)]` `FFI_*` vtable and a `Foreign*`
wrapper implementing the Rust trait:

| Rust trait | FFI vtable | Foreign wrapper |
|---|---|---|
| `TableProvider` | `FFI_TableProvider` | `ForeignTableProvider` |
| `SchemaProvider` | `FFI_SchemaProvider` | `ForeignSchemaProvider` |
| `CatalogProvider` | `FFI_CatalogProvider` | `ForeignCatalogProvider` |
| `CatalogProviderList` | `FFI_CatalogProviderList` | `ForeignCatalogProviderList` |
| `TableProviderFactory` | `FFI_TableProviderFactory` | `ForeignTableProviderFactory` |
| `TableFunctionImpl` | `FFI_TableFunction` | `ForeignTableFunction` |
| `Session` | — | `ForeignSession` |
| `ExecutionPlan` | — | `ForeignExecutionPlan` |

plus `FFI_TableType`, `FFI_TableProviderFilterPushDown`, `FFI_InsertOp`, `FFI_TableOptions`,
and `FFI_QueryPlanner` ▲ (new in 55.0.0).

```rust
pub fn FFI_TableProvider::new(
    provider: Arc<dyn TableProvider>,
    can_support_pushdown_filters: bool,
    runtime: Option<Handle>,
    task_ctx_provider: impl Into<FFI_TaskContextProvider>,
    logical_codec: Option<Arc<dyn LogicalExtensionCodec>>,
) -> Self
pub fn FFI_TableProvider::new_with_ffi_codec(
    provider: Arc<dyn TableProvider>, can_support_pushdown_filters: bool,
    runtime: Option<Handle>, logical_codec: FFI_LogicalExtensionCodec,
) -> Self
```

### ⚠ What does not cross the FFI boundary

The `FFI_TableProvider` vtable carries `schema`, `scan`, `table_type`,
`supports_filters_pushdown` (as an `Option`), `insert_into`, `statistics`, `logical_codec`,
`clone`, `release`, `version`, `private_data`, `library_marker_id`.

**It carries no `scan_with_args`, no `delete_from`, no `update`, no `truncate`, no
`merge_into`, no `constraints`, no `get_column_default`, no `get_logical_plan` and no
`get_table_definition`.** Only `scan` and `insert_into` cross for read and write.

`can_support_pushdown_filters == false` sets the pushdown slot to `None`, and the foreign side
then behaves as if every filter is `Unsupported`. Filters cross as a **prost-serialized
`LogicalExprList`**; statistics cross as a prost-encoded `datafusion_proto_common::Statistics`,
with `FFI_Option::None` meaning `statistics()` returned `None`.

Two non-obvious mechanisms:

1. **`library_marker_id`** — `From<&FFI_TableProvider> for Arc<dyn TableProvider>` checks
   whether the handle originated in the same library and, if so, unwraps back to the real
   `Arc<dyn TableProvider>` instead of wrapping in `ForeignTableProvider`. Zero-cost round trip
   within one library.
2. **Re-export instead of double-wrap** — `new_with_ffi_codec` downcasts to
   `ForeignTableProvider` and rebinds only the codec. ⚠ Its doc notes that `runtime` is honoured
   **only when a new wrapper is created**.

▲ 55.0.0: `FFI_LogicalExtensionCodec::task_ctx_provider` became private.

## 20.2 Serialized providers (`datafusion-proto`)

**Source** — `REG/datafusion-proto-55.1.0/src/logical_plan/mod.rs`.

```rust
pub trait LogicalExtensionCodec: Debug + Send + Sync + std::any::Any {
    fn try_decode(&self, buf: &[u8], inputs: &[LogicalPlan], ctx: &TaskContext) -> Result<Extension>;
    fn try_encode(&self, node: &Extension, buf: &mut Vec<u8>) -> Result<()>;
    fn try_decode_table_provider(&self, buf: &[u8], table_ref: &TableReference,
                                 schema: SchemaRef, ctx: &TaskContext)
        -> Result<Arc<dyn TableProvider>>;
    fn try_encode_table_provider(&self, table_ref: &TableReference,
                                 node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<()>;
    // then provided: try_{de,en}code_file_format / _udf / _higher_order_function / _udaf / …
}
```

⚠ **Both table-provider methods are REQUIRED** — you cannot write a codec that ignores them.
`DefaultLogicalExtensionCodec` supplies `not_impl_err!("LogicalExtensionCodec is not provided")`
for both.

⚠ The decode parameter is **`ctx: &TaskContext`**, not `&SessionContext` and not `&dyn Session`.
You get the runtime env and the function registries, **but no catalog**. A provider that must
re-resolve itself by name across the boundary has to carry whatever it needs in its own bytes.

**Where they are called:** on encode, the `TableScan` arm special-cases `CteWorkTable` and
`EmptyTable`; **everything else** falls through to `try_encode_table_provider`, producing a
`CustomTableScanNode { table_name, projection, schema, filters, custom_table_data }`. On decode,
`try_decode_table_provider` runs and the result is fed to
`LogicalPlanBuilder::scan_with_filters`.

⚠ **The round trip is lossy.** `CustomTableScanNode` has **no `fetch` field and no
`statistics_requests` field**, and the decode path uses `scan_with_filters` (no fetch). A
`TableScan` carrying a pushed-down `LIMIT` or a statistics request **loses both**. Pack anything
you need into `custom_table_data` yourself.

```rust
pub trait PhysicalExtensionCodec: Debug + Send + Sync + Any {
    fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext,
                  proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>;
    fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>,
                  proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>;
    // provided: try_decode_udf / try_encode_udf / try_{de,en}code_higher_order_function
    //         / try_decode_expr(buf, inputs, ctx: &PhysicalExprDecodeCtx<'_>) / …
}
```

▲ The `proto_converter` parameter is new; pre-55 material will not have it.
⚠ `try_decode_expr`'s doc: decode nested expressions through `ctx.decode(..)`, **not** the free
`parse_physical_expr`, so the active deduplicating deserializer re-shares a single
`Arc<dyn PhysicalExpr>` — which matters for a `DynamicFilterPhysicalExpr` referenced from both a
`SortExec` and your blob.

### Checklist for shipping a custom provider across processes

1. A `LogicalExtensionCodec` implementing all **four** required methods.
2. A `PhysicalExtensionCodec` if `scan` returns a custom `ExecutionPlan`.
3. `DataSink::try_to_proto` plus `DataSinkExec::encode_sort_order`/`decode_sort_order` (both
   `#[cfg(feature = "proto")]`) if `insert_into` returns a `DataSinkExec`.
   `DataSinkExec::try_to_proto` delegates to `self.sink().try_to_proto(self, ctx)`; `Ok(None)`
   falls back to the extension codec.
4. Whatever `CustomTableScanNode` drops — `fetch`, `statistics_requests` — encoded by hand.

---
# 21. What should be standardized across all providers

**Recommendation throughout.** These are this document's architectural positions, not library
facts.

## A. Registry mechanics

Use one registry primitive for catalogs, schemas and tables:

```rust
#[derive(Debug, Default)]
struct Registry<P: ?Sized> {
    entries: RwLock<BTreeMap<String, Arc<P>>>,
}

impl<P: ?Sized> Registry<P> {
    fn get(&self, name: &str) -> Option<Arc<P>> { /* clone Arc */ }
    fn names(&self) -> Vec<String> { /* deterministic keys */ }
    fn replace(&self, name: String, value: Arc<P>) -> Option<Arc<P>> { … }
    fn insert_new(&self, name: String, value: Arc<P>) -> Result<(), Arc<P>> { … }
    fn remove(&self, name: &str) -> Option<Arc<P>> { … }
}
```

Then:

```text
CatalogProviderList.register_catalog -> Registry::replace
CatalogProvider.register_schema      -> Registry::replace
SchemaProvider.register_table        -> Registry::insert_new
all lookups                          -> Registry::get
all enumerations                     -> Registry::names
all removals                         -> Registry::remove
```

One mechanism; intentionally different policy.

`BTreeMap` gives deterministic enumeration. `DashMap` is also reasonable — and DataFusion's
memory providers use concurrent maps — but deterministic sorting should still be imposed if
metadata, tests or serialization depend on stable order. ⚠ `MemoryCatalogProviderList` does
**not** sort, so anything rendering `information_schema` output from it must.

## B. Name policy

Centralize exactly once:

```text
validate → canonicalize → lookup → register → deregister
```

Every operation must use the same canonicalization function.

Do not independently lowercase names inside individual providers unless that is explicitly your
namespace contract; SQL identifier normalization occurs above this layer (§5.3) and quoted
identifiers may be case-sensitive. ⚠ `TableReference::bare` and `TableReference::from` normalize
differently (§5.1) — pick one entry point and use it everywhere.

## C. Absence and error semantics

```text
Ok(None) / None -> genuinely absent
Err(...)        -> operation attempted and failed
NotImplemented  -> provider does not support the capability
```

Never convert backend, RPC or authorization failures to `None`; that turns infrastructure
failures into misleading "object does not exist" behaviour.

This is particularly important because `CatalogProvider::schema()` and
`CatalogProviderList::catalog()` **cannot return an error at all**: failures must have been
resolved before the synchronous snapshot is exposed (§19). Pick the right `DataFusionError`
variant (§5.7) — in particular, `Internal` is for DataFusion bugs, not for your backend being
down.

## D. Mutability

Give every provider an explicit application-level mutation policy:

```rust
enum MutationPolicy { ReadOnly, Mutable }
```

Do not infer writability from concrete provider type.

For fallible catalog/schema/table methods, return a standardized unsupported/policy error when
appropriate. ⚠ The root list has no error channel: `register_catalog` must insert or replace and
return the previous owner. Keep a conforming list in private assembly state and enforce product
mutation through fallible commands of your own. **Returning `None` without inserting is not a
rejection protocol.**

This is registration-time policy. It is **not** the same as `SQLOptions` (§5.6), which is
query-time, nor the same as the `not_impl_err!` DML defaults, which only cover the paths that
call them. A read-only provider needs all three, because the DataFrame API bypasses `SQLOptions`.

## E. Metadata snapshots

Prefer immutable generation objects:

```text
Arc<CatalogSnapshot>
Arc<SchemaSnapshot>
Arc<TableMetadata>
```

and atomically replace generations when metadata refreshes.

This prevents:

```text
table_names() says X exists
table(X) observes a different generation and says absent
```

and gives planning stable schemas and statistics over the lifetime of a query. ⚠
`information_schema`'s `make_tables`/`make_views` call `catalog_list.catalog(&name).unwrap()`,
so a list whose enumeration and lookup disagree **panics** rather than degrading.

## F. Type and introspection

Standardize `is` / `downcast_ref` (§0.4) only for implementation-specific escape hatches.
Ordinary business logic should depend on provider interfaces or explicit capability interfaces,
not concrete types.

⚠ Downcasting `Session` to `SessionState` (§6.3) is a documented but explicitly unstable escape
hatch. A component that needs it will not work under a custom `Session` or across FFI; several
in-tree components have that limitation and two of them **panic**.

## G. `information_schema`

Treat provider metadata as the canonical source:

```text
schema_names · table_names · owner_name · table_type
TableProvider.schema · constraints · DDL metadata
```

Implement cheap `table_type()` instead of materializing tables solely for metadata enumeration
(§4). ⚠ Note that `information_schema.views` has no such shortcut and instantiates every
provider in every schema; if your catalog is expensive, that query is expensive.

## H. Capabilities

DataFusion exposes capabilities behaviourally rather than through one capabilities struct:

```text
registration       -> method succeeds vs NotImplemented
logical pushdown   -> TableProviderFilterPushDown
physical pushdown  -> PushedDown + updated_node
constraints        -> Option<Constraints>
statistics         -> Option<Statistics> / PartitionedFile::statistics / StatisticsProvider
ordering           -> PlanProperties / FileScanConfig::with_output_ordering
partitioning       -> Partitioning / output_partitioning
DML                -> method succeeds vs NotImplemented
transport          -> presence in an FFI vtable / a codec
```

For application introspection, derive descriptive capability rows from actual implementation
bindings and canonical operation contracts. Do not maintain a separately editable boolean
roster. Distinguish actual support, required policy, established facts and failed invocation;
an implementation method remains decisive. Candidate sources cannot advertise unproved keys.

Concretely: native constraints run no validation, optional statistics requests may be ignored
entirely, a declared ordering may be silently dropped, and a DML count does not prove atomic or
durable publication.

## I. Pushdown coherence across both systems — new

One predicate classifier feeds three consumers:

```text
                    PredicateClassifier
                   /        |          \
  supports_filters_pushdown | try_pushdown_filters
     (logical, §9)          |     (physical, §11)
                            |
                    scan-time evaluation
```

Model **absorbed** and **fully handled** as two separate booleans. Parquet absorbs every
predicate for pruning but reports `PushedDown::No` unless row-level filtering is on (§11.6). A
source that conflates the two deletes the residual `FilterExec` and returns rows it should have
excluded.

## J. Declared capability is a promise, not a description — new

`Exact` pushdown, `Constraints`, `with_sort_order`, `output_ordering` and `output_partitioning`
are all **trusted without verification**. Each one is a place where a wrong declaration produces
silently wrong results rather than an error. Standardize them behind whatever derives them —
an index, a manifest, a catalog commit — and never allow a hand-set flag to survive into
production without a test that a wrong value would fail (§23).

## K. Storage binding is a provider concern — new

The object-store registry is keyed by URL prefix, and `FileScanConfig::object_store_url` must
match a registered prefix. Standardize the mapping from your identity model to
`ObjectStoreUrl` in one place, and use `ObjectStoreRegistry::get_store`'s permitted lazy
registration for credentials resolved at first use rather than scattering
`register_object_store` calls through setup code.

## L. Serde identity — new

If a provider will cross a process boundary, its **identity** must be reconstructible from
bytes plus a `TaskContext` — no catalog is available on the decode side (§20.2). Decide early
whether the wire form is a *reference* (a name the far side re-resolves against its own
catalog) or a *description* (everything needed to rebuild). Whatever the logical plan drops —
`fetch`, `statistics_requests` — belongs in `custom_table_data`.

---

# 22. Recommended implementation decomposition

Avoid monolithic provider implementations.

```text
StandardCatalogList
├── Registry<dyn CatalogProvider>
├── NamePolicy
└── MutationPolicy            (enforced BEFORE register_catalog — no error channel there)

StandardCatalog
├── Registry<dyn SchemaProvider>
├── NamePolicy
├── MutationPolicy
└── SnapshotMetadata

StandardSchema
├── Registry<dyn TableProvider>
├── NamePolicy
├── MutationPolicy
├── owner
└── cheap TableSummary map
       └── TableType

StandardTable<B>
├── TableMetadata                       (§8.4)
├── PredicateClassifier                 (§21.I — feeds all three pushdown consumers)
├── ScanPlanner<B>
│   ├── projection translator
│   ├── predicate translator
│   ├── limit handling
│   ├── statistics resolver
│   └── ordering / partitioning declaration
├── optional MutationPlanner<B>
└── optional codec                      (§20.2)

ScanPlanner<B> for a file-backed B is not an ExecutionPlan:
    FileScanConfigBuilder -> FileScanConfig -> DataSourceExec
    with a FileSource that owns opener/morselizer, pruning and physical pushdown
```

`TableSummary` is useful because schema-level introspection should not require complete table
construction:

```rust
struct TableSummary {
    table_type: TableType,
    // optionally other catalog-visible lightweight metadata
}
```

**Mutation planner:**

```rust
trait MutationPlanner {
    async fn insert(...)   -> Result<Arc<dyn ExecutionPlan>>;
    async fn delete(...)   -> Result<Arc<dyn ExecutionPlan>>;
    async fn update(...)   -> Result<Arc<dyn ExecutionPlan>>;
    async fn truncate(...) -> Result<Arc<dyn ExecutionPlan>>;
    async fn merge(...)    -> Result<Arc<dyn ExecutionPlan>>;
}
```

with one helper for the canonical affected-row output schema (§18.3 — there is no public one),
or by returning `DataSinkExec` and letting it build the count for you.

---

# 23. Invariants an implementation agent should test

**Registry**

```text
name ∈ names()             ⇔ lookup(name).is_some()
register → lookup roundtrip
remove → lookup == None
canonicalized aliases behave identically
enumeration deterministic
```

**Catalog list**

```text
register_catalog replacement returns the previous object
register_catalog on a read-only list does NOT silently return None without inserting
catalog_names() and catalog() never disagree      (information_schema unwraps on this)
```

**Catalog**

```text
register_schema replacement returns previous object
deregister missing -> Ok(None)
cascade=false protects a nonempty schema
schema() performs no I/O and cannot block
```

**Schema**

```text
duplicate register_table -> error
table_exist(name) agrees with table_names / table(name)
table_type(name) == table(name)?.table_type()
metadata-only table_type path does not force expensive initialization
register_table receives an UNQUALIFIED name
```

**Name resolution**

```text
Bare / Partial / Full resolve against the configured defaults
quoted identifiers keep case; unquoted are normalized once
the same canonicalization is used on register and on lookup
```

**Table**

```text
scan output schema == projected schema
projection ordering preserved
filter-only columns need not appear in output
Exact predicates remove ALL failing rows
Inexact predicates retain residual filtering
Unsupported predicates are not assumed applied
inexact predicate + LIMIT cannot under-produce valid rows
filter -> limit -> projection semantics preserved
scan and scan_with_args produce identical plans for identical inputs
statistics_requests reaching scan_with_args are either answered or ignored, never misreported
```

**Pushdown coherence** — new

```text
supports_filters_pushdown and try_pushdown_filters agree about what is absorbed
a predicate reported PushedDown::Yes really needs no residual FilterExec
a container-granularity source never reports Yes for a row-level predicate
a dynamic filter updated mid-execution changes the rows produced
reset_state clears any dynamic-filter generation the plan holds
```

**Declared capability** — new

```text
a declared output ordering actually holds in the emitted batches
a declared Partitioning::Hash really routes rows by those columns
a declared PrimaryKey really is unique (nothing else will check)
declaring an ordering does not silently disable work stealing where that was unintended
statistics_from_inputs is consistent with what partition_statistics reports
child_stats_requests is non-Skip wherever statistics_from_inputs reads input_stats
```

**Schema evolution** — new

```text
a file missing a table column reads as null, not as an error
a file whose column type differs is cast, and the cast is visible in EXPLAIN
no code path reaches a deprecated SchemaAdapter method (they compile and fail at runtime)
```

**DML**

```text
all mutation plans return one UInt64 "count" row
empty DELETE filters == all rows
empty UPDATE filters == all rows
unsupported mutations return standardized capability errors
write_all commits or rolls back before returning
InsertOp::Replace really replaces (constraints are not enforced for you)
MergeIntoClauseKind::NotMatched and NotMatchedByTarget are handled identically
```

**Concurrency and snapshots**

```text
readers never observe partially constructed providers
one query sees one coherent metadata generation
provider objects remain Send + Sync
```

**Transport** — new

```text
a provider round-tripped through LogicalExtensionCodec produces the same rows
anything CustomTableScanNode drops (fetch, statistics_requests) is preserved by hand
a provider crossed via FFI degrades explicitly where the vtable lacks a method,
    rather than appearing to support it
```

---

# 24. Architectural bottom line

The cleanest interpretation:

```text
CatalogProviderList
    = catalog namespace contract, with no error channel

CatalogProvider
    = schema namespace contract, synchronous and I/O-free

SchemaProvider
    = table namespace + lightweight relational metadata contract

TableSource
    = the planning-time face: schema, constraints, pushdown capability, defaults

TableProvider
    = immutable relational metadata
      + scan capability negotiation
      + physical-plan factory
      + optional mutation-plan factory

DataSource / FileSource
    = the reusable scan body: partitioning, pruning, physical pushdown, schema evolution

Session
    = everything the above may reach: config, runtime, registries, planners
```

The maximum reusable standardization lies **around** the DataFusion traits rather than in
trying to replace them:

```text
                             STANDARDIZE
                                  |
  ┌─────────────┬─────────────────┼─────────────────┬─────────────┐
  v             v                 v                 v             v
registry/    metadata/        capability/       predicate      storage &
name         snapshot         error             classification serde
semantics    lifecycle        semantics         (one, three     identity
  |             |                 |              consumers)        |
  └─────────────┴─────────────────┼─────────────────┴──────────────┘
                                  v
                        thin DataFusion adapters
             CatalogProviderList / Catalog / Schema / TableProvider
                                  |
                                  v
                          assembled, not authored
              FileScanConfig -> DataSourceExec -> FileSource -> ObjectStore
```

The critical separations are **three**, not one:

1. **Metadata resolution vs. query execution.** Resolve names and remote metadata into coherent
   provider snapshots; keep `CatalogProviderList`/`CatalogProvider`/`SchemaProvider` thin
   registry contracts; push actual I/O and expensive work into execution rather than catalog
   traversal.
2. **Logical capability vs. physical capability.** `TableSource` answers the optimizer;
   `DataSource`/`FileSource` answer the physical optimizer. Both must be derived from one
   classifier or they will diverge.
3. **Authoring vs. assembling.** A provider's job is to translate optimizer requests into an
   `ExecutionPlan`. For file-backed data that plan should be **assembled** from
   `FileScanConfig` + `DataSourceExec` + a `FileSource`, not written from scratch — the fabric
   supplies partitioning, batching, cooperative scheduling, metrics, pruning and pushdown that
   a hand-written plan will not have.

This matches DataFusion's own architecture: `TableProvider` exists to provide planning
information and produce the physical execution plan, while the catalog abstractions provide the
names and metadata needed to reach it, and the datasource layer provides the body.

---

# A. Appendix — version deltas 49→55 that invalidate remembered patterns

**Guide + Index.** Each row is sourced from an upstream upgrade guide heading and its
present-day status confirmed against the pinned index or source. A guide heading alone is a
lead; where the two disagree, the source wins — the `SchemaAdapter` row is the worked example.

## A.1 Removed or dead

| Item | Release | Status at 55.1.0 |
|---|---|---|
| `as_any` on `TableProvider`, `SchemaProvider`, `CatalogProvider`, `CatalogProviderList`, `TableSource`, `FileSource`, `FileFormat`, `FileFormatFactory`, `DataSource`, `DataSink`, `ExecutionPlan`, `PhysicalExpr` | 54.0.0 | **removed**; `Any` supertrait + inherent `is`/`downcast_ref`. `Session` keeps its `as_any` |
| `ExecutionPlan::statistics` | 53.0.0 | **removed** |
| `ListingOptions::{target_partitions, collect_stat, with_target_partitions, with_collect_stat, with_session_config_options}` | 55.0.0 | **removed**; read from the live `SessionConfig` at scan time |
| `FileScanConfig::partitioned_by_file_group` | 55.0.0 | **removed** |
| `SchemaAdapter`, `SchemaAdapterFactory`, `SchemaMapper`, `DefaultSchemaAdapterFactory`, `SchemaMapping`, `CastColumnFn` | 52.0.0 announced | ⚠ **still present and still compile**, but deprecated with `not_impl_err!` bodies. Calling code **fails at runtime**. Use `PhysicalExprAdapterFactory` |
| `ListingTable::with_schema_adapter_factory` | 52.0.0 | ⚠ deprecated **and a literal no-op**; `schema_adapter_factory()` always returns `None` |
| `CachedParquetFileReader`; `ParquetFileReader` fields | 55.0.0 | removed / private |
| `spill_record_batch_by_size`, `RefCountedTempFile`, `Dialect::AVAILABLE` | 55.0.0 | replaced by `SpillFile` traits, `Dialect::available()` |
| `EnforceDistribution`, `EnforceSorting` optimizer rules | — | merged into the single idempotent `EnsureRequirements` |
| `MovingMin` / `MovingMax` | 55.0.0 | now `pub(crate)` |

## A.2 Deprecated but still live

| Item | Release | Replacement |
|---|---|---|
| `ExecutionPlan::with_new_children` | 55.0.0 | `replace_children(children, ReplaceChildrenOptions { ChildrenPropertiesMode })`. ⚠ **still a required method** |
| `with_new_children_and_same_properties`, `with_new_children_if_necessary` | 55.0.0 | `replace_children`, `replace_children_if_necessary` |
| `ExecutionPlan::partition_statistics` | 55.0.0 | `statistics_from_inputs` + `StatisticsContext`. ⚠ **one-way delegation** (§13.4) |
| `ExecutionPlan::required_input_distribution` | 55.0.0 | `input_distribution_requirements` |
| `Distribution::HashPartitioned` | 55.0.0 | `Distribution::KeyPartitioned` |
| `PruningPredicate::try_new` | 55.0.0 | `PruningPredicateBuilder` (which also snapshots dynamic filters) |
| `is_dynamic_physical_expr` | 55.0.0 | — |
| `TableScan::try_new` | 54.0.0 | `TableScanBuilder` |
| `PartitionedFile::with_extensions` (singular `Arc<dyn Any>`) | 54.0.0 | `with_extension::<T>` / `extension::<T>()`; `extensions` is a type-keyed map |
| `PartitionPruningStatistics` | 52.0.0 | `replace_columns_with_literals`; removal announced for 58.0.0 |
| `FileStream::new` | 54.0.0 | `FileStreamBuilder` |
| `FileSource::try_reverse_output`, `with_schema_adapter_factory`, `schema_adapter_factory` | 53.0.0 | `try_pushdown_sort`; `PhysicalExprAdapterFactory` |
| `TableFunctionImpl::call`, `TableFunction::create_table_provider` | 53.0.0 | `call_with_args` / `create_table_provider_with_args` (they also carry the `Session`) |
| `FileScanConfigBuilder::with_projection` | 51.0.0 | `with_projection_indices` (returns `Result`) |
| `ParquetSource::predicate` | 50.2.0 | `filter` |
| `PartitionedFileStream` | 54.0.0 | unused |

## A.3 Added or changed shape

| Item | Release | Note |
|---|---|---|
| Catalog/planner/optimizer contracts moved to `datafusion-session` | 55.0.0 | old paths still re-export; planner session arg is now `&dyn Session` |
| `Session::catalog_list()` | 55.0.0 | **new required method**; `EmptyCatalogProviderList` added for implementors with no catalog |
| `Session::{query_planner, optimize, physical_optimizers}` | 55.0.0 | new provided methods with ⚠ conservative defaults |
| `ExecutionPlan::apply_expressions` | 55.0.0 | **now required** |
| `ExecutionPlan::properties` returns `&Arc<PlanProperties>` | 53.0.0 | was `&PlanProperties` |
| `ExecutionPlan::partition_statistics` returns `Arc<Statistics>` | 54.0.0 | |
| `ExecutionPlan::reset_state` | 50.0.0 | |
| `QueryPlanner` gained an `Any` supertrait | 55.0.0 | |
| `CreateExternalTable.locations: Vec<String>` | 55.0.0 | was `location: String` |
| `DdlStatement::{CreateExternalTable, CreateFunction}` boxed | 55.0.0 | |
| `PhysicalPlanningContext` | 55.0.0 | physical-planning state moved into it |
| `FileSinkConfig::file_output_mode` | 53.0.0 | `Automatic` / `SingleFile` / `Directory` |
| `PruningStatistics::row_counts` lost its `column` parameter | 54.0.0 | container-level |
| File statistics cache memory-limited under `CacheManager`; unified LRU; `CachedFileMetadata` validates the file schema | 54.0.0, 55.0.0 | |
| `TableSchema` + virtual columns | 51.0.0 | |
| `FileScanConfig::projection` → `projection_exprs` | 51.0.0 | |
| Statistics moved `FileSource` → `FileScanConfig`; projection moved `FileScanConfig` → `FileSource` | 52.0.0 | |
| Partition-column handling moved out of `PhysicalExprAdapter` | 52.0.0 | |
| Adaptive filter representation in Parquet filter pushdown | 52.0.0 | |
| `ListingTable` auto-detects Hive partitioning | 50.0.0 | |
| `ListingTable` moved to `datafusion-catalog-listing` | 51.0.0 | ⚠ but `ListingSchemaProvider` stayed in `datafusion-catalog` |
| Morsel API (`Morselizer`, `MorselPlanner`, `Morsel`) | 54.0.0 | ⚠ explicitly experimental |
| `datafusion-proto`: expression deserialization takes a `TaskContext`; `PhysicalProtoConverterExtension` reshaped; parquet-option conversions now fallible | 54.0.0, 55.0.0 | |
| `FFI_LogicalExtensionCodec::task_ctx_provider` became private; `FFI_QueryPlanner` added | 55.0.0 | |
| MSRV 1.94.0 | 55.0.0 | |

## A.4 ⚠ Named in documentation but absent from 55.1.0

Each was checked against the index and the source:

| Named | Reality |
|---|---|
| `Constraints::empty()` | **does not exist** — named only in `TableProvider::constraints`'s own doc comment. Use `Constraints::default()` |
| `ScanArgs::projection_vec()` | **does not exist**. Use `args.projection().map(\|p\| p.to_vec())` |
| `ScanResult::with_statistics()` | **does not exist**. `ScanResult` is a newtype over the plan with three methods |
| a public affected-rows schema helper | `make_count_schema` is **private in both locations**; `make_count_batch` and `DmlResultExec` are private too |
| `ListingTableConfig::infer` / `infer_options` as inherent methods | they are on the `ListingTableConfigExt` extension trait in the `datafusion` core crate, and ⚠ **panic** on a non-`SessionState` session |
| `PruningStatistics::contained` "default" | there is **no default body**; all six methods are required. The doc phrase describes the recommended return value |

## A.5 Undocumented in the upgrade guides

- **`TableScan::statistics_requests`** — a seventh public field present in 55.1.0 source and
  mentioned in **no** upgrade guide from 46.0.0 to 55.0.0. Its introduction release is not
  established by the available evidence.
- **`ScanArgs` / `ScanResult` / `scan_with_args`** — likewise absent from every upgrade guide;
  the custom-table-providers guide mentions `scan_with_args()` only in passing. Introduction
  release not established.

---

# B. Appendix — settings that change provider behaviour

**Index** — from the generated settings catalogue, which joins each setting to the
`SessionConfig` / `RuntimeEnvBuilder` method that sets it (a dash means the setting is
reachable only as a string, through `set_bool`/`set_str` or SQL `SET`).

## B.1 `datafusion.catalog.*` — all eight

See §5.3. The two that most often surprise: `information_schema` is **`false`** by default, and
`create_default_catalog_and_schema` is **`true`** and will overwrite a catalog you registered
under the default name.

## B.2 Scan mechanics

| Setting | Default | Effect |
|---|---|---|
| `datafusion.execution.batch_size` | 8192 | `DataSourceExec::execute` splits your stream to this |
| `datafusion.execution.coalesce_batches` | true | coalescing between operators |
| `datafusion.execution.target_partitions` | 0 (= cores) | drives `repartitioned`; ⚠ `ListingTable` reads it live |
| `datafusion.execution.collect_statistics` | true | statistics gathered at table creation; ⚠ `ListingTable` reads it live |
| `datafusion.execution.meta_fetch_concurrency` | 32 | parallel file reads during schema/stats inference |
| `datafusion.execution.enable_file_stream_work_stealing` | true | runtime rebalancing of files across partitions; ⚠ gated off by `preserve_order` or a declared `output_partitioning` |
| `datafusion.execution.split_file_groups_by_statistics` | false | "currently experimental" — bin-pack non-overlapping files to eliminate sorts |
| `datafusion.execution.use_row_number_estimates_to_optimize_partitioning` | false | use estimated row counts when deciding to parallelize |
| `datafusion.execution.listing_table_ignore_subdirectory` | true | subdirectory scanning |
| `datafusion.execution.listing_table_factory_infer_partitions` | true | Hive partition inference via `ListingTableFactory` |
| `datafusion.execution.keep_partition_by_columns` | false | write side |
| `datafusion.execution.max_buffered_batches_per_output_file` | — | write side |
| `datafusion.execution.minimum_parallel_output_files` | — | write side |
| `datafusion.execution.objectstore_writer_buffer_size` | — | write side |

## B.3 Parquet read path (`datafusion.execution.parquet.*`)

| Setting | Default | Effect |
|---|---|---|
| `pushdown_filters` | **false** | late materialization — predicates evaluated during decode. OR'd with `ParquetSource::with_pushdown_filters`. ⚠ the deciding input for whether Parquet reports `PushedDown::Yes` |
| `reorder_filters` | **false** | heuristic reordering of `RowFilter` predicates |
| `pruning` | true | row-group min/max pruning |
| `enable_page_index` | true | page-index / `RowSelector` pruning |
| `bloom_filter_on_read` | true | bloom-filter pruning |
| `max_in_list_size` | 20 | cap on `IN (…)` list size for the pruning rewrite; 0 disables |
| `max_predicate_cache_size` | NULL | bytes of predicate-result cache when `pushdown_filters` |
| `force_filter_selections` | false | force `RowSelection` rather than a bitmap |
| `metadata_size_hint` | 524288 | footer prefetch bytes |
| `skip_metadata` | true | skip embedded Arrow metadata (avoids cross-file schema conflicts) |
| `schema_force_view_types` | true | `Utf8`→`Utf8View`, `Binary`→`BinaryView`. ⚠ changes your declared schema's compatibility |
| `binary_as_string` | false | legacy-writer BLOB→string fix |
| `coerce_int96` / `coerce_int96_tz` | NULL | INT96 timestamp resolution and timezone |
| `bloom_filter_on_write`, `bloom_filter_fpp`, `bloom_filter_ndv`, `statistics_enabled`, `statistics_truncate_length`, `column_index_truncate_length`, `data_page_row_count_limit`, `data_pagesize_limit`, `dictionary_enabled`, `dictionary_page_size_limit`, `encoding`, `compression`, `created_by`, `max_row_group_size`, `max_row_group_bytes`, `maximum_parallel_row_group_writers`, `maximum_buffered_record_batches_per_stream`, `allow_single_file_parallelism`, `content_defined_chunking.*` | — | write path |

## B.4 Optimizer and partitioning (`datafusion.optimizer.*`)

| Setting | Default | Effect |
|---|---|---|
| `repartition_file_scans` | true | gates the call to `ExecutionPlan::repartitioned` on a scan |
| `repartition_file_min_size` | 1048576 | minimum size for byte-range splitting; ⚠ the `FileGroupPartitioner` struct default is 10 MiB |
| `preserve_file_partitions` | 0 | minimum distinct partition values before grouping files by Hive value (enables declared output partitioning); 0 = off, 1 = always |
| `subset_repartition_threshold` | 4 | partition count above which `Hash(a)` may satisfy `Hash(a,b)` |
| `enable_round_robin_repartition` | true | |
| `prefer_existing_sort` | false | preserve order over maximising parallelism |
| `enable_sort_pushdown` | true | drives `try_pushdown_sort`; ⚠ the result is *inexact* — the Sort is kept |
| `enable_dynamic_filter_pushdown` | true | master switch for §11.5 |
| `enable_topk_dynamic_filter_pushdown` | true | |
| `enable_join_dynamic_filter_pushdown` | true | |
| `enable_aggregate_dynamic_filter_pushdown` | true | |
| `hash_join_inlist_pushdown_max_size` / `_max_distinct_values` | 131072 / 150 | when a join build side becomes an `InList` dynamic filter |
| `enable_leaf_expression_pushdown` | true | extracts leaf exprs (`get_field`) into projections near the scan |
| `use_statistics_registry` | false | switches to the pluggable `StatisticsProvider` chain (§13.4) |
| `default_filter_selectivity` | 20 | fallback selectivity |
| `filter_null_join_keys`, `unions_to_filter` (disabled by default since 54.0.0) | — | |

## B.5 Runtime (`datafusion.runtime.*`)

`memory_limit` (NULL), `temp_directory` (NULL), `max_temp_directory_size` (100G),
`max_spill_merge_fan_in` (0 = unlimited; a value below 2 is treated as 2),
`metadata_cache_limit` (50M), `file_statistics_cache_limit` (20M),
`list_files_cache_limit` (1M), `list_files_cache_ttl` (NULL).

---

# C. Appendix — verification recipes and provenance

## C.1 What this document was checked against

**Interface-checked** against two portable substrates, both of which reproduce anywhere the
crates are vendored. Neither depends on any host repository.

| Substrate | What it settles |
|---|---|
| The vendored crate sources at `~/.cargo/registry/src/<registry-hash>/datafusion-*-55.1.0/` — 38 crates | exact signatures, exact default bodies, deprecation attributes. **The tie-breaker.** |
| A generated API index over the pinned set — 7,134 canonical items, 21,047 methods, 2,335 trait-implementation edges, 6,406 access-path aliases, plus the upstream guide corpus and runnable examples | exhaustiveness: whether an enumeration is complete rather than merely plausible |

Pins: DataFusion 55.1.0, Arrow 59.3.0, `object_store` 0.13.2, `sqlparser` 0.62.0.
Retrieval date for every claim in this document: **2026-09-15**.

⚠ **A generated alias index under-reports re-exports.** `datafusion-catalog` declares
`mod catalog; mod schema; mod table; mod r#async; mod dynamic_file;` as **private** modules and
re-exports them with `pub use <mod>::*` at the crate root. Those globs produce no alias records,
so the index carries `datafusion_session::CatalogProvider` but **not**
`datafusion_catalog::CatalogProvider`, `datafusion::catalog::TableProvider` or
`datafusion_catalog::UrlTableFactory` — all of which compile, verified in source. Use the index
to prove a symbol **exists**; use the source to prove a path **resolves**.

⚠ The index records **no per-item feature gating** — rustdoc emits none at these format
versions. Feature membership comes from each crate's `[features]` table and nowhere else.
`datafusion-physical-expr-adapter` and `datafusion-pruning` are among the crates documented
**without `all-features`**, so their indexed surface is default-features-only; §12 and §14.8
were read from source for that reason.

Mutable online documentation is **not** proof of a pinned signature or of a remote consistency
guarantee. Context7 and the upstream catalog guide are discovery context only.

## C.2 Recipes

```bash
# Does it exist at this pin?
rg -i 'pushdown|prune|adapter|morsel|scan_args' content/index/symbols.tsv

# What can this type do? (a constructor reveals almost none of it)
rg -P '^datafusion_session::session::Session\t' content/index/methods.tsv | cut -f2,4

# Who implements this trait, exhaustively?
rg -P '^datafusion_session::table::TableProvider\t' content/index/impls.tsv | cut -f2

# Where is an access path really defined?
rg -P '^datafusion::catalog::TableProvider\t' content/index/aliases.tsv

# Every trait in the provider-owning crates — the coverage-closure query for this document
awk -F'\t' '$2=="trait" && $3 ~ /^datafusion-(session|catalog|catalog-listing|datasource|pruning|physical-expr-adapter|ffi)$/ {print $3"\t"$1}' \
  content/index/symbols.tsv | sort

# Exact signature and default body — the tie-breaker
rg -n -A6 'fn supports_filters_pushdown' \
  ~/.cargo/registry/src/*/datafusion-session-55.1.0/src/table.rs

# What changed under me? Read newest-first.
ls content/corpus/guides/library-user-guide/upgrading/
```

## C.3 Counts verified 2026-09-15

```text
TableProvider         3 required / 12 provided / 10 indexed implementors
SchemaProvider        3 / 4 / 6
CatalogProvider       2 / 2 / 3
CatalogProviderList   3 / 0 / 5
Session              16 / 6 / 2
TableSource           1 / 5 / 2
ContextProvider      11 / 7 / 0 indexed (the only real impl is private)
ExecutionPlan         6 / 29 / 47
DataSource            9 / 10 / 2
FileSource            6 / 14 / 5
FileFormat            7 / 3 / 5
PruningStatistics     6 / 0 / 4
38 traits total across datafusion-{session,catalog,catalog-listing,datasource,pruning,
                                   physical-expr-adapter,ffi}
```

⚠ Implementor counts are **lower bounds**. Private and test-only types are not indexed:
`ArrowFileSink`, the seven `information_schema` `PartitionStream`s, `ResolvedSchemaProvider`,
`ResolvedCatalogProvider`, `ResolvedCatalogProviderList`, `DynamicFileCatalogProvider`,
`SessionContextProvider`, `MockTableProvider` and `TestTempTable` are all real and all absent.

## C.4 Stated uncertainty

Recorded rather than smoothed over. Each is a claim this document does **not** make.

- The exact column list of `information_schema.df_settings` was not read; the other six views
  were.
- Whether `TableProvider::statistics()` has **any** consumer in 55.1.0 beyond the FFI wrapper
  was not exhaustively established. The trait doc says mainline does not use it.
- Whether any in-tree provider implements `truncate` or `merge_into`: none was found among
  `MemTable`, `ListingTable`, `ViewTable`, `EmptyTable`, `CteWorkTable`, `StreamTable` and
  `StreamingTable`, but the search was not exhaustive.
- The release that introduced `TableScan::statistics_requests`, `ScanArgs`, `ScanResult` and
  `scan_with_args` — absent from every upgrade guide 46.0.0 → 55.0.0.
- The consumption site inside the Parquet morsel/opener path for a `ParquetAccessPlan` attached
  via `PartitionedFile::with_extension`. The type is public and the doc names the use.
- Full vtable field lists for `FFI_TableProviderFactory`, `FFI_TableFunction` and
  `FFI_CatalogProviderList`; only `FFI_TableProvider`'s was read in full.
- Variant names of `RowGroupAccess` and `SortOrderPushdownResult`, the `MemoryLimit` enum shape,
  the `StatisticsContext::compute` parameter list, and
  `FileGroup::group_by_partition_values`'s parameters.
- Whether `CompositePruningStatistics` carries a deprecation attribute of its own.
- No comparison was made against live docs.rs or upstream `main`. Every ▲ flag is against the
  pinned source's own deprecation attributes and the upstream upgrade guides, not against a
  fetched document.
