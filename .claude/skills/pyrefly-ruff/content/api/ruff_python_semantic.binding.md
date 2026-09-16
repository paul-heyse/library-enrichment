# `ruff_python_semantic::binding`

Crate `ruff_python_semantic` · 12 public items · structured records in [`model/ruff_python_semantic.binding.json`](../model/ruff_python_semantic.binding.json)

## AnyImport

`enum` · `ruff_python_semantic::binding::AnyImport`

Also reachable as `ruff_python_semantic::AnyImport`

```rust
enum AnyImport<'a, 'ast>
```

**Variants**: `Import`, `SubmoduleImport`, `FromImport`

**Implements**: `ruff_python_semantic::binding::Imported`

**Derives**: Clone, Debug

**Methods** (15)

```rust
fn as_from_import(&self) -> Option<&&'a FromImport<'ast>>
fn as_import(&self) -> Option<&&'a Import<'ast>>
fn as_mut_from_import(&mut self) -> Option<&mut &'a FromImport<'ast>>
fn as_mut_import(&mut self) -> Option<&mut &'a Import<'ast>>
fn as_mut_submodule_import(&mut self) -> Option<&mut &'a SubmoduleImport<'ast>>
fn as_submodule_import(&self) -> Option<&&'a SubmoduleImport<'ast>>
fn expect_from_import(self) -> &'a FromImport<'ast> where Self: ::std::fmt::Debug
fn expect_import(self) -> &'a Import<'ast> where Self: ::std::fmt::Debug
fn expect_submodule_import(self) -> &'a SubmoduleImport<'ast> where Self: ::std::fmt::Debug
fn from_import(self) -> Option<&'a FromImport<'ast>>
fn import(self) -> Option<&'a Import<'ast>>
const fn is_from_import(&self) -> bool
const fn is_import(&self) -> bool
const fn is_submodule_import(&self) -> bool
fn submodule_import(self) -> Option<&'a SubmoduleImport<'ast>>
```

**via `ruff_python_semantic::binding::Imported`**

```rust
fn member_name(&self) -> Cow<'ast, str>
fn module_name(&self) -> &[&'ast str]
fn qualified_name(&self) -> &QualifiedName<'ast>
fn source_name(&self) -> &[&'ast str]
```

A wrapper around an import [`BindingKind`] that can be any of the three types of imports.

---

## BindingKind

`enum` · `ruff_python_semantic::binding::BindingKind`

Also reachable as `ruff_python_semantic::BindingKind`

```rust
enum BindingKind<'a>
```

**Variants**: `Annotation`, `Argument`, `NamedExprAssignment`, `Assignment`, `TypeParam`, `LoopVar`, `WithItemVar`, `Global`, `Nonlocal`, `Builtin`, `ClassDefinition`, `FunctionDefinition`, `Export`, `FutureImport`, `Import`, `FromImport`, `SubmoduleImport`, `Deletion`, `BoundException`, `UnboundException`, `DunderClassCell`

**Derives**: Clone, Debug

**Methods** (57)

```rust
fn as_class_definition(&self) -> Option<&ScopeId>
fn as_export(&self) -> Option<&Export<'a>>
fn as_from_import(&self) -> Option<&FromImport<'a>>
fn as_function_definition(&self) -> Option<&ScopeId>
fn as_global(&self) -> Option<&Option<BindingId>>
fn as_import(&self) -> Option<&Import<'a>>
fn as_mut_class_definition(&mut self) -> Option<&mut ScopeId>
fn as_mut_export(&mut self) -> Option<&mut Export<'a>>
fn as_mut_from_import(&mut self) -> Option<&mut FromImport<'a>>
fn as_mut_function_definition(&mut self) -> Option<&mut ScopeId>
fn as_mut_global(&mut self) -> Option<&mut Option<BindingId>>
fn as_mut_import(&mut self) -> Option<&mut Import<'a>>
fn as_mut_nonlocal(&mut self) -> Option<(&mut BindingId, &mut ScopeId)>
fn as_mut_submodule_import(&mut self) -> Option<&mut SubmoduleImport<'a>>
fn as_mut_unbound_exception(&mut self) -> Option<&mut Option<BindingId>>
fn as_nonlocal(&self) -> Option<(&BindingId, &ScopeId)>
fn as_submodule_import(&self) -> Option<&SubmoduleImport<'a>>
fn as_unbound_exception(&self) -> Option<&Option<BindingId>>
fn class_definition(self) -> Option<ScopeId>
fn expect_class_definition(self) -> ScopeId where Self: ::std::fmt::Debug
fn expect_export(self) -> Export<'a> where Self: ::std::fmt::Debug
fn expect_from_import(self) -> FromImport<'a> where Self: ::std::fmt::Debug
fn expect_function_definition(self) -> ScopeId where Self: ::std::fmt::Debug
fn expect_global(self) -> Option<BindingId> where Self: ::std::fmt::Debug
fn expect_import(self) -> Import<'a> where Self: ::std::fmt::Debug
fn expect_nonlocal(self) -> (BindingId, ScopeId) where Self: ::std::fmt::Debug
fn expect_submodule_import(self) -> SubmoduleImport<'a> where Self: ::std::fmt::Debug
fn expect_unbound_exception(self) -> Option<BindingId> where Self: ::std::fmt::Debug
fn export(self) -> Option<Export<'a>>
fn from_import(self) -> Option<FromImport<'a>>
fn function_definition(self) -> Option<ScopeId>
fn global(self) -> Option<Option<BindingId>>
fn import(self) -> Option<Import<'a>>
const fn is_annotation(&self) -> bool
const fn is_argument(&self) -> bool
const fn is_assignment(&self) -> bool
const fn is_bound_exception(&self) -> bool
const fn is_builtin(&self) -> bool
const fn is_class_definition(&self) -> bool
const fn is_deletion(&self) -> bool
const fn is_dunder_class_cell(&self) -> bool
const fn is_export(&self) -> bool
const fn is_from_import(&self) -> bool
const fn is_function_definition(&self) -> bool
const fn is_future_import(&self) -> bool
const fn is_global(&self) -> bool
const fn is_import(&self) -> bool
const fn is_loop_var(&self) -> bool
const fn is_named_expr_assignment(&self) -> bool
const fn is_nonlocal(&self) -> bool
const fn is_submodule_import(&self) -> bool
const fn is_type_param(&self) -> bool
const fn is_unbound_exception(&self) -> bool
const fn is_with_item_var(&self) -> bool
fn nonlocal(self) -> Option<(BindingId, ScopeId)>
fn submodule_import(self) -> Option<SubmoduleImport<'a>>
fn unbound_exception(self) -> Option<Option<BindingId>>
```

---

## Binding

`struct` · `ruff_python_semantic::binding::Binding`

Also reachable as `ruff_python_semantic::Binding`

```rust
struct Binding<'a>
```

**Fields**: `kind`, `range`, `scope`, `context`, `source`, `references`, `exceptions`, `flags`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (25)

```rust
fn as_any_import(&self) -> Option<AnyImport<'_, 'a>>
fn expression<'b>(&self, semantic: &SemanticModel<'b>) -> Option<&'b ast::Expr>
const fn in_assert_statement(&self) -> bool
const fn in_exception_handler(&self) -> bool
const fn is_alias(&self) -> bool
const fn is_annotated_type_alias(&self) -> bool
const fn is_deferred_type_alias(&self) -> bool
const fn is_deleted(&self) -> bool
const fn is_explicit_export(&self) -> bool
const fn is_external(&self) -> bool
const fn is_global(&self) -> bool
const fn is_invalid_all_format(&self) -> bool
const fn is_invalid_all_object(&self) -> bool
const fn is_nonlocal(&self) -> bool
const fn is_private_declaration(&self) -> bool
const fn is_type_alias(&self) -> bool
const fn is_unbound(&self) -> bool
const fn is_unpacked_assignment(&self) -> bool
fn is_unused(&self) -> bool
fn is_used(&self) -> bool
fn name<'b>(&self, source: &'b str) -> &'b str
fn parent_range(&self, semantic: &SemanticModel<'_>) -> Option<TextRange>
fn redefines(&self, existing: &Binding<'_>) -> bool
fn references(&self) -> impl Iterator<Item = ResolvedReferenceId> + '_
fn statement<'b>(&self, semantic: &SemanticModel<'b>) -> Option<&'b Stmt>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## BindingFlags

`struct` · `ruff_python_semantic::binding::BindingFlags`

Also reachable as `ruff_python_semantic::BindingFlags`

```rust
struct BindingFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> u16
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u16) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u16) -> Self
const fn from_bits_truncate(bits: u16) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<BindingFlags>
const fn iter_names(&self) -> iter::IterNames<BindingFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> BindingFlags
fn bits(&self) -> u16
fn from_bits_retain(bits: u16) -> BindingFlags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: BindingFlags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

Flags on a [`Binding`].

---

## BindingId

`struct` · `ruff_python_semantic::binding::BindingId`

Also reachable as `ruff_python_semantic::BindingId`

```rust
struct BindingId
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

ID uniquely identifying a [Binding] in a program.

Using a `u32` to identify [Binding]s should be sufficient because Ruff only supports documents with a
size smaller than or equal to `u32::MAX`. A document with the size of `u32::MAX` must have fewer than `u32::MAX`
bindings because bindings must be separated by whitespace (and have an assignment).

---

## Bindings

`struct` · `ruff_python_semantic::binding::Bindings`

Also reachable as `ruff_python_semantic::Bindings`

```rust
struct Bindings<'a>
```

**Implements**: `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Derives**: Clone, Debug, Default

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Binding<'a>>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

The bindings in a program.

Bindings are indexed by [`BindingId`]

---

## Exceptions

`struct` · `ruff_python_semantic::binding::Exceptions`

Also reachable as `ruff_python_semantic::Exceptions`

```rust
struct Exceptions
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (23)

```rust
const fn all() -> Self
const fn bits(&self) -> u8
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u8) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u8) -> Self
const fn from_bits_truncate(bits: u8) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn from_try_stmt(_: &ast::StmtTry, semantic: &SemanticModel<'_>) -> Self
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<Exceptions>
const fn iter_names(&self) -> iter::IterNames<Exceptions>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> Exceptions
fn bits(&self) -> u8
fn from_bits_retain(bits: u8) -> Exceptions
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: Exceptions) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

---

## Export

`struct` · `ruff_python_semantic::binding::Export`

Also reachable as `ruff_python_semantic::Export`

```rust
struct Export<'a>
```

**Fields**: `names`

**Derives**: Clone, Debug

---

## FromImport

`struct` · `ruff_python_semantic::binding::FromImport`

Also reachable as `ruff_python_semantic::FromImport`

```rust
struct FromImport<'a>
```

**Fields**: `qualified_name`

**Implements**: `ruff_python_semantic::binding::Imported`

**Derives**: Clone, Debug

**via `ruff_python_semantic::binding::Imported`**

```rust
fn member_name(&self) -> Cow<'a, str>
fn module_name(&self) -> &[&'a str]
fn qualified_name(&self) -> &QualifiedName<'a>
fn source_name(&self) -> &[&'a str]
```

A binding for a member imported from a module, keyed on the name to which the member is bound.
Ex) `from foo import bar` would be keyed on "bar".
Ex) `from foo import bar as baz` would be keyed on "baz".

---

## Import

`struct` · `ruff_python_semantic::binding::Import`

Also reachable as `ruff_python_semantic::Import`

```rust
struct Import<'a>
```

**Fields**: `qualified_name`

**Implements**: `ruff_python_semantic::binding::Imported`

**Derives**: Clone, Debug

**via `ruff_python_semantic::binding::Imported`**

```rust
fn member_name(&self) -> Cow<'a, str>
fn module_name(&self) -> &[&'a str]
fn qualified_name(&self) -> &QualifiedName<'a>
fn source_name(&self) -> &[&'a str]
```

A binding for an `import`, keyed on the name to which the import is bound.
Ex) `import foo` would be keyed on "foo".
Ex) `import foo as bar` would be keyed on "bar".

---

## SubmoduleImport

`struct` · `ruff_python_semantic::binding::SubmoduleImport`

Also reachable as `ruff_python_semantic::SubmoduleImport`

```rust
struct SubmoduleImport<'a>
```

**Fields**: `qualified_name`

**Implements**: `ruff_python_semantic::binding::Imported`

**Derives**: Clone, Debug

**via `ruff_python_semantic::binding::Imported`**

```rust
fn member_name(&self) -> Cow<'a, str>
fn module_name(&self) -> &[&'a str]
fn qualified_name(&self) -> &QualifiedName<'a>
fn source_name(&self) -> &[&'a str]
```

A binding for a submodule imported from a module, keyed on the name of the parent module.
Ex) `import foo.bar` would be keyed on "foo".

---

## Imported

`trait` · `ruff_python_semantic::binding::Imported`

Also reachable as `ruff_python_semantic::Imported`

```rust
trait Imported<'a>
```

**Implementors** (4)

- `ruff_python_semantic::binding::AnyImport`
- `ruff_python_semantic::binding::FromImport`
- `ruff_python_semantic::binding::Import`
- `ruff_python_semantic::binding::SubmoduleImport`

**Methods** (4)

```rust
fn member_name(&self) -> Cow<'a, str>
fn module_name(&self) -> &[&'a str]
fn qualified_name(&self) -> &QualifiedName<'a>
fn source_name(&self) -> &[&'a str]
```

A trait for imported symbols.

---
