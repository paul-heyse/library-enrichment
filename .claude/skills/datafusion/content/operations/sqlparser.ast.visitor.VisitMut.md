# `sqlparser::ast::visitor::VisitMut`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.visitor.VisitMut.json).

<a id="op-6ee42a19d2e900bc7740a2b8"></a>
## VisitMut

`trait` · `sqlparser::ast::visitor::VisitMut` · sqlparser 0.62.0

```rust
trait VisitMut
```

Source: `src/ast/visitor.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A type that can be visited by a [`VisitorMut`](../operations/sqlparser.ast.visitor.VisitorMut.md#op-9e92737f109af5e41462ab8b). See [`VisitorMut`](../operations/sqlparser.ast.visitor.VisitorMut.md#op-9e92737f109af5e41462ab8b) for
recursively visiting parsed SQL statements.

# Note

This trait should be automatically derived for sqlparser AST nodes
using the [VisitMut](sqlparser_derive::VisitMut) proc macro.

```text
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
```

Unresolved upstream links (retained, not inferred): `sqlparser_derive::VisitMut`.

<a id="op-b6780e903be788b10162e662"></a>
## visit

`function` · `sqlparser::ast::visitor::VisitMut::visit` · sqlparser 0.62.0

```rust
fn visit<V: VisitorMut>(&mut self, visitor: &mut V) -> ControlFlow<V::Break>
```

Source: `src/ast/visitor.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mutably visit this node with the provided [`VisitorMut`](../operations/sqlparser.ast.visitor.VisitorMut.md#op-9e92737f109af5e41462ab8b).

Implementations should call the appropriate mutable visitor hooks to
traverse and allow in-place mutation of child nodes. Returning a
`ControlFlow` value permits early termination of the traversal.
