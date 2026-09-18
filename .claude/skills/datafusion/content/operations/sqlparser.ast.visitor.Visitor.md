# `sqlparser::ast::visitor::Visitor`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.Visitor.json).

<a id="op-d4d91fdab966896b4e7fa827"></a>
## Visitor

`trait` · `sqlparser::ast::visitor::Visitor` · sqlparser 0.62.0

```rust
trait Visitor
```

Source: `src/ast/visitor.rs:192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A visitor that can be used to walk an AST tree.

`pre_visit_` methods are invoked before visiting all children of the
node and `post_visit_` methods are invoked after visiting all
children of the node.

# See also

These methods provide a more concise way of visiting nodes of a certain type:
* [visit_relations](../operations/sqlparser.ast.visitor.visit_relations.md#op-a26a90536872aa1e55452c41)
* [visit_expressions](../operations/sqlparser.ast.visitor.visit_expressions.md#op-4715fda2b83d63609f15713c)
* [visit_statements](../operations/sqlparser.ast.visitor.visit_statements.md#op-506c5a8268e10ca5bf478626)

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Visit, Visitor, ObjectName, Expr};
# use core::ops::ControlFlow;
// A structure that records statements and relations
#[derive(Default)]
struct V {
   visited: Vec<String>,
}

// Visit relations and exprs before children are visited (depth first walk)
// Note you can also visit statements and visit exprs after children have been visited
impl Visitor for V {
  type Break = ();

  fn pre_visit_relation(&mut self, relation: &ObjectName) -> ControlFlow<Self::Break> {
    self.visited.push(format!("PRE: RELATION: {}", relation));
    ControlFlow::Continue(())
  }

  fn pre_visit_expr(&mut self, expr: &Expr) -> ControlFlow<Self::Break> {
    self.visited.push(format!("PRE: EXPR: {}", expr));
    ControlFlow::Continue(())
  }
}

let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// Drive the visitor through the AST
let mut visitor = V::default();
statements.visit(&mut visitor);

// The visitor has visited statements and expressions in pre-traversal order
let expected : Vec<_> = [
  "PRE: EXPR: a",
  "PRE: RELATION: foo",
  "PRE: EXPR: x IN (SELECT y FROM bar)",
  "PRE: EXPR: x",
  "PRE: EXPR: y",
  "PRE: RELATION: bar",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visitor.visited, expected);
```

<a id="op-7eeaf990972605f3eba5e41f"></a>
## Break

`assoc_type` · `sqlparser::ast::visitor::Visitor::Break` · sqlparser 0.62.0

```rust
Break
```

Source: `src/ast/visitor.rs:198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type returned when the recursion returns early.

Important note: The `Break` type should be kept as small as possible to prevent
stack overflow during recursion. If you need to return an error, consider
boxing it with `Box` to minimize stack usage.

<a id="op-c508556ca1956f3435f99562"></a>
## post_visit_expr

`function` · `sqlparser::ast::visitor::Visitor::post_visit_expr` · sqlparser 0.62.0

```rust
fn post_visit_expr(&mut self, _expr: &Expr) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any expressions that appear in the AST

<a id="op-969735340882a02b6e8d6874"></a>
## post_visit_query

`function` · `sqlparser::ast::visitor::Visitor::post_visit_query` · sqlparser 0.62.0

```rust
fn post_visit_query(&mut self, _query: &Query) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any queries that appear in the AST after visiting children

<a id="op-ee8ab5e0570e710af0d42f8f"></a>
## post_visit_relation

`function` · `sqlparser::ast::visitor::Visitor::post_visit_relation` · sqlparser 0.62.0

```rust
fn post_visit_relation(&mut self, _relation: &ObjectName) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any relations (e.g. tables) that appear in the AST after visiting children

<a id="op-1cfd6885ae608cd3ee8cd804"></a>
## post_visit_select

`function` · `sqlparser::ast::visitor::Visitor::post_visit_select` · sqlparser 0.62.0

```rust
fn post_visit_select(&mut self, _select: &Select) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any [Select](../operations/sqlparser.ast.query.Select.md#op-10b575bc5d4cbdd9ae434f0d) that appear in the AST after visiting children

<a id="op-79fac24635cb462aea73242b"></a>
## post_visit_statement

`function` · `sqlparser::ast::visitor::Visitor::post_visit_statement` · sqlparser 0.62.0

```rust
fn post_visit_statement(&mut self, _statement: &Statement) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any statements that appear in the AST after visiting children

<a id="op-37214510c5013284e8f48a72"></a>
## post_visit_table_factor

`function` · `sqlparser::ast::visitor::Visitor::post_visit_table_factor` · sqlparser 0.62.0

```rust
fn post_visit_table_factor(&mut self, _table_factor: &TableFactor) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any table factors that appear in the AST after visiting children

<a id="op-aeac49185be94ea87dbc7e6d"></a>
## post_visit_value

`function` · `sqlparser::ast::visitor::Visitor::post_visit_value` · sqlparser 0.62.0

```rust
fn post_visit_value(&mut self, _value: &ValueWithSpan) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any Value that appear in the AST after visiting children

<a id="op-1a5ac92ccd8fdf7a89a47146"></a>
## pre_visit_expr

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_expr` · sqlparser 0.62.0

```rust
fn pre_visit_expr(&mut self, _expr: &Expr) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any expressions that appear in the AST before visiting children

<a id="op-66dbda6b0dbdb87bed9beaf8"></a>
## pre_visit_query

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_query` · sqlparser 0.62.0

```rust
fn pre_visit_query(&mut self, _query: &Query) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any queries that appear in the AST before visiting children

<a id="op-430c97c3af5c22c7a4162cc0"></a>
## pre_visit_relation

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_relation` · sqlparser 0.62.0

```rust
fn pre_visit_relation(&mut self, _relation: &ObjectName) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any relations (e.g. tables) that appear in the AST before visiting children

<a id="op-ecab5cdfebc39176371a7b41"></a>
## pre_visit_select

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_select` · sqlparser 0.62.0

```rust
fn pre_visit_select(&mut self, _select: &Select) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any [Select](../operations/sqlparser.ast.query.Select.md#op-10b575bc5d4cbdd9ae434f0d) that appear in the AST before visiting children

<a id="op-74692b8ce83d15278296d61b"></a>
## pre_visit_statement

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_statement` · sqlparser 0.62.0

```rust
fn pre_visit_statement(&mut self, _statement: &Statement) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any statements that appear in the AST before visiting children

<a id="op-072bba7510cbea8110188aa4"></a>
## pre_visit_table_factor

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_table_factor` · sqlparser 0.62.0

```rust
fn pre_visit_table_factor(&mut self, _table_factor: &TableFactor) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any table factors that appear in the AST before visiting children

<a id="op-eeeee23c3963d3c903c0ff1c"></a>
## pre_visit_value

`function` · `sqlparser::ast::visitor::Visitor::pre_visit_value` · sqlparser 0.62.0

```rust
fn pre_visit_value(&mut self, _value: &ValueWithSpan) -> ControlFlow<Self::Break>
```

Source: `src/ast/visitor.rs:261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Invoked for any Value that appear in the AST before visiting children
