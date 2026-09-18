# `sqlparser::parser::IsLateral`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.IsLateral.json).

<a id="op-8cec6240532782b33dbf6bb2"></a>
## IsLateral

`enum` · `sqlparser::parser::IsLateral` · sqlparser 0.62.0

```rust
enum IsLateral
```

Source: `src/parser/mod.rs:169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates if a table expression is lateral.

<a id="op-3bf04c5a2c6710ad6ecf1428"></a>
## Lateral

`variant` · `sqlparser::parser::IsLateral::Lateral` · sqlparser 0.62.0

```rust
Lateral
```

Source: `src/parser/mod.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression is lateral.

<a id="op-403a8d3977bd36fa03727e6e"></a>
## NotLateral

`variant` · `sqlparser::parser::IsLateral::NotLateral` · sqlparser 0.62.0

```rust
NotLateral
```

Source: `src/parser/mod.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression is not lateral.
