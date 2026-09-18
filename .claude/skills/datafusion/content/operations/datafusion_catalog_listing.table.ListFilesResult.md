# `datafusion_catalog_listing::table::ListFilesResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.table.ListFilesResult.json).

<a id="op-f4c5b358c91a8f87e5938723"></a>
## ListFilesResult

`struct` · `datafusion_catalog_listing::table::ListFilesResult` · datafusion-catalog-listing 55.1.0

```rust
struct ListFilesResult
```

Source: `src/table.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Result of a file listing operation from [`ListingTable::list_files_for_scan`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-ad3eafe20add1ea7c9515eda).

<a id="op-294bf9dab51641f1610507f6"></a>
## file_groups

`struct_field` · `datafusion_catalog_listing::table::ListFilesResult::file_groups` · datafusion-catalog-listing 55.1.0

```rust
file_groups: Vec<datafusion_datasource::file_groups::FileGroup>
```

Source: `src/table.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

File groups organized by the partitioning strategy.

<a id="op-68e015afa436f7698dfe10d0"></a>
## fmt

`function` · `datafusion_catalog_listing::table::ListFilesResult::fmt` · datafusion-catalog-listing 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::table::ListFilesResult", "path": "ListFilesResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21a9023f6af52cdc090d324d"></a>
## grouped_by_partition

`struct_field` · `datafusion_catalog_listing::table::ListFilesResult::grouped_by_partition` · datafusion-catalog-listing 55.1.0

```rust
grouped_by_partition: bool
```

Source: `src/table.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Whether files are grouped by partition values.

<a id="op-fe9d90d2122a473111b6df64"></a>
## statistics

`struct_field` · `datafusion_catalog_listing::table::ListFilesResult::statistics` · datafusion-catalog-listing 55.1.0

```rust
statistics: datafusion_common::Statistics
```

Source: `src/table.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Aggregated statistics for all files.
