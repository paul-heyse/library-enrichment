# `datafusion_common::extensions_options`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.extensions_options.json).

<a id="op-e1604c36c8db03b72814ca67"></a>
## extensions_options

`macro` · `datafusion_common::extensions_options` · datafusion-common 55.1.0

```rust
macro_rules! extensions_options
```

Source: `src/config.rs:2557`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience macro to create [`ExtensionsOptions`].

The created structure implements the following traits:

- [`Clone`]
- [`Debug`]
- [`Default`]
- [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

# Usage
The syntax is:

```text
extensions_options! {
     /// Struct docs (optional).
    [<vis>] struct <StructName> {
        /// Field docs (optional)
        [<vis>] <field_name>: <field_type>, default = <default_value>

        ... more fields
    }
}
```

The placeholders are:
- `[<vis>]`: Optional visibility modifier like `pub` or `pub(crate)`.
- `<StructName>`: Struct name like `MyStruct`.
- `<field_name>`: Field name like `my_field`.
- `<field_type>`: Field type like `u8`.
- `<default_value>`: Default value matching the field type like `42`.

# Example
See also a full example on the [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) documentation

```
use datafusion_common::extensions_options;

extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
}
```


[`Debug`]: std::fmt::Debug
[`ExtensionsOptions`]: crate::config::ExtensionOptions

Unresolved upstream links (retained, not inferred): `std::fmt::Debug`, ``Clone``, ``Default``.
