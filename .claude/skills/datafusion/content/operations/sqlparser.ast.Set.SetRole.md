# `sqlparser::ast::Set::SetRole`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Set.SetRole.json).

<a id="op-6e2304e070ad0df8431fe83c"></a>
## context_modifier

`struct_field` · `sqlparser::ast::Set::SetRole::context_modifier` · sqlparser 0.62.0

```rust
context_modifier: Option<ContextModifier>
```

Source: `src/ast/mod.rs:3300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Non-ANSI optional identifier to inform if the role is defined inside the current session (`SESSION`) or transaction (`LOCAL`).

<a id="op-6a28352070d1ddb3833747be"></a>
## role_name

`struct_field` · `sqlparser::ast::Set::SetRole::role_name` · sqlparser 0.62.0

```rust
role_name: Option<Ident>
```

Source: `src/ast/mod.rs:3302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Role name. If NONE is specified, then the current role name is removed.
