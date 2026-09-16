# `deltalake_opendal::config`

Crate `deltalake-opendal` · 1 public items · structured records in [`model/deltalake_opendal.config.json`](../model/deltalake_opendal.config.json)

## OPENDAL_PREFIX

`constant` · `deltalake_opendal::config::OPENDAL_PREFIX`

Also reachable as `deltalake::opendal::OPENDAL_PREFIX`, `deltalake_opendal::OPENDAL_PREFIX`

```rust
const OPENDAL_PREFIX: &str = "opendal."
```

Storage-option prefix whose entries are forwarded to OpenDAL.

A storage option `opendal.<key> = <value>` is passed to
[`opendal::Operator::via_iter`] as `<key> = <value>`. This keeps delta's own
reserved option keys (retry, runtime, limit, certificate, …) out of the
OpenDAL config and lets users reach any documented service key without a
per-service allow-list.

---
