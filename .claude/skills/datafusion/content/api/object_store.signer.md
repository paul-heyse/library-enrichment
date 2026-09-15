# `object_store::signer`

Crate `object_store` · 1 public items · structured records in [`model/object_store.signer.json`](../model/object_store.signer.json)

## Signer

`trait` · `object_store::signer::Signer`

```rust
trait Signer: Send + Sync + fmt::Debug + 'static
```

**Implementors** (3)

- `object_store::aws::AmazonS3`
- `object_store::azure::MicrosoftAzure`
- `object_store::gcp::GoogleCloudStorage`

**Methods** (2)

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
async fn signed_urls(&self, method: Method, paths: &[Path], expires_in: Duration) -> Result<Vec<Url>>
```

Universal API to generate presigned URLs from multiple object store services.

---
