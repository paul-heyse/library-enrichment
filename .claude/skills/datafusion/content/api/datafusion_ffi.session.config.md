# `datafusion_ffi::session::config`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.session.config.json`](../model/datafusion_ffi.session.config.json)

## FFI_SessionConfig

`struct` · `datafusion_ffi::session::config::FFI_SessionConfig`

```rust
struct FFI_SessionConfig
```

**Fields**: `config_options`, `clone`, `release`, `private_data`, `library_marker_id`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(session: &SessionConfig) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.session.config.FFI_SessionConfig.md).


A stable struct for sharing [`SessionConfig`] across FFI boundaries.
Instead of attempting to expose the entire SessionConfig interface, we
convert the config options into a map from a string to string and pass
those values across the FFI boundary. On the receiver side, we
reconstruct a SessionConfig from those values.

It is possible that using different versions of DataFusion across the
FFI boundary could have differing expectations of the config options.
This is a limitation of this approach, but exposing the entire
SessionConfig via a FFI interface would be extensive and provide limited
value over this version.

---
