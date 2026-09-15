# `parquet_variant_compute::variant_get`

Crate `parquet-variant-compute` · 2 public items · structured records in [`model/parquet_variant_compute.variant_get.json`](../model/parquet_variant_compute.variant_get.json)

## variant_get

`function` · `parquet_variant_compute::variant_get::variant_get`

Also reachable as `parquet::variant::variant_get`, `parquet_variant_compute::variant_get`

```rust
fn variant_get(input: &arrow::array::ArrayRef, options: GetOptions<'_>) -> arrow::error::Result<arrow::array::ArrayRef>
```

Returns an array with the specified path extracted from the variant values.

The return array type depends on the `as_type` field of the options parameter
1. `as_type: None`: a VariantArray is returned. The values in this new VariantArray will point
   to the specified path.
2. `as_type: Some(<specific field>)`: an array of the specified type is returned.

TODO: How would a caller request a struct or list type where the fields/elements can be any
variant? Caller can pass None as the requested type to fetch a specific path, but it would
quickly become annoying (and inefficient) to call `variant_get` for each leaf value in a struct or
list and then try to assemble the results.

---

## GetOptions

`struct` · `parquet_variant_compute::variant_get::GetOptions`

Also reachable as `parquet::variant::GetOptions`, `parquet_variant_compute::GetOptions`

```rust
struct GetOptions<'a>
```

**Fields**: `path`, `as_type`, `cast_options`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn new() -> Self
fn new_with_path(path: VariantPath<'a>) -> Self
fn with_as_type(self, as_type: Option<FieldRef>) -> Self
fn with_cast_options(self, cast_options: CastOptions<'a>) -> Self
```

Controls the action of the variant_get kernel.

---
