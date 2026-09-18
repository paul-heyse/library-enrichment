# `parquet_variant::variant::metadata::VariantMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.metadata.VariantMetadata.json).

<a id="op-6c1715f60a1297fbc36b12c8"></a>
## VariantMetadata

`struct` · `parquet_variant::variant::metadata::VariantMetadata` · parquet-variant 59.3.0

```rust
struct VariantMetadata<'m>
```

Source: `src/variant/metadata.rs:132`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

[`Variant`] Metadata

See the [Variant Spec] file for more information

# Validation

Every instance of variant metadata is either _valid_ or _invalid_. depending on whether the
underlying bytes are a valid encoding of variant metadata (see below).

Instances produced by [`Self::try_new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-e517e7c0f97dc18d829f1a14) or [`Self::with_full_validation`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-bc1bb9cc8c8161176cc3bed7) are fully _validated_. They always
contain _valid_ data, and infallible accesses such as iteration and indexing are panic-free. The
validation cost is linear in the number of underlying bytes.

Instances produced by [`Self::new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-0219420d34caf4c5d99e3432) are _unvalidated_ and so they may contain either _valid_ or
_invalid_ data. Infallible accesses such as iteration and indexing will panic if the underlying
bytes are _invalid_, and fallible alternatives such as [`Self::iter_try`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-417e8fa99f68dfdb0e9a2bf9) and [`Self::get`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-c68ef57cbf92e39e6b4b53a2) are
provided as panic-free alternatives. [`Self::with_full_validation`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-bc1bb9cc8c8161176cc3bed7) can also be used to _validate_ an
_unvalidated_ instance, if desired.

_Unvalidated_ instances can be constructed in constant time. This can be useful if the caller
knows the underlying bytes were already validated previously, or if the caller intends to
perform a small number of (fallible) accesses to a large dictionary.

A _validated_ variant [metadata instance guarantees that:

- header byte is valid
- dictionary size is in bounds
- offset array content is in-bounds
- first offset is zero
- last offset is in-bounds
- all other offsets are in-bounds (*)
- all offsets are monotonically increasing (*)
- all values are valid utf-8 (*)

NOTE: [`Self::new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-0219420d34caf4c5d99e3432) only skips expensive (non-constant cost) validation checks (marked by `(*)`
in the list above); it panics any of the other checks fails.

# Safety

Even an _invalid_ variant metadata instance is still _safe_ to use in the Rust sense. Accessing
it with infallible methods may cause panics but will never lead to undefined behavior.

[`Variant`]: crate::Variant
[Variant Spec]: https://github.com/apache/parquet-format/blob/master/VariantEncoding.md#metadata-encoding

<a id="op-7c11fbcfee780ba4be3dc828"></a>
## Output

`assoc_type` · `parquet_variant::variant::metadata::VariantMetadata::Output` · parquet-variant 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [421, 2], "filename": "src/variant/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/variant/metadata.rs:416`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b57128a643bac28a41ddd115"></a>
## clone

`function` · `parquet_variant::variant::metadata::VariantMetadata::clone` · parquet-variant 59.3.0

```rust
fn clone(&self) -> VariantMetadata<'m>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 17], "end": [131, 22], "filename": "src/variant/metadata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant/metadata.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-008bd2128f0e7430b388dd58"></a>
## eq

`function` · `parquet_variant::variant::metadata::VariantMetadata::eq` · parquet-variant 59.3.0

```rust
fn eq(&self, other: &VariantMetadata<'m>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 24], "end": [131, 33], "filename": "src/variant/metadata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variant/metadata.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00809431660491cad0abab21"></a>
## fmt

`function` · `parquet_variant::variant::metadata::VariantMetadata::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 10], "end": [131, 15], "filename": "src/variant/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant/metadata.rs:131`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c68ef57cbf92e39e6b4b53a2"></a>
## get

`function` · `parquet_variant::variant::metadata::VariantMetadata::get` · parquet-variant 59.3.0

```rust
fn get(&self, i: usize) -> Result<&'m str, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:362`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to retrieve a dictionary entry by index, failing if out of bounds or if the
underlying bytes are [invalid].

[invalid]: Self#Validation

<a id="op-3bb43fb930e18cc6a68dd000"></a>
## get_entry

`function` · `parquet_variant::variant::metadata::VariantMetadata::get_entry` · parquet-variant 59.3.0

```rust
fn get_entry(&self, field_name: &str) -> Option<(u32, &'m str)>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:381`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to retrieve a dictionary entry and its field id, returning None if the requested field
name is not present. The search cost is logarithmic if [`Self::is_sorted`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-60edcc738f60e02260496086) and linear
otherwise.

WARNING: This method panics if the underlying bytes are [invalid].

[invalid]: Self#Validation

<a id="op-14c65366055715cf40daa0ca"></a>
## index

`function` · `parquet_variant::variant::metadata::VariantMetadata::index` · parquet-variant 59.3.0

```rust
fn index(&self, i: usize) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [415, 1], "end": [421, 2], "filename": "src/variant/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "usize"}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/variant/metadata.rs:418`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad02b11f21c34c90c30e58f9"></a>
## is_empty

`function` · `parquet_variant::variant::metadata::VariantMetadata::is_empty` · parquet-variant 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:257`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if this metadata dictionary contains no entries

<a id="op-fd1345e0044d2e9631a5d626"></a>
## is_fully_validated

`function` · `parquet_variant::variant::metadata::VariantMetadata::is_fully_validated` · parquet-variant 59.3.0

```rust
fn is_fully_validated(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:264`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

True if this instance is fully [validated] for panic-free infallible accesses.

[validated]: Self#Validation

<a id="op-60edcc738f60e02260496086"></a>
## is_sorted

`function` · `parquet_variant::variant::metadata::VariantMetadata::is_sorted` · parquet-variant 59.3.0

```rust
fn is_sorted(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:330`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Whether the dictionary keys are sorted and unique

<a id="op-b0ff1195679aed15f2304b33"></a>
## iter

`function` · `parquet_variant::variant::metadata::VariantMetadata::iter` · parquet-variant 59.3.0

```rust
fn iter(&self) -> impl Iterator<Item = &'m str> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:405`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Iterates over all dictionary entries. When working with [unvalidated] input, consider
[`Self::iter_try`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-417e8fa99f68dfdb0e9a2bf9) to avoid panics due to invalid data.

[unvalidated]: Self#Validation

<a id="op-417e8fa99f68dfdb0e9a2bf9"></a>
## iter_try

`function` · `parquet_variant::variant::metadata::VariantMetadata::iter_try` · parquet-variant 59.3.0

```rust
fn iter_try(&self) -> impl Iterator<Item = Result<&'m str, ArrowError>> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:397`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns an iterator that attempts to visit all dictionary entries, producing `Err` if the
iterator encounters [invalid] data.

[invalid]: Self#Validation

<a id="op-3335a53586f05a6adda6b39f"></a>
## len

`function` · `parquet_variant::variant::metadata::VariantMetadata::len` · parquet-variant 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:252`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The number of metadata dictionary entries

<a id="op-0219420d34caf4c5d99e3432"></a>
## new

`function` · `parquet_variant::variant::metadata::VariantMetadata::new` · parquet-variant 59.3.0

```rust
fn new(bytes: &'m [u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:201`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Interprets `bytes` as a variant metadata instance, without attempting to [validate] dictionary
entries. Panics if basic sanity checking fails, and subsequent infallible accesses such as
indexing and iteration could also panic if the underlying bytes are invalid.

This constructor can be a useful lightweight alternative to [`Self::try_new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-e517e7c0f97dc18d829f1a14) if the bytes
were already validated previously by other means, or if the caller expects a small number of
accesses to a large dictionary (preferring to use a small number of fallible accesses as
needed, instead of paying expensive full validation up front).

[validate]: Self#Validation

<a id="op-10951106d6e2dee9a8e24541"></a>
## size

`function` · `parquet_variant::variant::metadata::VariantMetadata::size` · parquet-variant 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:354`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Returns the total size, in bytes, of the metadata.

Note this value may be smaller than what was passed to [`Self::new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-0219420d34caf4c5d99e3432) or
[`Self::try_new`](../operations/parquet_variant.variant.metadata.VariantMetadata.md#op-e517e7c0f97dc18d829f1a14) if the input was larger than necessary to encode the
metadata dictionary.

<a id="op-e517e7c0f97dc18d829f1a14"></a>
## try_new

`function` · `parquet_variant::variant::metadata::VariantMetadata::try_new` · parquet-variant 59.3.0

```rust
fn try_new(bytes: &'m [u8]) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:187`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Attempts to interpret `bytes` as a variant metadata instance, with full [validation] of all
dictionary entries.

[validation]: Self#Validation

<a id="op-8e7284e15ea2039c591f1f11"></a>
## version

`function` · `parquet_variant::variant::metadata::VariantMetadata::version` · parquet-variant 59.3.0

```rust
const fn version(&self) -> u8
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:335`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The variant protocol version

<a id="op-bc1bb9cc8c8161176cc3bed7"></a>
## with_full_validation

`function` · `parquet_variant::variant::metadata::VariantMetadata::with_full_validation` · parquet-variant 59.3.0

```rust
fn with_full_validation(self) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'m"}], "constraints": []}}, "id": "parquet_variant::variant::metadata::VariantMetadata", "path": "VariantMetadata"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'m"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [409, 2], "filename": "src/variant/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant/metadata.rs:271`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Performs a full [validation] of this metadata dictionary and returns the result.

[validation]: Self#Validation
