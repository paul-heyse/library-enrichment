# `sqlparser::parser::WildcardExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.WildcardExpr.json).

<a id="op-aa844245b915f07730245410"></a>
## WildcardExpr

`enum` · `sqlparser::parser::WildcardExpr` · sqlparser 0.62.0

```rust
enum WildcardExpr
```

Source: `src/parser/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a wildcard expression used in SELECT lists.

<a id="op-768e6db932ddaa5af2327592"></a>
## Expr

`variant` · `sqlparser::parser::WildcardExpr::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/parser/mod.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A specific expression used instead of a wildcard.

<a id="op-f418501eee32c3d5efd7bc75"></a>
## QualifiedWildcard

`variant` · `sqlparser::parser::WildcardExpr::QualifiedWildcard` · sqlparser 0.62.0

```rust
QualifiedWildcard
```

Source: `src/parser/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A qualified wildcard like `table.*`.

<a id="op-7f0f7f8c552cb51dc5241010"></a>
## Wildcard

`variant` · `sqlparser::parser::WildcardExpr::Wildcard` · sqlparser 0.62.0

```rust
Wildcard
```

Source: `src/parser/mod.rs:183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unqualified `*` wildcard.
