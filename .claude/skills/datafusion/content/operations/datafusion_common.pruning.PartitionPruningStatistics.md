# `datafusion_common::pruning::PartitionPruningStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.pruning.PartitionPruningStatistics.json).

<a id="op-deb682514c0a2b7d8423a8cb"></a>
## PartitionPruningStatistics

`struct` · `datafusion_common::pruning::PartitionPruningStatistics` · datafusion-common 55.1.0

```rust
struct PartitionPruningStatistics
```

Source: `src/pruning.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Prune files based on their partition values.

This is used both at planning time and execution time to prune
files based on their partition values.
This feeds into [`CompositePruningStatistics`](../operations/datafusion_common.pruning.CompositePruningStatistics.md#op-a5d7eede713af44570a95609) to allow pruning
with filters that depend both on partition columns and data columns
(e.g. `WHERE partition_col = data_col`).

<a id="op-978c77ef9b5e0e63f989a34b"></a>
## clone

`function` · `datafusion_common::pruning::PartitionPruningStatistics::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> PartitionPruningStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 10], "end": [145, 15], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/pruning.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e11aec04c114e4b5ecfadad8"></a>
## contained

`function` · `datafusion_common::pruning::PartitionPruningStatistics::contained` · datafusion-common 55.1.0

```rust
fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15d6373f9cad41d84726c0f3"></a>
## max_values

`function` · `datafusion_common::pruning::PartitionPruningStatistics::max_values` · datafusion-common 55.1.0

```rust
fn max_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d20b2f5d6d2317f47732d166"></a>
## min_values

`function` · `datafusion_common::pruning::PartitionPruningStatistics::min_values` · datafusion-common 55.1.0

```rust
fn min_values(&self, column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b4e40fdab299d550ba37fdd"></a>
## null_counts

`function` · `datafusion_common::pruning::PartitionPruningStatistics::null_counts` · datafusion-common 55.1.0

```rust
fn null_counts(&self, _column: &Column) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8f550fa3fbd5e9f0e9000f7"></a>
## num_containers

`function` · `datafusion_common::pruning::PartitionPruningStatistics::num_containers` · datafusion-common 55.1.0

```rust
fn num_containers(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19d644ba7c470c575af8d42"></a>
## row_counts

`function` · `datafusion_common::pruning::PartitionPruningStatistics::row_counts` · datafusion-common 55.1.0

```rust
fn row_counts(&self) -> Option<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 1], "end": [300, 2], "filename": "src/pruning.rs"}, "trait": {"args": null, "id": "datafusion_common::pruning::PruningStatistics", "path": "PruningStatistics"}, "trait_path": "datafusion_common::pruning::PruningStatistics"}`

Source: `src/pruning.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-937e5eead28a391f0d85e5d6"></a>
## try_new

`function` · `datafusion_common::pruning::PartitionPruningStatistics::try_new` · datafusion-common 55.1.0

```rust
fn try_new(partition_values: Vec<Vec<ScalarValue>>, partition_fields: Vec<FieldRef>) -> Result<Self, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::pruning::PartitionPruningStatistics", "path": "PartitionPruningStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [241, 2], "filename": "src/pruning.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new instance of [`PartitionPruningStatistics`](../operations/datafusion_common.pruning.PartitionPruningStatistics.md#op-deb682514c0a2b7d8423a8cb).

Args:
* `partition_values`: A vector of vectors of [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b)s.
  The outer vector represents the containers while the inner
  vector represents the partition values for each column.
  Note that this is the **opposite** of the order of the
  partition columns in `PartitionPruningStatistics::partition_schema`.
* `partition_schema`: The schema of the partition columns.
  This must **not** be the schema of the entire file or table:
  instead it must only be the schema of the partition columns,
  in the same order as the values in `partition_values`.

# Example

To create [`PartitionPruningStatistics`](../operations/datafusion_common.pruning.PartitionPruningStatistics.md#op-deb682514c0a2b7d8423a8cb) for two partition columns `a` and `b`,
for three containers like this:

| a | b |
| - | - |
| 1 | 2 |
| 3 | 4 |
| 5 | 6 |

```
# use std::sync::Arc;
# use datafusion_common::ScalarValue;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::pruning::PartitionPruningStatistics;

let partition_values = vec![
    vec![ScalarValue::from(1i32), ScalarValue::from(2i32)],
    vec![ScalarValue::from(3i32), ScalarValue::from(4i32)],
    vec![ScalarValue::from(5i32), ScalarValue::from(6i32)],
];
let partition_fields = vec![
    Arc::new(Field::new("a", DataType::Int32, false)),
    Arc::new(Field::new("b", DataType::Int32, false)),
];
let partition_stats =
    PartitionPruningStatistics::try_new(partition_values, partition_fields).unwrap();
```
