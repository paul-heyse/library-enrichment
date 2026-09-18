# `datafusion_common::config_field`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config_field.json).

<a id="op-fb56011aa897154826e267fd"></a>
## config_field

`macro` · `datafusion_common::config_field` · datafusion-common 55.1.0

```rust
macro_rules! config_field
```

Source: `src/config.rs:2406`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Macro that generates [`ConfigField`](../operations/datafusion_common.config.ConfigField.md#op-679f6f8b2eea5f3367d79f93) for a given type.

# Usage
This always requires [`Display`] to be implemented for the given type.

There are two ways to invoke this macro. The first one uses
[`default_config_transform`](../operations/datafusion_common.config.default_config_transform.md#op-700fa755c604b89800a424b7)/[`FromStr`] to parse the data:

```ignore
config_field(MyType);
```

Note that the parsing error MUST implement [`std::error::Error`]!

Or you can specify how you want to parse an [`str`] into the type:

```ignore
fn parse_it(s: &str) -> Result<MyType> {
    ...
}

config_field(
    MyType,
    value => parse_it(value)
);
```

Unresolved upstream links (retained, not inferred): ``std::error::Error``, ``FromStr``, ``str``, ``Display``.
