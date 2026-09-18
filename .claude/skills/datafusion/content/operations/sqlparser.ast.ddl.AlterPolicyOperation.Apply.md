# `sqlparser::ast::ddl::AlterPolicyOperation::Apply`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterPolicyOperation.Apply.json).

<a id="op-705ae5e60b88f4d1e2d4a00f"></a>
## to

`struct_field` · `sqlparser::ast::ddl::AlterPolicyOperation::Apply::to` · sqlparser 0.62.0

```rust
to: Option<Vec<Owner>>
```

Source: `src/ast/ddl.rs:554`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of owners the policy applies to.

<a id="op-a4b3563e9f6b05eb63ed6c03"></a>
## using

`struct_field` · `sqlparser::ast::ddl::AlterPolicyOperation::Apply::using` · sqlparser 0.62.0

```rust
using: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `USING` expression for the policy.

<a id="op-686852c61e8cbf1576457d35"></a>
## with_check

`struct_field` · `sqlparser::ast::ddl::AlterPolicyOperation::Apply::with_check` · sqlparser 0.62.0

```rust
with_check: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:558`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `WITH CHECK` expression for the policy.
