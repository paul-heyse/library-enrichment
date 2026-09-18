# `deltalake_opendal::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.register_handlers.json).

<a id="op-a3b6ac60139298c49c68eb4f"></a>
## register_handlers

`function` · `deltalake_opendal::register_handlers` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/lib.rs#L105).

Source: `crates/opendal/src/lib.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register the OpenDAL-backed storage handlers enabled by feature flags.

Called automatically at program start via the OpenDAL feature flags in the
top-level `deltalake` crate. Each enabled service is registered under
`opendal+<service>://`, plus its bare `<service>://` scheme when that neither
collides with a native delta backend nor is a WHATWG special scheme that
can't form a bare `scheme://` URL (e.g. `ftp`) — see [`GENERIC_SERVICES`].

Unresolved upstream links (retained, not inferred): ``GENERIC_SERVICES``.
