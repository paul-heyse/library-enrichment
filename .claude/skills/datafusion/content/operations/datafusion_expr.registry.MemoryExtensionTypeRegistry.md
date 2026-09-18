# `datafusion_expr::registry::MemoryExtensionTypeRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.MemoryExtensionTypeRegistry.json).

<a id="op-8399103f981849c95be41d7e"></a>
## MemoryExtensionTypeRegistry

`struct` · `datafusion_expr::registry::MemoryExtensionTypeRegistry` · datafusion-expr 55.1.0

```rust
struct MemoryExtensionTypeRegistry
```

Source: `src/registry.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An [`ExtensionTypeRegistry`](../operations/datafusion_expr.registry.ExtensionTypeRegistry.md#op-b25230d4d8f3bc59da15fb85) that uses in memory [`HashMap`](../operations/datafusion_common.HashMap.md#op-12b499036bd6e05ac6fbeefd)s.

<a id="op-2fa9557cf741a04b1746dddc"></a>
## add_extension_type_registration

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::add_extension_type_registration` · datafusion-expr 55.1.0

```rust
fn add_extension_type_registration(&self, extension_type: ExtensionTypeRegistrationRef) -> Result<Option<ExtensionTypeRegistrationRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [588, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistry", "path": "ExtensionTypeRegistry"}, "trait_path": "datafusion_expr::registry::ExtensionTypeRegistry"}`

Source: `src/registry.rs:567`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91d0dc9103d19b097c7aad12"></a>
## all_extension_types

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::all_extension_types` · datafusion-expr 55.1.0

```rust
fn all_extension_types(&self) -> Vec<ExtensionTypeRegistrationRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [543, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a list of all registered types.

<a id="op-2f55ba5c7b9c688c8834e5c4"></a>
## clone

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> MemoryExtensionTypeRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 10], "end": [430, 15], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/registry.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392fd749372bbd02ddbb9928"></a>
## default

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 1], "end": [440, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/registry.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2431583db91d95a75307fa7a"></a>
## extension_type_registration

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::extension_type_registration` · datafusion-expr 55.1.0

```rust
fn extension_type_registration(&self, name: &str) -> Result<ExtensionTypeRegistrationRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [588, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistry", "path": "ExtensionTypeRegistry"}, "trait_path": "datafusion_expr::registry::ExtensionTypeRegistry"}`

Source: `src/registry.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ca7122948eb773ed482d53f"></a>
## extension_type_registrations

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::extension_type_registrations` · datafusion-expr 55.1.0

```rust
fn extension_type_registrations(&self) -> Vec<ExtensionTypeRegistrationRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [588, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistry", "path": "ExtensionTypeRegistry"}, "trait_path": "datafusion_expr::registry::ExtensionTypeRegistry"}`

Source: `src/registry.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c893c04f05e170ecbdfe619a"></a>
## fmt

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 17], "end": [430, 22], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/registry.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-becf525ed399936b57066eda"></a>
## from

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::from` · datafusion-expr 55.1.0

```rust
fn from(value: HashMap<String, ExtensionTypeRegistrationRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 1], "end": [596, 2], "filename": "src/registry.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistration", "path": "ExtensionTypeRegistration"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "hashbrown::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/registry.rs:591`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64c92f06ee81b728212074bd"></a>
## new_empty

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::new_empty` · datafusion-expr 55.1.0

```rust
fn new_empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [543, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an empty [MemoryExtensionTypeRegistry](../operations/datafusion_expr.registry.MemoryExtensionTypeRegistry.md#op-8399103f981849c95be41d7e).

<a id="op-51b8c97eef2c0099d2d50fe5"></a>
## new_with_canonical_extension_types

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::new_with_canonical_extension_types` · datafusion-expr 55.1.0

```rust
fn new_with_canonical_extension_types() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [543, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Pre-registers the [canonical extension types](https://arrow.apache.org/docs/format/CanonicalExtensions.html)
in the extension type registry.

<a id="op-5d1a7aebcba44f53aa0de1df"></a>
## new_with_types

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::new_with_types` · datafusion-expr 55.1.0

```rust
fn new_with_types(types: impl IntoIterator<Item = ExtensionTypeRegistrationRef>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 1], "end": [543, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new [MemoryExtensionTypeRegistry](../operations/datafusion_expr.registry.MemoryExtensionTypeRegistry.md#op-8399103f981849c95be41d7e) with the provided `types`.

# Errors

Returns an error if one of the `types` is a native type.

<a id="op-56689e6a7826fd8583876464"></a>
## remove_extension_type_registration

`function` · `datafusion_expr::registry::MemoryExtensionTypeRegistry::remove_extension_type_registration` · datafusion-expr 55.1.0

```rust
fn remove_extension_type_registration(&self, name: &str) -> Result<Option<ExtensionTypeRegistrationRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::MemoryExtensionTypeRegistry", "path": "MemoryExtensionTypeRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [588, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistry", "path": "ExtensionTypeRegistry"}, "trait_path": "datafusion_expr::registry::ExtensionTypeRegistry"}`

Source: `src/registry.rs:578`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
