# `datafusion_ffi::config::extension_options::FFI_ExtensionOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.config.extension_options.FFI_ExtensionOptions.json).

<a id="op-e89582d8425fd1a15f3d4425"></a>
## FFI_ExtensionOptions

`struct` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions` · datafusion-ffi 55.1.0

```rust
struct FFI_ExtensionOptions
```

Source: `src/config/extension_options.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

A stable struct for sharing [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db) across FFI boundaries.

Unlike other FFI structs in this crate, we do not construct a foreign
variant of this object. This is due to the typical method for interacting
with extension options is by creating a local struct of your concrete type.
To support this methodology use the `to_extension` method instead.

When using [`FFI_ExtensionOptions`](../operations/datafusion_ffi.config.extension_options.FFI_ExtensionOptions.md#op-e89582d8425fd1a15f3d4425) with multiple extensions, all extension
values are stored on a single [`FFI_ExtensionOptions`](../operations/datafusion_ffi.config.extension_options.FFI_ExtensionOptions.md#op-e89582d8425fd1a15f3d4425) object. The keys
are stored with the full path prefix to avoid overwriting values when using
multiple extensions.

<a id="op-a30fd9cfd680093a0d106d7d"></a>
## PREFIX

`assoc_const` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::PREFIX` · datafusion-ffi 55.1.0

```rust
PREFIX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [160, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigExtension", "path": "ConfigExtension"}, "trait_path": "datafusion_common::config::ConfigExtension"}`

Source: `src/config/extension_options.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1695a806499a1d12ee3c507"></a>
## add_config

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::add_config` · datafusion-ffi 55.1.0

```rust
fn add_config<C: ConfigExtension>(&mut self, config: &C) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [244, 2], "filename": "src/config/extension_options.rs"}, "trait": null, "trait_path": null}`

Source: `src/config/extension_options.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Add all of the values in a concrete configuration extension to the
FFI variant. This is safe to call on either side of the FFI
boundary.

<a id="op-2a8def3e6ba0415e5aa7f104"></a>
## as_any

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::as_any` · datafusion-ffi 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [196, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ExtensionOptions", "path": "ExtensionOptions"}, "trait_path": "datafusion_common::config::ExtensionOptions"}`

Source: `src/config/extension_options.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-688ac8253709e353ba933175"></a>
## as_any_mut

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::as_any_mut` · datafusion-ffi 55.1.0

```rust
fn as_any_mut(&mut self) -> &mut dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [196, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ExtensionOptions", "path": "ExtensionOptions"}, "trait_path": "datafusion_common::config::ExtensionOptions"}`

Source: `src/config/extension_options.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-159d315c931e418ad9aa277e"></a>
## clone

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::clone` · datafusion-ffi 55.1.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 1], "end": [155, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config/extension_options.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-346a354f46385b91bee2565e"></a>
## cloned

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::cloned` · datafusion-ffi 55.1.0

```rust
fn cloned(&self) -> Box<dyn ExtensionOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [196, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ExtensionOptions", "path": "ExtensionOptions"}, "trait_path": "datafusion_common::config::ExtensionOptions"}`

Source: `src/config/extension_options.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8afc3f21f3d7b550f482e0f"></a>
## cloned

`struct_field` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::cloned` · datafusion-ffi 55.1.0

```rust
cloned: unsafe fn(&Self) -> FFI_ExtensionOptions
```

Source: `src/config/extension_options.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Return a deep clone of this [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

<a id="op-fbe5bc2f9539105cc2ec4a98"></a>
## default

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::default` · datafusion-ffi 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [129, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config/extension_options.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05c4ebc744686743fd9de376"></a>
## drop

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::drop` · datafusion-ffi 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [149, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/config/extension_options.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5ff7db04bfdc84b9b585e7d"></a>
## entries

`struct_field` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::entries` · datafusion-ffi 55.1.0

```rust
entries: unsafe fn(&Self) -> stabby::vec::Vec<(stabby::string::String, stabby::string::String)>
```

Source: `src/config/extension_options.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Returns the [`ConfigEntry`](../operations/datafusion_common.config.ConfigEntry.md#op-e610e9614a810518fe36454b) stored in this [`ExtensionOptions`](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db)

<a id="op-f0f7e875e108efb4fcc026d3"></a>
## entries

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::entries` · datafusion-ffi 55.1.0

```rust
fn entries(&self) -> Vec<ConfigEntry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [196, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ExtensionOptions", "path": "ExtensionOptions"}, "trait_path": "datafusion_common::config::ExtensionOptions"}`

Source: `src/config/extension_options.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38d070acdee6cf9b3e384ef2"></a>
## fmt

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config/extension_options.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41d8eb9a50fe3697055f6187"></a>
## from

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::from` · datafusion-ffi 55.1.0

```rust
fn from(options: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [143, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/config/extension_options.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9ffa7c1cc791b92d3e5d660"></a>
## merge

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::merge` · datafusion-ffi 55.1.0

```rust
fn merge(&mut self, other: &FFI_ExtensionOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [244, 2], "filename": "src/config/extension_options.rs"}, "trait": null, "trait_path": null}`

Source: `src/config/extension_options.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Merge another `FFI_ExtensionOptions` configurations into this one.
This is safe to call on either side of the FFI boundary.

<a id="op-fc9ba0f1e0e8aa7678b6e21d"></a>
## private_data

`struct_field` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::private_data` · datafusion-ffi 55.1.0

```rust
private_data: *mut std::ffi::c_void
```

Source: `src/config/extension_options.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Internal data. This is only to be accessed by the provider of the options.

<a id="op-c26f7014924404db85738eb0"></a>
## release

`struct_field` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::release` · datafusion-ffi 55.1.0

```rust
release: unsafe fn(&mut Self)
```

Source: `src/config/extension_options.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Release the memory of the private data when it is no longer being used.

<a id="op-75b2d5e0224f640e23b0beae"></a>
## set

`struct_field` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::set` · datafusion-ffi 55.1.0

```rust
set: unsafe fn(&mut Self, stabby::str::Str<'_>, stabby::str::Str<'_>) -> util::FFI_Result<()>
```

Source: `src/config/extension_options.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Set the given `key`, `value` pair

<a id="op-b059aa4760a54bbb4e0ff9c1"></a>
## set

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::set` · datafusion-ffi 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [196, 2], "filename": "src/config/extension_options.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ExtensionOptions", "path": "ExtensionOptions"}, "trait_path": "datafusion_common::config::ExtensionOptions"}`

Source: `src/config/extension_options.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a210d135ef1669405346c8e"></a>
## to_extension

`function` · `datafusion_ffi::config::extension_options::FFI_ExtensionOptions::to_extension` · datafusion-ffi 55.1.0

```rust
fn to_extension<C: ConfigExtension + Default>(&self) -> Result<C>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::config::extension_options::FFI_ExtensionOptions", "path": "FFI_ExtensionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [244, 2], "filename": "src/config/extension_options.rs"}, "trait": null, "trait_path": null}`

Source: `src/config/extension_options.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Create a concrete extension type from the FFI variant.
This is safe to call on either side of the FFI boundary.
