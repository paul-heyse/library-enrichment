# `parquet::schema::types::ColumnDescriptor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.ColumnDescriptor.json).

<a id="op-31e0bc49c5df5529126a8abd"></a>
## ColumnDescriptor

`struct` · `parquet::schema::types::ColumnDescriptor` · parquet 59.3.0

```rust
struct ColumnDescriptor
```

Source: `src/schema/types.rs:845`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Physical type for leaf-level primitive columns.

Also includes the maximum definition and repetition levels required to
re-assemble nested data.

<a id="op-3ef54d65ca8fcd082499cc3f"></a>
## converted_type

`function` · `parquet::schema::types::ColumnDescriptor::converted_type` · parquet 59.3.0

```rust
fn converted_type(&self) -> ConvertedType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:937`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`ConvertedType`](../operations/parquet.basic.ConvertedType.md#op-89a0d32ad627c75d0804c1a4) for this column.

<a id="op-016eeb882df81aa922612eb5"></a>
## eq

`function` · `parquet::schema::types::ColumnDescriptor::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnDescriptor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [844, 17], "end": [844, 26], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema/types.rs:844`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb36363dfd46f5b8b8a0aa66"></a>
## fmt

`function` · `parquet::schema::types::ColumnDescriptor::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [844, 10], "end": [844, 15], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema/types.rs:844`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd3878afb013b0f5d50ebdb5"></a>
## logical_type

`function` · `parquet::schema::types::ColumnDescriptor::logical_type` · parquet 59.3.0

```rust
fn logical_type(&self) -> Option<LogicalType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) for this column.

Note that this function will clone the `LogicalType`. If performance is a concern,
use [`Self::logical_type_ref`](../operations/parquet.schema.types.ColumnDescriptor.md#op-0d751c732b1df0691f0673ac) instead.

<a id="op-0d751c732b1df0691f0673ac"></a>
## logical_type_ref

`function` · `parquet::schema::types::ColumnDescriptor::logical_type_ref` · parquet 59.3.0

```rust
fn logical_type_ref(&self) -> Option<&LogicalType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:957`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a reference to the [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) for this column.

<a id="op-e4b030e7bc9b9d2fbcbfd7df"></a>
## max_def_level

`function` · `parquet::schema::types::ColumnDescriptor::max_def_level` · parquet 59.3.0

```rust
fn max_def_level(&self) -> i16
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:899`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns maximum definition level for this column.

<a id="op-070cfbf63322c2ae763a721d"></a>
## max_rep_level

`function` · `parquet::schema::types::ColumnDescriptor::max_rep_level` · parquet 59.3.0

```rust
fn max_rep_level(&self) -> i16
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:905`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns maximum repetition level for this column.

<a id="op-82996b31ab5f970312bc8994"></a>
## name

`function` · `parquet::schema::types::ColumnDescriptor::name` · parquet 59.3.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:932`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns column name.

<a id="op-bec4b66e1dae63c170ec9e71"></a>
## new

`function` · `parquet::schema::types::ColumnDescriptor::new` · parquet 59.3.0

```rust
fn new(primitive_type: TypePtr, max_def_level: i16, max_rep_level: i16, path: ColumnPath) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:872`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new descriptor for leaf-level column.

<a id="op-3973d6a23a215c07b4f99af2"></a>
## path

`function` · `parquet::schema::types::ColumnDescriptor::path` · parquet 59.3.0

```rust
fn path(&self) -> &ColumnPath
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:916`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`ColumnPath`](../operations/parquet.schema.types.ColumnPath.md#op-af9bcd597f4e17b094dee0f4) for this column.

<a id="op-3b2c23dcb427d5438740b61b"></a>
## physical_type

`function` · `parquet::schema::types::ColumnDescriptor::physical_type` · parquet 59.3.0

```rust
fn physical_type(&self) -> PhysicalType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:963`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns physical type for this column.
Note that it will panic if called on a non-primitive type.

<a id="op-d5db1ec77c37ad736fa26161"></a>
## repeated_ancestor_def_level

`function` · `parquet::schema::types::ColumnDescriptor::repeated_ancestor_def_level` · parquet 59.3.0

```rust
fn repeated_ancestor_def_level(&self) -> i16
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:911`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the definition level at the nearest REPEATED ancestor, or 0 if none.

<a id="op-e250784ebaa38bbd4333824d"></a>
## self_type

`function` · `parquet::schema::types::ColumnDescriptor::self_type` · parquet 59.3.0

```rust
fn self_type(&self) -> &Type
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:921`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns self type [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc) for this leaf column.

<a id="op-e73ff58312d90d792ea1f249"></a>
## self_type_ptr

`function` · `parquet::schema::types::ColumnDescriptor::self_type_ptr` · parquet 59.3.0

```rust
fn self_type_ptr(&self) -> TypePtr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:927`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns self type [`TypePtr`](../operations/parquet.schema.types.TypePtr.md#op-1685bf8b2a3ab0d4f7c0b98c)  for this leaf
column.

<a id="op-ed1766f469367fec28cfabb8"></a>
## sort_order

`function` · `parquet::schema::types::ColumnDescriptor::sort_order` · parquet 59.3.0

```rust
fn sort_order(&self) -> SortOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:998`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the sort order for this column

<a id="op-aae4a4bcd6c9c9d752b7dec0"></a>
## type_length

`function` · `parquet::schema::types::ColumnDescriptor::type_length` · parquet 59.3.0

```rust
fn type_length(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:972`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns type length for this column.
Note that it will panic if called on a non-primitive type.

<a id="op-834a68e7ae9faffc0ab48da6"></a>
## type_precision

`function` · `parquet::schema::types::ColumnDescriptor::type_precision` · parquet 59.3.0

```rust
fn type_precision(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:981`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns type precision for this column.
Note that it will panic if called on a non-primitive type.

<a id="op-8a9ad2bfd5861fc6339baf31"></a>
## type_scale

`function` · `parquet::schema::types::ColumnDescriptor::type_scale` · parquet 59.3.0

```rust
fn type_scale(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnDescriptor", "path": "ColumnDescriptor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [870, 1], "end": [1005, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:990`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns type scale for this column.
Note that it will panic if called on a non-primitive type.
