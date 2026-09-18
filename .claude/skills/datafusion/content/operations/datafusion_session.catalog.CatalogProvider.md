# `datafusion_session::catalog::CatalogProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.catalog.CatalogProvider.json).

<a id="op-37a065b67403b669ccbe6bad"></a>
## CatalogProvider

`trait` · `datafusion_session::catalog::CatalogProvider` · datafusion-session 55.1.0

```rust
trait CatalogProvider: Any + Debug + Sync + Send
```

Source: `src/catalog.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Represents a catalog, comprising a number of named schemas.

# Catalog Overview

To plan and execute queries, DataFusion needs a "Catalog" that provides
metadata such as which schemas and tables exist, their columns and data
types, and how to access the data.

The Catalog API consists:
* [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6): a collection of `CatalogProvider`s
* [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad): a collection of `SchemaProvider`s (sometimes called a "database" in other systems)
* [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788):  a collection of `TableProvider`s (often called a "schema" in other systems)
* [`TableProvider`]:  individual tables

# Implementing Catalogs

To implement a catalog, you implement at least one of the [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6),
[`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) and [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) traits and register them
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

The [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) can support this use case, but it takes some care.
The planning APIs in DataFusion are not `async` and thus network IO can not
be performed "lazily" / "on demand" during query planning. The rationale for
this design is that using remote procedure calls for all catalog accesses
required for query planning would likely result in multiple network calls
per plan, resulting in very poor planning performance.

To implement [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) and [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) for remote catalogs,
you need to provide an in memory snapshot of the required metadata. Most
systems typically either already have this information cached locally or can
batch access to the remote catalog to retrieve multiple schemas and tables
in a single network call.

Note that [`SchemaProvider::table`](../operations/datafusion_session.schema.SchemaProvider.md#op-2f6985f66089b074a19145db) **is** an `async` function in order to
simplify implementing simple [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788)s. For many table formats it
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

<a id="op-5943fc7567a0557e14c47947"></a>
## deregister_schema

`function` · `datafusion_session::catalog::CatalogProvider::deregister_schema` · datafusion-session 55.1.0

```rust
fn deregister_schema(&self, _name: &str, _cascade: bool) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Source: `src/catalog.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Removes a schema from this catalog. Implementations of this method should return
errors if the schema exists but cannot be dropped. For example, in DataFusion's
default in-memory catalog, `MemoryCatalogProvider`, a non-empty schema
will only be successfully dropped when `cascade` is true.
This is equivalent to how DROP SCHEMA works in PostgreSQL.

Implementations of this method should return None if schema with `name`
does not exist.

By default returns a "Not Implemented" error

<a id="op-75aadd6b4e97aa917c28bf67"></a>
## register_schema

`function` · `datafusion_session::catalog::CatalogProvider::register_schema` · datafusion-session 55.1.0

```rust
fn register_schema(&self, name: &str, schema: Arc<dyn SchemaProvider>) -> Result<Option<Arc<dyn SchemaProvider>>>
```

Source: `src/catalog.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Adds a new schema to this catalog.

If a schema of the same name existed before, it is replaced in
the catalog and returned.

By default returns a "Not Implemented" error

<a id="op-b79abc665424f2077d1417a5"></a>
## schema

`function` · `datafusion_session::catalog::CatalogProvider::schema` · datafusion-session 55.1.0

```rust
fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>>
```

Source: `src/catalog.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves a specific schema from the catalog by name, provided it exists.

<a id="op-a8a71bc7379553097f1cb20c"></a>
## schema_names

`function` · `datafusion_session::catalog::CatalogProvider::schema_names` · datafusion-session 55.1.0

```rust
fn schema_names(&self) -> Vec<String>
```

Source: `src/catalog.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves the list of available schema names in this catalog.
