# `deltalake_opendal::OPENDAL_SCHEME_PREFIX`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.OPENDAL_SCHEME_PREFIX.json).

<a id="op-c0a46b0140c51a19a610ef4d"></a>
## OPENDAL_SCHEME_PREFIX

`constant` · `deltalake_opendal::OPENDAL_SCHEME_PREFIX` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const OPENDAL_SCHEME_PREFIX: &str = "opendal+"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/lib.rs#L96).

Source: `crates/opendal/src/lib.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The scheme prefix that makes any OpenDAL service reachable unambiguously,
e.g. `opendal+s3://`. `+` is a valid URL scheme character (RFC 3986).
