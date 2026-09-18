# `datafusion_ffi::session::config::FFI_SessionConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.session.config.FFI_SessionConfig.json).

<a id="op-a90bedd5818bf2dcf650e0d6"></a>
## FFI_SessionConfig

`struct` · `datafusion_ffi::session::config::FFI_SessionConfig` · datafusion-ffi 55.1.0

```rust
struct FFI_SessionConfig
```

Source: `src/session/config.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) across FFI boundaries.
Instead of attempting to expose the entire SessionConfig interface, we
convert the config options into a map from a string to string and pass
those values across the FFI boundary. On the receiver side, we
reconstruct a SessionConfig from those values.

It is possible that using different versions of DataFusion across the
FFI boundary could have differing expectations of the config options.
This is a limitation of this approach, but exposing the entire
SessionConfig via a FFI interface would be extensive and provide limited
value over this version.

<a id="op-194e5b57b4d427174b977a96"></a>
## clone

`struct_field` · `datafusion_ffi::session::config::FFI_SessionConfig::clone` · datafusion-ffi 55.1.0

```rust
clone: unsafe fn(&Self) -> Self
```

Source: `src/session/config.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Used to create a clone on the provider of the execution plan. This should
only need to be called by the receiver of the plan.

<a id="op-1af4709d8682a148e46aaa4d"></a>
## clone

`function` · `datafusion_ffi::session::config::FFI_SessionConfig::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::config::FFI_SessionConfig", "path": "FFI_SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [121, 2], "filename": "src/session/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/session/config.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e8e00e9dc8aa4af78f14bff"></a>
## config_options

`struct_field` · `datafusion_ffi::session::config::FFI_SessionConfig::config_options` · datafusion-ffi 55.1.0

```rust
config_options: config::FFI_ConfigOptions
```

Source: `src/session/config.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI stable configuration options.

<a id="op-072299302b82c3524cfa7186"></a>
## drop

`function` · `datafusion_ffi::session::config::FFI_SessionConfig::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::config::FFI_SessionConfig", "path": "FFI_SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [123, 1], "end": [127, 2], "filename": "src/session/config.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/session/config.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9c48aebe5ba40a571131168"></a>
## fmt

`function` · `datafusion_ffi::session::config::FFI_SessionConfig::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::config::FFI_SessionConfig", "path": "FFI_SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/session/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/session/config.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21dc5651dae3aa6da0ad3122"></a>
## from

`function` · `datafusion_ffi::session::config::FFI_SessionConfig::from` · datafusion-ffi 55.1.0

```rust
fn from(session: &SessionConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::config::FFI_SessionConfig", "path": "FFI_SessionConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [115, 2], "filename": "src/session/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_execution::config::SessionConfig", "path": "SessionConfig"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/session/config.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3bbb1dba5e79d49c1735f1d"></a>
## library_marker_id

`struct_field` · `datafusion_ffi::session::config::FFI_SessionConfig::library_marker_id` · datafusion-ffi 55.1.0

```rust
library_marker_id: fn() -> usize
```

Source: `src/session/config.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Utility to identify when FFI objects are accessed locally through
the foreign interface. See [`crate::get_library_marker_id`](../operations/datafusion_ffi.get_library_marker_id.md#op-66c1f07f1e28422eccc970cc) and
the crate's `README.md` for more information.

<a id="op-b1ed2adc6f28f8f8aed45f7d"></a>
## private_data

`struct_field` · `datafusion_ffi::session::config::FFI_SessionConfig::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/session/config.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the plan.

<a id="op-e8d4b63559e7c862932d4f09"></a>
## release

`struct_field` · `datafusion_ffi::session::config::FFI_SessionConfig::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/session/config.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.
