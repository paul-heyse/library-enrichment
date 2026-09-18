# `datafusion_expr_common::signature::Arity`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.Arity.json).

<a id="op-4e3296b09b8a7645dcc0561e"></a>
## Arity

`enum` · `datafusion_expr_common::signature::Arity` · datafusion-expr-common 55.1.0

```rust
enum Arity
```

Source: `src/signature.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Represents the arity (number of arguments) of a function signature

<a id="op-3e6d3f9e53f9dd9403da6d0f"></a>
## Fixed

`variant` · `datafusion_expr_common::signature::Arity::Fixed` · datafusion-expr-common 55.1.0

```rust
Fixed
```

Source: `src/signature.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Fixed number of arguments

<a id="op-0d7648d71b2d14bcfa7b0e37"></a>
## Variable

`variant` · `datafusion_expr_common::signature::Arity::Variable` · datafusion-expr-common 55.1.0

```rust
Variable
```

Source: `src/signature.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Variable number of arguments (e.g., Variadic, VariadicAny, UserDefined)

<a id="op-8ae6ea4c6f9cdf989d4262e7"></a>
## clone

`function` · `datafusion_expr_common::signature::Arity::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Arity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Arity", "path": "Arity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 17], "end": [91, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1220c76d3c8d6603cb33aa4f"></a>
## eq

`function` · `datafusion_expr_common::signature::Arity::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Arity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Arity", "path": "Arity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 30], "end": [91, 39], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a84b612e5f63e93e8ed07a1"></a>
## fmt

`function` · `datafusion_expr_common::signature::Arity::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Arity", "path": "Arity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 10], "end": [91, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
