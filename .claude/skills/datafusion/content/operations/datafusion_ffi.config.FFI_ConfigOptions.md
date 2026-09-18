# `datafusion_ffi::config::FFI_ConfigOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.config.FFI_ConfigOptions.json).

<a id="op-b7c75d666010a02198737021"></a>
## FFI_ConfigOptions

`struct` · `datafusion_ffi::config::FFI_ConfigOptions` · datafusion-ffi 55.1.0

```rust
struct FFI_ConfigOptions
```

Source: `src/config/mod.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) across FFI boundaries.

Accessing FFI extension options require a slightly different pattern
than local extensions. The trait [`ExtensionOptionsFFIProvider`](../operations/datafusion_ffi.config.ExtensionOptionsFFIProvider.md#op-79d7f2d95343853ab3adb15f) can
be used to simplify accessing FFI extensions.

<a id="op-f029d4b20bb773a84c7b4e16"></a>
## clone

`function` · `datafusion_ffi::config::FFI_ConfigOptions::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_ConfigOptions", "path": "FFI_ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 22], "filename": "src/config/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b40bb483fd02ee5a9fede152"></a>
## fmt

`function` · `datafusion_ffi::config::FFI_ConfigOptions::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_ConfigOptions", "path": "FFI_ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/config/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config/mod.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b854c0de108a3fe1cafa766a"></a>
## from

`function` · `datafusion_ffi::config::FFI_ConfigOptions::from` · datafusion-ffi 55.1.0

```rust
fn from(options: &ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_ConfigOptions", "path": "FFI_ConfigOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [67, 2], "filename": "src/config/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::ConfigOptions", "path": "ConfigOptions"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/config/mod.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
