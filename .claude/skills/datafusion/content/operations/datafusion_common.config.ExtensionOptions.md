# `datafusion_common::config::ExtensionOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ExtensionOptions.json).

<a id="op-cf0c51673459a43f486d37db"></a>
## ExtensionOptions

`trait` · `datafusion_common::config::ExtensionOptions` · datafusion-common 55.1.0

```rust
trait ExtensionOptions: Send + Sync + fmt::Debug + 'static
```

Source: `src/config.rs:2255`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An object-safe API for storing arbitrary configuration.

See [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) for user defined configuration

<a id="op-bd4e1cc1dd1c17a5d4fa7eb6"></a>
## as_any

`function` · `datafusion_common::config::ExtensionOptions::as_any` · datafusion-common 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/config.rs:2259`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return `self` as [`Any`]

This is needed until trait upcasting is stabilized

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-71db1be97a3875028d2b86a9"></a>
## as_any_mut

`function` · `datafusion_common::config::ExtensionOptions::as_any_mut` · datafusion-common 55.1.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Source: `src/config.rs:2264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return `self` as [`Any`]

This is needed until trait upcasting is stabilized

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-1dd5fb4b2b10ab7ba8b5f4a1"></a>
## cloned

`function` · `datafusion_common::config::ExtensionOptions::cloned` · datafusion-common 55.1.0

```rust
fn cloned(&self) -> Box<dyn ExtensionOptions>
```

Source: `src/config.rs:2270`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a deep clone of this [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

It is important this does not share mutable state to avoid consistency issues
with configuration changing whilst queries are executing

<a id="op-ca781330b06aed47e8d1d2b8"></a>
## entries

`function` · `datafusion_common::config::ExtensionOptions::entries` · datafusion-common 55.1.0

```rust
fn entries(&self) -> Vec<ConfigEntry>
```

Source: `src/config.rs:2276`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the [`ConfigEntry`](../operations/datafusion_common.config.ConfigEntry.md#op-e610e9614a810518fe36454b) stored in this [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

<a id="op-4dfad4f41965a5c6b1612c3e"></a>
## set

`function` · `datafusion_common::config::ExtensionOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Source: `src/config.rs:2273`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Set the given `key`, `value` pair
