# `datafusion_ffi::config::ExtensionOptionsFFIProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.config.ExtensionOptionsFFIProvider.json).

<a id="op-79d7f2d95343853ab3adb15f"></a>
## ExtensionOptionsFFIProvider

`trait` · `datafusion_ffi::config::ExtensionOptionsFFIProvider` · datafusion-ffi 55.1.0

```rust
trait ExtensionOptionsFFIProvider
```

Source: `src/config/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fabc3acaff021a692e13981d"></a>
## local_or_ffi_extension

`function` · `datafusion_ffi::config::ExtensionOptionsFFIProvider::local_or_ffi_extension` · datafusion-ffi 55.1.0

```rust
fn local_or_ffi_extension<C: ConfigExtension + Clone + Default>(&self) -> Option<C>
```

Source: `src/config/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Extract a [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c). This method should attempt to first extract
the extension from the local options when possible. Should that fail, it
should attempt to extract the FFI options and then convert them to the
desired [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c).
