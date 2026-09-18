# `deltalake_core::logstore::config::StorageConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.StorageConfig.json).

<a id="op-83934d7bd9af6992fec7cb57"></a>
## StorageConfig

`struct` · `deltalake_core::logstore::config::StorageConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L100).

Source: `crates/core/src/logstore/config.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Resolved configuration for constructing and decorating an object store backend.

Aggregates the optional dedicated IO runtime, retry/limit/certificate settings and the raw
passthrough options used to build the underlying [`ObjectStore`].

Unresolved upstream links (retained, not inferred): ``ObjectStore``.

<a id="op-3fc9980842d1ca774b1e4e83"></a>
## certificate

`struct_field` · `deltalake_core::logstore::config::StorageConfig::certificate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
certificate: Option<super::storage::CertificateConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L121).

Source: `crates/core/src/logstore/config.rs:121`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Certificate configuration.

Configuration for custom TLS root certificates.

<a id="op-7cfc9d2b86c1f1d212cbb354"></a>
## clone

`function` · `deltalake_core::logstore::config::StorageConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 26], "end": [99, 31], "filename": "crates/core/src/logstore/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/config.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4be9675a18efb3ff4a07b653"></a>
## decorate_store

`function` · `deltalake_core::logstore::config::StorageConfig::decorate_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn decorate_store<T: ObjectStore + Clone>(&self, store: T, table_root: &url::Url) -> DeltaResult<Box<dyn ObjectStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L141).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [161, 2], "filename": "crates/core/src/logstore/config.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/config.rs:141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Wrap an object store with additional layers of functionality.

Depending on the configuration, the following layers may be added:
- Retry layer: Adds retry logic to the object store.
- Limit layer: Limits the number of concurrent requests to the object store.

<a id="op-b0202fc87669fb4d0992abdd"></a>
## default

`function` · `deltalake_core::logstore::config::StorageConfig::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> StorageConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 10], "end": [99, 17], "filename": "crates/core/src/logstore/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/config.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f936c3c887b2047fda4c339"></a>
## fmt

`function` · `deltalake_core::logstore::config::StorageConfig::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 19], "end": [99, 24], "filename": "crates/core/src/logstore/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/config.rs:99`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ef2d295c33d16dddd7f9c7"></a>
## from_iter

`function` · `deltalake_core::logstore::config::StorageConfig::from_iter` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L168).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [163, 1], "end": [200, 2], "filename": "crates/core/src/logstore/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "K"}, {"generic": "V"}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `crates/core/src/logstore/config.rs:168`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb0c00ed5031f41ef71706e8"></a>
## limit

`struct_field` · `deltalake_core::logstore::config::StorageConfig::limit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
limit: Option<super::storage::LimitConfig>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L116).

Source: `crates/core/src/logstore/config.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Limit configuration.

Configuration to limit the number of concurrent requests to the object store.

<a id="op-73b223a5abc1cc0b606ec3b9"></a>
## parse_options

`function` · `deltalake_core::logstore::config::StorageConfig::parse_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_options<K, V, I>(options: I) -> DeltaResult<Self> where I: IntoIterator<Item = (K, V)>, K: AsRef<str> + Into<String>, V: AsRef<str> + Into<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [262, 2], "filename": "crates/core/src/logstore/config.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/config.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse options into a StorageConfig.

This method will raise if it cannot parse a value. StorageConfig can also
be constructed from an iterator of key-value pairs which will ignore any
parsing errors.

# Raises

Raises a `DeltaError` if any of the options are invalid - i.e. cannot be parsed into target type.

<a id="op-1a53cc3fa683af183775b16c"></a>
## raw

`function` · `deltalake_core::logstore::config::StorageConfig::raw` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn raw(&self) -> impl Iterator<Item = (&String, &String)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L204).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [262, 2], "filename": "crates/core/src/logstore/config.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/config.rs:204`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Iterate over the raw, unparsed key/value options retained on this config.

<a id="op-a37382d45337a90c221bcdab"></a>
## raw

`struct_field` · `deltalake_core::logstore::config::StorageConfig::raw` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
raw: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L132).

Source: `crates/core/src/logstore/config.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Original unprocessed properties.

Since we remove properties during processing, but downstream integrations may
use them for their own purposes, we keep a copy of the original properties.

<a id="op-61c60795b46acd01daf25224"></a>
## retry

`struct_field` · `deltalake_core::logstore::config::StorageConfig::retry` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
retry: ::object_store::RetryConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L111).

Source: `crates/core/src/logstore/config.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retry config for object stores

Configuration for how object store should retry failed requests

<a id="op-dc5a6467936ed1b6a5fe93df"></a>
## runtime

`struct_field` · `deltalake_core::logstore::config::StorageConfig::runtime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
runtime: Option<super::IORuntime>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L105).

Source: `crates/core/src/logstore/config.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Runtime configuration.

Configuration to set up a dedicated IO runtime to execute IO related operations or
dedicated handle.

<a id="op-5bbdfd459a8ab0fe4fad0bc5"></a>
## unknown_properties

`struct_field` · `deltalake_core::logstore::config::StorageConfig::unknown_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
unknown_properties: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L126).

Source: `crates/core/src/logstore/config.rs:126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties that are not recognized by the storage configuration.

These properties are ignored by the storage configuration and can be used for custom purposes.

<a id="op-5a02f86f91eb44de9785514c"></a>
## with_io_runtime

`function` · `deltalake_core::logstore::config::StorageConfig::with_io_runtime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_io_runtime(self, rt: IORuntime) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L258).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::config::StorageConfig", "path": "StorageConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [262, 2], "filename": "crates/core/src/logstore/config.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/config.rs:258`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Attach a dedicated IO [`IORuntime`](../operations/deltalake_core.logstore.storage.runtime.IORuntime.md#op-efee47ae417b2a7e236e3c83) used to execute storage operations.
