# `datafusion_functions_window::ntile::Ntile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.ntile.Ntile.json).

<a id="op-11610fa470a3bfdc5639da2f"></a>
## Ntile

`struct` · `datafusion_functions_window::ntile::Ntile` · datafusion-functions-window 55.1.0

```rust
struct Ntile
```

Source: `src/ntile.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca28237e2e7d75425848da83"></a>
## default

`function` · `datafusion_functions_window::ntile::Ntile::default` · datafusion-functions-window 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [106, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ntile.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d367df7badbd467a9e9ed63"></a>
## documentation

`function` · `datafusion_functions_window::ntile::Ntile::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cef67f8ae3a8edc6d3724d12"></a>
## eq

`function` · `datafusion_functions_window::ntile::Ntile::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &Ntile) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 17], "end": [75, 26], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ntile.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d6e0c18c29706b495b4979"></a>
## field

`function` · `datafusion_functions_window::ntile::Ntile::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369e9c72cee0f058ea3bce89"></a>
## fmt

`function` · `datafusion_functions_window::ntile::Ntile::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 10], "end": [75, 15], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ntile.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0782db31e112907271eebd0"></a>
## hash

`function` · `datafusion_functions_window::ntile::Ntile::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 32], "end": [75, 36], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ntile.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc02c3a4d29ba7c0b2e641d8"></a>
## limit_effect

`function` · `datafusion_functions_window::ntile::Ntile::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e087129932d62b71cb235427"></a>
## name

`function` · `datafusion_functions_window::ntile::Ntile::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-057215eb24877d2dd1f62d7f"></a>
## new

`function` · `datafusion_functions_window::ntile::Ntile::new` · datafusion-functions-window 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [100, 2], "filename": "src/ntile.rs"}, "trait": null, "trait_path": null}`

Source: `src/ntile.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a new `ntile` function

<a id="op-97160d3b2652050cda7c2b7d"></a>
## partition_evaluator

`function` · `datafusion_functions_window::ntile::Ntile::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d3a05214ee2ee02567d1217"></a>
## signature

`function` · `datafusion_functions_window::ntile::Ntile::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::ntile::Ntile", "path": "Ntile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [157, 2], "filename": "src/ntile.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/ntile.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
