# `buoyant_kernel::transaction::data_layout::DataLayout`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.data_layout.DataLayout.json).

<a id="op-561bccca31f14f965fa3442d"></a>
## DataLayout

`enum` · `buoyant_kernel::transaction::data_layout::DataLayout` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DataLayout
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L27).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data layout configuration for a Delta table.

Determines how data files are organized within the table:

- [`DataLayout::None`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-3783ef5f92c5836175ff5aa9): No special organization (default)
- [`DataLayout::Clustered`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-27e884dc8915498098e4969a): Data files optimized for queries on clustering columns
- [`DataLayout::Partitioned`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-cc99590ac79729c808febb25): Data files organized into directories by partition column values

Partitioning and clustering are mutually exclusive -- only one variant can be active at a time.

<a id="op-27e884dc8915498098e4969a"></a>
## Clustered

`variant` · `buoyant_kernel::transaction::data_layout::DataLayout::Clustered` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Clustered
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L35).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:35`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data files optimized for queries on clustering columns.
Both top-level and nested columns are supported. Each column's leaf field must
have a stats-eligible primitive type.

<a id="op-3783ef5f92c5836175ff5aa9"></a>
## None

`variant` · `buoyant_kernel::transaction::data_layout::DataLayout::None` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
None
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No special data organization (default).

<a id="op-cc99590ac79729c808febb25"></a>
## Partitioned

`variant` · `buoyant_kernel::transaction::data_layout::DataLayout::Partitioned` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Partitioned
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data files organized into directories by partition column values.
Only top-level columns are supported. Partition column values are stored
in the directory path rather than in the data files themselves.

<a id="op-9f3ac66146891da3ea890620"></a>
## clone

`function` · `buoyant_kernel::transaction::data_layout::DataLayout::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DataLayout
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::data_layout::DataLayout", "path": "DataLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-218a351ff2aa3b4fb43b372f"></a>
## clustered

`function` · `buoyant_kernel::transaction::data_layout::DataLayout::clustered` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clustered<I, S>(columns: I) -> Self where I: IntoIterator<Item = S>, S: AsRef<str>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::data_layout::DataLayout", "path": "DataLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [126, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a clustered layout with the given top-level column names.

Each string is treated as a single top-level column name. For nested columns,
construct the [`DataLayout::Clustered`](../operations/buoyant_kernel.transaction.data_layout.DataLayout.md#op-27e884dc8915498098e4969a) variant directly with multi-segment
[`ColumnName`](../operations/buoyant_kernel.expressions.column_names.ColumnName.md#op-9a9657c136296c6d9c579ddd) values.

This method constructs the layout without validation. Full validation
(duplicates, schema compatibility, data types) is performed during
`CreateTableTransactionBuilder::build()` via `validate_clustering_columns()`.

# Examples

Top-level columns:

```ignore
let layout = DataLayout::clustered(["id", "timestamp"]);
```

Nested columns (construct the variant directly):

```ignore
let layout = DataLayout::Clustered {
    columns: vec![ColumnName::new(["user", "address", "city"])],
};
```

<a id="op-cfbba069e65d490d87f40985"></a>
## default

`function` · `buoyant_kernel::transaction::data_layout::DataLayout::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DataLayout
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::data_layout::DataLayout", "path": "DataLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 24], "end": [26, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9213742ca3518dd17997b690"></a>
## fmt

`function` · `buoyant_kernel::transaction::data_layout::DataLayout::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::data_layout::DataLayout", "path": "DataLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ca3dfbaff36534817094adb"></a>
## partitioned

`function` · `buoyant_kernel::transaction::data_layout::DataLayout::partitioned` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partitioned<I, S>(columns: I) -> Self where I: IntoIterator<Item = S>, S: AsRef<str>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/data_layout.rs#L102).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::data_layout::DataLayout", "path": "DataLayout"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [126, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/data_layout.rs:102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a partitioned layout with the given top-level column names.

Each string is treated as a single top-level column name. Partition columns
must be top-level columns in the schema (nested columns are not supported).

This method constructs the layout without validation. Full validation
(duplicates, schema compatibility, data types) is performed during
`CreateTableTransactionBuilder::build()` via `validate_partition_columns()`.

# Example

```ignore
let layout = DataLayout::partitioned(["year", "month"]);
```
