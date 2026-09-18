# `datafusion_datasource::projection::PartitionColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.projection.PartitionColumnIndex.json).

<a id="op-d7bc3b349f540c3d53a76569"></a>
## PartitionColumnIndex

`struct` · `datafusion_datasource::projection::PartitionColumnIndex` · datafusion-datasource 55.1.0

```rust
struct PartitionColumnIndex
```

Source: `src/projection.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e0b866e9fc8cdaeff4fc4a7"></a>
## clone

`function` · `datafusion_datasource::projection::PartitionColumnIndex::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> PartitionColumnIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::PartitionColumnIndex", "path": "PartitionColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 17], "end": [107, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33fa5aeeea91c87741faef85"></a>
## fmt

`function` · `datafusion_datasource::projection::PartitionColumnIndex::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::projection::PartitionColumnIndex", "path": "PartitionColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c0d6534cf02b59515a20930"></a>
## in_partition_values

`struct_field` · `datafusion_datasource::projection::PartitionColumnIndex::in_partition_values` · datafusion-datasource 55.1.0

```rust
in_partition_values: usize
```

Source: `src/projection.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The index of this partition column in the partition_values array

<a id="op-8a9b05a2d7c7bdb11020d1f9"></a>
## in_remainder_projection

`struct_field` · `datafusion_datasource::projection::PartitionColumnIndex::in_remainder_projection` · datafusion-datasource 55.1.0

```rust
in_remainder_projection: usize
```

Source: `src/projection.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The index of this partition column in the remainder projection (>= num_file_columns)
