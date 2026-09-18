# `sqlparser::ast::visitor::visit_statements_mut`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_statements_mut.json).

<a id="op-c82f8742aa640331023e0cd0"></a>
## visit_statements_mut

`function` · `sqlparser::ast::visitor::visit_statements_mut` · sqlparser 0.62.0

```rust
fn visit_statements_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::Statement) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:695`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure on all statements (e.g. `SELECT`, `CREATE TABLE`, etc) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Statement, visit_statements_mut};
# use core::ops::ControlFlow;
let sql = "SELECT x FROM foo LIMIT 9+$limit; SELECT * FROM t LIMIT f()";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Remove all select limits in outer statements (not in sub-queries)
visit_statements_mut(&mut statements, |stmt| {
  if let Statement::Query(q) = stmt {
     q.limit_clause = None;
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT x FROM foo");
assert_eq!(statements[1].to_string(), "SELECT * FROM t");
```
