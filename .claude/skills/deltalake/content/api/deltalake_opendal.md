# `deltalake_opendal`

Crate `deltalake-opendal` · 3 public items · structured records in [`model/deltalake_opendal.json`](../model/deltalake_opendal.json)

## OPENDAL_SCHEME_PREFIX

`constant` · `deltalake_opendal::OPENDAL_SCHEME_PREFIX`

Also reachable as `deltalake::opendal::OPENDAL_SCHEME_PREFIX`

```rust
const OPENDAL_SCHEME_PREFIX: &str = "opendal+"
```

The scheme prefix that makes any OpenDAL service reachable unambiguously,
e.g. `opendal+s3://`. `+` is a valid URL scheme character (RFC 3986).

---

## register_handlers

`function` · `deltalake_opendal::register_handlers`

Also reachable as `deltalake::opendal::register_handlers`

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

Register the OpenDAL-backed storage handlers enabled by feature flags.

Called automatically at program start via the OpenDAL feature flags in the
top-level `deltalake` crate. Each enabled service is registered under
`opendal+<service>://`, plus its bare `<service>://` scheme when that neither
collides with a native delta backend nor is a WHATWG special scheme that
can't form a bare `scheme://` URL (e.g. `ftp`) — see [`GENERIC_SERVICES`].

---

## register_opendal_handlers

`function` · `deltalake_opendal::register_opendal_handlers`

Also reachable as `deltalake::opendal::register_opendal_handlers`

```rust
fn register_opendal_handlers<A: OpendalAdapter + 'static>(scheme: &str, adapter: std::sync::Arc<A>)
```

Register an [`OpendalAdapter`] as the object-store and log-store factory for
`scheme`. This overwrites any existing factory registered for the scheme.

---
