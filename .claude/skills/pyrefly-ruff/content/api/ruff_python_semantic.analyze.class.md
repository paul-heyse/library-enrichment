# `ruff_python_semantic::analyze::class`

Crate `ruff_python_semantic` · 12 public items · structured records in [`model/ruff_python_semantic.analyze.class.json`](../model/ruff_python_semantic.analyze.class.json)

## ClassMemberBoundness

`enum` · `ruff_python_semantic::analyze::class::ClassMemberBoundness`

```rust
enum ClassMemberBoundness
```

**Variants**: `PossiblyUnbound`, `Bound`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## ClassMemberKind

`enum` · `ruff_python_semantic::analyze::class::ClassMemberKind`

```rust
enum ClassMemberKind<'a>
```

**Variants**: `Assign`, `AnnAssign`, `FunctionDef`

**Derives**: Clone, Copy, Debug

---

## IsMetaclass

`enum` · `ruff_python_semantic::analyze::class::IsMetaclass`

```rust
enum IsMetaclass
```

**Variants**: `Yes`, `No`, `Maybe`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_yes(self) -> bool
```

Whether or not a class is a metaclass. Constructed by [`is_metaclass`].

---

## any_base_class

`function` · `ruff_python_semantic::analyze::class::any_base_class`

```rust
fn any_base_class<F>(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>, func: F) -> bool where F: FnMut(&ruff_python_ast::Expr) -> bool
```

Return `true` if any base class matches an [`Expr`] predicate.

---

## any_member_declaration

`function` · `ruff_python_semantic::analyze::class::any_member_declaration`

```rust
fn any_member_declaration<F>(class: &ast::StmtClassDef, func: F) -> bool where F: FnMut(ClassMemberDeclaration<'_>) -> bool
```

---

## any_qualified_base_class

`function` · `ruff_python_semantic::analyze::class::any_qualified_base_class`

```rust
fn any_qualified_base_class<F>(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>, func: F) -> bool where F: Fn(ruff_python_ast::name::QualifiedName<'_>) -> bool
```

Return `true` if any base class matches a [`QualifiedName`] predicate.

---

## any_super_class

`function` · `ruff_python_semantic::analyze::class::any_super_class`

```rust
fn any_super_class<F>(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>, func: F) -> bool where F: Fn(&ast::StmtClassDef) -> bool
```

Return `true` if any base class, including the given class,
matches an [`ast::StmtClassDef`] predicate.

---

## is_enumeration

`function` · `ruff_python_semantic::analyze::class::is_enumeration`

```rust
fn is_enumeration(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>) -> bool
```

Return `true` if `class_def` is a class that has one or more enum classes in its mro

---

## is_metaclass

`function` · `ruff_python_semantic::analyze::class::is_metaclass`

```rust
fn is_metaclass(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>) -> IsMetaclass
```

Returns `IsMetaclass::Yes` if the given class is definitely a metaclass,
`IsMetaclass::No` if it's definitely *not* a metaclass, and
`IsMetaclass::Maybe` otherwise.

---

## iter_super_class

`function` · `ruff_python_semantic::analyze::class::iter_super_class`

```rust
fn iter_super_class<'stmt>(class_def: &'stmt ast::StmtClassDef, semantic: &'stmt SemanticModel<'_>) -> impl Iterator<Item = &'stmt ast::StmtClassDef>
```

Returns an iterator over all base classes, beginning with the
given class.

The traversal of the class hierarchy is breadth-first, since
this graph tends to have small width but could be rather deep.

---

## might_be_generic

`function` · `ruff_python_semantic::analyze::class::might_be_generic`

```rust
fn might_be_generic(class_def: &ast::StmtClassDef, semantic: &SemanticModel<'_>) -> bool
```

Returns true if a class might be generic.

A class is considered generic if at least one of its direct bases
is subscripted with a `TypeVar`-like,
or if it is defined using PEP 695 syntax.

Therefore, a class *might* be generic if it uses PEP-695 syntax
or at least one of its direct bases is a subscript expression that
is subscripted with an object that *might* be a `TypeVar`-like.

---

## ClassMemberDeclaration

`struct` · `ruff_python_semantic::analyze::class::ClassMemberDeclaration`

```rust
struct ClassMemberDeclaration<'a>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn boundness(&self) -> ClassMemberBoundness
fn kind(&self) -> &ClassMemberKind<'a>
```

---
