# `datafusion_execution::parquet_encryption::EncryptionFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.parquet_encryption.EncryptionFactory.json).

<a id="op-7306e44c25717f033e9116e5"></a>
## EncryptionFactory

`trait` · `datafusion_execution::parquet_encryption::EncryptionFactory` · datafusion-execution 55.1.0

```rust
trait EncryptionFactory: Send + Sync + std::fmt::Debug + 'static
```

Source: `src/parquet_encryption.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Trait for types that generate file encryption and decryption properties to
write and read encrypted Parquet files.
This allows flexibility in how encryption keys are managed, for example, to
integrate with a user's key management service (KMS).
For example usage, see the [`parquet_encrypted_with_kms` example].

[`parquet_encrypted_with_kms` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/data_io/parquet_encrypted_with_kms.rs

<a id="op-49ab77763b9cde401770f4db"></a>
## get_file_decryption_properties

`function` · `datafusion_execution::parquet_encryption::EncryptionFactory::get_file_decryption_properties` · datafusion-execution 55.1.0

```rust
async fn get_file_decryption_properties(&self, config: &EncryptionFactoryOptions, file_path: &Path) -> Result<Option<Arc<FileDecryptionProperties>>>
```

Source: `src/parquet_encryption.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Generate file decryption properties to use when reading a Parquet file.

<a id="op-64d3690cf5980057f8d2cb60"></a>
## get_file_encryption_properties

`function` · `datafusion_execution::parquet_encryption::EncryptionFactory::get_file_encryption_properties` · datafusion-execution 55.1.0

```rust
async fn get_file_encryption_properties(&self, config: &EncryptionFactoryOptions, schema: &SchemaRef, file_path: &Path) -> Result<Option<Arc<FileEncryptionProperties>>>
```

Source: `src/parquet_encryption.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Generate file encryption properties to use when writing a Parquet file.
