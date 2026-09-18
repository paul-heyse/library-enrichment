# `datafusion_expr::registry::ExtensionTypeRegistration`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.registry.ExtensionTypeRegistration.json).

<a id="op-7a56e431478a615d9d67405c"></a>
## ExtensionTypeRegistration

`struct` · `datafusion_expr::registry::ExtensionTypeRegistration` · datafusion-expr 55.1.0

```rust
struct ExtensionTypeRegistration
```

Source: `src/registry.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The registration of an extension type. Implementations of this trait are responsible for
*creating* instances of [`DFExtensionType`] that represent the entire semantics of an extension
type.

# Why do we need a Registration?

A good question is why this trait is even necessary. Why not directly register the
[`DFExtensionType`] in a registry?

While this works for extension types requiring no additional metadata (e.g., `arrow.uuid`), it
does not work for more complex extension types with metadata. For example, consider an extension
type `custom.shortened(n)` that aims to short the pretty-printing string to `n` characters.
Here, `n` is a parameter of the extension type and should be a field in the struct that
implements the [`DFExtensionType`]. The job of the registration is to read the metadata from the
field and create the corresponding [`DFExtensionType`] instance with the correct `n` set.

[`DFExtensionType`]: datafusion_common::types::DFExtensionType

<a id="op-3d156c2d9927ada0c9d4df84"></a>
## create_df_extension_type

`function` · `datafusion_expr::registry::ExtensionTypeRegistration::create_df_extension_type` · datafusion-expr 55.1.0

```rust
fn create_df_extension_type(&self, storage_type: &DataType, metadata: Option<&str>) -> Result<DFExtensionTypeRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistration", "path": "ExtensionTypeRegistration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [419, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an extension type instance from the optional metadata. The name of the extension
type is not a parameter as it's already defined by the registration itself.

<a id="op-4ea6ce2c1ebd213628896307"></a>
## fmt

`function` · `datafusion_expr::registry::ExtensionTypeRegistration::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistration", "path": "ExtensionTypeRegistration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [421, 1], "end": [427, 2], "filename": "src/registry.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/registry.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9c57d47412a1335950dd699"></a>
## new_arc

`function` · `datafusion_expr::registry::ExtensionTypeRegistration::new_arc` · datafusion-expr 55.1.0

```rust
fn new_arc(name: impl Into<String>, factory: impl Fn(&DataType, Option<&str>) -> Result<DFExtensionTypeRef> + Send + Sync + 'static) -> ExtensionTypeRegistrationRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistration", "path": "ExtensionTypeRegistration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 1], "end": [399, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new registration for an extension type. The factory is required to validate that
the storage [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) is compatible with the extension type.

<a id="op-3d0e1eae8f1de265588fe6b9"></a>
## type_name

`function` · `datafusion_expr::registry::ExtensionTypeRegistration::type_name` · datafusion-expr 55.1.0

```rust
fn type_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::registry::ExtensionTypeRegistration", "path": "ExtensionTypeRegistration"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [401, 1], "end": [419, 2], "filename": "src/registry.rs"}, "trait": null, "trait_path": null}`

Source: `src/registry.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the extension type.

This name will be used to find the correct [ExtensionTypeRegistration](../operations/datafusion_expr.registry.ExtensionTypeRegistration.md#op-7a56e431478a615d9d67405c) when an extension
type is encountered.
