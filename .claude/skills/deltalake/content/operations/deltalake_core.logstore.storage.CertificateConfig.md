# `deltalake_core::logstore::storage::CertificateConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.CertificateConfig.json).

<a id="op-6b65c696af432640a42ce578"></a>
## CertificateConfig

`struct` · `deltalake_core::logstore::storage::CertificateConfig` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
struct CertificateConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L105).

Source: `crates/core/src/logstore/storage/mod.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29bfd1730f6adde9c8edb5ac"></a>
## certificate_path

`struct_field` · `deltalake_core::logstore::storage::CertificateConfig::certificate_path` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
certificate_path: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L108).

Source: `crates/core/src/logstore/storage/mod.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Path to a PEM-encoded root certificate file for TLS connections.

<a id="op-9bddd91218a476c54f39dbed"></a>
## clone

`function` · `deltalake_core::logstore::storage::CertificateConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CertificateConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 17], "end": [104, 22], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6b6db2cfd2541bd2698e439"></a>
## default

`function` · `deltalake_core::logstore::storage::CertificateConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> CertificateConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 24], "end": [104, 31], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fc1a57fc1c7587238a72085"></a>
## fmt

`function` · `deltalake_core::logstore::storage::CertificateConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 10], "end": [104, 15], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe4e69c3bf206d7eda60fd7c"></a>
## from_iter

`function` · `deltalake_core::logstore::storage::CertificateConfig::from_iter` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [104, 33], "end": [104, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d936ab53bffd7a574792832"></a>
## load_from_environment

`function` · `deltalake_core::logstore::storage::CertificateConfig::load_from_environment` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 33], "end": [104, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c43e6b4ca98ef5431e533edf"></a>
## try_update_key

`function` · `deltalake_core::logstore::storage::CertificateConfig::try_update_key` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::CertificateConfig", "path": "CertificateConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 33], "end": [104, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/mod.rs:104`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
