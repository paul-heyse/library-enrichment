# DisplayWith

`pyrefly_util::display::DisplayWith`

```rust
trait DisplayWith<Ctx: ?Sized>
```

Prose: [`api/pyrefly_util.display.md`](../api/pyrefly_util.display.md#displaywith) · records: [`model/pyrefly_util.display.json`](../model/pyrefly_util.display.json)

## Required

Every implementation must supply these.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Ctx) -> fmt::Result
```

## Implementors (74)

Read one before writing your own.

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

## Documentation

Like `Display`, but allows passing some additional context.
