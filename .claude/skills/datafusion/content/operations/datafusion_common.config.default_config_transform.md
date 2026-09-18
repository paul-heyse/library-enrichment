# `datafusion_common::config::default_config_transform`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.default_config_transform.json).

<a id="op-700fa755c604b89800a424b7"></a>
## default_config_transform

`function` · `datafusion_common::config::default_config_transform` · datafusion-common 55.1.0

```rust
fn default_config_transform<T>(input: &str) -> Result<T> where T: FromStr, <T as FromStr>::Err: Sync + Send + Error + 'static
```

Source: `src/config.rs:2362`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Default transformation to parse a [`ConfigField`](../operations/datafusion_common.config.ConfigField.md#op-679f6f8b2eea5f3367d79f93) for a string.

This uses [`FromStr`] to parse the data.

Unresolved upstream links (retained, not inferred): ``FromStr``.
