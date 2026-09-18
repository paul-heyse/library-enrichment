# `datafusion_ffi::tests::config`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.tests.config.json`](../model/datafusion_ffi.tests.config.json)

## ExternalConfig

`struct` · `datafusion_ffi::tests::config::ExternalConfig`

```rust
struct ExternalConfig
```

**Fields**: `is_enabled`, `base_number`

**Implements**: `datafusion_common::config::ConfigExtension`, `datafusion_common::config::ConfigField`, `datafusion_common::config::ExtensionOptions`

**Derives**: Clone, Debug, Default, Eq, PartialEq

**via `datafusion_common::config::ConfigField`**

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
fn visit<V: config::Visit>(&self, v: &mut V, _key_prefix: &str, _description: &'static str)
```

**via `datafusion_common::config::ExtensionOptions`**

```rust
fn as_any(&self) -> &dyn ::std::any::Any
fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any
fn cloned(&self) -> Box<dyn config::ExtensionOptions>
fn entries(&self) -> Vec<config::ConfigEntry>
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.tests.config.ExternalConfig.md).


---
