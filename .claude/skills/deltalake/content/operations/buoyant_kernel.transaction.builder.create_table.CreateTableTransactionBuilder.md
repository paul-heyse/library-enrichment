# `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.builder.create_table.CreateTableTransactionBuilder.json).

<a id="op-05656d6a5734e8cc8804084f"></a>
## CreateTableTransactionBuilder

`struct` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CreateTableTransactionBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L743).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:743`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builder for configuring a new Delta table.

Use this to configure table properties before building a [`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf).
If the table build fails, no transaction will be created.

Created via [`create_table()`](super::super::create_table::create_table).

<a id="op-9485533f10797e0d6ca1999e"></a>
## build

`function` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, engine: &dyn Engine, committer: Box<dyn Committer>) -> DeltaResult<CreateTableTransaction>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L893).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder", "path": "CreateTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:893`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds a [`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf) that can be committed to create the table.

The returned [`CreateTableTransaction`](../operations/buoyant_kernel.transaction.create_table.CreateTableTransaction.md#op-32c02d6f39623e6417b55daf) only exposes operations that are valid for
table creation. Operations like removing files, removing domain metadata, or updating
deletion vectors are not available, preventing misuse at compile time.

This method performs validation:
- Checks that the table path is valid
- Verifies the table doesn't already exist
- Rejects schemas with `delta.invariants` metadata annotations (unsupported by kernel)
- Validates the data layout is valid
- Validates table properties against the allow list

Non-null columns (`nullable: false`) are allowed. The `invariants` writer feature is
auto-added to the protocol when the schema has any non-null column.

Empty schemas are accepted. The resulting table cannot be read or blind-appended to
until columns are added via `ALTER TABLE ADD COLUMN`.

# Arguments

* `engine` - The engine instance to use for validation
* `committer` - The committer to use for the transaction

# Errors

Returns an error if:
- The table path is invalid
- A table already exists at the given path
- The schema has `delta.invariants` metadata on any column
- The data layout is invalid
- Unsupported delta properties or feature flags are specified

<a id="op-355c09a478446bbf8ab0ea77"></a>
## new

`function` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(path: impl AsRef<str>, schema: SchemaRef, engine_info: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L757).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder", "path": "CreateTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:757`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new CreateTableTransactionBuilder.

This is typically called via
[`create_table()`](super::super::create_table::create_table) rather than directly.

<a id="op-076dac616c2ade69438392ac"></a>
## with_correlation_id

`function` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::with_correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L856).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder", "path": "CreateTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:856`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attach an opaque, caller-supplied correlation id for joining the create-table commit's
metric events to the caller's own request or operation id. An empty id is treated as unset.

<a id="op-36f8e6e15941d76e4c073e84"></a>
## with_data_layout

`function` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::with_data_layout` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_data_layout(self, layout: DataLayout) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L849).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder", "path": "CreateTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:849`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Sets the data layout for the new Delta table.

The data layout determines how data files are organized within the table:

- [`DataLayout::None`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-3783ef5f92c5836175ff5aa9): No special organization (default)
- [`DataLayout::Clustered`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-27e884dc8915498098e4969a): Data files are optimized for queries on clustering columns
- [`DataLayout::Partitioned`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-cc99590ac79729c808febb25): Data files are organized into directories by partition column
  values

Partitioning and clustering are mutually exclusive.

Calling this method multiple times replaces the previous layout. Only the last
`with_data_layout()` call takes effect.

# Example

```rust,no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transaction::create_table::create_table;
# use delta_kernel::transaction::data_layout::DataLayout;
# use delta_kernel::schema::{StructType, DataType, StructField};
# use std::sync::Arc;
# fn example() -> delta_kernel::DeltaResult<()> {
# let schema = Arc::new(StructType::try_new(vec![
#     StructField::new("id", DataType::INTEGER, true),
#     StructField::new("date", DataType::STRING, true),
# ])?);
// Clustered layout:
let builder = create_table("/path/to/table", schema.clone(), "MyApp/1.0")
    .with_data_layout(DataLayout::clustered(["id"]));

// Partitioned layout:
let builder = create_table("/path/to/table", schema, "MyApp/1.0")
    .with_data_layout(DataLayout::partitioned(["date"]));
# Ok(())
# }
```

<a id="op-1451d5fa141e2cc1e60904ae"></a>
## with_table_properties

`function` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::with_table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_table_properties<I, K, V>(self, properties: I) -> Self where I: IntoIterator<Item = (K, V)>, K: Into<String>, V: Into<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L801).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder", "path": "CreateTableTransactionBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [752, 1], "end": [986, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:801`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Sets table properties for the new Delta table.

Custom application properties (those not starting with `delta.`) are always allowed.
Delta properties (`delta.*`) are validated against an allow list during [`build()`].
Feature flags (`delta.feature.*=supported`) are supported for the subset of features
listed in `ALLOWED_DELTA_FEATURES`.

This method can be called multiple times. If a property key already exists from a
previous call, the new value will overwrite the old one.

# Arguments

* `properties` - A map of table property names to their values

# Example

```rust,no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transaction::create_table::create_table;
# use delta_kernel::schema::{StructType, DataType, StructField};
# use std::sync::Arc;
# fn example() -> delta_kernel::DeltaResult<()> {
# let schema = Arc::new(StructType::try_new(vec![StructField::new("id", DataType::INTEGER, true)])?);
let builder = create_table("/path/to/table", schema, "MyApp/1.0")
    .with_table_properties([
        ("myapp.version", "1.0"),
        ("myapp.author", "test"),
    ]);
# Ok(())
# }
```

[`build()`]: CreateTableTransactionBuilder::build

<a id="op-c396c140a02ae93fade7ec70"></a>
## correlation_id

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::correlation_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
correlation_id: Option<std::sync::Arc<str>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L749).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:749`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc13a4b6652c02a25cbecb5d"></a>
## data_layout

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::data_layout` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data_layout: transaction::data_layout::DataLayout
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L748).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:748`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa867c3414dc94a1e94de7a8"></a>
## engine_info

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::engine_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
engine_info: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L746).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:746`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b6cbf5f2fab80ad1351f6c6"></a>
## path

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L744).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:744`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3399077f5661104ebcfc6a3b"></a>
## schema

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
schema: schema::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L745).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:745`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b997933edb1a776d1f8e76a1"></a>
## table_properties

`struct_field` · `buoyant_kernel::transaction::builder::create_table::CreateTableTransactionBuilder::table_properties` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/builder/create_table.rs#L747).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/builder/create_table.rs:747`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
