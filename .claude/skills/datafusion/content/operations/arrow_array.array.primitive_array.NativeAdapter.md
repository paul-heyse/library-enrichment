# `arrow_array::array::primitive_array::NativeAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.NativeAdapter.json).

<a id="op-d2cfd34119b25dd32861af13"></a>
## NativeAdapter

`struct` · `arrow_array::array::primitive_array::NativeAdapter` · arrow-array 59.3.0

```rust
struct NativeAdapter<T: ArrowPrimitiveType>
```

Source: `src/array/primitive_array.rs:1437`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An optional primitive value

This struct is used as an adapter when creating `PrimitiveArray` from an iterator.
`FromIterator` for `PrimitiveArray` takes an iterator where the elements can be `into`
this struct. So once implementing `From` or `Into` trait for a type, an iterator of
the type can be collected to `PrimitiveArray`.

<a id="op-e100962a7a31e44b499f366d"></a>
## fmt

`function` · `arrow_array::array::primitive_array::NativeAdapter::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [1436, 10], "end": [1436, 15], "filename": "src/array/primitive_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/primitive_array.rs:1436`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-121b1e4b8d6d29fc3fb3b463"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int16Type", "path": "Int16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1455, 1], "end": [1455, 40], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1455`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3333d4bac4af6e857910d2dc"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt8Type", "path": "UInt8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1458, 1], "end": [1458, 39], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1458`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-363901ad5a37b8bbe116f6ff"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int32Type", "path": "Int32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1456, 1], "end": [1456, 40], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1456`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-416b5703103fa799f2eecb92"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int8Type", "path": "Int8Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1454, 1], "end": [1454, 38], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1454`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ada3f588c1453513457461e"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt32Type", "path": "UInt32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1460, 1], "end": [1460, 41], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1460`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c37ed8151099c5203b38321"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: Option<<T as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1470, 1], "end": [1474, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1471`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f70ee57b61b706dd575c803"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal64Type", "path": "Decimal64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1466, 1], "end": [1466, 44], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1466`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-718df30fed4ef1ccaa5223a6"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: f16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float16Type", "path": "Float16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1462, 1], "end": [1462, 42], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "half::binary16::f16", "path": "f16"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1462`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72cea8027c296052fe54cb25"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i128) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal128Type", "path": "Decimal128Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1467, 1], "end": [1467, 46], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i128"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1467`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79d846bcadfda23bc2e5b1b0"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt64Type", "path": "UInt64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1461, 1], "end": [1461, 41], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1461`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-923088f9cab93129d4917c41"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float64Type", "path": "Float64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1464, 1], "end": [1464, 42], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1464`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae69c2ce9ddd4f3e00312635"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal32Type", "path": "Decimal32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1465, 1], "end": [1465, 44], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1465`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a412f9b19a727ba70d7e73"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i256) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Decimal256Type", "path": "Decimal256Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1468, 1], "end": [1468, 46], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::bigint::i256", "path": "i256"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1468`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b708212671ffb96d32600e91"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: &Option<<T as ArrowPrimitiveType>::Native>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1480, 2], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1477`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcda786db89cd2f3076b6585"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: f32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Float32Type", "path": "Float32Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1463, 1], "end": [1463, 42], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1463`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df2bd64a52bf27be937e08c6"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: u16) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::UInt16Type", "path": "UInt16Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 1], "end": [1459, 41], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1459`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1363ab675584350712de300"></a>
## from

`function` · `arrow_array::array::primitive_array::NativeAdapter::from` · arrow-array 59.3.0

```rust
fn from(value: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::Int64Type", "path": "Int64Type"}}}], "constraints": []}}, "id": "arrow_array::array::primitive_array::NativeAdapter", "path": "NativeAdapter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1457, 1], "end": [1457, 40], "filename": "src/array/primitive_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/primitive_array.rs:1457`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c741b50ca67569e9182119f2"></a>
## native

`struct_field` · `arrow_array::array::primitive_array::NativeAdapter::native` · arrow-array 59.3.0

```rust
native: Option<T::Native>
```

Source: `src/array/primitive_array.rs:1439`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Corresponding Rust native type if available
