# `datafusion::dataframe::DataFrameWriteOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.dataframe.DataFrameWriteOptions.json).

<a id="op-f7e216e68500822f7ef2d22d"></a>
## DataFrameWriteOptions

`struct` · `datafusion::dataframe::DataFrameWriteOptions` · datafusion 55.1.0

```rust
struct DataFrameWriteOptions
```

Source: `src/dataframe/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Contains options that control how data is
written out from a DataFrame

<a id="op-55bf84fc336d90cb919270bd"></a>
## default

`function` · `datafusion::dataframe::DataFrameWriteOptions::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [148, 2], "filename": "src/dataframe/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dataframe/mod.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5c90a75579b925b69cee818"></a>
## new

`function` · `datafusion::dataframe::DataFrameWriteOptions::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [142, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a new DataFrameWriteOptions with default values

<a id="op-f3a18e46924e83a665b96b89"></a>
## with_insert_operation

`function` · `datafusion::dataframe::DataFrameWriteOptions::with_insert_operation` · datafusion 55.1.0

```rust
fn with_insert_operation(self, insert_op: InsertOp) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [142, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the insert operation

<a id="op-584dd718edcd54e63cdee020"></a>
## with_partition_by

`function` · `datafusion::dataframe::DataFrameWriteOptions::with_partition_by` · datafusion 55.1.0

```rust
fn with_partition_by(self, partition_by: Vec<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [142, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Sets the partition_by columns for output partitioning

<a id="op-980bab49ce7c361d175cd340"></a>
## with_single_file_output

`function` · `datafusion::dataframe::DataFrameWriteOptions::with_single_file_output` · datafusion 55.1.0

```rust
fn with_single_file_output(self, single_file_output: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [142, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the single_file_output value to true or false

- `true`: Force single file output at the exact path specified
- `false`: Force directory output with generated filenames

When not called, automatic mode is used (extension-based heuristic).
When set to true, an output file will always be created even if the DataFrame is empty.

<a id="op-d65b3c142cc8b3cfd1e7c706"></a>
## with_sort_by

`function` · `datafusion::dataframe::DataFrameWriteOptions::with_sort_by` · datafusion 55.1.0

```rust
fn with_sort_by(self, sort_by: Vec<SortExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrameWriteOptions", "path": "DataFrameWriteOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [142, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Sets the sort_by columns for output sorting
