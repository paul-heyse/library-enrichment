# `deltalake_core::logstore::storage::LimitConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.LimitConfig.json).

<a id="op-3b366e8999c48134851090bf"></a>
## LimitConfig

`struct` · `deltalake_core::logstore::storage::LimitConfig` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
struct LimitConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L99).

Source: `crates/core/src/logstore/storage/mod.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc40d6ef9a641b6e39824c3f"></a>
## max_concurrency

`struct_field` · `deltalake_core::logstore::storage::LimitConfig::max_concurrency` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
max_concurrency: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L101).

Source: `crates/core/src/logstore/storage/mod.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0de0fa5a93be993d4aaeab6f"></a>
## clone

`function` · `deltalake_core::logstore::storage::LimitConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LimitConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 17], "end": [98, 22], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3347ee3f6cacc869c3f8a06a"></a>
## default

`function` · `deltalake_core::logstore::storage::LimitConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> LimitConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 24], "end": [98, 31], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b33008538c45257f5fd3f4"></a>
## fmt

`function` · `deltalake_core::logstore::storage::LimitConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 10], "end": [98, 15], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-317e29e32f83614e27e8df8f"></a>
## from_iter

`function` · `deltalake_core::logstore::storage::LimitConfig::from_iter` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [98, 33], "end": [98, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d72aa5242fdebfcf07fde195"></a>
## load_from_environment

`function` · `deltalake_core::logstore::storage::LimitConfig::load_from_environment` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 33], "end": [98, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cb9870b672f352cdb2ec17f"></a>
## try_update_key

`function` · `deltalake_core::logstore::storage::LimitConfig::try_update_key` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::LimitConfig", "path": "LimitConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 33], "end": [98, 44], "filename": "crates/core/src/logstore/storage/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
