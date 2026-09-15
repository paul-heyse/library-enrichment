# `datafusion_execution::parquet_encryption`

Crate `datafusion-execution` · 2 public items · structured records in [`model/datafusion_execution.parquet_encryption.json`](../model/datafusion_execution.parquet_encryption.json)

## EncryptionFactoryRegistry

`struct` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry`

```rust
struct EncryptionFactoryRegistry
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn get_factory(&self, id: &str) -> Result<Arc<dyn EncryptionFactory>>
fn register_factory(&self, id: &str, factory: Arc<dyn EncryptionFactory>) -> Option<Arc<dyn EncryptionFactory>>
```

Stores [`EncryptionFactory`] implementations that can be retrieved by a unique string identifier

---

## EncryptionFactory

`trait` · `datafusion_execution::parquet_encryption::EncryptionFactory`

```rust
trait EncryptionFactory: Send + Sync + std::fmt::Debug + 'static
```

**Methods** (2)

```rust
async fn get_file_decryption_properties(&self, config: &EncryptionFactoryOptions, file_path: &Path) -> Result<Option<Arc<FileDecryptionProperties>>>
async fn get_file_encryption_properties(&self, config: &EncryptionFactoryOptions, schema: &SchemaRef, file_path: &Path) -> Result<Option<Arc<FileEncryptionProperties>>>
```

Trait for types that generate file encryption and decryption properties to
write and read encrypted Parquet files.
This allows flexibility in how encryption keys are managed, for example, to
integrate with a user's key management service (KMS).
For example usage, see the [`parquet_encrypted_with_kms` example].

[`parquet_encrypted_with_kms` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/parquet_encrypted_with_kms.rs

---
