# `datafusion_expr::function::Hint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.Hint.json).

<a id="op-b1dfe2178f9477e6effd0e05"></a>
## Hint

`enum` · `datafusion_expr::function::Hint` · datafusion-expr 55.1.0

```rust
enum Hint
```

Source: `src/function.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db29979ab935052c6ceff5f2"></a>
## AcceptsSingular

`variant` · `datafusion_expr::function::Hint::AcceptsSingular` · datafusion-expr 55.1.0

```rust
AcceptsSingular
```

Source: `src/function.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates the argument can be converted to an array of length 1

<a id="op-cac2240ea93a71921c3bc0b4"></a>
## Pad

`variant` · `datafusion_expr::function::Hint::Pad` · datafusion-expr 55.1.0

```rust
Pad
```

Source: `src/function.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates the argument needs to be padded if it is scalar

<a id="op-ef98840a680f5121835ecc63"></a>
## clone

`function` · `datafusion_expr::function::Hint::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Hint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::function::Hint", "path": "Hint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/function.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec76c7e7ca5ed7c779900022"></a>
## fmt

`function` · `datafusion_expr::function::Hint::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::function::Hint", "path": "Hint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
