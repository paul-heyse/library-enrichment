# `deltalake_core::logstore::config::TryUpdateKey`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.TryUpdateKey.json).

<a id="op-a62d931c739a64a1f127e04f"></a>
## TryUpdateKey

`trait` · `deltalake_core::logstore::config::TryUpdateKey` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TryUpdateKey: Default
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L22).

Source: `crates/core/src/logstore/config.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A configuration type that can be incrementally populated from string key/value pairs.

Implemented by the various storage configuration structs so that options coming from user
input or the environment can be applied generically by key name.

<a id="op-c8299fd4086b18eb62cb2bc3"></a>
## load_from_environment

`function` · `deltalake_core::logstore::config::TryUpdateKey::load_from_environment` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L37).

Source: `crates/core/src/logstore/config.rs:37`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Load configuration values from environment variables

For Option<T> fields, this will only set values that are None
For non-optional fields, environment variables will update the
value if the current value corresponds to the default value.

<a id="op-5d1366bf2998bdfa9d63daa1"></a>
## try_update_key

`function` · `deltalake_core::logstore::config::TryUpdateKey::try_update_key` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_update_key(&mut self, key: &str, value: &str) -> DeltaResult<Option<()>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L30).

Source: `crates/core/src/logstore/config.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update an internal field in the configuration.

## Returns
- `Ok(Some(()))` if the key was updated.
- `Ok(None)` if the key was not found and no internal field was updated.
- `Err(_)` if the update failed. Failed updates may include finding a known key,
  but failing to parse the value into the expected type.
