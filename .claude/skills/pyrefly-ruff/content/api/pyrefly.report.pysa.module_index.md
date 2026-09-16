# `pyrefly::report::pysa::module_index`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.report.pysa.module_index.json`](../model/pyrefly.report.pysa.module_index.json)

## GraphQLDecoratorRef

`struct` · `pyrefly::report::pysa::module_index::GraphQLDecoratorRef`

```rust
struct GraphQLDecoratorRef
```

**Fields**: `module`, `name`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## PysaModuleIndex

`struct` · `pyrefly::report::pysa::module_index::PysaModuleIndex`

```rust
struct PysaModuleIndex
```

**Methods** (6)

```rust
fn build(context: &ModuleAnswersContext) -> PysaModuleIndex
fn get_function_ref_by_func_def_index(&self, func_def_index: FuncDefIndex) -> FunctionRef
fn get_function_ref_by_short_identifier(&self, short_identifier: ShortIdentifier, skip_property_getter: bool) -> Option<FunctionRef>
fn get_function_ref_for_class_field(&self, class_id: ClassId, field_name: &Name) -> Option<FunctionRef>
fn get_graphql_decorated_class_fields(&self, class_id: ClassId, predicate: impl Fn(&GraphQLDecoratorRef) -> bool) -> Vec<FunctionRef>
fn get_property_getter_ref(&self, field_range: TextRange) -> Option<&FunctionRef>
```

Per-module information required for the pysa report step.

Built while AST + bindings + answers are still available, persists after
eviction.

---
