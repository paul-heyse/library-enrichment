# `ruff_python_semantic::definition`

Crate `ruff_python_semantic` · 10 public items · structured records in [`model/ruff_python_semantic.definition.json`](../model/ruff_python_semantic.definition.json)

## Definition

`enum` · `ruff_python_semantic::definition::Definition`

Also reachable as `ruff_python_semantic::Definition`

```rust
enum Definition<'a>
```

**Variants**: `Module`, `Member`

**Derives**: Debug

**Methods** (15)

```rust
fn as_class_def(&self) -> Option<&'a ast::StmtClassDef>
fn as_function_def(&self) -> Option<&'a ast::StmtFunctionDef>
fn as_member(&self) -> Option<&Member<'a>>
fn as_module(&self) -> Option<&Module<'a>>
fn as_mut_member(&mut self) -> Option<&mut Member<'a>>
fn as_mut_module(&mut self) -> Option<&mut Module<'a>>
fn expect_member(self) -> Member<'a> where Self: ::std::fmt::Debug
fn expect_module(self) -> Module<'a> where Self: ::std::fmt::Debug
const fn is_member(&self) -> bool
const fn is_method(&self) -> bool
const fn is_module(&self) -> bool
fn is_property<P, I>(&self, extra_properties: P, semantic: &SemanticModel<'_>) -> bool where P: IntoIterator<IntoIter = I>, I: Iterator<Item = QualifiedName<'a>> + Clone
fn member(self) -> Option<Member<'a>>
fn module(self) -> Option<Module<'a>>
fn name(&self) -> Option<&'a str>
```

A definition within a Python program.

---

## MemberKind

`enum` · `ruff_python_semantic::definition::MemberKind`

Also reachable as `ruff_python_semantic::MemberKind`

```rust
enum MemberKind<'a>
```

**Variants**: `Class`, `NestedClass`, `Function`, `NestedFunction`, `Method`

**Derives**: Clone, Copy, Debug

**Methods** (25)

```rust
fn as_class(&self) -> Option<&&'a ast::StmtClassDef>
fn as_function(&self) -> Option<&&'a ast::StmtFunctionDef>
fn as_method(&self) -> Option<&&'a ast::StmtFunctionDef>
fn as_mut_class(&mut self) -> Option<&mut &'a ast::StmtClassDef>
fn as_mut_function(&mut self) -> Option<&mut &'a ast::StmtFunctionDef>
fn as_mut_method(&mut self) -> Option<&mut &'a ast::StmtFunctionDef>
fn as_mut_nested_class(&mut self) -> Option<&mut &'a ast::StmtClassDef>
fn as_mut_nested_function(&mut self) -> Option<&mut &'a ast::StmtFunctionDef>
fn as_nested_class(&self) -> Option<&&'a ast::StmtClassDef>
fn as_nested_function(&self) -> Option<&&'a ast::StmtFunctionDef>
fn class(self) -> Option<&'a ast::StmtClassDef>
fn expect_class(self) -> &'a ast::StmtClassDef where Self: ::std::fmt::Debug
fn expect_function(self) -> &'a ast::StmtFunctionDef where Self: ::std::fmt::Debug
fn expect_method(self) -> &'a ast::StmtFunctionDef where Self: ::std::fmt::Debug
fn expect_nested_class(self) -> &'a ast::StmtClassDef where Self: ::std::fmt::Debug
fn expect_nested_function(self) -> &'a ast::StmtFunctionDef where Self: ::std::fmt::Debug
fn function(self) -> Option<&'a ast::StmtFunctionDef>
const fn is_class(&self) -> bool
const fn is_function(&self) -> bool
const fn is_method(&self) -> bool
const fn is_nested_class(&self) -> bool
const fn is_nested_function(&self) -> bool
fn method(self) -> Option<&'a ast::StmtFunctionDef>
fn nested_class(self) -> Option<&'a ast::StmtClassDef>
fn nested_function(self) -> Option<&'a ast::StmtFunctionDef>
```

---

## ModuleKind

`enum` · `ruff_python_semantic::definition::ModuleKind`

Also reachable as `ruff_python_semantic::ModuleKind`

```rust
enum ModuleKind
```

**Variants**: `Module`, `Package`

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
const fn is_module(&self) -> bool
const fn is_package(&self) -> bool
```

---

## ModuleSource

`enum` · `ruff_python_semantic::definition::ModuleSource`

Also reachable as `ruff_python_semantic::ModuleSource`

```rust
enum ModuleSource<'a>
```

**Variants**: `Path`, `File`

**Derives**: Clone, Copy, Debug

A Python module can either be defined as a module path (i.e., the dot-separated path to the
module) or, if the module can't be resolved, as a file path (i.e., the path to the file defining
the module).

---

## ContextualizedDefinition

`struct` · `ruff_python_semantic::definition::ContextualizedDefinition`

Also reachable as `ruff_python_semantic::ContextualizedDefinition`

```rust
struct ContextualizedDefinition<'a>
```

**Fields**: `definition`, `visibility`

A [`Definition`] in a Python program with its resolved [`Visibility`].

---

## ContextualizedDefinitions

`struct` · `ruff_python_semantic::definition::ContextualizedDefinitions`

Also reachable as `ruff_python_semantic::ContextualizedDefinitions`

```rust
struct ContextualizedDefinitions<'a>
```

**Methods** (1)

```rust
fn iter(&self) -> impl Iterator<Item = &ContextualizedDefinition<'a>>
```

A collection of [`Definition`] structs in a Python program with resolved [`Visibility`].

---

## DefinitionId

`struct` · `ruff_python_semantic::definition::DefinitionId`

Also reachable as `ruff_python_semantic::DefinitionId`

```rust
struct DefinitionId
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `ruff_index::idx::Idx`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn as_u32(self) -> u32
const fn as_usize(self) -> usize
const fn from_u32(value: u32) -> Self
const fn from_usize(value: usize) -> Self
const fn index(self) -> usize
```

**via `core::convert::From`**

```rust
fn from(value: usize) -> Self
fn from(value: u32) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: usize) -> Self::Output
fn add(self, rhs: Self) -> Self::Output
```

**via `ruff_index::idx::Idx`**

```rust
fn index(self) -> usize
fn new(value: usize) -> Self
```

Id uniquely identifying a definition in a program.

---

## Definitions

`struct` · `ruff_python_semantic::definition::Definitions`

Also reachable as `ruff_python_semantic::Definitions`

```rust
struct Definitions<'a>
```

**Implements**: `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn python_ast(&self) -> Option<&'a [Stmt]>
fn resolve(&'a self, exports: Option<&[DunderAllName<'_>]>) -> ContextualizedDefinitions<'a>
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

The definitions within a Python program indexed by [`DefinitionId`].

---

## Member

`struct` · `ruff_python_semantic::definition::Member`

Also reachable as `ruff_python_semantic::Member`

```rust
struct Member<'a>
```

**Fields**: `kind`, `parent`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Debug

**Methods** (1)

```rust
fn body(&self) -> &'a [Stmt]
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A member of a Python module.

---

## Module

`struct` · `ruff_python_semantic::definition::Module`

Also reachable as `ruff_python_semantic::Module`

```rust
struct Module<'a>
```

**Fields**: `kind`, `source`, `python_ast`, `name`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
const fn qualified_name(&self) -> Option<&'a [String]>
```

A Python module.

---
