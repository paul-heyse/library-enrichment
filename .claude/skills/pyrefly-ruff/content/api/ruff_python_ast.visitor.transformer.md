# `ruff_python_ast::visitor::transformer`

Crate `ruff_python_ast` · 31 public items · structured records in [`model/ruff_python_ast.visitor.transformer.json`](../model/ruff_python_ast.visitor.transformer.json)

## walk_alias

`function` · `ruff_python_ast::visitor::transformer::walk_alias`

```rust
fn walk_alias<V: Transformer + ?Sized>(_visitor: &V, _alias: &mut Alias)
```

---

## walk_annotation

`function` · `ruff_python_ast::visitor::transformer::walk_annotation`

```rust
fn walk_annotation<V: Transformer + ?Sized>(visitor: &V, expr: &mut Expr)
```

---

## walk_arguments

`function` · `ruff_python_ast::visitor::transformer::walk_arguments`

```rust
fn walk_arguments<V: Transformer + ?Sized>(visitor: &V, arguments: &mut Arguments)
```

---

## walk_body

`function` · `ruff_python_ast::visitor::transformer::walk_body`

```rust
fn walk_body<V: Transformer + ?Sized>(visitor: &V, body: &mut [Stmt])
```

---

## walk_bool_op

`function` · `ruff_python_ast::visitor::transformer::walk_bool_op`

```rust
fn walk_bool_op<V: Transformer + ?Sized>(_visitor: &V, _bool_op: &mut BoolOp)
```

---

## walk_bytes_literal

`function` · `ruff_python_ast::visitor::transformer::walk_bytes_literal`

```rust
fn walk_bytes_literal<V: Transformer + ?Sized>(_visitor: &V, _bytes_literal: &mut BytesLiteral)
```

---

## walk_cmp_op

`function` · `ruff_python_ast::visitor::transformer::walk_cmp_op`

```rust
fn walk_cmp_op<V: Transformer + ?Sized>(_visitor: &V, _cmp_op: &mut CmpOp)
```

---

## walk_comprehension

`function` · `ruff_python_ast::visitor::transformer::walk_comprehension`

```rust
fn walk_comprehension<V: Transformer + ?Sized>(visitor: &V, comprehension: &mut Comprehension)
```

---

## walk_decorator

`function` · `ruff_python_ast::visitor::transformer::walk_decorator`

```rust
fn walk_decorator<V: Transformer + ?Sized>(visitor: &V, decorator: &mut Decorator)
```

---

## walk_elif_else_clause

`function` · `ruff_python_ast::visitor::transformer::walk_elif_else_clause`

```rust
fn walk_elif_else_clause<V: Transformer + ?Sized>(visitor: &V, elif_else_clause: &mut ElifElseClause)
```

---

## walk_except_handler

`function` · `ruff_python_ast::visitor::transformer::walk_except_handler`

```rust
fn walk_except_handler<V: Transformer + ?Sized>(visitor: &V, except_handler: &mut ExceptHandler)
```

---

## walk_expr

`function` · `ruff_python_ast::visitor::transformer::walk_expr`

```rust
fn walk_expr<V: Transformer + ?Sized>(visitor: &V, expr: &mut Expr)
```

---

## walk_expr_context

`function` · `ruff_python_ast::visitor::transformer::walk_expr_context`

```rust
fn walk_expr_context<V: Transformer + ?Sized>(_visitor: &V, _expr_context: &mut ExprContext)
```

---

## walk_f_string

`function` · `ruff_python_ast::visitor::transformer::walk_f_string`

```rust
fn walk_f_string<V: Transformer + ?Sized>(visitor: &V, f_string: &mut FString)
```

---

## walk_interpolated_string_element

`function` · `ruff_python_ast::visitor::transformer::walk_interpolated_string_element`

```rust
fn walk_interpolated_string_element<V: Transformer + ?Sized>(visitor: &V, interpolated_string_element: &mut InterpolatedStringElement)
```

---

## walk_keyword

`function` · `ruff_python_ast::visitor::transformer::walk_keyword`

```rust
fn walk_keyword<V: Transformer + ?Sized>(visitor: &V, keyword: &mut Keyword)
```

---

## walk_match_case

`function` · `ruff_python_ast::visitor::transformer::walk_match_case`

```rust
fn walk_match_case<V: Transformer + ?Sized>(visitor: &V, match_case: &mut MatchCase)
```

---

## walk_operator

`function` · `ruff_python_ast::visitor::transformer::walk_operator`

```rust
fn walk_operator<V: Transformer + ?Sized>(_visitor: &V, _operator: &mut Operator)
```

---

## walk_parameter

`function` · `ruff_python_ast::visitor::transformer::walk_parameter`

```rust
fn walk_parameter<V: Transformer + ?Sized>(visitor: &V, parameter: &mut Parameter)
```

---

## walk_parameters

`function` · `ruff_python_ast::visitor::transformer::walk_parameters`

```rust
fn walk_parameters<V: Transformer + ?Sized>(visitor: &V, parameters: &mut Parameters)
```

---

## walk_pattern

`function` · `ruff_python_ast::visitor::transformer::walk_pattern`

```rust
fn walk_pattern<V: Transformer + ?Sized>(visitor: &V, pattern: &mut Pattern)
```

---

## walk_pattern_arguments

`function` · `ruff_python_ast::visitor::transformer::walk_pattern_arguments`

```rust
fn walk_pattern_arguments<V: Transformer + ?Sized>(visitor: &V, pattern_arguments: &mut PatternArguments)
```

---

## walk_pattern_keyword

`function` · `ruff_python_ast::visitor::transformer::walk_pattern_keyword`

```rust
fn walk_pattern_keyword<V: Transformer + ?Sized>(visitor: &V, pattern_keyword: &mut PatternKeyword)
```

---

## walk_stmt

`function` · `ruff_python_ast::visitor::transformer::walk_stmt`

```rust
fn walk_stmt<V: Transformer + ?Sized>(visitor: &V, stmt: &mut Stmt)
```

---

## walk_string_literal

`function` · `ruff_python_ast::visitor::transformer::walk_string_literal`

```rust
fn walk_string_literal<V: Transformer + ?Sized>(_visitor: &V, _string_literal: &mut StringLiteral)
```

---

## walk_t_string

`function` · `ruff_python_ast::visitor::transformer::walk_t_string`

```rust
fn walk_t_string<V: Transformer + ?Sized>(visitor: &V, t_string: &mut TString)
```

---

## walk_type_param

`function` · `ruff_python_ast::visitor::transformer::walk_type_param`

```rust
fn walk_type_param<V: Transformer + ?Sized>(visitor: &V, type_param: &mut TypeParam)
```

---

## walk_type_params

`function` · `ruff_python_ast::visitor::transformer::walk_type_params`

```rust
fn walk_type_params<V: Transformer + ?Sized>(visitor: &V, type_params: &mut TypeParams)
```

---

## walk_unary_op

`function` · `ruff_python_ast::visitor::transformer::walk_unary_op`

```rust
fn walk_unary_op<V: Transformer + ?Sized>(_visitor: &V, _unary_op: &mut UnaryOp)
```

---

## walk_with_item

`function` · `ruff_python_ast::visitor::transformer::walk_with_item`

```rust
fn walk_with_item<V: Transformer + ?Sized>(visitor: &V, with_item: &mut WithItem)
```

---

## Transformer

`trait` · `ruff_python_ast::visitor::transformer::Transformer`

```rust
trait Transformer
```

**Methods** (30)

```rust
fn visit_alias(&self, alias: &mut Alias)
fn visit_annotation(&self, expr: &mut Expr)
fn visit_arguments(&self, arguments: &mut Arguments)
fn visit_body(&self, body: &mut [Stmt])
fn visit_bool_op(&self, bool_op: &mut BoolOp)
fn visit_bytes_literal(&self, bytes_literal: &mut BytesLiteral)
fn visit_cmp_op(&self, cmp_op: &mut CmpOp)
fn visit_comprehension(&self, comprehension: &mut Comprehension)
fn visit_decorator(&self, decorator: &mut Decorator)
fn visit_elif_else_clause(&self, elif_else_clause: &mut ElifElseClause)
fn visit_except_handler(&self, except_handler: &mut ExceptHandler)
fn visit_expr(&self, expr: &mut Expr)
fn visit_expr_context(&self, expr_context: &mut ExprContext)
fn visit_f_string(&self, f_string: &mut FString)
fn visit_interpolated_string_element(&self, interpolated_string_element: &mut InterpolatedStringElement)
fn visit_keyword(&self, keyword: &mut Keyword)
fn visit_match_case(&self, match_case: &mut MatchCase)
fn visit_operator(&self, operator: &mut Operator)
fn visit_parameter(&self, parameter: &mut Parameter)
fn visit_parameters(&self, parameters: &mut Parameters)
fn visit_pattern(&self, pattern: &mut Pattern)
fn visit_pattern_arguments(&self, pattern_arguments: &mut PatternArguments)
fn visit_pattern_keyword(&self, pattern_keyword: &mut PatternKeyword)
fn visit_stmt(&self, stmt: &mut Stmt)
fn visit_string_literal(&self, string_literal: &mut StringLiteral)
fn visit_t_string(&self, t_string: &mut TString)
fn visit_type_param(&self, type_param: &mut TypeParam)
fn visit_type_params(&self, type_params: &mut TypeParams)
fn visit_unary_op(&self, unary_op: &mut UnaryOp)
fn visit_with_item(&self, with_item: &mut WithItem)
```

A trait for transforming ASTs. Visits all nodes in the AST recursively in evaluation-order.

---
