# `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udf.return_type_args.FFI_ReturnFieldArgs.json).

<a id="op-4517f75b187be13ba1237aae"></a>
## FFI_ReturnFieldArgs

`struct` · `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs` · datafusion-ffi 55.1.0

```rust
struct FFI_ReturnFieldArgs
```

Source: `src/udf/return_type_args.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing a [`ReturnFieldArgs`](../operations/datafusion_expr.udf.ReturnFieldArgs.md#op-45e9203bcbe458f8338d7ffd) across FFI boundaries.

<a id="op-1231291cb7ea90e323e9f304"></a>
## Error

`assoc_type` · `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs::Error` · datafusion-ffi 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs", "path": "FFI_ReturnFieldArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [69, 2], "filename": "src/udf/return_type_args.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::udf::ReturnFieldArgs", "path": "ReturnFieldArgs"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/udf/return_type_args.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d42c8dd0a4633444c7250b7"></a>
## fmt

`function` · `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs", "path": "FFI_ReturnFieldArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/udf/return_type_args.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf/return_type_args.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5731d89eb0984f3be6a94834"></a>
## try_from

`function` · `datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs::try_from` · datafusion-ffi 55.1.0

```rust
fn try_from(value: ReturnFieldArgs<'_>) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::return_type_args::FFI_ReturnFieldArgs", "path": "FFI_ReturnFieldArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [69, 2], "filename": "src/udf/return_type_args.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::udf::ReturnFieldArgs", "path": "ReturnFieldArgs"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/udf/return_type_args.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
