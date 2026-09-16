# TryUpdateKey

`deltalake_core::logstore::config::TryUpdateKey`

```rust
trait TryUpdateKey: Default
```

Also reachable as `deltalake::logstore::config::TryUpdateKey`

Prose: [`api/deltalake_core.logstore.config.md`](../api/deltalake_core.logstore.config.md#tryupdatekey) · records: [`model/deltalake_core.logstore.config.json`](../model/deltalake_core.logstore.config.json)

## Required

Every implementation must supply these.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, value: &str) -> DeltaResult<Option<()>>
```

## Implementors (5)

Read one before writing your own.

- `deltalake_core::logstore::storage::CertificateConfig`
- `deltalake_core::logstore::storage::LimitConfig`
- `deltalake_core::logstore::storage::runtime::RuntimeConfig`
- `deltalake_core::table::builder::DeltaTableConfig`
- `object_store::client::retry::RetryConfig`

## Documentation

A configuration type that can be incrementally populated from string key/value pairs.

Implemented by the various storage configuration structs so that options coming from user
input or the environment can be applied generically by key name.
