# `datafusion_ffi::udf::FFI_ScalarUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udf.FFI_ScalarUDF.json).

<a id="op-289d6175d2f7605090f2a8d2"></a>
## FFI_ScalarUDF

`struct` · `datafusion_ffi::udf::FFI_ScalarUDF` · datafusion-ffi 55.1.0

```rust
struct FFI_ScalarUDF
```

Source: `src/udf/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0) across FFI boundaries.

<a id="op-5222a2c27823fb732a5538c8"></a>
## aliases

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::aliases` · datafusion-ffi 55.1.0

```rust
aliases: stabby::vec::Vec<stabby::string::String>
```

Source: `src/udf/mod.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `aliases` of a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)

<a id="op-20b6796b28a7256f7ea30029"></a>
## clone

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/udf/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the udf. This should
only need to be called by the receiver of the udf.

<a id="op-2179aad75236506fb8f6185e"></a>
## clone

`function` · `datafusion_ffi::udf::FFI_ScalarUDF::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::FFI_ScalarUDF", "path": "FFI_ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [294, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udf/mod.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49d1049cfffc457e672256d8"></a>
## coerce_types

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
coerce_types: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> util::FFI_Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>>
```

Source: `src/udf/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Performs type coercion. To simply this interface, all UDFs are treated as having
user defined signatures, which will in turn call coerce_types to be called. This
call should be transparent to most users as the internal function performs the
appropriate calls on the underlying [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)

<a id="op-61e2815d08dc8bf1c0ddee37"></a>
## drop

`function` · `datafusion_ffi::udf::FFI_ScalarUDF::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::FFI_ScalarUDF", "path": "FFI_ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 1], "end": [332, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/udf/mod.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c982a2409d55ad1a19f0d01"></a>
## fmt

`function` · `datafusion_ffi::udf::FFI_ScalarUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::FFI_ScalarUDF", "path": "FFI_ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf/mod.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-981f89da5b7a67548715fef3"></a>
## from

`function` · `datafusion_ffi::udf::FFI_ScalarUDF::from` · datafusion-ffi 55.1.0

```rust
fn from(udf: Arc<ScalarUDF>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::FFI_ScalarUDF", "path": "FFI_ScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [296, 1], "end": [326, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::udf::ScalarUDF", "path": "ScalarUDF"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udf/mod.rs:297`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b999088834d8c386d98bc0bd"></a>
## invoke_with_args

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::invoke_with_args` · datafusion-ffi 55.1.0

```rust
invoke_with_args: unsafe fn(&Self, stabby::vec::Vec<arrow_wrappers::WrappedArray>, stabby::vec::Vec<arrow_wrappers::WrappedSchema>, usize, arrow_wrappers::WrappedSchema, config::FFI_ConfigOptions) -> util::FFI_Result<expr::columnar_value::FFI_ColumnarValue>
```

Source: `src/udf/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Execute the underlying [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0) and return the result as a `FFI_ArrowArray`
within an AbiStable wrapper.

<a id="op-2997f2b15d83030c421898fe"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/udf/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-94596cf072b8882bdba4b20c"></a>
## name

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::name` · datafusion-ffi 55.1.0

```rust
name: stabby::string::String
```

Source: `src/udf/mod.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `name` of a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)

<a id="op-c796f31d2df82c40df584b7e"></a>
## placement

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::placement` · datafusion-ffi 55.1.0

```rust
placement: unsafe fn(&Self, stabby::vec::Vec<placement::FFI_ExpressionPlacement>) -> placement::FFI_ExpressionPlacement
```

Source: `src/udf/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `placement` of a [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0). Returns the
placement hint for the underlying [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0) given each argument's
placement. Infallible, so it returns the value directly, not an `FFI_Result`.

<a id="op-23bcdf8d3087bc3fe07cabd3"></a>
## preserves_lex_ordering

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::preserves_lex_ordering` · datafusion-ffi 55.1.0

```rust
preserves_lex_ordering: unsafe fn(&Self, stabby::vec::Vec<expr::expr_properties::FFI_ExprProperties>) -> util::FFI_Result<bool>
```

Source: `src/udf/mod.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`ScalarUDFImpl::preserves_lex_ordering`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-ddd814ea6d24274264defc02).

<a id="op-d30f60653a4a7985efd74b8f"></a>
## private_data

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/udf/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the udf.
A [`ForeignScalarUDF`](../operations/datafusion_ffi.udf.ForeignScalarUDF.md#op-673b7043db9b397c0e396af5) should never attempt to access this data.

<a id="op-9936f6a584e6ae056f734fa8"></a>
## release

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/udf/mod.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-5e99787715de9892df4a84bf"></a>
## return_field_from_args

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::return_field_from_args` · datafusion-ffi 55.1.0

```rust
return_field_from_args: unsafe fn(&Self, return_type_args::FFI_ReturnFieldArgs) -> util::FFI_Result<arrow_wrappers::WrappedSchema>
```

Source: `src/udf/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Determines the return info of the underlying [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0).

<a id="op-fbddd7958c995948cff285f7"></a>
## short_circuits

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::short_circuits` · datafusion-ffi 55.1.0

```rust
short_circuits: bool
```

Source: `src/udf/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

See [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) for details on short_circuits

<a id="op-c4a293dc682c6e5f98befe16"></a>
## volatility

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::volatility` · datafusion-ffi 55.1.0

```rust
volatility: volatility::FFI_Volatility
```

Source: `src/udf/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to the `volatility` of a [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0)

<a id="op-a85ebef8e14c62573e7197eb"></a>
## with_updated_config

`struct_field` · `datafusion_ffi::udf::FFI_ScalarUDF::with_updated_config` · datafusion-ffi 55.1.0

```rust
with_updated_config: unsafe fn(&Self, config::FFI_ConfigOptions) -> util::FFI_Result<util::FFI_Option<FFI_ScalarUDF>>
```

Source: `src/udf/mod.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI equivalent to [`ScalarUDFImpl::with_updated_config`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-f0dc74cfc8974de0bebe2f8a).
