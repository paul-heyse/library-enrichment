# `datafusion_common::config_namespace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config_namespace.json).

<a id="op-1012b502781667e7c6f2e912"></a>
## config_namespace

`macro` · `datafusion_common::config_namespace` · datafusion-common 55.1.0

```rust
macro_rules! config_namespace
```

Source: `src/config.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A macro that wraps a configuration struct and automatically derives
[`Default`] and [`ConfigField`](../operations/datafusion_common.config.ConfigField.md#op-679f6f8b2eea5f3367d79f93) for it, allowing it to be used
in the [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) configuration tree.

`transform` is used to normalize values before parsing.

For example,

```ignore
config_namespace! {
   /// Amazing config
   pub struct MyConfig {
       /// Field 1 doc
       field1: String, transform = str::to_lowercase, default = "".to_string()

       /// Field 2 doc
       field2: usize, default = 232

       /// Field 3 doc
       field3: Option<usize>, default = None
   }
}
```

Will generate

```ignore
/// Amazing config
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MyConfig {
    /// Field 1 doc
    field1: String,
    /// Field 2 doc
    field2: usize,
    /// Field 3 doc
    field3: Option<usize>,
}
impl ConfigField for MyConfig {
    fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let (key, rem) = key.split_once('.').unwrap_or((key, ""));
        match key {
            "field1" => {
                let value = str::to_lowercase(value);
                self.field1.set(rem, value.as_ref())
            },
            "field2" => self.field2.set(rem, value.as_ref()),
            "field3" => self.field3.set(rem, value.as_ref()),
            _ => _internal_err!(
                "Config value \"{}\" not found on MyConfig",
                key
            ),
        }
    }

    fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str) {
        let key = format!("{}.field1", key_prefix);
        let desc = "Field 1 doc";
        self.field1.visit(v, key.as_str(), desc);
        let key = format!("{}.field2", key_prefix);
        let desc = "Field 2 doc";
        self.field2.visit(v, key.as_str(), desc);
        let key = format!("{}.field3", key_prefix);
        let desc = "Field 3 doc";
        self.field3.visit(v, key.as_str(), desc);
    }
}

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            field1: "".to_string(),
            field2: 232,
            field3: None,
        }
    }
}
```

NB: Misplaced commas may result in nonsensical errors

Unresolved upstream links (retained, not inferred): ``Default``.
