# `datafusion_ffi::config::FFI_TableOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.config.FFI_TableOptions.json).

<a id="op-8dd5b170db08f6f0d00a4754"></a>
## FFI_TableOptions

`struct` · `datafusion_ffi::config::FFI_TableOptions` · datafusion-ffi 55.1.0

```rust
struct FFI_TableOptions
```

Source: `src/config/mod.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`TableOptions`](../operations/datafusion_common.config.TableOptions.md#op-0523542c8dd1656aea13ac4e) across FFI boundaries.

Accessing FFI extension options require a slightly different pattern
than local extensions. The trait [`ExtensionOptionsFFIProvider`](../operations/datafusion_ffi.config.ExtensionOptionsFFIProvider.md#op-79d7f2d95343853ab3adb15f) can
be used to simplify accessing FFI extensions.

<a id="op-6195187bc2f8f79991dc3576"></a>
## clone

`function` · `datafusion_ffi::config::FFI_TableOptions::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> FFI_TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_TableOptions", "path": "FFI_TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 17], "end": [123, 22], "filename": "src/config/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de66834dde6c354cadb0808b"></a>
## fmt

`function` · `datafusion_ffi::config::FFI_TableOptions::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_TableOptions", "path": "FFI_TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 10], "end": [123, 15], "filename": "src/config/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bcda8f26d93072a69c3961f"></a>
## from

`function` · `datafusion_ffi::config::FFI_TableOptions::from` · datafusion-ffi 55.1.0

```rust
fn from(options: &TableOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::FFI_TableOptions", "path": "FFI_TableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [155, 2], "filename": "src/config/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_common::config::TableOptions", "path": "TableOptions"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/config/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
