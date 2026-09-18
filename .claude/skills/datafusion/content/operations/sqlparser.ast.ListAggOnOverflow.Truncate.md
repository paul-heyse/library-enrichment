# `sqlparser::ast::ListAggOnOverflow::Truncate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ListAggOnOverflow.Truncate.json).

<a id="op-99b846b5ab8f6d128517b406"></a>
## filler

`struct_field` · `sqlparser::ast::ListAggOnOverflow::Truncate::filler` · sqlparser 0.62.0

```rust
filler: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:8398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional filler expression used when truncating.

<a id="op-8724e21a923f9ecebfff754c"></a>
## with_count

`struct_field` · `sqlparser::ast::ListAggOnOverflow::Truncate::with_count` · sqlparser 0.62.0

```rust
with_count: bool
```

Source: `src/ast/mod.rs:8400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to include a count when truncating.
