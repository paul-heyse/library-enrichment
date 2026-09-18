# `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.parquet_encryption.EncryptionFactoryRegistry.json).

<a id="op-953b5a5c8ac949b61181e97e"></a>
## EncryptionFactoryRegistry

`struct` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry` · datafusion-execution 55.1.0

```rust
struct EncryptionFactoryRegistry
```

Source: `src/parquet_encryption.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Stores [`EncryptionFactory`](../operations/datafusion_execution.parquet_encryption.EncryptionFactory.md#op-7306e44c25717f033e9116e5) implementations that can be retrieved by a unique string identifier

<a id="op-80c0a0091939c3aafd8711cd"></a>
## clone

`function` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> EncryptionFactoryRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::parquet_encryption::EncryptionFactoryRegistry", "path": "EncryptionFactoryRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/parquet_encryption.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parquet_encryption.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f656748d5d1c8a0b9fbf2079"></a>
## default

`function` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry::default` · datafusion-execution 55.1.0

```rust
fn default() -> EncryptionFactoryRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::parquet_encryption::EncryptionFactoryRegistry", "path": "EncryptionFactoryRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 24], "end": [55, 31], "filename": "src/parquet_encryption.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/parquet_encryption.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99840923644fc5cda1440336"></a>
## fmt

`function` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::parquet_encryption::EncryptionFactoryRegistry", "path": "EncryptionFactoryRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 22], "filename": "src/parquet_encryption.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parquet_encryption.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c15c2e2f5850f9043031fcc5"></a>
## get_factory

`function` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry::get_factory` · datafusion-execution 55.1.0

```rust
fn get_factory(&self, id: &str) -> Result<Arc<dyn EncryptionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::parquet_encryption::EncryptionFactoryRegistry", "path": "EncryptionFactoryRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [83, 2], "filename": "src/parquet_encryption.rs"}, "trait": null, "trait_path": null}`

Source: `src/parquet_encryption.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Retrieve an [`EncryptionFactory`](../operations/datafusion_execution.parquet_encryption.EncryptionFactory.md#op-7306e44c25717f033e9116e5) by its identifier

<a id="op-29a3573bf6a033b46127a713"></a>
## register_factory

`function` · `datafusion_execution::parquet_encryption::EncryptionFactoryRegistry::register_factory` · datafusion-execution 55.1.0

```rust
fn register_factory(&self, id: &str, factory: Arc<dyn EncryptionFactory>) -> Option<Arc<dyn EncryptionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::parquet_encryption::EncryptionFactoryRegistry", "path": "EncryptionFactoryRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [83, 2], "filename": "src/parquet_encryption.rs"}, "trait": null, "trait_path": null}`

Source: `src/parquet_encryption.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Register an [`EncryptionFactory`](../operations/datafusion_execution.parquet_encryption.EncryptionFactory.md#op-7306e44c25717f033e9116e5) with an associated identifier that can be later
used to configure encryption when reading or writing Parquet.
If an encryption factory with the same identifier was already registered, it is replaced and returned.
