# `deltalake_azure::AzureFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_azure.AzureFactory.json).

<a id="op-7b12312ea27c766c1edbf1ba"></a>
## AzureFactory

`struct` · `deltalake_azure::AzureFactory` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AzureFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L36).

Source: `crates/azure/src/lib.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90820679955482b48cd22d36"></a>
## clone

`function` · `deltalake_azure::AzureFactory::clone` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> AzureFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_azure::AzureFactory", "path": "AzureFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "crates/azure/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/azure/src/lib.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50c9088590219d94f250147d"></a>
## default

`function` · `deltalake_azure::AzureFactory::default` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> AzureFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_azure::AzureFactory", "path": "AzureFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 24], "filename": "crates/azure/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/azure/src/lib.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392a5b9c482131096e0489f4"></a>
## fmt

`function` · `deltalake_azure::AzureFactory::fmt` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L35).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_azure::AzureFactory", "path": "AzureFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 26], "end": [35, 31], "filename": "crates/azure/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/azure/src/lib.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e410acb510fafc38855a7e1"></a>
## parse_url_opts

`function` · `deltalake_azure::AzureFactory::parse_url_opts` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L39).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_azure::AzureFactory", "path": "AzureFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [74, 2], "filename": "crates/azure/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/azure/src/lib.rs:39`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19138120bd2b2d32d735b078"></a>
## with_options

`function` · `deltalake_azure::AzureFactory::with_options` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L77).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_azure::AzureFactory", "path": "AzureFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [91, 2], "filename": "crates/azure/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/azure/src/lib.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
