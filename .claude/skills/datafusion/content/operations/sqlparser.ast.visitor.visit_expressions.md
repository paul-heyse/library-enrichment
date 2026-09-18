# `sqlparser::ast::visitor::visit_expressions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_expressions.json).

<a id="op-4715fda2b83d63609f15713c"></a>
## visit_expressions

`function` · `sqlparser::ast::visitor::visit_expressions` · sqlparser 0.62.0

```rust
fn visit_expressions<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::Expr) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure on all expressions (e.g. `1 + 2`) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_expressions};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit all expressions
let mut visited = vec![];
visit_expressions(&statements, |expr| {
  visited.push(format!("EXPR: {}", expr));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "EXPR: a",
  "EXPR: x IN (SELECT y FROM bar)",
  "EXPR: x",
  "EXPR: y",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```
