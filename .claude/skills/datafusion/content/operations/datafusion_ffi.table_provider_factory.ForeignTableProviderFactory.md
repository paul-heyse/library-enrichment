# `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.table_provider_factory.ForeignTableProviderFactory.json).

<a id="op-4ebd6e51c382668372554ebe"></a>
## ForeignTableProviderFactory

`struct` · `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory` · datafusion-ffi 55.1.0

```rust
struct ForeignTableProviderFactory
```

Source: `src/table_provider_factory.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_TableProviderFactory to interact with the foreign table provider factory.

<a id="op-16598c200e2e350de6961ff3"></a>
## 0

`struct_field` · `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory::0` · datafusion-ffi 55.1.0

```rust
0: FFI_TableProviderFactory
```

Source: `src/table_provider_factory.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa1127a051e368fedb413303"></a>
## create

`function` · `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory::create` · datafusion-ffi 55.1.0

```rust
async fn create(&self, session: &dyn Session, cmd: &CreateExternalTable) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::ForeignTableProviderFactory", "path": "ForeignTableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 1], "end": [308, 2], "filename": "src/table_provider_factory.rs"}, "trait": {"args": null, "id": "datafusion_session::table::TableProviderFactory", "path": "TableProviderFactory"}, "trait_path": "datafusion_session::table::TableProviderFactory"}`

Source: `src/table_provider_factory.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51f8896c377cd47f36db3b41"></a>
## fmt

`function` · `datafusion_ffi::table_provider_factory::ForeignTableProviderFactory::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::table_provider_factory::ForeignTableProviderFactory", "path": "ForeignTableProviderFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 10], "end": [264, 15], "filename": "src/table_provider_factory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_provider_factory.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
