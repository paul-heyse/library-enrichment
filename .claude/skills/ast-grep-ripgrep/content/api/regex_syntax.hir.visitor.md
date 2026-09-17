# `regex_syntax::hir::visitor`

Crate `regex-syntax` · 2 public items · structured records in [`model/regex_syntax.hir.visitor.json`](../model/regex_syntax.hir.visitor.json)

## visit

`function` · `regex_syntax::hir::visitor::visit`

Also reachable as `regex_syntax::hir::visit`

```rust
fn visit<V: Visitor>(hir: &hir::Hir, visitor: V) -> Result<V::Output, V::Err>
```

Executes an implementation of `Visitor` in constant stack space.

This function will visit every node in the given `Hir` while calling
appropriate methods provided by the [`Visitor`] trait.

The primary use case for this method is when one wants to perform case
analysis over an `Hir` without using a stack size proportional to the depth
of the `Hir`. Namely, this method will instead use constant stack space,
but will use heap space proportional to the size of the `Hir`. This may be
desirable in cases where the size of `Hir` is proportional to end user
input.

If the visitor returns an error at any point, then visiting is stopped and
the error is returned.

---

## Visitor

`trait` · `regex_syntax::hir::visitor::Visitor`

Also reachable as `regex_syntax::hir::Visitor`

```rust
trait Visitor
```

**Methods** (6)

```rust
fn finish(self) -> Result<Self::Output, Self::Err>
fn start(&mut self)
fn visit_alternation_in(&mut self) -> Result<(), Self::Err>
fn visit_concat_in(&mut self) -> Result<(), Self::Err>
fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err>
fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err>
```

A trait for visiting the high-level IR (HIR) in depth first order.

The principle aim of this trait is to enable callers to perform case
analysis on a high-level intermediate representation of a regular
expression without necessarily using recursion. In particular, this permits
callers to do case analysis with constant stack usage, which can be
important since the size of an HIR may be proportional to end user input.

Typical usage of this trait involves providing an implementation and then
running it using the [`visit`] function.

---
