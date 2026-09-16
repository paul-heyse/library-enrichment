# `ruff_python_ast::visitor`

Crate `ruff_python_ast` · 31 public items · structured records in [`model/ruff_python_ast.visitor.json`](../model/ruff_python_ast.visitor.json)

## walk_alias

`function` · `ruff_python_ast::visitor::walk_alias`

```rust
fn walk_alias<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _alias: &'a Alias)
```

---

## walk_annotation

`function` · `ruff_python_ast::visitor::walk_annotation`

```rust
fn walk_annotation<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, expr: &'a Expr)
```

---

## walk_arguments

`function` · `ruff_python_ast::visitor::walk_arguments`

```rust
fn walk_arguments<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, arguments: &'a Arguments)
```

---

## walk_body

`function` · `ruff_python_ast::visitor::walk_body`

```rust
fn walk_body<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, body: &'a [Stmt])
```

---

## walk_bool_op

`function` · `ruff_python_ast::visitor::walk_bool_op`

```rust
fn walk_bool_op<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _bool_op: &'a BoolOp)
```

---

## walk_bytes_literal

`function` · `ruff_python_ast::visitor::walk_bytes_literal`

```rust
fn walk_bytes_literal<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _bytes_literal: &'a BytesLiteral)
```

---

## walk_cmp_op

`function` · `ruff_python_ast::visitor::walk_cmp_op`

```rust
fn walk_cmp_op<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _cmp_op: &'a CmpOp)
```

---

## walk_comprehension

`function` · `ruff_python_ast::visitor::walk_comprehension`

```rust
fn walk_comprehension<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, comprehension: &'a Comprehension)
```

---

## walk_decorator

`function` · `ruff_python_ast::visitor::walk_decorator`

```rust
fn walk_decorator<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, decorator: &'a Decorator)
```

---

## walk_elif_else_clause

`function` · `ruff_python_ast::visitor::walk_elif_else_clause`

```rust
fn walk_elif_else_clause<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, elif_else_clause: &'a ElifElseClause)
```

---

## walk_except_handler

`function` · `ruff_python_ast::visitor::walk_except_handler`

```rust
fn walk_except_handler<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, except_handler: &'a ExceptHandler)
```

---

## walk_expr

`function` · `ruff_python_ast::visitor::walk_expr`

```rust
fn walk_expr<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, expr: &'a Expr)
```

---

## walk_expr_context

`function` · `ruff_python_ast::visitor::walk_expr_context`

```rust
fn walk_expr_context<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _expr_context: &'a ExprContext)
```

---

## walk_f_string

`function` · `ruff_python_ast::visitor::walk_f_string`

```rust
fn walk_f_string<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, f_string: &'a FString)
```

---

## walk_interpolated_string_element

`function` · `ruff_python_ast::visitor::walk_interpolated_string_element`

```rust
fn walk_interpolated_string_element<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, interpolated_string_element: &'a InterpolatedStringElement)
```

---

## walk_keyword

`function` · `ruff_python_ast::visitor::walk_keyword`

```rust
fn walk_keyword<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, keyword: &'a Keyword)
```

---

## walk_match_case

`function` · `ruff_python_ast::visitor::walk_match_case`

```rust
fn walk_match_case<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, match_case: &'a MatchCase)
```

---

## walk_operator

`function` · `ruff_python_ast::visitor::walk_operator`

```rust
fn walk_operator<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _operator: &'a Operator)
```

---

## walk_parameter

`function` · `ruff_python_ast::visitor::walk_parameter`

```rust
fn walk_parameter<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, parameter: &'a Parameter)
```

---

## walk_parameters

`function` · `ruff_python_ast::visitor::walk_parameters`

```rust
fn walk_parameters<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, parameters: &'a Parameters)
```

---

## walk_pattern

`function` · `ruff_python_ast::visitor::walk_pattern`

```rust
fn walk_pattern<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, pattern: &'a Pattern)
```

---

## walk_pattern_arguments

`function` · `ruff_python_ast::visitor::walk_pattern_arguments`

```rust
fn walk_pattern_arguments<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, pattern_arguments: &'a PatternArguments)
```

---

## walk_pattern_keyword

`function` · `ruff_python_ast::visitor::walk_pattern_keyword`

```rust
fn walk_pattern_keyword<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, pattern_keyword: &'a PatternKeyword)
```

---

## walk_stmt

`function` · `ruff_python_ast::visitor::walk_stmt`

```rust
fn walk_stmt<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, stmt: &'a Stmt)
```

---

## walk_string_literal

`function` · `ruff_python_ast::visitor::walk_string_literal`

```rust
fn walk_string_literal<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _string_literal: &'a StringLiteral)
```

---

## walk_t_string

`function` · `ruff_python_ast::visitor::walk_t_string`

```rust
fn walk_t_string<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, t_string: &'a TString)
```

---

## walk_type_param

`function` · `ruff_python_ast::visitor::walk_type_param`

```rust
fn walk_type_param<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, type_param: &'a TypeParam)
```

---

## walk_type_params

`function` · `ruff_python_ast::visitor::walk_type_params`

```rust
fn walk_type_params<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, type_params: &'a TypeParams)
```

---

## walk_unary_op

`function` · `ruff_python_ast::visitor::walk_unary_op`

```rust
fn walk_unary_op<'a, V: Visitor<'a> + ?Sized>(_visitor: &V, _unary_op: &'a UnaryOp)
```

---

## walk_with_item

`function` · `ruff_python_ast::visitor::walk_with_item`

```rust
fn walk_with_item<'a, V: Visitor<'a> + ?Sized>(visitor: &mut V, with_item: &'a WithItem)
```

---

## Visitor

`trait` · `ruff_python_ast::visitor::Visitor`

```rust
trait Visitor<'a>
```

**Implementors** (5)

- `pyrefly_python::ast::YieldFinder`
- `ruff_python_ast::helpers::AwaitVisitor`
- `ruff_python_ast::helpers::NameFinder`
- `ruff_python_ast::helpers::ReturnStatementVisitor`
- `ruff_python_ast::helpers::StoredNameFinder`

**Methods** (30)

```rust
fn visit_alias(&mut self, alias: &'a Alias)
fn visit_annotation(&mut self, expr: &'a Expr)
fn visit_arguments(&mut self, arguments: &'a Arguments)
fn visit_body(&mut self, body: &'a [Stmt])
fn visit_bool_op(&mut self, bool_op: &'a BoolOp)
fn visit_bytes_literal(&mut self, bytes_literal: &'a BytesLiteral)
fn visit_cmp_op(&mut self, cmp_op: &'a CmpOp)
fn visit_comprehension(&mut self, comprehension: &'a Comprehension)
fn visit_decorator(&mut self, decorator: &'a Decorator)
fn visit_elif_else_clause(&mut self, elif_else_clause: &'a ElifElseClause)
fn visit_except_handler(&mut self, except_handler: &'a ExceptHandler)
fn visit_expr(&mut self, expr: &'a Expr)
fn visit_expr_context(&mut self, expr_context: &'a ExprContext)
fn visit_f_string(&mut self, f_string: &'a FString)
fn visit_interpolated_string_element(&mut self, interpolated_string_element: &'a InterpolatedStringElement)
fn visit_keyword(&mut self, keyword: &'a Keyword)
fn visit_match_case(&mut self, match_case: &'a MatchCase)
fn visit_operator(&mut self, operator: &'a Operator)
fn visit_parameter(&mut self, parameter: &'a Parameter)
fn visit_parameters(&mut self, parameters: &'a Parameters)
fn visit_pattern(&mut self, pattern: &'a Pattern)
fn visit_pattern_arguments(&mut self, pattern_arguments: &'a PatternArguments)
fn visit_pattern_keyword(&mut self, pattern_keyword: &'a PatternKeyword)
fn visit_stmt(&mut self, stmt: &'a Stmt)
fn visit_string_literal(&mut self, string_literal: &'a StringLiteral)
fn visit_t_string(&mut self, t_string: &'a TString)
fn visit_type_param(&mut self, type_param: &'a TypeParam)
fn visit_type_params(&mut self, type_params: &'a TypeParams)
fn visit_unary_op(&mut self, unary_op: &'a UnaryOp)
fn visit_with_item(&mut self, with_item: &'a WithItem)
```

A trait for AST visitors. Visits all nodes in the AST recursively in evaluation-order.

Prefer [`crate::statement_visitor::StatementVisitor`] for visitors that only need to visit
statements.

Use the [`PreorderVisitor`](source_order::SourceOrderVisitor) if you want to visit the nodes
in source-order rather than evaluation order.

Use the [`Transformer`](transformer::Transformer) if you want to modify the nodes.

---
