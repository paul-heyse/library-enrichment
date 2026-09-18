# `deltalake_aws::storage::S3StorageOptions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.storage.S3StorageOptions.json).

<a id="op-eb24adcde6e58acacf2987a1"></a>
## S3StorageOptions

`struct` · `deltalake_aws::storage::S3StorageOptions` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct S3StorageOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L135).

Source: `crates/aws/src/storage.rs:135`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Options used to configure the [S3StorageBackend](../operations/deltalake_aws.storage.S3StorageBackend.md#op-06dee9d109146210a1b9e486).

Available options are described in [constants](../modules/deltalake_aws.constants.md#op-1da55f2e5231c3610d5e2034).

<a id="op-f24f0faeebc1e739df2b975c"></a>
## allow_unsafe_rename

`struct_field` · `deltalake_aws::storage::S3StorageOptions::allow_unsafe_rename` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
allow_unsafe_rename: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L141).

Source: `crates/aws/src/storage.rs:141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Allow unsafe rename operations

<a id="op-2e3395a581b0f26cf928bc73"></a>
## builder

`function` · `deltalake_aws::storage::S3StorageOptions::builder` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder() -> S3StorageOptionsBuilder<((), ())>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 24], "end": [133, 36], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a builder for building `S3StorageOptions`.
On the builder, call `.locking_provider(...)`(optional), `.allow_unsafe_rename(...)`(optional) to set the values of the fields.
Finally, call `.build()` to create the instance of `S3StorageOptions`.
                

<a id="op-d5cae082aff5c56fadaf14d4"></a>
## clone

`function` · `deltalake_aws::storage::S3StorageOptions::clone` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> S3StorageOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 10], "end": [133, 15], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57f6d1fd7e6a763a11d2043d"></a>
## eq

`function` · `deltalake_aws::storage::S3StorageOptions::eq` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &S3StorageOptions) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 38], "end": [133, 47], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7caac45f3854126fda96d776"></a>
## fmt

`function` · `deltalake_aws::storage::S3StorageOptions::fmt` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 17], "end": [133, 22], "filename": "crates/aws/src/storage.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/aws/src/storage.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33b8a7928ebdcb0b1f435823"></a>
## from_map

`function` · `deltalake_aws::storage::S3StorageOptions::from_map` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_map(options: &HashMap<String, String>) -> DeltaResult<S3StorageOptions>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [158, 2], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates an instance of [`S3StorageOptions`](../operations/deltalake_aws.storage.S3StorageOptions.md#op-eb24adcde6e58acacf2987a1) from the given HashMap.

<a id="op-2e04b7889ad8cad42763c736"></a>
## locking_provider

`struct_field` · `deltalake_aws::storage::S3StorageOptions::locking_provider` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
locking_provider: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L138).

Source: `crates/aws/src/storage.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Locking provider to use (e.g., "dynamodb")

<a id="op-bb917c70c000227623471904"></a>
## try_default

`function` · `deltalake_aws::storage::S3StorageOptions::try_default` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_default() -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/storage.rs#L155).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_aws::storage::S3StorageOptions", "path": "S3StorageOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [158, 2], "filename": "crates/aws/src/storage.rs"}, "trait": null, "trait_path": null}`

Source: `crates/aws/src/storage.rs:155`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
