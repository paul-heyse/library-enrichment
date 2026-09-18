# `parquet_variant::path::VariantPathElement`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.path.VariantPathElement.json).

<a id="op-740ae15d16b7e2533737aec4"></a>
## VariantPathElement

`enum` · `parquet_variant::path::VariantPathElement` · parquet-variant 59.3.0

```rust
enum VariantPathElement<'a>
```

Source: `src/path.rs:166`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Element of a [`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) that can be a field name or an index.

See [`VariantPath`](../operations/parquet_variant.path.VariantPath.md#op-80e1a932cd6b1f8c63058dd5) for more details and examples.

<a id="op-e30b860f54fe4e7153ca8488"></a>
## Field

`variant` · `parquet_variant::path::VariantPathElement::Field` · parquet-variant 59.3.0

```rust
Field
```

Source: `src/path.rs:168`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Access field with name `name`

<a id="op-08267f795dc024d652521375"></a>
## Index

`variant` · `parquet_variant::path::VariantPathElement::Index` · parquet-variant 59.3.0

```rust
Index
```

Source: `src/path.rs:170`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Access the list element at `index`

<a id="op-fd67b84caa2660bd406d0748"></a>
## clone

`function` · `parquet_variant::path::VariantPathElement::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantPathElement<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 17], "end": [165, 22], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/path.rs:165`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e25f00dd871966ce71cfb6d"></a>
## eq

`function` · `parquet_variant::path::VariantPathElement::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantPathElement<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 24], "end": [165, 33], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/path.rs:165`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d27a358660bae330ba925002"></a>
## field

`function` · `parquet_variant::path::VariantPathElement::field` · parquet-variant 59.3.0

```rust
fn field(name: impl Into<Cow<'a, str>>) -> VariantPathElement<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [182, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:174`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-467cf687092ba87983a90b6c"></a>
## fmt

`function` · `parquet_variant::path::VariantPathElement::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [165, 10], "end": [165, 15], "filename": "src/path.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/path.rs:165`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10c192fd4e5264da88ef8584"></a>
## from

`function` · `parquet_variant::path::VariantPathElement::from` · parquet-variant 59.3.0

```rust
fn from(name: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [195, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:192`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33e96fd5ae38dd6d0599acca"></a>
## from

`function` · `parquet_variant::path::VariantPathElement::from` · parquet-variant 59.3.0

```rust
fn from(name: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [197, 1], "end": [201, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e4f0755466afd7222b3a249"></a>
## from

`function` · `parquet_variant::path::VariantPathElement::from` · parquet-variant 59.3.0

```rust
fn from(index: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [213, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcfb0da59d29d6cb1ed11973"></a>
## from

`function` · `parquet_variant::path::VariantPathElement::from` · parquet-variant 59.3.0

```rust
fn from(name: Cow<'a, str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [189, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"primitive": "str"}}], "constraints": []}}, "id": "alloc::borrow::Cow", "path": "Cow"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:186`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9867cc74a4018bc3dbd161d"></a>
## from

`function` · `parquet_variant::path::VariantPathElement::from` · parquet-variant 59.3.0

```rust
fn from(name: &'a String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 1], "end": [207, 2], "filename": "src/path.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/path.rs:204`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36fc91266d8aa04bedb7f46"></a>
## index

`function` · `parquet_variant::path::VariantPathElement::index` · parquet-variant 59.3.0

```rust
fn index(index: usize) -> VariantPathElement<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::path::VariantPathElement", "path": "VariantPathElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [182, 2], "filename": "src/path.rs"}, "trait": null, "trait_path": null}`

Source: `src/path.rs:179`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
