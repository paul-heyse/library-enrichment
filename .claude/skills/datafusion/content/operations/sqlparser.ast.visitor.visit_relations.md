# `sqlparser::ast::visitor::visit_relations`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_relations.json).

<a id="op-a26a90536872aa1e55452c41"></a>
## visit_relations

`function` · `sqlparser::ast::visitor::visit_relations` · sqlparser 0.62.0

```rust
fn visit_relations<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::ObjectName) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure on all relations (e.g. table names) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_relations};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit statements, capturing relations (table names)
let mut visited = vec![];
visit_relations(&statements, |relation| {
  visited.push(format!("RELATION: {}", relation));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "RELATION: foo",
  "RELATION: bar",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```
