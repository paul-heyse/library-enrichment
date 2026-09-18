# `datafusion_common::config::ConfigExtension`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ConfigExtension.json).

<a id="op-ef20648fc6d3eb8f6cc2530c"></a>
## ConfigExtension

`trait` · `datafusion_common::config::ConfigExtension` · datafusion-common 55.1.0

```rust
trait ConfigExtension: ExtensionOptions
```

Source: `src/config.rs:2245`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

[`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) provides a mechanism to store third-party configuration
within DataFusion [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4)

This mechanism can be used to pass configuration to user defined functions
or optimizer passes

# Example
```
use datafusion_common::{
    config::ConfigExtension, config::ConfigOptions, extensions_options,
};
// Define a new configuration struct using the `extensions_options` macro
extensions_options! {
   /// My own config options.
   pub struct MyConfig {
       /// Should "foo" be replaced by "bar"?
       pub foo_to_bar: bool, default = true

       /// How many "baz" should be created?
       pub baz_count: usize, default = 1337
   }
}

impl ConfigExtension for MyConfig {
    const PREFIX: &'static str = "my_config";
}

// set up config struct and register extension
let mut config = ConfigOptions::default();
config.extensions.insert(MyConfig::default());

// overwrite config default
config.set("my_config.baz_count", "42").unwrap();

// check config state
let my_config = config.extensions.get::<MyConfig>().unwrap();
assert!(my_config.foo_to_bar,);
assert_eq!(my_config.baz_count, 42,);
```

# Note:
Unfortunately associated constants are not currently object-safe, and so this
extends the object-safe [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

<a id="op-28170d1a37a98a8081af3309"></a>
## PREFIX

`assoc_const` · `datafusion_common::config::ConfigExtension::PREFIX` · datafusion-common 55.1.0

```rust
PREFIX
```

Source: `src/config.rs:2249`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Configuration namespace prefix to use

All values under this will be prefixed with `$PREFIX + "."`
