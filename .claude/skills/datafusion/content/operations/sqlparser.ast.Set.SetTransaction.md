# `sqlparser::ast::Set::SetTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Set.SetTransaction.json).

<a id="op-ccc72a416ecbf3ebf1b75dd5"></a>
## modes

`struct_field` · `sqlparser::ast::Set::SetTransaction::modes` · sqlparser 0.62.0

```rust
modes: Vec<TransactionMode>
```

Source: `src/ast/mod.rs:3339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Transaction modes (e.g., ISOLATION LEVEL, READ ONLY).

<a id="op-6122f12f9ed7b768a620d0e6"></a>
## session

`struct_field` · `sqlparser::ast::Set::SetTransaction::session` · sqlparser 0.62.0

```rust
session: bool
```

Source: `src/ast/mod.rs:3343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the `SESSION` keyword was used.

<a id="op-1d4a00f39ed3ccf52b98255f"></a>
## snapshot

`struct_field` · `sqlparser::ast::Set::SetTransaction::snapshot` · sqlparser 0.62.0

```rust
snapshot: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:3341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional snapshot value for transaction snapshot control.
