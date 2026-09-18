# `deltalake_aws::storage::S3ObjectStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.storage.S3ObjectStoreFactory.json).

<a id="op-f39a2d0f9388f5ddef3d632b"></a>
## S3ObjectStoreFactory

`struct` · `deltalake_aws::storage::S3ObjectStoreFactory` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3ObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L32).

Source: `crates/aws/src/storage.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c1b5b8408322d771e5dd265"></a>
## clone

`function` · `deltalake_aws::storage::S3ObjectStoreFactory::clone` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> S3ObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3ObjectStoreFactory", "path": "S3ObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/aws/src/storage.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5b54393d9244181f824173f"></a>
## default

`function` · `deltalake_aws::storage::S3ObjectStoreFactory::default` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> S3ObjectStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3ObjectStoreFactory", "path": "S3ObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 24], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/aws/src/storage.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ffd6b80372229117eb5d28"></a>
## fmt

`function` · `deltalake_aws::storage::S3ObjectStoreFactory::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3ObjectStoreFactory", "path": "S3ObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 26], "end": [31, 31], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/aws/src/storage.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbca3f771b9bee179776207e"></a>
## parse_url_opts

`function` · `deltalake_aws::storage::S3ObjectStoreFactory::parse_url_opts` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_url_opts(&self, url: &Url, config: &StorageConfig) -> DeltaResult<(ObjectStoreRef, Path)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L37).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3ObjectStoreFactory", "path": "S3ObjectStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [87, 2], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::ObjectStoreFactory", "path": "ObjectStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::ObjectStoreFactory"}`

Source: `crates/aws/src/storage.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
