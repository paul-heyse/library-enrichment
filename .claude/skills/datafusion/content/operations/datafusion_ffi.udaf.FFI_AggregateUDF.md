# `datafusion_ffi::udaf::FFI_AggregateUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.FFI_AggregateUDF.json).

<a id="op-d1d3d516f3972a641fff322c"></a>
## FFI_AggregateUDF

`struct` · `datafusion_ffi::udaf::FFI_AggregateUDF` · datafusion-ffi 55.1.0

```rust
struct FFI_AggregateUDF
```

Source: `src/udaf/mod.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379) across FFI boundaries.

<a id="op-e032026a36e9bda3fa173870"></a>
## accumulator

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::accumulator` · datafusion-ffi 55.1.0

```rust
accumulator: unsafe fn(&FFI_AggregateUDF, accumulator_args::FFI_AccumulatorArgs) -> util::FFI_Result<accumulator::FFI_Accumulator>
```

Source: `src/udaf/mod.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::accumulator`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::accumulator``.

<a id="op-0a4d97d73c1d53661f87687a"></a>
## aliases

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::aliases` · datafusion-ffi 55.1.0

```rust
aliases: stabby::vec::Vec<stabby::string::String>
```

Source: `src/udaf/mod.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `aliases` of a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

<a id="op-427efa7fc145ab742a109613"></a>
## clone

`function` · `datafusion_ffi::udaf::FFI_AggregateUDF::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateUDF", "path": "FFI_AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 1], "end": [381, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udaf/mod.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b68d551941ff73729895e8e"></a>
## clone

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/udaf/mod.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the udaf. This should
only need to be called by the receiver of the udaf.

<a id="op-f112a841ecb6c7f13e3198a2"></a>
## coerce_types

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
coerce_types: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> util::FFI_Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>>
```

Source: `src/udaf/mod.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Performs type coercion. To simply this interface, all UDFs are treated as having
user defined signatures, which will in turn call coerce_types to be called. This
call should be transparent to most users as the internal function performs the
appropriate calls on the underlying [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

<a id="op-611ece4ae94e063e4054560e"></a>
## create_groups_accumulator

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::create_groups_accumulator` · datafusion-ffi 55.1.0

```rust
create_groups_accumulator: unsafe fn(&FFI_AggregateUDF, accumulator_args::FFI_AccumulatorArgs) -> util::FFI_Result<groups_accumulator::FFI_GroupsAccumulator>
```

Source: `src/udaf/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::create_groups_accumulator`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::create_groups_accumulator``.

<a id="op-463c9dff17243b389db4f533"></a>
## create_sliding_accumulator

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::create_sliding_accumulator` · datafusion-ffi 55.1.0

```rust
create_sliding_accumulator: unsafe fn(&FFI_AggregateUDF, accumulator_args::FFI_AccumulatorArgs) -> util::FFI_Result<accumulator::FFI_Accumulator>
```

Source: `src/udaf/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::create_sliding_accumulator`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::create_sliding_accumulator``.

<a id="op-40a2f27dfca191676a1163f6"></a>
## drop

`function` · `datafusion_ffi::udaf::FFI_AggregateUDF::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateUDF", "path": "FFI_AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [423, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/udaf/mod.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4db19a4cee8e7e7eb24c1fc"></a>
## fmt

`function` · `datafusion_ffi::udaf::FFI_AggregateUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateUDF", "path": "FFI_AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e238d1ff0e954e60ab0189d"></a>
## from

`function` · `datafusion_ffi::udaf::FFI_AggregateUDF::from` · datafusion-ffi 55.1.0

```rust
fn from(udaf: Arc<AggregateUDF>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::FFI_AggregateUDF", "path": "FFI_AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [383, 1], "end": [417, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udaf/mod.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-137b3e0d5caf0cd955bf47a8"></a>
## groups_accumulator_supported

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::groups_accumulator_supported` · datafusion-ffi 55.1.0

```rust
groups_accumulator_supported: unsafe fn(&FFI_AggregateUDF, accumulator_args::FFI_AccumulatorArgs) -> bool
```

Source: `src/udaf/mod.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::groups_accumulator_supported`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::groups_accumulator_supported``.

<a id="op-cd6920b6bb7d5b7edb750b18"></a>
## is_nullable

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::is_nullable` · datafusion-ffi 55.1.0

```rust
is_nullable: bool
```

Source: `src/udaf/mod.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `is_nullable` of a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

<a id="op-2a5f32c23af23eb179ee25f2"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udaf/mod.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-5a118576010637d9a22a34aa"></a>
## name

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::name` · datafusion-ffi 55.1.0

```rust
name: stabby::string::String
```

Source: `src/udaf/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `name` of a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

<a id="op-90f9bcf4a5ca237004a3eb0d"></a>
## order_sensitivity

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::order_sensitivity` · datafusion-ffi 55.1.0

```rust
order_sensitivity: unsafe fn(&FFI_AggregateUDF) -> FFI_AggregateOrderSensitivity
```

Source: `src/udaf/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::order_sensitivity`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::order_sensitivity``.

<a id="op-8c043dd02705838649507242"></a>
## private_data

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udaf/mod.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the udaf.
A [`ForeignAggregateUDF`](../operations/datafusion_ffi.udaf.ForeignAggregateUDF.md#op-b1fbbd6be3a777c6e6a91f42) should never attempt to access this data.

<a id="op-55f71fc6bdb62f8187f6d0a5"></a>
## release

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udaf/mod.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-99339b56fa5829a0f19902bd"></a>
## return_field

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::return_field` · datafusion-ffi 55.1.0

```rust
return_field: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> util::FFI_Result<arrow_wrappers::WrappedSchema>
```

Source: `src/udaf/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Determines the return field of the underlying [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379) based on the
argument fields.

<a id="op-2c3a556a1930206bdda31d75"></a>
## state_fields

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::state_fields` · datafusion-ffi 55.1.0

```rust
state_fields: unsafe fn(&FFI_AggregateUDF, &stabby::str::Str<'_>, stabby::vec::Vec<arrow_wrappers::WrappedSchema>, arrow_wrappers::WrappedSchema, stabby::vec::Vec<stabby::vec::Vec<u8>>, bool) -> util::FFI_Result<stabby::vec::Vec<stabby::vec::Vec<u8>>>
```

Source: `src/udaf/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::state_fields`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::state_fields``.

<a id="op-2f8653c085ecced4575ba879"></a>
## supports_null_handling_clause

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::supports_null_handling_clause` · datafusion-ffi 55.1.0

```rust
supports_null_handling_clause: unsafe fn(&FFI_AggregateUDF) -> bool
```

Source: `src/udaf/mod.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::supports_null_handling_clause`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::supports_null_handling_clause``.

<a id="op-e8bab4a353ff71b72e56d60a"></a>
## volatility

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::volatility` · datafusion-ffi 55.1.0

```rust
volatility: volatility::FFI_Volatility
```

Source: `src/udaf/mod.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `volatility` of a [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

<a id="op-df84bd3d49f4351c8c988466"></a>
## with_beneficial_ordering

`struct_field` · `datafusion_ffi::udaf::FFI_AggregateUDF::with_beneficial_ordering` · datafusion-ffi 55.1.0

```rust
with_beneficial_ordering: unsafe fn(&FFI_AggregateUDF, bool) -> util::FFI_Result<util::FFI_Option<FFI_AggregateUDF>>
```

Source: `src/udaf/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`AggregateUDF::with_beneficial_ordering`]

Unresolved upstream links (retained, not inferred): ``AggregateUDF::with_beneficial_ordering``.
