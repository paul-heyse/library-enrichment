# `sqlparser::ast::visitor::VisitorMut`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.VisitorMut.json).

<a id="op-9e92737f109af5e41462ab8b"></a>
## VisitorMut

`trait` · `sqlparser::ast::visitor::VisitorMut` · sqlparser 0.62.0

```rust
trait VisitorMut
```

Source: `src/ast/visitor.rs:314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A visitor that can be used to mutate an AST tree.

`pre_visit_` methods are invoked before visiting all children of the
node and `post_visit_` methods are invoked after visiting all
children of the node.

# See also

These methods provide a more concise way of visiting nodes of a certain type:
* [visit_relations_mut](../operations/sqlparser.ast.visitor.visit_relations_mut.md#op-ddf3ceab2afdc24fefd8bb6d)
* [visit_expressions_mut](../operations/sqlparser.ast.visitor.visit_expressions_mut.md#op-5aea00478f8cd79b07e7eac1)
* [visit_statements_mut](../operations/sqlparser.ast.visitor.visit_statements_mut.md#op-c82f8742aa640331023e0cd0)

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{VisitMut, VisitorMut, ObjectName, Expr, Ident};
# use core::ops::ControlFlow;

// A visitor that replaces "to_replace" with "replaced" in all expressions
struct Replacer;

// Visit each expression after its children have been visited
impl VisitorMut for Replacer {
  type Break = ();

  fn post_visit_expr(&mut self, expr: &mut Expr) -> ControlFlow<Self::Break> {
    if let Expr::Identifier(Ident{ value, ..}) = expr {
        *value = value.replace("to_replace", "replaced")
    }
    ControlFlow::Continue(())
  }
}

let sql = "SELECT to_replace FROM foo where to_replace IN (SELECT to_replace FROM bar)";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Drive the visitor through the AST
statements.visit(&mut Replacer);

assert_eq!(statements[0].to_string(), "SELECT replaced FROM foo WHERE replaced IN (SELECT replaced FROM bar)");
```

<a id="op-7298ab4f894f3bfdcab2914b"></a>
## Break

`assoc_type` · `sqlparser::ast::visitor::VisitorMut::Break` · sqlparser 0.62.0

```rust
Break
```

Source: `src/ast/visitor.rs:320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type returned when the recursion returns early.

Important note: The `Break` type should be kept as small as possible to prevent
stack overflow during recursion. If you need to return an error, consider
boxing it with `Box` to minimize stack usage.

<a id="op-6f494d9117e7b4893da57cee"></a>
## post_visit_expr

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_expr` · sqlparser 0.62.0

```rust
fn post_visit_expr(&mut self, _expr: &mut Expr) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any expressions that appear in the AST

<a id="op-7393749132e16762a5c7382f"></a>
## post_visit_query

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_query` · sqlparser 0.62.0

```rust
fn post_visit_query(&mut self, _query: &mut Query) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any queries that appear in the AST after visiting children

<a id="op-09070c7980368d06435b006f"></a>
## post_visit_relation

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_relation` · sqlparser 0.62.0

```rust
fn post_visit_relation(&mut self, _relation: &mut ObjectName) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any relations (e.g. tables) that appear in the AST after visiting children

<a id="op-4d13a7e42315f1f7fedf64bf"></a>
## post_visit_select

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_select` · sqlparser 0.62.0

```rust
fn post_visit_select(&mut self, _select: &mut Select) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any [Select](../operations/sqlparser.ast.query.Select.md#op-10b575bc5d4cbdd9ae434f0d) that appear in the AST after visiting children

<a id="op-a8c3d50bb784794c32c3c1de"></a>
## post_visit_statement

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_statement` · sqlparser 0.62.0

```rust
fn post_visit_statement(&mut self, _statement: &mut Statement) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any statements that appear in the AST after visiting children

<a id="op-e3a3a8245f92e85243614c24"></a>
## post_visit_table_factor

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_table_factor` · sqlparser 0.62.0

```rust
fn post_visit_table_factor(&mut self, _table_factor: &mut TableFactor) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any table factors that appear in the AST after visiting children

<a id="op-903ef9ddcc467fa96ee87197"></a>
## post_visit_value

`function` · `sqlparser::ast::visitor::VisitorMut::post_visit_value` · sqlparser 0.62.0

```rust
fn post_visit_value(&mut self, _value: &mut ValueWithSpan) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any statements that appear in the AST after visiting children

<a id="op-bfd954e0e10f8ce6475158db"></a>
## pre_visit_expr

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_expr` · sqlparser 0.62.0

```rust
fn pre_visit_expr(&mut self, _expr: &mut Expr) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any expressions that appear in the AST before visiting children

<a id="op-ed9fcf03ef56d73e3d2f33db"></a>
## pre_visit_query

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_query` · sqlparser 0.62.0

```rust
fn pre_visit_query(&mut self, _query: &mut Query) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any queries that appear in the AST before visiting children

<a id="op-42f414d93b0ea1d27f5e5f95"></a>
## pre_visit_relation

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_relation` · sqlparser 0.62.0

```rust
fn pre_visit_relation(&mut self, _relation: &mut ObjectName) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any relations (e.g. tables) that appear in the AST before visiting children

<a id="op-ccac86ad671f5d0ba6bd7740"></a>
## pre_visit_select

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_select` · sqlparser 0.62.0

```rust
fn pre_visit_select(&mut self, _select: &mut Select) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any [Select](../operations/sqlparser.ast.query.Select.md#op-10b575bc5d4cbdd9ae434f0d) that appear in the AST before visiting children

<a id="op-80c5c7336761c9192e170c56"></a>
## pre_visit_statement

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_statement` · sqlparser 0.62.0

```rust
fn pre_visit_statement(&mut self, _statement: &mut Statement) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:379`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any statements that appear in the AST before visiting children

<a id="op-115d2e5891b570c945dbaa32"></a>
## pre_visit_table_factor

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_table_factor` · sqlparser 0.62.0

```rust
fn pre_visit_table_factor(&mut self, _table_factor: &mut TableFactor) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any table factors that appear in the AST before visiting children

<a id="op-efcb21f2329fa26f284286c4"></a>
## pre_visit_value

`function` · `sqlparser::ast::visitor::VisitorMut::pre_visit_value` · sqlparser 0.62.0

```rust
fn pre_visit_value(&mut self, _value: &mut ValueWithSpan) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any value that appear in the AST before visiting children
