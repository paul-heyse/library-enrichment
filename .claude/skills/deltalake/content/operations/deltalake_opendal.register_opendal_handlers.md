# `deltalake_opendal::register_opendal_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.register_opendal_handlers.json).

<a id="op-9ab6f8acef8a8fa1a06c0d0a"></a>
## register_opendal_handlers

`function` · `deltalake_opendal::register_opendal_handlers` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_opendal_handlers<A: OpendalAdapter + 'static>(scheme: &str, adapter: std::sync::Arc<A>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/lib.rs#L30).

Source: `crates/opendal/src/lib.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [`OpendalAdapter`](../operations/deltalake_opendal.adapter.OpendalAdapter.md#op-710e9b27c5cc68f393fd433d) as the object-store and log-store factory for
`scheme`. This overwrites any existing factory registered for the scheme.
