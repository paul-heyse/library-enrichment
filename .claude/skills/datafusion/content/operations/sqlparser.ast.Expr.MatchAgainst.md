# `sqlparser::ast::Expr::MatchAgainst`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.MatchAgainst.json).

<a id="op-3afad1cdb4db72343b7467cf"></a>
## columns

`struct_field` · `sqlparser::ast::Expr::MatchAgainst::columns` · sqlparser 0.62.0

```rust
columns: Vec<ObjectName>
```

Source: `src/ast/mod.rs:1333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`(<col>, <col>, ...)`.

<a id="op-3dbb86896df30338941e8a23"></a>
## match_value

`struct_field` · `sqlparser::ast::Expr::MatchAgainst::match_value` · sqlparser 0.62.0

```rust
match_value: ValueWithSpan
```

Source: `src/ast/mod.rs:1335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<expr>`.

<a id="op-6b17cab439a0a1e14d8f3973"></a>
## opt_search_modifier

`struct_field` · `sqlparser::ast::Expr::MatchAgainst::opt_search_modifier` · sqlparser 0.62.0

```rust
opt_search_modifier: Option<SearchModifier>
```

Source: `src/ast/mod.rs:1337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<search modifier>`
