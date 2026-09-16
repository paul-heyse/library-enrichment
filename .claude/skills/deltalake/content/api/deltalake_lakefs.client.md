# `deltalake_lakefs::client`

Crate `deltalake-lakefs` · 2 public items · structured records in [`model/deltalake_lakefs.client.json`](../model/deltalake_lakefs.client.json)

## LakeFSClient

`struct` · `deltalake_lakefs::client::LakeFSClient`

```rust
struct LakeFSClient
```

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn clear_transaction(&self, id: Uuid)
async fn commit(&self, repo: String, branch: String, commit_message: String, allow_empty: bool) -> DeltaResult<()>
async fn create_branch(&self, source_url: &Url, operation_id: Uuid) -> DeltaResult<(Url, String)>
fn decompose_url(&self, url: String) -> (String, String, String)
async fn delete_branch(&self, repo: String, branch: String) -> Result<(), TransactionError>
fn get_transaction(&self, id: Uuid) -> Result<String, TransactionError>
async fn has_changes(&self, repo: &str, base_branch: &str, compare_branch: &str) -> Result<bool, TransactionError>
async fn merge(&self, repo: String, target_branch: String, transaction_branch: String, commit_version: Version, commit_message: String, allow_empty: bool) -> Result<(), TransactionError>
fn set_transaction(&self, id: Uuid, branch: String)
fn with_config(config: LakeFSConfig) -> Self
```

Slim LakeFS client for lakefs branch operations.

---

## LakeFSConfig

`struct` · `deltalake_lakefs::client::LakeFSConfig`

```rust
struct LakeFSConfig
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(host: String, username: String, password: String) -> Self
```

---
