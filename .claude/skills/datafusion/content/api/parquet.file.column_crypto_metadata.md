# `parquet::file::column_crypto_metadata`

Crate `parquet` · 2 public items · structured records in [`model/parquet.file.column_crypto_metadata.json`](../model/parquet.file.column_crypto_metadata.json)

## ColumnCryptoMetaData

`enum` · `parquet::file::column_crypto_metadata::ColumnCryptoMetaData`

```rust
enum ColumnCryptoMetaData
```

**Variants**: `ENCRYPTION_WITH_FOOTER_KEY`, `ENCRYPTION_WITH_COLUMN_KEY`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/parquet.file.column_crypto_metadata.ColumnCryptoMetaData.md).


ColumnCryptoMetadata for a column chunk

---

## EncryptionWithColumnKey

`struct` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey`

```rust
struct EncryptionWithColumnKey
```

**Fields**: `path_in_schema`, `key_metadata`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/parquet.file.column_crypto_metadata.EncryptionWithColumnKey.md).


Encryption metadata for a column chunk encrypted with a column-specific key

---
