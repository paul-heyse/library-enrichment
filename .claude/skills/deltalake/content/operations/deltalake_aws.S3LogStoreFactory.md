# `deltalake_aws::S3LogStoreFactory`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.S3LogStoreFactory.json).

<a id="op-b693401c73acc781c097ca50"></a>
## S3LogStoreFactory

`struct` · `deltalake_aws::S3LogStoreFactory` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3LogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L24).

Source: `crates/aws/src/lib.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe72f5e8ac2eb78d28ff64f1"></a>
## clone

`function` · `deltalake_aws::S3LogStoreFactory::clone` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> S3LogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::S3LogStoreFactory", "path": "S3LogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "crates/aws/src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/aws/src/lib.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4747b2a09120643913c28b9"></a>
## default

`function` · `deltalake_aws::S3LogStoreFactory::default` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> S3LogStoreFactory
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::S3LogStoreFactory", "path": "S3LogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 24], "end": [23, 31], "filename": "crates/aws/src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/aws/src/lib.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b96c09cc3407fbe9e694ad5f"></a>
## fmt

`function` · `deltalake_aws::S3LogStoreFactory::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::S3LogStoreFactory", "path": "S3LogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 22], "filename": "crates/aws/src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/aws/src/lib.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eec7c78005af5df921af1c3"></a>
## with_options

`function` · `deltalake_aws::S3LogStoreFactory::with_options` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_options(&self, prefixed_store: ObjectStoreRef, root_store: ObjectStoreRef, location: &Url, options: &StorageConfig) -> DeltaResult<Arc<dyn LogStore>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L29).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::S3LogStoreFactory", "path": "S3LogStoreFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 1], "end": [52, 2], "filename": "crates/aws/src/lib.rs"}, "trait": {"args": null, "id": "deltalake_core::logstore::factories::LogStoreFactory", "path": "LogStoreFactory"}, "trait_path": "deltalake_core::logstore::factories::LogStoreFactory"}`

Source: `crates/aws/src/lib.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
