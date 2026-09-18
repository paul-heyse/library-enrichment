# `datafusion_expr::expr::Lambda`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Lambda.json).

<a id="op-2eab7bf8a4f8a5e91acad843"></a>
## Lambda

`struct` · `datafusion_expr::expr::Lambda` · datafusion-expr 55.1.0

```rust
struct Lambda
```

Source: `src/expr.rs:1472`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A Lambda expression with a set of parameters names and a body

<a id="op-38ae3460f44032e0b0ec23b9"></a>
## body

`struct_field` · `datafusion_expr::expr::Lambda::body` · datafusion-expr 55.1.0

```rust
body: Box<Expr>
```

Source: `src/expr.rs:1476`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The body expression

<a id="op-84735046713042d9866be52d"></a>
## clone

`function` · `datafusion_expr::expr::Lambda::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Lambda
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1471, 10], "end": [1471, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3bdf6f9013f0abbe2f953d1"></a>
## eq

`function` · `datafusion_expr::expr::Lambda::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Lambda) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1471, 17], "end": [1471, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36132f45844d8e1694f9b9b"></a>
## fmt

`function` · `datafusion_expr::expr::Lambda::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1471, 50], "end": [1471, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f8894489aef5bb0f70585df"></a>
## hash

`function` · `datafusion_expr::expr::Lambda::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1471, 44], "end": [1471, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27f647c8e6cfe36057fdbe35"></a>
## new

`function` · `datafusion_expr::expr::Lambda::new` · datafusion-expr 55.1.0

```rust
fn new(params: Vec<String>, body: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1479, 1], "end": [1487, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1481`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new lambda expression

<a id="op-aa0037bb49afb1b112cba70e"></a>
## params

`struct_field` · `datafusion_expr::expr::Lambda::params` · datafusion-expr 55.1.0

```rust
params: Vec<String>
```

Source: `src/expr.rs:1474`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The parameters names

<a id="op-936f2f4c250feebd5bea4ac3"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Lambda::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Lambda) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Lambda", "path": "Lambda"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1471, 32], "end": [1471, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
