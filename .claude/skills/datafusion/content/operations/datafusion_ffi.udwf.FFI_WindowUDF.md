# `datafusion_ffi::udwf::FFI_WindowUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udwf.FFI_WindowUDF.json).

<a id="op-62903d44d7f184382bcc0bcf"></a>
## FFI_WindowUDF

`struct` · `datafusion_ffi::udwf::FFI_WindowUDF` · datafusion-ffi 55.1.0

```rust
struct FFI_WindowUDF
```

Source: `src/udwf/mod.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65) across FFI boundaries.

<a id="op-d425412a5c29085ba314da32"></a>
## aliases

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::aliases` · datafusion-ffi 55.1.0

```rust
aliases: stabby::vec::Vec<stabby::string::String>
```

Source: `src/udwf/mod.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `aliases` of a [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)

<a id="op-65775201ffc20036dea5a93f"></a>
## clone

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/udwf/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the udf. This should
only need to be called by the receiver of the udf.

<a id="op-82882f1f3dd3e3a2c45210ae"></a>
## clone

`function` · `datafusion_ffi::udwf::FFI_WindowUDF::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::FFI_WindowUDF", "path": "FFI_WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [218, 1], "end": [222, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udwf/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6afea2dda9fef3ee374c7d0"></a>
## coerce_types

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
coerce_types: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> util::FFI_Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>>
```

Source: `src/udwf/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Performs type coercion. To simply this interface, all UDFs are treated as having
user defined signatures, which will in turn call coerce_types to be called. This
call should be transparent to most users as the internal function performs the
appropriate calls on the underlying [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)

<a id="op-72f08918f1d78666cb63c928"></a>
## drop

`function` · `datafusion_ffi::udwf::FFI_WindowUDF::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::FFI_WindowUDF", "path": "FFI_WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [257, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/udwf/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d6e778298338d4284b34e0a"></a>
## field

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::field` · datafusion-ffi 55.1.0

```rust
field: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedSchema>, stabby::string::String) -> util::FFI_Result<arrow_wrappers::WrappedSchema>
```

Source: `src/udwf/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5c2107a401c32b22086ef39"></a>
## fmt

`function` · `datafusion_ffi::udwf::FFI_WindowUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::FFI_WindowUDF", "path": "FFI_WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udwf/mod.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b61218015744dca0f34e8ba"></a>
## from

`function` · `datafusion_ffi::udwf::FFI_WindowUDF::from` · datafusion-ffi 55.1.0

```rust
fn from(udf: Arc<WindowUDF>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::FFI_WindowUDF", "path": "FFI_WindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [224, 1], "end": [251, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udwf/mod.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d8110380a9d3d18299dda53"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udwf/mod.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-9058e983a77eeddbaee2470a"></a>
## name

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::name` · datafusion-ffi 55.1.0

```rust
name: stabby::string::String
```

Source: `src/udwf/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `name` of a [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)

<a id="op-9dd78073a71ba8634dc04177"></a>
## partition_evaluator

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::partition_evaluator` · datafusion-ffi 55.1.0

```rust
partition_evaluator: unsafe fn(&Self, partition_evaluator_args::FFI_PartitionEvaluatorArgs) -> util::FFI_Result<partition_evaluator::FFI_PartitionEvaluator>
```

Source: `src/udwf/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec9557c8bdefdd76408cdc6f"></a>
## private_data

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udwf/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the udf.
A [`ForeignWindowUDF`](../operations/datafusion_ffi.udwf.ForeignWindowUDF.md#op-cfc1c6fd5bd041227c3a9172) should never attempt to access this data.

<a id="op-ba2a4927206d5796d3f2417d"></a>
## release

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udwf/mod.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-b9c72c793f473397c86cb1d9"></a>
## sort_options

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::sort_options` · datafusion-ffi 55.1.0

```rust
sort_options: util::FFI_Option<FFI_SortOptions>
```

Source: `src/udwf/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4270e941ef37c972840f075f"></a>
## volatility

`struct_field` · `datafusion_ffi::udwf::FFI_WindowUDF::volatility` · datafusion-ffi 55.1.0

```rust
volatility: volatility::FFI_Volatility
```

Source: `src/udwf/mod.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `volatility` of a [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)
