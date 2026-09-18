# `deltalake_opendal::config::OPENDAL_PREFIX`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.config.OPENDAL_PREFIX.json).

<a id="op-34b9b70b7e35bbb411e0366d"></a>
## OPENDAL_PREFIX

`constant` · `deltalake_opendal::config::OPENDAL_PREFIX` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const OPENDAL_PREFIX: &str = "opendal."
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/config.rs#L12).

Source: `crates/opendal/src/config.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage-option prefix whose entries are forwarded to OpenDAL.

A storage option `opendal.<key> = <value>` is passed to
[`opendal::Operator::via_iter`] as `<key> = <value>`. This keeps delta's own
reserved option keys (retry, runtime, limit, certificate, …) out of the
OpenDAL config and lets users reach any documented service key without a
per-service allow-list.

Unresolved upstream links (retained, not inferred): ``opendal::Operator::via_iter``.
