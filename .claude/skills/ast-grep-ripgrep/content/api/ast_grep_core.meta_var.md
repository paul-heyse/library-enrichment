# `ast_grep_core::meta_var`

Crate `ast-grep-core` · 4 public items · structured records in [`model/ast_grep_core.meta_var.json`](../model/ast_grep_core.meta_var.json)

## MetaVariable

`enum` · `ast_grep_core::meta_var::MetaVariable`

```rust
enum MetaVariable
```

**Variants**: `Capture`, `Dropped`, `Multiple`, `MultiCapture`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## MetaVarEnv

`struct` · `ast_grep_core::meta_var::MetaVarEnv`

```rust
struct MetaVarEnv<'tree, D: Doc>
```

**Derives**: Clone, Default

**Methods** (12)

```rust
fn add_label(&mut self, label: &str, node: Node<'t, D>)
fn get_labels(&self, label: &str) -> Option<&Vec<Node<'t, D>>>
fn get_match(&self, var: &str) -> Option<&Node<'t, D>>
fn get_matched_variables(&self) -> impl Iterator<Item = MetaVariable> + use<{'lifetime': "'_"}, {'lifetime': "'t"}, {'param': 'D'}>
fn get_multiple_matches(&self, var: &str) -> Vec<Node<'t, D>>
fn get_transformed(&self, var: &str) -> Option<&Underlying<D>>
fn get_var_bytes<'s>(&'s self, var: &MetaVariable) -> Option<&'s [<D::Source as Content>::Underlying]>
fn insert(&mut self, id: &str, ret: Node<'t, D>) -> Option<&mut Self>
fn insert_multi(&mut self, id: &str, ret: Vec<Node<'t, D>>) -> Option<&mut Self>
fn insert_transformation(&mut self, var: &MetaVariable, name: &str, slice: Underlying<D>)
fn match_constraints<M: Matcher>(&mut self, var_matchers: &HashMap<MetaVariableID, M>) -> bool
fn new() -> Self
```

a dictionary that stores metavariable instantiation
const a = 123 matched with const a = $A will produce env: $A => 123

---

## MetaVariableID

`type_alias` · `ast_grep_core::meta_var::MetaVariableID`

```rust
type MetaVariableID = String
```

---

## Underlying

`type_alias` · `ast_grep_core::meta_var::Underlying`

```rust
type Underlying<D> = Vec<<<D as Doc>::Source as Content>::Underlying>
```

---
