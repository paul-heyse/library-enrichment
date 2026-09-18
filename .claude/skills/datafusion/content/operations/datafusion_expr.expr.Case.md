# `datafusion_expr::expr::Case`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Case.json).

<a id="op-358933dad32fc3f4ce9d3573"></a>
## Case

`struct` · `datafusion_expr::expr::Case` · datafusion-expr 55.1.0

```rust
struct Case
```

Source: `src/expr.rs:866`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

CASE expression

The CASE expression is similar to a series of nested if/else and there are two forms that
can be used. The first form consists of a series of boolean "when" expressions with
corresponding "then" expressions, and an optional "else" expression.

```text
CASE WHEN condition THEN result
     [WHEN ...]
     [ELSE result]
END
```

The second form uses a base expression and then a series of "when" clauses that match on a
literal value.

```text
CASE expression
    WHEN value THEN result
    [WHEN ...]
    [ELSE result]
END
```

<a id="op-d49b87d334d404c1af52262e"></a>
## clone

`function` · `datafusion_expr::expr::Case::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Case
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 10], "end": [865, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:865`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b39f3b097f8c23f2395e875"></a>
## else_expr

`struct_field` · `datafusion_expr::expr::Case::else_expr` · datafusion-expr 55.1.0

```rust
else_expr: Option<Box<Expr>>
```

Source: `src/expr.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional "else" expression

<a id="op-10b869bbd760f8ebccf84943"></a>
## eq

`function` · `datafusion_expr::expr::Case::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Case) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 24], "end": [865, 33], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:865`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6024f50f8acfa85ab3d91cd5"></a>
## expr

`struct_field` · `datafusion_expr::expr::Case::expr` · datafusion-expr 55.1.0

```rust
expr: Option<Box<Expr>>
```

Source: `src/expr.rs:868`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional base expression that can be compared to literal values in the "when" expressions

<a id="op-9b4f7c343fece08cbb1c8b95"></a>
## fmt

`function` · `datafusion_expr::expr::Case::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 17], "end": [865, 22], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:865`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dad750e2daaf0bde91dee919"></a>
## hash

`function` · `datafusion_expr::expr::Case::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 51], "end": [865, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:865`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa21fd13b398041a7602ca0b"></a>
## new

`function` · `datafusion_expr::expr::Case::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Option<Box<Expr>>, when_then_expr: Vec<(Box<Expr>, Box<Expr>)>, else_expr: Option<Box<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [875, 1], "end": [888, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Case expression

<a id="op-b074f5b286ef5d0b8136a355"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Case::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Case) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Case", "path": "Case"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 39], "end": [865, 49], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:865`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7549172d57b33c0ed2eee8e4"></a>
## when_then_expr

`struct_field` · `datafusion_expr::expr::Case::when_then_expr` · datafusion-expr 55.1.0

```rust
when_then_expr: Vec<(Box<Expr>, Box<Expr>)>
```

Source: `src/expr.rs:870`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

One or more when/then expressions
