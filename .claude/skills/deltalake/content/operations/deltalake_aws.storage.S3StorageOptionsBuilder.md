# `deltalake_aws::storage::S3StorageOptionsBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.storage.S3StorageOptionsBuilder.json).

<a id="op-5a27a64a88a5d1cebf6c0824"></a>
## S3StorageOptionsBuilder

`struct` · `deltalake_aws::storage::S3StorageOptionsBuilder` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3StorageOptionsBuilder<TypedBuilderFields = ((), ())>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for [`S3StorageOptions`](../operations/deltalake_aws.storage.S3StorageOptions.md#op-eb24adcde6e58acacf2987a1) instances.

See [`S3StorageOptions::builder()`](../operations/deltalake_aws.storage.S3StorageOptions.md#op-2e3395a581b0f26cf928bc73) for more info.
                    

<a id="op-763f9a1903a94aa5dd9ee457"></a>
## allow_unsafe_rename

`function` · `deltalake_aws::storage::S3StorageOptionsBuilder::allow_unsafe_rename` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn allow_unsafe_rename(self, allow_unsafe_rename: bool) -> S3StorageOptionsBuilder<(__locking_provider, (bool,))>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "__locking_provider"}, {"tuple": []}]}}], "constraints": []}}, "id": "deltalake_aws::storage::S3StorageOptionsBuilder", "path": "S3StorageOptionsBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "__locking_provider"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 36], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow unsafe rename operations

<a id="op-2067566b5cf566b54adb57d7"></a>
## build

`function` · `deltalake_aws::storage::S3StorageOptionsBuilder::build` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> S3StorageOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "__locking_provider"}, {"generic": "__allow_unsafe_rename"}]}}], "constraints": []}}, "id": "deltalake_aws::storage::S3StorageOptionsBuilder", "path": "S3StorageOptionsBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "__locking_provider"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "__allow_unsafe_rename"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'__typed_builder_lifetime_for_default"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"generic": "__locking_provider"}]}}], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, "name": "Output"}]}}, "id": "typed_builder::NextFieldDefault", "path": "::typed_builder::NextFieldDefault"}}}, {"trait_bound": {"generic_params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'__typed_builder_lifetime_for_default"}], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"borrowed_ref": {"is_mutable": false, "lifetime": "'__typed_builder_lifetime_for_default", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, {"generic": "__allow_unsafe_rename"}]}}], "constraints": [{"args": null, "binding": {"equality": {"type": {"primitive": "bool"}}}, "name": "Output"}]}}, "id": "typed_builder::NextFieldDefault", "path": "::typed_builder::NextFieldDefault"}}}], "generic_params": [], "type": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}}}]}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 36], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Finalise the builder and create its [`S3StorageOptions`](../operations/deltalake_aws.storage.S3StorageOptions.md#op-eb24adcde6e58acacf2987a1) instance

<a id="op-b9719db205fe3a9be086b1ad"></a>
## clone

`function` · `deltalake_aws::storage::S3StorageOptionsBuilder::clone` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "TypedBuilderFields"}}], "constraints": []}}, "id": "deltalake_aws::storage::S3StorageOptionsBuilder", "path": "S3StorageOptionsBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "TypedBuilderFields"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}}}], "generic_params": [], "type": {"generic": "TypedBuilderFields"}}}]}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 36], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-446e2ac47ce5082b4effb05c"></a>
## locking_provider

`function` · `deltalake_aws::storage::S3StorageOptionsBuilder::locking_provider` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn locking_provider(self, locking_provider: impl ::core::convert::Into<String>) -> S3StorageOptionsBuilder<((Option<String>,), __allow_unsafe_rename)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"tuple": []}, {"generic": "__allow_unsafe_rename"}]}}], "constraints": []}}, "id": "deltalake_aws::storage::S3StorageOptionsBuilder", "path": "S3StorageOptionsBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "__allow_unsafe_rename"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 36], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Locking provider to use (e.g., "dynamodb")

<a id="op-3fd02f1191547216451473d6"></a>
## fields

`struct_field` · `deltalake_aws::storage::S3StorageOptionsBuilder::fields` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
fields: TypedBuilderFields
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-847eb7790fb0f0447f108446"></a>
## phantom

`struct_field` · `deltalake_aws::storage::S3StorageOptionsBuilder::phantom` · deltalake-aws 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
phantom: ::core::marker::PhantomData<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
