# `sqlparser::ast::visitor::visit_statements`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_statements.json).

<a id="op-506c5a8268e10ca5bf478626"></a>
## visit_statements

`function` · `sqlparser::ast::visitor::visit_statements` · sqlparser 0.62.0

```rust
fn visit_statements<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::Statement) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure iteratively with a mutable reference to all statements
present in `v` (e.g. `SELECT`, `CREATE TABLE`, etc).

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_statements};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar); CREATE TABLE baz(q int)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit all statements
let mut visited = vec![];
visit_statements(&statements, |stmt| {
  visited.push(format!("STATEMENT: {}", stmt));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "STATEMENT: SELECT a FROM foo WHERE x IN (SELECT y FROM bar)",
  "STATEMENT: CREATE TABLE baz (q INT)"
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```
