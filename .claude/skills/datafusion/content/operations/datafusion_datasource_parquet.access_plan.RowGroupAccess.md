# `datafusion_datasource_parquet::access_plan::RowGroupAccess`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.access_plan.RowGroupAccess.json).

<a id="op-78e2484ee2017bec500ea3e7"></a>
## RowGroupAccess

`enum` · `datafusion_datasource_parquet::access_plan::RowGroupAccess` · datafusion-datasource-parquet 55.1.0

```rust
enum RowGroupAccess
```

Source: `src/access_plan.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Describes how the parquet reader will access a row group

<a id="op-ac7b3116c91c00b620d74af7"></a>
## Scan

`variant` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::Scan` · datafusion-datasource-parquet 55.1.0

```rust
Scan
```

Source: `src/access_plan.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Read all rows from the row group

<a id="op-9dc55412f081ee8c5854d131"></a>
## Selection

`variant` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::Selection` · datafusion-datasource-parquet 55.1.0

```rust
Selection
```

Source: `src/access_plan.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Scan only the specified rows within the row group

<a id="op-43889f9fbbe5544a160eab6f"></a>
## Skip

`variant` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::Skip` · datafusion-datasource-parquet 55.1.0

```rust
Skip
```

Source: `src/access_plan.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Do not read the row group at all

<a id="op-7cbce95d9a053dc33a6c6f27"></a>
## clone

`function` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> RowGroupAccess
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::RowGroupAccess", "path": "RowGroupAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 17], "end": [143, 22], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/access_plan.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e61055545e04d6bf7604804c"></a>
## eq

`function` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::eq` · datafusion-datasource-parquet 55.1.0

```rust
fn eq(&self, other: &RowGroupAccess) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::RowGroupAccess", "path": "RowGroupAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 24], "end": [143, 33], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/access_plan.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3146e0c2ca8f713f711d0272"></a>
## fmt

`function` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::RowGroupAccess", "path": "RowGroupAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 10], "end": [143, 15], "filename": "src/access_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/access_plan.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aea188bf1d2d3549272c85c"></a>
## should_scan

`function` · `datafusion_datasource_parquet::access_plan::RowGroupAccess::should_scan` · datafusion-datasource-parquet 55.1.0

```rust
fn should_scan(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::access_plan::RowGroupAccess", "path": "RowGroupAccess"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [161, 2], "filename": "src/access_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/access_plan.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Return true if this row group should be scanned
