# `sqlparser::ast::visitor::Visit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.Visit.json).

<a id="op-ad80f72cb0b9ad6b3c7bcb5a"></a>
## Visit

`trait` · `sqlparser::ast::visitor::Visit` · sqlparser 0.62.0

```rust
trait Visit
```

Source: `src/ast/visitor.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A type that can be visited by a [`Visitor`](../operations/sqlparser.ast.visitor.Visitor.md#op-d4d91fdab966896b4e7fa827). See [`Visitor`](../operations/sqlparser.ast.visitor.Visitor.md#op-d4d91fdab966896b4e7fa827) for
recursively visiting parsed SQL statements.

# Note

This trait should be automatically derived for sqlparser AST nodes
using the [Visit](sqlparser_derive::Visit) proc macro.

```text
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
```

Unresolved upstream links (retained, not inferred): `sqlparser_derive::Visit`.

<a id="op-8a576c197dce180ebd95b2bb"></a>
## visit

`function` · `sqlparser::ast::visitor::Visit::visit` · sqlparser 0.62.0

```rust
fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break>
```

Source: `src/ast/visitor.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Visit this node with the provided [`Visitor`](../operations/sqlparser.ast.visitor.Visitor.md#op-d4d91fdab966896b4e7fa827).

Implementations should call the appropriate visitor hooks to traverse
child nodes and return a `ControlFlow` value to allow early exit.
