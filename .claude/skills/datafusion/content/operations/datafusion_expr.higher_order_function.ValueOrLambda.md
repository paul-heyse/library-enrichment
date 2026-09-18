# `datafusion_expr::higher_order_function::ValueOrLambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.ValueOrLambda.json).

<a id="op-b2f3d7652f730f6b00c39f6e"></a>
## ValueOrLambda

`enum` · `datafusion_expr::higher_order_function::ValueOrLambda` · datafusion-expr 55.1.0

```rust
enum ValueOrLambda<V, L>
```

Source: `src/higher_order_function.rs:487`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An argument to a higher order function

<a id="op-d68d7d1b330c93c8815d9782"></a>
## Lambda

`variant` · `datafusion_expr::higher_order_function::ValueOrLambda::Lambda` · datafusion-expr 55.1.0

```rust
Lambda
```

Source: `src/higher_order_function.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A lambda with associated data

<a id="op-a231926a173790a29ccf9c5b"></a>
## Value

`variant` · `datafusion_expr::higher_order_function::ValueOrLambda::Value` · datafusion-expr 55.1.0

```rust
Value
```

Source: `src/higher_order_function.rs:489`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A value with associated data

<a id="op-93ea705d87d2fea4eecbd995"></a>
## clone

`function` · `datafusion_expr::higher_order_function::ValueOrLambda::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ValueOrLambda<V, L>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}, {"type": {"generic": "L"}}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::ValueOrLambda", "path": "ValueOrLambda"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "L"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 10], "end": [486, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb5f41aeadd1b4fce5bf3028"></a>
## eq

`function` · `datafusion_expr::higher_order_function::ValueOrLambda::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ValueOrLambda<V, L>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}, {"type": {"generic": "L"}}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::ValueOrLambda", "path": "ValueOrLambda"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "L"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 24], "end": [486, 33], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/higher_order_function.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fb3c9309fdd2b2500219dd8"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::ValueOrLambda::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}, {"type": {"generic": "L"}}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::ValueOrLambda", "path": "ValueOrLambda"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "L"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 17], "end": [486, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-681edb9ec196e2185a4ad2e5"></a>
## hash

`function` · `datafusion_expr::higher_order_function::ValueOrLambda::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}, {"type": {"generic": "L"}}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::ValueOrLambda", "path": "ValueOrLambda"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "$crate::hash::Hash"}}}], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "$crate::hash::Hash"}}}], "default": null, "is_synthetic": false}}, "name": "L"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 51], "end": [486, 55], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/higher_order_function.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6eb296b042e849a0aa1dc41"></a>
## partial_cmp

`function` · `datafusion_expr::higher_order_function::ValueOrLambda::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &ValueOrLambda<V, L>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}, {"type": {"generic": "L"}}], "constraints": []}}, "id": "datafusion_expr::higher_order_function::ValueOrLambda", "path": "ValueOrLambda"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "$crate::cmp::PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "V"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "$crate::cmp::PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "L"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [486, 39], "end": [486, 49], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/higher_order_function.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
