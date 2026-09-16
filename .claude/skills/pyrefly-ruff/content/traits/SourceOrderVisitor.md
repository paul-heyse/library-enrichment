# SourceOrderVisitor

`ruff_python_ast::visitor::source_order::SourceOrderVisitor`

```rust
trait SourceOrderVisitor<'a>
```

Prose: [`api/ruff_python_ast.visitor.source_order.md`](../api/ruff_python_ast.visitor.source_order.md#sourceordervisitor) · records: [`model/ruff_python_ast.visitor.source_order.json`](../model/ruff_python_ast.visitor.source_order.json)

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn enter_node(&mut self, _node: AnyNodeRef<'a>) -> TraversalSignal
fn leave_node(&mut self, _node: AnyNodeRef<'a>)
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
fn visit_f_string(&mut self, f_string: &'a FString)
fn visit_identifier(&mut self, identifier: &'a Identifier)
fn visit_interpolated_string_element(&mut self, interpolated_string_element: &'a InterpolatedStringElement)
fn visit_keyword(&mut self, keyword: &'a Keyword)
fn visit_match_case(&mut self, match_case: &'a MatchCase)
fn visit_mod(&mut self, module: &'a Mod)
fn visit_operator(&mut self, operator: &'a Operator)
fn visit_parameter(&mut self, arg: &'a Parameter)
fn visit_parameter_with_default(&mut self, parameter_with_default: &'a ParameterWithDefault)
fn visit_parameters(&mut self, parameters: &'a Parameters)
fn visit_pattern(&mut self, pattern: &'a Pattern)
fn visit_pattern_arguments(&mut self, pattern_arguments: &'a PatternArguments)
fn visit_pattern_keyword(&mut self, pattern_keyword: &'a PatternKeyword)
fn visit_singleton(&mut self, _singleton: &'a Singleton)
fn visit_stmt(&mut self, stmt: &'a Stmt)
fn visit_string_literal(&mut self, string_literal: &'a StringLiteral)
fn visit_t_string(&mut self, t_string: &'a TString)
fn visit_type_param(&mut self, type_param: &'a TypeParam)
fn visit_type_params(&mut self, type_params: &'a TypeParams)
fn visit_unary_op(&mut self, unary_op: &'a UnaryOp)
fn visit_with_item(&mut self, with_item: &'a WithItem)
```

## Implementors (2)

Read one before writing your own.

- `pyrefly::binding::function::SuperMethodCallFinder`
- `pyrefly_python::ast::CoveringNodeVisitor`

## Documentation

Visitor that traverses all nodes recursively in the order they appear in the source.

If you need a visitor that visits the nodes in the order they're evaluated at runtime,
use [`Visitor`](super::Visitor) instead.
