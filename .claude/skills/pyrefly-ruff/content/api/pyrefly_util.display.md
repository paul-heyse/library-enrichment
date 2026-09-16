# `pyrefly_util::display`

Crate `pyrefly_util` · 9 public items · structured records in [`model/pyrefly_util.display.json`](../model/pyrefly_util.display.json)

## append

`function` · `pyrefly_util::display::append`

```rust
fn append<A, B>(a: A, b: B) -> impl Iterator<Item = itertools::Either<A::Item, B::Item>> where A: IntoIterator, B: IntoIterator
```

---

## commas_iter

`function` · `pyrefly_util::display::commas_iter`

```rust
fn commas_iter<F, A>(a: F) -> impl Display where F: Fn() -> A, A: IntoIterator<Item: Display>
```

---

## count

`function` · `pyrefly_util::display::count`

```rust
fn count(n: usize, descriptor: &str) -> String
```

Format the number and descriptor word with the right pluralization.
E.g., count(2, "cat") -> "2 cats". Pluralizes by tacking on an "s".

---

## intersperse_iter

`function` · `pyrefly_util::display::intersperse_iter`

```rust
fn intersperse_iter<F, A, S>(separator: S, a: F) -> impl Display where F: Fn() -> A, A: IntoIterator<Item: Display>, S: Display
```

---

## number_thousands

`function` · `pyrefly_util::display::number_thousands`

```rust
fn number_thousands(x: usize) -> impl Display
```

Print a number using commas between thousands.

---

## pluralize

`function` · `pyrefly_util::display::pluralize`

```rust
fn pluralize(n: usize, descriptor: &str) -> String
```

Pluralizes by tacking on an "s".

---

## Fmt

`struct` · `pyrefly_util::display::Fmt`

```rust
struct Fmt<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>
```

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## DisplayWith

`trait` · `pyrefly_util::display::DisplayWith`

```rust
trait DisplayWith<Ctx: ?Sized>
```

**Implementors** (74)

- `alloc::boxed::Box`
- `alloc::rc::Rc`
- `alloc::sync::Arc`
- `core::option::Option`
- `pyrefly::alt::answers::Answers`
- `pyrefly::binding::binding::AnyIdx`
- `pyrefly::binding::binding::Binding`
- `pyrefly::binding::binding::BindingAbstractClassCheck`
- `pyrefly::binding::binding::BindingAnnotation`
- `pyrefly::binding::binding::BindingClass`
- `pyrefly::binding::binding::BindingClassBaseType`
- `pyrefly::binding::binding::BindingClassChecks`
- `pyrefly::binding::binding::BindingClassDisjointBase`
- `pyrefly::binding::binding::BindingClassField`
- `pyrefly::binding::binding::BindingClassMetadata`
- `pyrefly::binding::binding::BindingClassMro`
- `pyrefly::binding::binding::BindingClassSubscriptSymmetry`
- `pyrefly::binding::binding::BindingClassSynthesizedFields`
- `pyrefly::binding::binding::BindingDecoratedFunction`
- `pyrefly::binding::binding::BindingDecorator`
- `pyrefly::binding::binding::BindingDjangoRelations`
- `pyrefly::binding::binding::BindingExpect`
- `pyrefly::binding::binding::BindingExport`
- `pyrefly::binding::binding::BindingLegacyTypeParam`
- `pyrefly::binding::binding::BindingTParams`
- `pyrefly::binding::binding::BindingTypeAlias`
- `pyrefly::binding::binding::BindingUndecoratedFunction`
- `pyrefly::binding::binding::BindingUndecoratedFunctionRange`
- `pyrefly::binding::binding::BindingVariance`
- `pyrefly::binding::binding::BindingYield`
- `pyrefly::binding::binding::BindingYieldFrom`
- `pyrefly::binding::binding::ClassFieldDefinition`
- `pyrefly::binding::binding::ExprOrBinding`
- `pyrefly::binding::binding::Key`
- `pyrefly::binding::binding::KeyAbstractClassCheck`
- `pyrefly::binding::binding::KeyAnnotation`
- `pyrefly::binding::binding::KeyClass`
- `pyrefly::binding::binding::KeyClassBaseType`
- `pyrefly::binding::binding::KeyClassChecks`
- `pyrefly::binding::binding::KeyClassDisjointBase`
- `pyrefly::binding::binding::KeyClassField`
- `pyrefly::binding::binding::KeyClassMetadata`
- `pyrefly::binding::binding::KeyClassMro`
- `pyrefly::binding::binding::KeyClassSubscriptSymmetry`
- `pyrefly::binding::binding::KeyClassSynthesizedFields`
- `pyrefly::binding::binding::KeyDecoratedFunction`
- `pyrefly::binding::binding::KeyDecorator`
- `pyrefly::binding::binding::KeyDjangoRelations`
- `pyrefly::binding::binding::KeyExpect`
- `pyrefly::binding::binding::KeyExport`
- `pyrefly::binding::binding::KeyLegacyTypeParam`
- `pyrefly::binding::binding::KeyTParams`
- `pyrefly::binding::binding::KeyTypeAlias`
- `pyrefly::binding::binding::KeyUndecoratedFunction`
- `pyrefly::binding::binding::KeyUndecoratedFunctionRange`
- `pyrefly::binding::binding::KeyVariance`
- `pyrefly::binding::binding::KeyYield`
- `pyrefly::binding::binding::KeyYieldFrom`
- `pyrefly::binding::binding::NarrowUseLocation`
- `pyrefly::binding::narrow::AtomicNarrowOp`
- `pyrefly::binding::narrow::NarrowOp`
- `pyrefly_python::nesting_context::NestingContext`
- `pyrefly_python::short_identifier::ShortIdentifier`
- `ruff_python_ast::generated::AnyNodeRef`
- `ruff_python_ast::generated::Expr`
- `ruff_python_ast::generated::ExprCall`
- `ruff_python_ast::generated::ExprYield`
- `ruff_python_ast::generated::ExprYieldFrom`
- `ruff_python_ast::generated::Stmt`
- `ruff_python_ast::generated::StmtAugAssign`
- `ruff_python_ast::nodes::Arguments`
- `ruff_python_ast::nodes::Decorator`
- `ruff_text_size::range::TextRange`
- `ruff_text_size::size::TextSize`

**Methods** (1)

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Ctx) -> fmt::Result
```

Like `Display`, but allows passing some additional context.

---

## DisplayWithCtx

`trait` · `pyrefly_util::display::DisplayWithCtx`

```rust
trait DisplayWithCtx<Ctx: ?Sized>: DisplayWith<Ctx>
```

**Methods** (1)

```rust
fn display_with<'a>(&'a self, ctx: &'a Ctx) -> impl Display + 'a
```

---
