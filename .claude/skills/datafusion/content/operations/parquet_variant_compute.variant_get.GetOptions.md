# `parquet_variant_compute::variant_get::GetOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_get.GetOptions.json).

<a id="op-7a3f8907301ee8c7b830fa86"></a>
## GetOptions

`struct` · `parquet_variant_compute::variant_get::GetOptions` · parquet-variant-compute 59.3.0

```rust
struct GetOptions<'a>
```

Source: `src/variant_get.rs:445`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Controls the action of the variant_get kernel.

<a id="op-2d2e7c940bc1550bad335ffc"></a>
## as_type

`struct_field` · `parquet_variant_compute::variant_get::GetOptions::as_type` · parquet-variant-compute 59.3.0

```rust
as_type: Option<arrow_schema::FieldRef>
```

Source: `src/variant_get.rs:451`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

if `as_type` is None, the returned array will itself be a VariantArray.

if `as_type` is `Some(type)` the field is returned as the specified type.

<a id="op-7cb779cf6b8fd128c5b2b8db"></a>
## cast_options

`struct_field` · `parquet_variant_compute::variant_get::GetOptions::cast_options` · parquet-variant-compute 59.3.0

```rust
cast_options: arrow::compute::CastOptions<'a>
```

Source: `src/variant_get.rs:453`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Controls the casting behavior (e.g. error vs substituting null on cast error).

<a id="op-5499a1d35ebf2aa5389a56e1"></a>
## clone

`function` · `parquet_variant_compute::variant_get::GetOptions::clone` · parquet-variant-compute 59.3.0

```rust
fn clone(&self) -> GetOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 17], "end": [444, 22], "filename": "src/variant_get.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/variant_get.rs:444`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bbab097c31b0790adea3b8a"></a>
## default

`function` · `parquet_variant_compute::variant_get::GetOptions::default` · parquet-variant-compute 59.3.0

```rust
fn default() -> GetOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 24], "end": [444, 31], "filename": "src/variant_get.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/variant_get.rs:444`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-678dc083eaf1aafd251e96c7"></a>
## fmt

`function` · `parquet_variant_compute::variant_get::GetOptions::fmt` · parquet-variant-compute 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 10], "end": [444, 15], "filename": "src/variant_get.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variant_get.rs:444`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2676e5a21733994353e50270"></a>
## new

`function` · `parquet_variant_compute::variant_get::GetOptions::new` · parquet-variant-compute 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [482, 2], "filename": "src/variant_get.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_get.rs:458`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Construct default options to get the specified path as a variant.

<a id="op-104aaae8ca9e9c5b988a00d1"></a>
## new_with_path

`function` · `parquet_variant_compute::variant_get::GetOptions::new_with_path` · parquet-variant-compute 59.3.0

```rust
fn new_with_path(path: VariantPath<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [482, 2], "filename": "src/variant_get.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_get.rs:463`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Construct options to get the specified path as a variant.

<a id="op-ce178735997a8e955d8e5a2d"></a>
## path

`struct_field` · `parquet_variant_compute::variant_get::GetOptions::path` · parquet-variant-compute 59.3.0

```rust
path: parquet_variant::VariantPath<'a>
```

Source: `src/variant_get.rs:447`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

What path to extract

<a id="op-dc0f8cda71ecb436754a05a4"></a>
## with_as_type

`function` · `parquet_variant_compute::variant_get::GetOptions::with_as_type` · parquet-variant-compute 59.3.0

```rust
fn with_as_type(self, as_type: Option<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [482, 2], "filename": "src/variant_get.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_get.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Specify the type to return.

<a id="op-641a8825feb92417e889372e"></a>
## with_cast_options

`function` · `parquet_variant_compute::variant_get::GetOptions::with_cast_options` · parquet-variant-compute 59.3.0

```rust
fn with_cast_options(self, cast_options: CastOptions<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant_compute::variant_get::GetOptions", "path": "GetOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 1], "end": [482, 2], "filename": "src/variant_get.rs"}, "trait": null, "trait_path": null}`

Source: `src/variant_get.rs:478`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Specify the cast options to use when casting to the specified type.
