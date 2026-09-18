# `datafusion_session::catalog`

Crate `datafusion-session` · 3 public items · structured records in [`model/datafusion_session.catalog.json`](../model/datafusion_session.catalog.json)

## EmptyCatalogProviderList

`struct` · `datafusion_session::catalog::EmptyCatalogProviderList`

Also reachable as `datafusion_session::EmptyCatalogProviderList`

```rust
struct EmptyCatalogProviderList
```

**Implements**: `datafusion_session::catalog::CatalogProviderList`

**Derives**: Debug, Default

**via `datafusion_session::catalog::CatalogProviderList`**

```rust
fn catalog(&self, _name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, _name: String, _catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.catalog.EmptyCatalogProviderList.md).


A catalog list that contains no catalogs.

[`Session`](crate::Session) implementations that do not provide catalog
access can return this list explicitly.

---

## CatalogProvider

`trait` · `datafusion_session::catalog::CatalogProvider`

Also reachable as `datafusion_session::CatalogProvider`

```rust
trait CatalogProvider: Any + Debug + Sync + Send
```

**Implementors** (3)

- `datafusion_catalog::memory::catalog::MemoryCatalogProvider`
- `datafusion_ffi::catalog_provider::ForeignCatalogProvider`
- `datafusion_ffi::tests::catalog::FixedCatalogProvider`

**Methods** (4)

```rust
fn deregister_schema(&self, _name: &str, _cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
fn schema_names(&self) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.catalog.CatalogProvider.md).


Represents a catalog, comprising a number of named schemas.

# Catalog Overview

To plan and execute queries, DataFusion needs a "Catalog" that provides
metadata such as which schemas and tables exist, their columns and data
types, and how to access the data.

The Catalog API consists:
* [`CatalogProviderList`]: a collection of `CatalogProvider`s
* [`CatalogProvider`]: a collection of `SchemaProvider`s (sometimes called a "database" in other systems)
* [`SchemaProvider`]:  a collection of `TableProvider`s (often called a "schema" in other systems)
* [`TableProvider`]:  individual tables

# Implementing Catalogs

To implement a catalog, you implement at least one of the [`CatalogProviderList`],
[`CatalogProvider`] and [`SchemaProvider`] traits and register them
appropriately in the `SessionContext`.

DataFusion comes with a simple in-memory catalog implementation,
`MemoryCatalogProvider`, that is used by default and has no persistence.
DataFusion does not include more complex Catalog implementations because
catalog management is a key design choice for most data systems, and thus
it is unlikely that any general-purpose catalog implementation will work
well across many use cases.

# Implementing "Remote" catalogs

See [`remote_catalog`] for an end to end example of how to implement a
remote catalog.

Sometimes catalog information is stored remotely and requires a network call
to retrieve. For example, the [Delta Lake] table format stores table
metadata in files on S3 that must be first downloaded to discover what
schemas and tables exist.

[Delta Lake]: https://delta.io/
[`remote_catalog`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/remote_catalog.rs

The [`CatalogProvider`] can support this use case, but it takes some care.
The planning APIs in DataFusion are not `async` and thus network IO can not
be performed "lazily" / "on demand" during query planning. The rationale for
this design is that using remote procedure calls for all catalog accesses
required for query planning would likely result in multiple network calls
per plan, resulting in very poor planning performance.

To implement [`CatalogProvider`] and [`SchemaProvider`] for remote catalogs,
you need to provide an in memory snapshot of the required metadata. Most
systems typically either already have this information cached locally or can
batch access to the remote catalog to retrieve multiple schemas and tables
in a single network call.

Note that [`SchemaProvider::table`] **is** an `async` function in order to
simplify implementing simple [`SchemaProvider`]s. For many table formats it
is easy to list all available tables but there is additional non trivial
access required to read table details (e.g. statistics).

The pattern that DataFusion itself uses to plan SQL queries is to walk over
the query to find all table references, performing required remote catalog
lookups in parallel, storing the results in a cached snapshot, and then plans
the query using that snapshot.

# Example Catalog Implementations

Here are some examples of how to implement custom catalogs:

* [`datafusion-cli`]: [`DynamicFileCatalogProvider`] catalog provider
  that treats files and directories on a filesystem as tables.

* The [`catalog.rs`]:  a simple directory based catalog.

* [delta-rs]:  [`UnityCatalogProvider`] implementation that can
  read from Delta Lake tables

[`datafusion-cli`]: https://datafusion.apache.org/user-guide/cli/index.html
[`DynamicFileCatalogProvider`]: https://github.com/apache/datafusion/blob/31b9b48b08592b7d293f46e75707aad7dadd7cbc/datafusion-cli/src/catalog.rs#L75
[`catalog.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/catalog.rs
[delta-rs]: https://github.com/delta-io/delta-rs
[`UnityCatalogProvider`]: https://github.com/delta-io/delta-rs/blob/951436ecec476ce65b5ed3b58b50fb0846ca7b91/crates/deltalake-core/src/data_catalog/unity/datafusion.rs#L111-L123

[`TableProvider`]: crate::TableProvider

---

## CatalogProviderList

`trait` · `datafusion_session::catalog::CatalogProviderList`

Also reachable as `datafusion_session::CatalogProviderList`

```rust
trait CatalogProviderList: Any + Debug + Sync + Send
```

**Implementors** (5)

- `datafusion_catalog::dynamic_file::catalog::DynamicFileCatalog`
- `datafusion_catalog::memory::catalog::MemoryCatalogProviderList`
- `datafusion_ffi::catalog_provider_list::ForeignCatalogProviderList`
- `datafusion_ffi::tests::catalog::FixedCatalogProviderList`
- `datafusion_session::catalog::EmptyCatalogProviderList`

**Methods** (3)

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn register_catalog(&self, name: String, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_session.catalog.CatalogProviderList.md).


Represent a list of named [`CatalogProvider`]s.

Please see the documentation on [`CatalogProvider`] for details of
implementing a custom catalog.

---
