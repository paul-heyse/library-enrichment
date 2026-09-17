# `ra_ap_syntax::syntax_editor::edits`

Crate `ra_ap_syntax` · 2 public items · structured records in [`model/ra_ap_syntax.syntax_editor.edits.json`](../model/ra_ap_syntax.syntax_editor.edits.json)

## GetOrCreateWhereClause

`trait` · `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

Also reachable as `ra_ap_syntax::syntax_editor::GetOrCreateWhereClause`

```rust
trait GetOrCreateWhereClause: ast::HasGenericParams
```

**Implementors** (6)

- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::Impl`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`

**Methods** (2)

```rust
fn get_or_create_where_clause(&self, editor: &SyntaxEditor, new_preds: impl Iterator<Item = ast::WherePred>)
fn where_clause_position(&self) -> Option<Position>
```

---

## Removable

`trait` · `ra_ap_syntax::syntax_editor::edits::Removable`

Also reachable as `ra_ap_syntax::syntax_editor::Removable`

```rust
trait Removable: AstNode
```

**Implementors** (3)

- `ra_ap_syntax::ast::generated::nodes::TypeBoundList`
- `ra_ap_syntax::ast::generated::nodes::Use`
- `ra_ap_syntax::ast::generated::nodes::UseTree`

**Methods** (1)

```rust
fn remove(&self, editor: &SyntaxEditor)
```

---
