# `sqlparser::ast::FunctionArg::ExprNamed`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArg.ExprNamed.json).

<a id="op-758b727940e03ca7475a549e"></a>
## arg

`struct_field` · `sqlparser::ast::FunctionArg::ExprNamed::arg` · sqlparser 0.62.0

```rust
arg: FunctionArgExpr
```

Source: `src/ast/mod.rs:7936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The argument expression or wildcard form.

<a id="op-6d4fc2876042a4037371ab00"></a>
## name

`struct_field` · `sqlparser::ast::FunctionArg::ExprNamed::name` · sqlparser 0.62.0

```rust
name: Expr
```

Source: `src/ast/mod.rs:7934`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression used as the argument name.

<a id="op-22ed8a13b6b646fe201b3855"></a>
## operator

`struct_field` · `sqlparser::ast::FunctionArg::ExprNamed::operator` · sqlparser 0.62.0

```rust
operator: FunctionArgOperator
```

Source: `src/ast/mod.rs:7938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operator separating name and value.
