# `ast_grep_core::matcher::node_match`

Crate `ast-grep-core` · 1 public items · structured records in [`model/ast_grep_core.matcher.node_match.json`](../model/ast_grep_core.matcher.node_match.json)

## NodeMatch

`struct` · `ast_grep_core::matcher::node_match::NodeMatch`

Also reachable as `ast_grep_core::NodeMatch`, `ast_grep_core::matcher::NodeMatch`

```rust
struct NodeMatch<'t, D: Doc>
```

**Implements**: `core::borrow::Borrow`, `core::convert::From`, `core::ops::deref::Deref`

**Derives**: Clone

**Methods** (5)

```rust
fn get_env(&self) -> &MetaVarEnv<'tree, D>
fn get_env_mut(&mut self) -> &mut MetaVarEnv<'tree, D>
fn get_node(&self) -> &Node<'tree, D>
fn new(node: Node<'tree, D>, env: MetaVarEnv<'tree, D>) -> Self
fn replace_by<R: Replacer<D>>(&self, replacer: R) -> source::Edit<<D as Doc>::Source>
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &Node<'tree, D>
```

**via `core::convert::From`**

```rust
fn from(node: Node<'tree, D>) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Represents the matched node with populated MetaVarEnv.
It derefs to the Node so you can use it as a Node.
To access the underlying MetaVarEnv, call `get_env` method.

---
