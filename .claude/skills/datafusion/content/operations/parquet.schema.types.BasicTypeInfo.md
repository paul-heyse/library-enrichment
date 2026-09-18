# `parquet::schema::types::BasicTypeInfo`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.BasicTypeInfo.json).

<a id="op-ece702306c864d1632f29019"></a>
## BasicTypeInfo

`struct` · `parquet::schema::types::BasicTypeInfo` · parquet 59.3.0

```rust
struct BasicTypeInfo
```

Source: `src/schema/types.rs:669`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Basic type info. This contains information such as the name of the type,
the repetition level, the logical type and the kind of the type (group, primitive).

<a id="op-9e71700abd6927de75bcf953"></a>
## clone

`function` · `parquet::schema::types::BasicTypeInfo::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BasicTypeInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [668, 10], "end": [668, 15], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema/types.rs:668`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f5755c5f8ae8971e9ac422f"></a>
## converted_type

`function` · `parquet::schema::types::BasicTypeInfo::converted_type` · parquet 59.3.0

```rust
fn converted_type(&self) -> ConvertedType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:704`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`ConvertedType`](../operations/parquet.basic.ConvertedType.md#op-89a0d32ad627c75d0804c1a4) value for the type.

<a id="op-121bacbcc030b58ef877584d"></a>
## eq

`function` · `parquet::schema::types::BasicTypeInfo::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BasicTypeInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [668, 24], "end": [668, 33], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema/types.rs:668`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bea87a220600b01468d6266"></a>
## fmt

`function` · `parquet::schema::types::BasicTypeInfo::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [668, 17], "end": [668, 22], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema/types.rs:668`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bec7f1f13937aba4f71c645e"></a>
## has_id

`function` · `parquet::schema::types::BasicTypeInfo::has_id` · parquet 59.3.0

```rust
fn has_id(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:727`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if id is set, `false` otherwise.

<a id="op-165094cba76b54bd1f7ead45"></a>
## has_repetition

`function` · `parquet::schema::types::BasicTypeInfo::has_repetition` · parquet 59.3.0

```rust
fn has_repetition(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:693`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns `true` if type has repetition field set, `false` otherwise.
This is mostly applied to group type, because primitive type always has
repetition set.

<a id="op-57eca5f90bf4251f00cd1770"></a>
## id

`function` · `parquet::schema::types::BasicTypeInfo::id` · parquet 59.3.0

```rust
fn id(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:732`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns id value for the type.

<a id="op-f05272b921835996d5a4422e"></a>
## logical_type

`function` · `parquet::schema::types::BasicTypeInfo::logical_type` · parquet 59.3.0

```rust
fn logical_type(&self) -> Option<LogicalType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:716`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) value for the type.

Note that this function will clone the `LogicalType`. If performance is a concern,
use [`Self::logical_type_ref`](../operations/parquet.schema.types.BasicTypeInfo.md#op-6b4c735799a43ce61d0747fa) instead.

<a id="op-6b4c735799a43ce61d0747fa"></a>
## logical_type_ref

`function` · `parquet::schema::types::BasicTypeInfo::logical_type_ref` · parquet 59.3.0

```rust
fn logical_type_ref(&self) -> Option<&LogicalType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:722`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return a reference to the [`LogicalType`](../operations/parquet.basic.LogicalType.md#op-20280a63f013d6b86b4da0d5) value for the type.

<a id="op-40697526b20540e2ab7449d5"></a>
## name

`function` · `parquet::schema::types::BasicTypeInfo::name` · parquet 59.3.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:686`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns field name.

<a id="op-c29ebedeeac96f1bbe20c1b1"></a>
## repetition

`function` · `parquet::schema::types::BasicTypeInfo::repetition` · parquet 59.3.0

```rust
fn repetition(&self) -> Repetition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::BasicTypeInfo", "path": "BasicTypeInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [736, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:698`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns [`Repetition`](../operations/parquet.basic.Repetition.md#op-4fa8af3a1b1426b5b33250d1) value for the type.
