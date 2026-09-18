# `datafusion_common::config::Extensions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.Extensions.json).

<a id="op-4cc8f359023731bc0599b465"></a>
## Extensions

`struct` · `datafusion_common::config::Extensions` · datafusion-common 55.1.0

```rust
struct Extensions
```

Source: `src/config.rs:2281`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A type-safe container for [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c)

<a id="op-1f58e79d15d4d5de19bd590a"></a>
## clone

`function` · `datafusion_common::config::Extensions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2280, 26], "end": [2280, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:2280`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a8d41add80ed38745c0a7ae"></a>
## default

`function` · `datafusion_common::config::Extensions::default` · datafusion-common 55.1.0

```rust
fn default() -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2280, 17], "end": [2280, 24], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:2280`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93189050fe0f7e1cc2a76ad1"></a>
## fmt

`function` · `datafusion_common::config::Extensions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2280, 10], "end": [2280, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:2280`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c28a3562d918459d4ebcd40"></a>
## get

`function` · `datafusion_common::config::Extensions::get` · datafusion-common 55.1.0

```rust
fn get<T: ConfigExtension>(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 1], "end": [2314, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2297`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieves the extension of the given type if any

<a id="op-c593a927c6db9245f7125597"></a>
## get_mut

`function` · `datafusion_common::config::Extensions::get_mut` · datafusion-common 55.1.0

```rust
fn get_mut<T: ConfigExtension>(&mut self) -> Option<&mut T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 1], "end": [2314, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2302`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieves the extension of the given type if any

<a id="op-7b82f2b1c0fb03c21d2ca5a2"></a>
## insert

`function` · `datafusion_common::config::Extensions::insert` · datafusion-common 55.1.0

```rust
fn insert<T: ConfigExtension>(&mut self, extension: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 1], "end": [2314, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2290`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Registers a [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) with this [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4)

<a id="op-3ae957d3ee290a21e647d78e"></a>
## iter

`function` · `datafusion_common::config::Extensions::iter` · datafusion-common 55.1.0

```rust
fn iter(&self) -> impl Iterator<Item = (&'static str, &Box<dyn ExtensionOptions>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 1], "end": [2314, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2309`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Iterates all the config extension entries yielding their prefix and their
[ExtensionOptions](../operations/datafusion_common.config.ExtensionOptions.md#op-cf0c51673459a43f486d37db) implementation.

<a id="op-7ffd0c4d986c2bcbe1dd89ac"></a>
## new

`function` · `datafusion_common::config::Extensions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2283, 1], "end": [2314, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:2285`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new, empty [`Extensions`](../operations/datafusion_common.config.Extensions.md#op-4cc8f359023731bc0599b465)
