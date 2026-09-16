# `pyrefly::query`

Crate `pyrefly` · 8 public items · structured records in [`model/pyrefly.query.json`](../model/pyrefly.query.json)

## TypeShapeTrait

`enum` · `pyrefly::query::TypeShapeTrait`

```rust
enum TypeShapeTrait
```

**Variants**: `TypedDict`, `PartialTypedDict`, `Tuple`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Traits surfaced for collapsed `named` shapes when the outer Pyrefly `Type`
already carries a value-shape property.

These traits are emitted for value shapes such as `Type::Tuple`,
`Type::TypedDict`, and `Type::PartialTypedDict`.
They are not synthesized from class metadata, and therefore are not emitted
for class/type-position shapes that would require metadata lookup.

---

## python_ast_range_for_expr

`function` · `pyrefly::query::python_ast_range_for_expr`

```rust
fn python_ast_range_for_expr(module_info: &pyrefly_python::module::Module, original_range: ruff_text_size::TextRange, expr: &ruff_python_ast::Expr, parent_expr: Option<&ruff_python_ast::Expr>) -> pyrefly_util::lined_buffer::PythonASTRange
```

Thin wrapper around `LinedBuffer::python_ast_range_for_expr` that accepts
a `ModuleInfo` for convenience. Callers with direct access to a
`LinedBuffer` can call the method directly.

---

## Attribute

`struct` · `pyrefly::query::Attribute`

```rust
struct Attribute
```

**Fields**: `name`, `kind`, `annotation`, `is_final`

---

## Callee

`struct` · `pyrefly::query::Callee`

```rust
struct Callee
```

**Fields**: `kind`, `target`, `class_name`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Query

`struct` · `pyrefly::query::Query`

```rust
struct Query
```

**Methods** (10)

```rust
fn add_files(&self, files: Vec<(ModuleName, ModulePath)>) -> Vec<String>
fn change_files(&self, events: &CategorizedEvents)
fn get_attributes(&self, name: ModuleName, path: ModulePath, class_name: &str) -> Option<Vec<Attribute>>
fn get_callees_with_location(&self, name: ModuleName, path: ModulePath, location: Option<PythonASTRange>) -> Option<Vec<(PythonASTRange, Callee)>>
fn get_type_table_in_file(&self, name: ModuleName, path: ModulePath, walker: Option<&TypeQueryStmtWalker>) -> Option<TypeTableResponseData>
fn get_type_table_in_file_with_timing(&self, name: ModuleName, path: ModulePath) -> Option<(TypeTableResponseData, TypeQueryTiming)>
fn get_type_table_in_file_with_timing_filtered(&self, name: ModuleName, path: ModulePath, walker: Option<&TypeQueryStmtWalker>) -> Option<(TypeTableResponseData, TypeQueryTiming)>
fn is_subtype(&self, name: ModuleName, path: PathBuf, lt: &str, gt: &str) -> Result<bool, String>
fn new(config_finder: ConfigFinder, thread_count: ThreadCount) -> Self
fn resolve_target_from_qualified_name(&self, name: ModuleName, path: PathBuf, qualified_name: &str) -> Option<Vec<Callee>>
```

---

## TypeQueryTiming

`struct` · `pyrefly::query::TypeQueryTiming`

```rust
struct TypeQueryTiming
```

**Fields**: `located_count`, `setup`, `transform`, `total`

**Derives**: Clone, Debug, Default

---

## TypeQueryExprVisitor

`type_alias` · `pyrefly::query::TypeQueryExprVisitor`

```rust
type TypeQueryExprVisitor<'a> = dyn FnMut(&'a ruff_python_ast::Expr, Option<&'a ruff_python_ast::Expr>) -> bool + 'a
```

Records a single expression selected by a filtered query walker.

This callback does not recurse into child expressions. Walkers that include
an expression are responsible for walking its children if they want them.
Returns whether the expression produced a located type.

---

## TypeQueryStmtWalker

`type_alias` · `pyrefly::query::TypeQueryStmtWalker`

```rust
type TypeQueryStmtWalker = dyn Fn(&'a [ruff_python_ast::Stmt], &mut TypeQueryExprVisitor<'a>)
```

---
