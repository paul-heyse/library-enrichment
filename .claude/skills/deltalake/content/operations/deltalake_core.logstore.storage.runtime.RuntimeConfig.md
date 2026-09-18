# `deltalake_core::logstore::storage::runtime::RuntimeConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.runtime.RuntimeConfig.json).

<a id="op-959696f817aa2d67063203f2"></a>
## RuntimeConfig

`struct` · `deltalake_core::logstore::storage::runtime::RuntimeConfig` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
struct RuntimeConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L70).

Source: `crates/core/src/logstore/storage/runtime.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration for Tokio runtime

<a id="op-4e7c68edc2bb46f2b58c6c94"></a>
## clone

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> RuntimeConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 17], "end": [69, 22], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6eb4f4ca8917e17b2df984e4"></a>
## default

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> RuntimeConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 48], "end": [69, 55], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cfe0d6b91dbd75b1b76b48e"></a>
## deserialize

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 35], "end": [69, 46], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e34613f7ed6702bb7dc64222"></a>
## enable_io

`struct_field` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::enable_io` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
enable_io: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L78).

Source: `crates/core/src/logstore/storage/runtime.rs:78`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to enable IO

<a id="op-d04273b9445dda305a5bb4de"></a>
## enable_time

`struct_field` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::enable_time` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
enable_time: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L80).

Source: `crates/core/src/logstore/storage/runtime.rs:80`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to enable time

<a id="op-e620300f084f56b4c7495b5e"></a>
## fmt

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 10], "end": [69, 15], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a5a350ce964b58f154e3f8"></a>
## from_iter

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::from_iter` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [69, 57], "end": [69, 68], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-862e34346cb8621bd69c60ea"></a>
## load_from_environment

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::load_from_environment` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 57], "end": [69, 68], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c027c19bd7bb7f9b639e2f4"></a>
## multi_threaded

`struct_field` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::multi_threaded` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
multi_threaded: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L72).

Source: `crates/core/src/logstore/storage/runtime.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether to use a multi-threaded runtime

<a id="op-b75a90de205540c2a1684bc6"></a>
## serialize

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 24], "end": [69, 33], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab3517385b0e90ffaa0f27c"></a>
## thread_name

`struct_field` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::thread_name` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
thread_name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L76).

Source: `crates/core/src/logstore/storage/runtime.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the thread

<a id="op-c1914462b26c1c198e421c85"></a>
## try_update_key

`function` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::try_update_key` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L69).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::RuntimeConfig", "path": "RuntimeConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 57], "end": [69, 68], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::config::TryUpdateKey", "path": "TryUpdateKey"}, "trait_path": "deltalake_core::logstore::config::TryUpdateKey"}`

Source: `crates/core/src/logstore/storage/runtime.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b7b535f5b3736e14a0f3448"></a>
## worker_threads

`struct_field` · `deltalake_core::logstore::storage::runtime::RuntimeConfig::worker_threads` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
worker_threads: Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L74).

Source: `crates/core/src/logstore/storage/runtime.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of worker threads to use
