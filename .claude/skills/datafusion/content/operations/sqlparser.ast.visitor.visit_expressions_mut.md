# `sqlparser::ast::visitor::visit_expressions_mut`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.visit_expressions_mut.json).

<a id="op-5aea00478f8cd79b07e7eac1"></a>
## visit_expressions_mut

`function` · `sqlparser::ast::visitor::visit_expressions_mut` · sqlparser 0.62.0

```rust
fn visit_expressions_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::Expr) -> core::ops::ControlFlow<E>
```

Source: `src/ast/visitor.rs:608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invokes the provided closure iteratively with a mutable reference to all expressions
present in `v`.

This performs a depth-first search, so if the closure mutates the expression

# Example

## Remove all select limits in sub-queries
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Expr, visit_expressions_mut, visit_statements_mut};
# use core::ops::ControlFlow;
let sql = "SELECT (SELECT y FROM z LIMIT 9) FROM t LIMIT 3";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Remove all select limits in sub-queries
visit_expressions_mut(&mut statements, |expr| {
  if let Expr::Subquery(q) = expr {
     q.limit_clause = None;
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT (SELECT y FROM z) FROM t LIMIT 3");
```

## Wrap column name in function call

This demonstrates how to effectively replace an expression with another more complicated one
that references the original. This example avoids unnecessary allocations by using the
[`std::mem`] family of functions.

```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::*;
# use core::ops::ControlFlow;
let sql = "SELECT x, y FROM t";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

visit_expressions_mut(&mut statements, |expr| {
  if matches!(expr, Expr::Identifier(col_name) if col_name.value == "x") {
    let old_expr = std::mem::replace(expr, Expr::value(Value::Null));
    *expr = Expr::Function(Function {
          name: ObjectName::from(vec![Ident::new("f")]),
          uses_odbc_syntax: false,
          args: FunctionArguments::List(FunctionArgumentList {
              duplicate_treatment: None,
              args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(old_expr))],
              clauses: vec![],
          }),
          null_treatment: None,
          filter: None,
          over: None,
          parameters: FunctionArguments::None,
          within_group: vec![],
     });
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT f(x), y FROM t");
```

Unresolved upstream links (retained, not inferred): ``std::mem``.
