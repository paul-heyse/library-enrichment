# `deltalake_catalog_unity::client::retry::Result`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.client.retry.Result.json).

<a id="op-0ddcec76f3ea6a88e52ec978"></a>
## Result

`type_alias` · `deltalake_catalog_unity::client::retry::Result` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Result<T, E = RetryError> = std::result::Result<T, E>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/client/retry.rs#L70).

Source: `crates/catalog-unity/src/client/retry.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error retrying http requests
