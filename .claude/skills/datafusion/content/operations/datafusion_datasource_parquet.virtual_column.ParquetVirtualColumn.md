# `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.virtual_column.ParquetVirtualColumn.json).

<a id="op-63719987fb2500e17afd517f"></a>
## ParquetVirtualColumn

`enum` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn` · datafusion-datasource-parquet 55.1.0

```rust
enum ParquetVirtualColumn
```

Source: `src/virtual_column.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

A parquet virtual column validated to have a supported arrow extension
type.

Construct via [`TryFrom<&FieldRef>`]; add a new variant (and update the
`TryFrom` impl) when DataFusion gains support for another arrow-rs virtual
extension type.

Unresolved upstream links (retained, not inferred): ``TryFrom<&FieldRef>``.

<a id="op-6dc2dcbb15e9d4f6bfdfcba8"></a>
## Error

`assoc_type` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::Error` · datafusion-datasource-parquet 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn", "path": "ParquetVirtualColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [84, 2], "filename": "src/virtual_column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/virtual_column.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4abc369f919260d40d5d96f6"></a>
## RowNumber

`variant` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::RowNumber` · datafusion-datasource-parquet 55.1.0

```rust
RowNumber
```

Source: `src/virtual_column.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Absolute row number within the parquet file. Backed by arrow-rs's
[`RowNumber`] extension type.

Unresolved upstream links (retained, not inferred): ``RowNumber``.

<a id="op-4289f38c017b41ac7dcff9a3"></a>
## clone

`function` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::clone` · datafusion-datasource-parquet 55.1.0

```rust
fn clone(&self) -> ParquetVirtualColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn", "path": "ParquetVirtualColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 22], "filename": "src/virtual_column.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/virtual_column.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-597d29138eefc6a516c36fec"></a>
## field

`function` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::field` · datafusion-datasource-parquet 55.1.0

```rust
fn field(&self) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn", "path": "ParquetVirtualColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/virtual_column.rs"}, "trait": null, "trait_path": null}`

Source: `src/virtual_column.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ab851e516c3e22454a867c0"></a>
## fmt

`function` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::fmt` · datafusion-datasource-parquet 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn", "path": "ParquetVirtualColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/virtual_column.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/virtual_column.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1db5b7111d2088af5e084232"></a>
## try_from

`function` · `datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn::try_from` · datafusion-datasource-parquet 55.1.0

```rust
fn try_from(field: &FieldRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_parquet::virtual_column::ParquetVirtualColumn", "path": "ParquetVirtualColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [84, 2], "filename": "src/virtual_column.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/virtual_column.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
