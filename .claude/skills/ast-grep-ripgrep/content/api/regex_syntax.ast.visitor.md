# `regex_syntax::ast::visitor`

Crate `regex-syntax` · 2 public items · structured records in [`model/regex_syntax.ast.visitor.json`](../model/regex_syntax.ast.visitor.json)

## visit

`function` · `regex_syntax::ast::visitor::visit`

Also reachable as `regex_syntax::ast::visit`

```rust
fn visit<V: Visitor>(ast: &ast::Ast, visitor: V) -> Result<V::Output, V::Err>
```

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Ast` while calling the
appropriate methods provided by the [`Visitor`] trait.

The primary use case for this method is when one wants to perform case
analysis over an `Ast` without using a stack size proportional to the depth
of the `Ast`. Namely, this method will instead use constant stack size, but
will use heap space proportional to the size of the `Ast`. This may be
desirable in cases where the size of `Ast` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

---

## Visitor

`trait` · `regex_syntax::ast::visitor::Visitor`

Also reachable as `regex_syntax::ast::Visitor`

```rust
trait Visitor
```

**Methods** (11)

```rust
fn finish(self) -> Result<Self::Output, Self::Err>
fn start(&mut self)
fn visit_alternation_in(&mut self) -> Result<(), Self::Err>
fn visit_class_set_binary_op_in(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), Self::Err>
fn visit_class_set_binary_op_post(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), Self::Err>
fn visit_class_set_binary_op_pre(&mut self, _ast: &ast::ClassSetBinaryOp) -> Result<(), Self::Err>
fn visit_class_set_item_post(&mut self, _ast: &ast::ClassSetItem) -> Result<(), Self::Err>
fn visit_class_set_item_pre(&mut self, _ast: &ast::ClassSetItem) -> Result<(), Self::Err>
fn visit_concat_in(&mut self) -> Result<(), Self::Err>
fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err>
fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err>
```

A trait for visiting an abstract syntax tree (AST) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on an abstract syntax tree without necessarily using recursion.
In particular, this permits callers to do case analysis with constant stack
usage, which can be important since the size of an abstract syntax tree
may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`] function.

Note that the abstract syntax tree for a regular expression is quite
complex. Unless you specifically need it, you might be able to use the much
simpler [high-level intermediate representation](crate::hir::Hir) and its
[corresponding `Visitor` trait](crate::hir::Visitor) instead.

---
