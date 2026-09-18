# `datafusion_substrait::extensions::Extensions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.extensions.Extensions.json).

<a id="op-721f87201c35d0521d8e800c"></a>
## Extensions

`struct` · `datafusion_substrait::extensions::Extensions` · datafusion-substrait 55.1.0

```rust
struct Extensions
```

Source: `src/extensions.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Substrait uses [SimpleExtensions](https://substrait.io/extensions/#simple-extensions) to define
behavior of plans in addition to what's supported directly by the protobuf definitions.
That includes functions, but also provides support for custom types and variations for existing
types. This structs facilitates the use of these extensions in DataFusion.
TODO: DF doesn't yet use extensions for type variations <https://github.com/apache/datafusion/issues/11544>
TODO: DF doesn't yet provide valid extensionUris <https://github.com/apache/datafusion/issues/11545>

<a id="op-789cbe9327a8a6400798d80e"></a>
## Error

`assoc_type` · `datafusion_substrait::extensions::Extensions::Error` · datafusion-substrait 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [113, 2], "filename": "src/extensions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "substrait::proto::extensions::SimpleExtensionDeclaration", "path": "SimpleExtensionDeclaration"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/extensions.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adc4f9068e4f816876ed461a"></a>
## default

`function` · `datafusion_substrait::extensions::Extensions::default` · datafusion-substrait 55.1.0

```rust
fn default() -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 17], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extensions.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6aece9ed5acb61bba813535f"></a>
## eq

`function` · `datafusion_substrait::extensions::Extensions::eq` · datafusion-substrait 55.1.0

```rust
fn eq(&self, other: &Extensions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 26], "end": [30, 35], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extensions.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0253b11efaa7faee9c09ab"></a>
## fmt

`function` · `datafusion_substrait::extensions::Extensions::fmt` · datafusion-substrait 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 19], "end": [30, 24], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extensions.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b543fa07c58c07df55166526"></a>
## functions

`struct_field` · `datafusion_substrait::extensions::Extensions::functions` · datafusion-substrait 55.1.0

```rust
functions: datafusion::common::HashMap<u32, String>
```

Source: `src/extensions.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5a796417697f3cb0d229bb5"></a>
## register_function

`function` · `datafusion_substrait::extensions::Extensions::register_function` · datafusion-substrait 55.1.0

```rust
fn register_function(&mut self, function_name: &str) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [79, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Registers a function and returns the anchor (reference) to it. If the function has already
been registered, it returns the existing anchor.
Function names are case-insensitive (converted to lowercase).

<a id="op-4ac0cf68498f8e9108bbe413"></a>
## register_type

`function` · `datafusion_substrait::extensions::Extensions::register_type` · datafusion-substrait 55.1.0

```rust
fn register_type(&mut self, type_name: &str) -> u32
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [79, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Registers a type and returns the anchor (reference) to it. If the type has already
been registered, it returns the existing anchor.

<a id="op-4c43d7e09fabad765396d285"></a>
## try_from

`function` · `datafusion_substrait::extensions::Extensions::try_from` · datafusion-substrait 55.1.0

```rust
fn try_from(value: &Vec<SimpleExtensionDeclaration>) -> datafusion::common::Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_substrait::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [113, 2], "filename": "src/extensions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "substrait::proto::extensions::SimpleExtensionDeclaration", "path": "SimpleExtensionDeclaration"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/extensions.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-288740b547453c6733d4894e"></a>
## type_variations

`struct_field` · `datafusion_substrait::extensions::Extensions::type_variations` · datafusion-substrait 55.1.0

```rust
type_variations: datafusion::common::HashMap<u32, String>
```

Source: `src/extensions.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34a09609d3f77a5d7d55ce69"></a>
## types

`struct_field` · `datafusion_substrait::extensions::Extensions::types` · datafusion-substrait 55.1.0

```rust
types: datafusion::common::HashMap<u32, String>
```

Source: `src/extensions.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
