# `datafusion_session::schema::SchemaProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.schema.SchemaProvider.json).

<a id="op-009a61a5d6d9122859b6d788"></a>
## SchemaProvider

`trait` · `datafusion_session::schema::SchemaProvider` · datafusion-session 55.1.0

```rust
trait SchemaProvider: Any + Debug + Sync + Send
```

Source: `src/schema.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Represents a schema, comprising a number of named tables.

Please see [`CatalogProvider`] for details of implementing a custom catalog.

[`CatalogProvider`]: super::CatalogProvider

<a id="op-43c65e4799f5bbad3d633102"></a>
## deregister_table

`function` · `datafusion_session::schema::SchemaProvider::deregister_table` · datafusion-session 55.1.0

```rust
fn deregister_table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>>
```

Source: `src/schema.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

If supported by the implementation, removes the `name` table from this
schema and returns the previously registered [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e), if any.

If no `name` table exists, returns Ok(None).

<a id="op-0e4183161324d2938d99a213"></a>
## owner_name

`function` · `datafusion_session::schema::SchemaProvider::owner_name` · datafusion-session 55.1.0

```rust
fn owner_name(&self) -> Option<&str>
```

Source: `src/schema.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Returns the owner of the Schema, default is None. This value is reported
as part of `information_schema.schemata`.

<a id="op-e4ae14cc39be08d4e7fd82b7"></a>
## register_table

`function` · `datafusion_session::schema::SchemaProvider::register_table` · datafusion-session 55.1.0

```rust
fn register_table(&self, name: String, table: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
```

Source: `src/schema.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

If supported by the implementation, adds a new table named `name` to
this schema.

If a table of the same name was already registered, returns "Table
already exists" error.

<a id="op-2f6985f66089b074a19145db"></a>
## table

`function` · `datafusion_session::schema::SchemaProvider::table` · datafusion-session 55.1.0

```rust
async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError>
```

Source: `src/schema.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves a specific table from the schema by name, if it exists,
otherwise returns `None`.

<a id="op-25890347bbea065a46ae243e"></a>
## table_exist

`function` · `datafusion_session::schema::SchemaProvider::table_exist` · datafusion-session 55.1.0

```rust
fn table_exist(&self, name: &str) -> bool
```

Source: `src/schema.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Returns true if table exist in the schema provider, false otherwise.

<a id="op-6298b7c113bfe95c8780b1d0"></a>
## table_names

`function` · `datafusion_session::schema::SchemaProvider::table_names` · datafusion-session 55.1.0

```rust
fn table_names(&self) -> Vec<String>
```

Source: `src/schema.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves the list of available table names in this schema.

<a id="op-adc44d082aac9107e413f77b"></a>
## table_type

`function` · `datafusion_session::schema::SchemaProvider::table_type` · datafusion-session 55.1.0

```rust
async fn table_type(&self, name: &str) -> Result<Option<TableType>>
```

Source: `src/schema.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Retrieves the type of a specific table from the schema by name, if it exists, otherwise
returns `None`.  Implementations for which this operation is cheap but [Self::table](../operations/datafusion_session.schema.SchemaProvider.md#op-2f6985f66089b074a19145db) is
expensive can override this to improve operations that only need the type, e.g.
`SELECT * FROM information_schema.tables`.
