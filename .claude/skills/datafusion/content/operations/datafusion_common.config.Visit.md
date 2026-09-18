# `datafusion_common::config::Visit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.Visit.json).

<a id="op-446cb14b96e615d8eb3b0377"></a>
## Visit

`trait` · `datafusion_common::config::Visit` · datafusion-common 55.1.0

```rust
trait Visit
```

Source: `src/config.rs:2498`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An implementation trait used to recursively walk configuration

<a id="op-327b63c8d3dda084d5f0b034"></a>
## none

`function` · `datafusion_common::config::Visit::none` · datafusion-common 55.1.0

```rust
fn none(&mut self, key: &str, description: &'static str)
```

Source: `src/config.rs:2501`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84be43159832d7a061a7f51b"></a>
## some

`function` · `datafusion_common::config::Visit::some` · datafusion-common 55.1.0

```rust
fn some<V: Display>(&mut self, key: &str, value: V, description: &'static str)
```

Source: `src/config.rs:2499`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
