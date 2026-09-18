# `deltalake_lakefs::client::LakeFSClient`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.client.LakeFSClient.json).

<a id="op-f3fd0d5e23a3f016211206ae"></a>
## LakeFSClient

`struct` · `deltalake_lakefs::client::LakeFSClient` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LakeFSClient
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L33).

Source: `crates/lakefs/src/client.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Slim LakeFS client for lakefs branch operations.

<a id="op-896650e91ee905503d7e38ae"></a>
## clear_transaction

`function` · `deltalake_lakefs::client::LakeFSClient::clear_transaction` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clear_transaction(&self, id: Uuid)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L303).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:303`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff425e8129617f77e0be478e"></a>
## clone

`function` · `deltalake_lakefs::client::LakeFSClient::clone` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> LakeFSClient
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "crates/lakefs/src/client.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/lakefs/src/client.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92cb751b42f718b52143c2ff"></a>
## commit

`function` · `deltalake_lakefs::client::LakeFSClient::commit` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn commit(&self, repo: String, branch: String, commit_message: String, allow_empty: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d222f668b028d1615f17609"></a>
## create_branch

`function` · `deltalake_lakefs::client::LakeFSClient::create_branch` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn create_branch(&self, source_url: &Url, operation_id: Uuid) -> DeltaResult<(Url, String)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1237c082ee15c6659482e0eb"></a>
## decompose_url

`function` · `deltalake_lakefs::client::LakeFSClient::decompose_url` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn decompose_url(&self, url: String) -> (String, String, String)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L308).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f6e1a6978168e6557a1cbc6"></a>
## delete_branch

`function` · `deltalake_lakefs::client::LakeFSClient::delete_branch` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn delete_branch(&self, repo: String, branch: String) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L101).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef3a54b4605a88d3cb2dcc92"></a>
## fmt

`function` · `deltalake_lakefs::client::LakeFSClient::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "crates/lakefs/src/client.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/client.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07ebc899b6db73d1bfd5b871"></a>
## get_transaction

`function` · `deltalake_lakefs::client::LakeFSClient::get_transaction` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_transaction(&self, id: Uuid) -> Result<String, TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L293).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:293`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-494e794fb6d6c0444ac389ff"></a>
## has_changes

`function` · `deltalake_lakefs::client::LakeFSClient::has_changes` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn has_changes(&self, repo: &str, base_branch: &str, compare_branch: &str) -> Result<bool, TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L236).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:236`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50da4eeb443411ad39a69db3"></a>
## merge

`function` · `deltalake_lakefs::client::LakeFSClient::merge` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn merge(&self, repo: String, target_branch: String, transaction_branch: String, commit_version: Version, commit_message: String, allow_empty: bool) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L184).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:184`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e54c959fb823bdb899207541"></a>
## set_transaction

`function` · `deltalake_lakefs::client::LakeFSClient::set_transaction` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set_transaction(&self, id: Uuid, branch: String)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L288).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:288`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d111424c4721c80b96131ff"></a>
## with_config

`function` · `deltalake_lakefs::client::LakeFSClient::with_config` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_config(config: LakeFSConfig) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::client::LakeFSClient", "path": "LakeFSClient"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [320, 2], "filename": "crates/lakefs/src/client.rs"}, "trait": null, "trait_path": null}`

Source: `crates/lakefs/src/client.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9571fcde073f99889a7910a"></a>
## config

`struct_field` · `deltalake_lakefs::client::LakeFSClient::config` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
config: LakeFSConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L35).

Source: `crates/lakefs/src/client.rs:35`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

configuration of the lakefs client

<a id="op-6008d3e1173fa33f292e1f5e"></a>
## http_client

`struct_field` · `deltalake_lakefs::client::LakeFSClient::http_client` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
http_client: reqwest::Client
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L36).

Source: `crates/lakefs/src/client.rs:36`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-875102339139c6cd2227028f"></a>
## transactions

`struct_field` · `deltalake_lakefs::client::LakeFSClient::transactions` · deltalake-lakefs 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
transactions: dashmap::DashMap<uuid::Uuid, String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/client.rs#L38).

Source: `crates/lakefs/src/client.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Holds the running delta lake operations, each operation propagates the operation ID into execution handler.
