# `deltalake_opendal`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.json).

<a id="op-2b89f0f0c10afb7c67426b68"></a>
## deltalake_opendal

`module` · `deltalake_opendal` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod deltalake_opendal
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/lib.rs#L1).

Source: `crates/opendal/src/lib.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generic [OpenDAL](https://opendal.apache.org/) storage backend for delta-rs.

This crate plugs any OpenDAL service into delta's storage registry through a
single pair of generic factories ([`OpendalObjectStoreFactory`](../operations/deltalake_opendal.factory.OpendalObjectStoreFactory.md#op-3357fd1c29a9679d63313282) and
[`OpendalLogStoreFactory`](../operations/deltalake_opendal.factory.OpendalLogStoreFactory.md#op-5e5583e81279a2143b0475a4)) parameterized by an [`OpendalAdapter`](../operations/deltalake_opendal.adapter.OpendalAdapter.md#op-710e9b27c5cc68f393fd433d). Simple
services (operator scoped at the bucket root, URL path == table prefix) use
the built-in [`GenericAdapter`](../operations/deltalake_opendal.adapter.GenericAdapter.md#op-bdda3815c77aad989e184fca); services with bespoke URL or path semantics
supply their own [`OpendalAdapter`](../operations/deltalake_opendal.adapter.OpendalAdapter.md#op-710e9b27c5cc68f393fd433d) in a downstream crate.

Each backend is registered for a URL scheme via [`register_opendal_handlers`](../operations/deltalake_opendal.register_opendal_handlers.md#op-9ab6f8acef8a8fa1a06c0d0a).
