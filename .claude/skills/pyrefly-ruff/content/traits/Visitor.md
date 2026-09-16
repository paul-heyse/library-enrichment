# Visitor

`ruff_python_ast::visitor::Visitor`

```rust
trait Visitor<'a>
```

Prose: [`api/ruff_python_ast.visitor.md`](../api/ruff_python_ast.visitor.md#visitor) · records: [`model/ruff_python_ast.visitor.json`](../model/ruff_python_ast.visitor.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

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

## Implementors (5)

Read one before writing your own.

- `pyrefly_python::ast::YieldFinder`
- `ruff_python_ast::helpers::AwaitVisitor`
- `ruff_python_ast::helpers::NameFinder`
- `ruff_python_ast::helpers::ReturnStatementVisitor`
- `ruff_python_ast::helpers::StoredNameFinder`

## Documentation

A trait for AST visitors. Visits all nodes in the AST recursively in evaluation-order.

Prefer [`crate::statement_visitor::StatementVisitor`] for visitors that only need to visit
statements.

Use the [`PreorderVisitor`](source_order::SourceOrderVisitor) if you want to visit the nodes
in source-order rather than evaluation order.

Use the [`Transformer`](transformer::Transformer) if you want to modify the nodes.
