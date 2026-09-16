# Transformer

`ruff_python_ast::visitor::transformer::Transformer`

```rust
trait Transformer
```

Prose: [`api/ruff_python_ast.visitor.transformer.md`](../api/ruff_python_ast.visitor.transformer.md#transformer) · records: [`model/ruff_python_ast.visitor.transformer.json`](../model/ruff_python_ast.visitor.transformer.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

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

## Documentation

A trait for transforming ASTs. Visits all nodes in the AST recursively in evaluation-order.
