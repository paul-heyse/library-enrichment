# `datafusion_common::config::ConfigField`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigField.json).

<a id="op-679f6f8b2eea5f3367d79f93"></a>
## ConfigField

`trait` · `datafusion_common::config::ConfigField` · datafusion-common 55.1.0

```rust
trait ConfigField
```

Source: `src/config.rs:2327`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A trait implemented by `config_namespace` and for field types that provides
the ability to walk and mutate the configuration tree

<a id="op-5e1be3e777d7ecb738d01be8"></a>
## reset

`function` · `datafusion_common::config::ConfigField::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> Result<()>
```

Source: `src/config.rs:2332`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3dea2e17bfee5033b2b52cf"></a>
## set

`function` · `datafusion_common::config::ConfigField::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Source: `src/config.rs:2330`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5395e9fc1c663aa83b42d64"></a>
## visit

`function` · `datafusion_common::config::ConfigField::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Source: `src/config.rs:2328`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
