# `datafusion_ffi::config`

Crate `datafusion-ffi` · 3 public items · structured records in [`model/datafusion_ffi.config.json`](../model/datafusion_ffi.config.json)

## FFI_ConfigOptions

`struct` · `datafusion_ffi::config::FFI_ConfigOptions`

```rust
struct FFI_ConfigOptions
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(options: &ConfigOptions) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.config.FFI_ConfigOptions.md).


A stable struct for sharing [`ConfigOptions`] across FFI boundaries.

Accessing FFI extension options require a slightly different pattern
than local extensions. The trait [`ExtensionOptionsFFIProvider`] can
be used to simplify accessing FFI extensions.

---

## FFI_TableOptions

`struct` · `datafusion_ffi::config::FFI_TableOptions`

```rust
struct FFI_TableOptions
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(options: &TableOptions) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.config.FFI_TableOptions.md).


A stable struct for sharing [`TableOptions`] across FFI boundaries.

Accessing FFI extension options require a slightly different pattern
than local extensions. The trait [`ExtensionOptionsFFIProvider`] can
be used to simplify accessing FFI extensions.

---

## ExtensionOptionsFFIProvider

`trait` · `datafusion_ffi::config::ExtensionOptionsFFIProvider`

```rust
trait ExtensionOptionsFFIProvider
```

**Implementors** (2)

- `datafusion_common::config::ConfigOptions`
- `datafusion_common::config::TableOptions`

**Methods** (1)

```rust
fn local_or_ffi_extension<C: ConfigExtension + Clone + Default>(&self) -> Option<C>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.config.ExtensionOptionsFFIProvider.md).


---
