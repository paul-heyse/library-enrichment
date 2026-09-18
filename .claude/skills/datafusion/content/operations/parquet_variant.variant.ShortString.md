# `parquet_variant::variant::ShortString`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.ShortString.json).

<a id="op-dd5857ffca98b4a0186b7d1c"></a>
## ShortString

`struct` · `parquet_variant::variant::ShortString` · parquet-variant 59.3.0

```rust
struct ShortString<'a>
```

Source: `src/variant.rs:57`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A Variant [`ShortString`](../operations/parquet_variant.variant.ShortString.md#op-dd5857ffca98b4a0186b7d1c)

This implementation is a zero cost wrapper over `&str` that ensures
the length of the underlying string is a valid Variant short string (63 bytes or less)

<a id="op-53ad3ad1e942325a88a987a6"></a>
## Error

`assoc_type` · `parquet_variant::variant::ShortString::Error` · parquet-variant 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [94, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:89`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39038375e7b01e91925c7a9c"></a>
## Target

`assoc_type` · `parquet_variant::variant::ShortString::Target` · parquet-variant 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [108, 2], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/variant.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c585f05fc0d05539b77795ab"></a>
## as_ref

`function` · `parquet_variant::variant::ShortString::as_ref` · parquet-variant 59.3.0

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [100, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/variant.rs:97`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91662c9a97ced0b16e66f5eb"></a>
## as_str

`function` · `parquet_variant::variant::ShortString::as_str` · parquet-variant 59.3.0

```rust
fn as_str(&self) -> &'a str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [80, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:77`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the underlying Variant short string as a &str

<a id="op-30b1b4ef6b2efbfe77444bb0"></a>
## clone

`function` · `parquet_variant::variant::ShortString::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> ShortString<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 22], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8a9bf5cca7c0451abcc964a"></a>
## deref

`function` · `parquet_variant::variant::ShortString::deref` · parquet-variant 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [108, 2], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/variant.rs:105`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7df451239fb30a3b8f45b81f"></a>
## eq

`function` · `parquet_variant::variant::ShortString::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &ShortString<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 30], "end": [56, 39], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abb6dee8a7198ddc3c1f5976"></a>
## fmt

`function` · `parquet_variant::variant::ShortString::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/variant.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eca1878ebe804871ac67052"></a>
## try_from

`function` · `parquet_variant::variant::ShortString::try_from` · parquet-variant 59.3.0

```rust
fn try_from(value: &'a str) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [94, 2], "filename": "src/variant.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/variant.rs:91`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380b5580d51ac6771bc00d05"></a>
## try_new

`function` · `parquet_variant::variant::ShortString::try_new` · parquet-variant 59.3.0

```rust
fn try_new(value: &'a str) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::variant::ShortString", "path": "ShortString"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [80, 2], "filename": "src/variant.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpret `value` as a variant short string value.

# Errors

Returns an error if  `value` is longer than the maximum allowed length
of a Variant short string (63 bytes).
