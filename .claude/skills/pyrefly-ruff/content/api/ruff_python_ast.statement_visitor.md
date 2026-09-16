# `ruff_python_ast::statement_visitor`

Crate `ruff_python_ast` · 6 public items · structured records in [`model/ruff_python_ast.statement_visitor.json`](../model/ruff_python_ast.statement_visitor.json)

## walk_body

`function` · `ruff_python_ast::statement_visitor::walk_body`

```rust
fn walk_body<'a, V: StatementVisitor<'a> + ?Sized>(visitor: &mut V, body: &'a [Stmt])
```

---

## walk_elif_else_clause

`function` · `ruff_python_ast::statement_visitor::walk_elif_else_clause`

```rust
fn walk_elif_else_clause<'a, V: StatementVisitor<'a> + ?Sized>(visitor: &mut V, elif_else_clause: &'a ElifElseClause)
```

---

## walk_except_handler

`function` · `ruff_python_ast::statement_visitor::walk_except_handler`

```rust
fn walk_except_handler<'a, V: StatementVisitor<'a> + ?Sized>(visitor: &mut V, except_handler: &'a ExceptHandler)
```

---

## walk_match_case

`function` · `ruff_python_ast::statement_visitor::walk_match_case`

```rust
fn walk_match_case<'a, V: StatementVisitor<'a> + ?Sized>(visitor: &mut V, match_case: &'a MatchCase)
```

---

## walk_stmt

`function` · `ruff_python_ast::statement_visitor::walk_stmt`

```rust
fn walk_stmt<'a, V: StatementVisitor<'a> + ?Sized>(visitor: &mut V, stmt: &'a Stmt)
```

---

## StatementVisitor

`trait` · `ruff_python_ast::statement_visitor::StatementVisitor`

```rust
trait StatementVisitor<'a>
```

**Implementors** (3)

- `pyrefly::report::cinderx::collect::ContextualTypeCollector`
- `pyrefly::report::pysa::class::AnnAssignCollector`
- `ruff_python_ast::helpers::RaiseStatementVisitor`

**Methods** (5)

```rust
fn visit_body(&mut self, body: &'a [Stmt])
fn visit_elif_else_clause(&mut self, elif_else_clause: &'a ElifElseClause)
fn visit_except_handler(&mut self, except_handler: &'a ExceptHandler)
fn visit_match_case(&mut self, match_case: &'a MatchCase)
fn visit_stmt(&mut self, stmt: &'a Stmt)
```

A trait for AST visitors that only need to visit statements.

---
