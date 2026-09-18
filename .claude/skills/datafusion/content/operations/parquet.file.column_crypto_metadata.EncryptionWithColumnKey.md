# `parquet::file::column_crypto_metadata::EncryptionWithColumnKey`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.column_crypto_metadata.EncryptionWithColumnKey.json).

<a id="op-3e6770416b964e296dee5945"></a>
## EncryptionWithColumnKey

`struct` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey` · parquet 59.3.0

```rust
struct EncryptionWithColumnKey
```

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Encryption metadata for a column chunk encrypted with a column-specific key

<a id="op-426a02745f94222b4d6fadea"></a>
## clone

`function` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey::clone` · parquet 59.3.0

```rust
fn clone(&self) -> EncryptionWithColumnKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::column_crypto_metadata::EncryptionWithColumnKey", "path": "EncryptionWithColumnKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [42, 2], "filename": "src/file/column_crypto_metadata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5b102b44d606a1e6bab282b"></a>
## eq

`function` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &EncryptionWithColumnKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::column_crypto_metadata::EncryptionWithColumnKey", "path": "EncryptionWithColumnKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [42, 2], "filename": "src/file/column_crypto_metadata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ac33a4d54f02cac3739ece"></a>
## fmt

`function` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::column_crypto_metadata::EncryptionWithColumnKey", "path": "EncryptionWithColumnKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [42, 2], "filename": "src/file/column_crypto_metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27ac0cdd671c9a1a600496d7"></a>
## key_metadata

`struct_field` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey::key_metadata` · parquet 59.3.0

```rust
key_metadata: Option<Vec<u8>>
```

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Path to the column in the Parquet schema

<a id="op-8753b97ab9a03a0e950db605"></a>
## path_in_schema

`struct_field` · `parquet::file::column_crypto_metadata::EncryptionWithColumnKey::path_in_schema` · parquet 59.3.0

```rust
path_in_schema: Vec<String>
```

Source: `src/file/column_crypto_metadata.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Path to the column in the Parquet schema
