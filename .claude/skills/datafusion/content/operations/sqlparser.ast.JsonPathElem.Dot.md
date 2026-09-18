# `sqlparser::ast::JsonPathElem::Dot`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.JsonPathElem.Dot.json).

<a id="op-b4f10d4f9e1ba7324e4fb3e5"></a>
## key

`struct_field` · `sqlparser::ast::JsonPathElem::Dot::key` · sqlparser 0.62.0

```rust
key: String
```

Source: `src/ast/mod.rs:682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The object key text (without quotes).

<a id="op-79003e283e1a74907c891906"></a>
## quoted

`struct_field` · `sqlparser::ast::JsonPathElem::Dot::quoted` · sqlparser 0.62.0

```rust
quoted: bool
```

Source: `src/ast/mod.rs:684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the key was quoted in the source.
