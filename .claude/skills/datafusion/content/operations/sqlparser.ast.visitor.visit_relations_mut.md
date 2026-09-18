# `sqlparser::ast::visitor::visit_relations_mut`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_relations_mut.json).

<a id="op-ddf3ceab2afdc24fefd8bb6d"></a>
## visit_relations_mut

`function` · `sqlparser::ast::visitor::visit_relations_mut` · sqlparser 0.62.0

```rust
fn visit_relations_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::ObjectName) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure with a mutable reference to all relations (e.g. table names)
present in `v`.

When the closure mutates its argument, the new mutated relation will not be visited again.

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{ObjectName, ObjectNamePart, Ident, visit_relations_mut};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit statements, renaming table foo to bar
visit_relations_mut(&mut statements, |table| {
  table.0[0] = ObjectNamePart::Identifier(Ident::new("bar"));
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT a FROM bar");
```
