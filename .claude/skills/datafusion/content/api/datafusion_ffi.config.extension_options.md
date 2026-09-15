# `datafusion_ffi::config::extension_options`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.config.extension_options.json`](../model/datafusion_ffi.config.extension_options.json)

## ExtensionOptionsPrivateData

`struct` · `datafusion_ffi::config::extension_options::ExtensionOptionsPrivateData`

```rust
struct ExtensionOptionsPrivateData
```

**Fields**: `options`

---

## FFI_ExtensionOptions

`struct` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions`

```rust
struct FFI_ExtensionOptions
```

**Fields**: `cloned`, `set`, `entries`, `release`, `private_data`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`, `datafusion_common::config::ConfigExtension`, `datafusion_common::config::ExtensionOptions`

**Derives**: Clone, Debug, Default, Send, Sync

**Methods** (3)

```rust
fn add_config<C: ConfigExtension>(&mut self, config: &C) -> Result<()>
fn merge(&mut self, other: &FFI_ExtensionOptions) -> Result<()>
fn to_extension<C: ConfigExtension + Default>(&self) -> Result<C>
```

**via `core::convert::From`**

```rust
fn from(options: HashMap<String, String>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

**via `datafusion_common::config::ExtensionOptions`**

```rust
fn as_any(&self) -> &dyn Any
fn as_any_mut(&mut self) -> &mut dyn Any
fn cloned(&self) -> Box<dyn ExtensionOptions>
fn entries(&self) -> Vec<ConfigEntry>
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

A stable struct for sharing [`ExtensionOptions`] across FFI boundaries.

Unlike other FFI structs in this crate, we do not construct a foreign
variant of this object. This is due to the typical method for interacting
with extension options is by creating a local struct of your concrete type.
To support this methodology use the `to_extension` method instead.

When using [`FFI_ExtensionOptions`] with multiple extensions, all extension
values are stored on a single [`FFI_ExtensionOptions`] object. The keys
are stored with the full path prefix to avoid overwriting values when using
multiple extensions.

---
