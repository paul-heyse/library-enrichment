# `datafusion_datasource::file_groups::FileGroupPartitioner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_groups.FileGroupPartitioner.json).

<a id="op-7b24c3a42bf8f20212e4a135"></a>
## FileGroupPartitioner

`struct` · `datafusion_datasource::file_groups::FileGroupPartitioner` · datafusion-datasource 55.1.0

```rust
struct FileGroupPartitioner
```

Source: `src/file_groups.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Repartition input files into `target_partitions` partitions, if total file size exceed
`repartition_file_min_size`

This partitions evenly by file byte range, and does not have any knowledge
of how data is laid out in specific files. The specific `FileOpener` are
responsible for the actual partitioning on specific data source type. (e.g.
the `CsvOpener` will read lines overlap with byte range as well as
handle boundaries to ensure all lines will be read exactly once)

# Example

For example, if there are two files `A` and `B` that we wish to read with 4
partitions (with 4 threads) they will be divided as follows:

```text
                                   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
│  File A (7MB)   │   ────────▶     ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
└─────────────────┘                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│  File B (1MB)   │                  ┌─────────────────┐
│                 │                │ │     File A      │ │
└─────────────────┘                  │  Range: 6-7MB   │
                                   │ └─────────────────┘ │
                                     ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

# Maintaining Order

Within each group files are read sequentially. Thus, if the overall order of
tuples must be preserved, multiple files can not be mixed in the same group.

In this case, the code will split the largest files evenly into any
available empty groups, but the overall distribution may not be as even
as if the order did not need to be preserved.

```text
                                  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
┌─────────────────┐                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│  File A (6MB)   │   ────────▶      │                 │
│    (ordered)    │                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
└─────────────────┘                  │                 │
┌─────────────────┐                │ └─────────────────┘ │
│  File B (1MB)   │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│    (ordered)    │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
└─────────────────┘                  ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

<a id="op-226313486e44d3c98232c689"></a>
## clone

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> FileGroupPartitioner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 17], "end": [130, 22], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file_groups.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b2d98b80a46e0f976acf314"></a>
## default

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::default` · datafusion-datasource 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [144, 2], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file_groups.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cd72a6bf4fbebcbd9c96220"></a>
## fmt

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 10], "end": [130, 15], "filename": "src/file_groups.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file_groups.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-770fe0b5244570dd6cbc75ec"></a>
## new

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::new` · datafusion-datasource 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [363, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new [`FileGroupPartitioner`](../operations/datafusion_datasource.file_groups.FileGroupPartitioner.md#op-7b24c3a42bf8f20212e4a135) with default values:
1. `target_partitions = 1`
2. `repartition_file_min_size = 10MB`
3. `preserve_order_within_groups = false`

<a id="op-2a6e269934d0213262fd0e89"></a>
## repartition_file_groups

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::repartition_file_groups` · datafusion-datasource 55.1.0

```rust
fn repartition_file_groups(&self, file_groups: &[FileGroup]) -> Option<Vec<FileGroup>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [363, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Repartition input files according to the settings on this [`FileGroupPartitioner`](../operations/datafusion_datasource.file_groups.FileGroupPartitioner.md#op-7b24c3a42bf8f20212e4a135).

If no repartitioning is needed or possible, return `None`.

<a id="op-3ee4746760b297fbe9832ada"></a>
## with_preserve_order_within_groups

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::with_preserve_order_within_groups` · datafusion-datasource 55.1.0

```rust
fn with_preserve_order_within_groups(self, preserve_order_within_groups: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [363, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set whether the order of tuples within a file must be preserved

<a id="op-851e2b1701bf577140545ca1"></a>
## with_repartition_file_min_size

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::with_repartition_file_min_size` · datafusion-datasource 55.1.0

```rust
fn with_repartition_file_min_size(self, repartition_file_min_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [363, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the minimum size at which to repartition a file

<a id="op-7147cb1dbedda6d011d293bc"></a>
## with_target_partitions

`function` · `datafusion_datasource::file_groups::FileGroupPartitioner::with_target_partitions` · datafusion-datasource 55.1.0

```rust
fn with_target_partitions(self, target_partitions: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_groups::FileGroupPartitioner", "path": "FileGroupPartitioner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [363, 2], "filename": "src/file_groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_groups.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the target partitions
