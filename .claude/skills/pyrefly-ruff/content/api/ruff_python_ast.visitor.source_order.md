# `ruff_python_ast::visitor::source_order`

Crate `ruff_python_ast` · 36 public items · structured records in [`model/ruff_python_ast.visitor.source_order.json`](../model/ruff_python_ast.visitor.source_order.json)

## TraversalSignal

`enum` · `ruff_python_ast::visitor::source_order::TraversalSignal`

```rust
enum TraversalSignal
```

**Variants**: `Traverse`, `Skip`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_traverse(self) -> bool
```

---

## walk_alias

`function` · `ruff_python_ast::visitor::source_order::walk_alias`

```rust
fn walk_alias<'a, V>(visitor: &mut V, alias: &'a Alias) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_annotation

`function` · `ruff_python_ast::visitor::source_order::walk_annotation`

```rust
fn walk_annotation<'a, V: SourceOrderVisitor<'a> + ?Sized>(visitor: &mut V, expr: &'a Expr)
```

---

## walk_arguments

`function` · `ruff_python_ast::visitor::source_order::walk_arguments`

```rust
fn walk_arguments<'a, V>(visitor: &mut V, arguments: &'a Arguments) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_body

`function` · `ruff_python_ast::visitor::source_order::walk_body`

```rust
fn walk_body<'a, V>(visitor: &mut V, body: &'a [Stmt]) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_bool_op

`function` · `ruff_python_ast::visitor::source_order::walk_bool_op`

```rust
fn walk_bool_op<'a, V>(_visitor: &mut V, _bool_op: &'a BoolOp) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_bytes_literal

`function` · `ruff_python_ast::visitor::source_order::walk_bytes_literal`

```rust
fn walk_bytes_literal<'a, V>(visitor: &mut V, bytes_literal: &'a BytesLiteral) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_cmp_op

`function` · `ruff_python_ast::visitor::source_order::walk_cmp_op`

```rust
fn walk_cmp_op<'a, V>(_visitor: &mut V, _cmp_op: &'a CmpOp) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_comprehension

`function` · `ruff_python_ast::visitor::source_order::walk_comprehension`

```rust
fn walk_comprehension<'a, V>(visitor: &mut V, comprehension: &'a Comprehension) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_decorator

`function` · `ruff_python_ast::visitor::source_order::walk_decorator`

```rust
fn walk_decorator<'a, V>(visitor: &mut V, decorator: &'a Decorator) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_elif_else_clause

`function` · `ruff_python_ast::visitor::source_order::walk_elif_else_clause`

```rust
fn walk_elif_else_clause<'a, V>(visitor: &mut V, elif_else_clause: &'a ElifElseClause) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_except_handler

`function` · `ruff_python_ast::visitor::source_order::walk_except_handler`

```rust
fn walk_except_handler<'a, V>(visitor: &mut V, except_handler: &'a ExceptHandler) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_expr

`function` · `ruff_python_ast::visitor::source_order::walk_expr`

```rust
fn walk_expr<'a, V>(visitor: &mut V, expr: &'a Expr) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_f_string

`function` · `ruff_python_ast::visitor::source_order::walk_f_string`

```rust
fn walk_f_string<'a, V>(visitor: &mut V, f_string: &'a FString) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_format_spec

`function` · `ruff_python_ast::visitor::source_order::walk_format_spec`

```rust
fn walk_format_spec<'a, V: SourceOrderVisitor<'a> + ?Sized>(visitor: &mut V, format_spec: &'a Expr)
```

---

## walk_identifier

`function` · `ruff_python_ast::visitor::source_order::walk_identifier`

```rust
fn walk_identifier<'a, V: SourceOrderVisitor<'a> + ?Sized>(visitor: &mut V, identifier: &'a Identifier)
```

---

## walk_interpolated_string_element

`function` · `ruff_python_ast::visitor::source_order::walk_interpolated_string_element`

```rust
fn walk_interpolated_string_element<'a, V: SourceOrderVisitor<'a> + ?Sized>(visitor: &mut V, f_string_element: &'a InterpolatedStringElement)
```

---

## walk_keyword

`function` · `ruff_python_ast::visitor::source_order::walk_keyword`

```rust
fn walk_keyword<'a, V>(visitor: &mut V, keyword: &'a Keyword) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_match_case

`function` · `ruff_python_ast::visitor::source_order::walk_match_case`

```rust
fn walk_match_case<'a, V>(visitor: &mut V, match_case: &'a MatchCase) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_module

`function` · `ruff_python_ast::visitor::source_order::walk_module`

```rust
fn walk_module<'a, V>(visitor: &mut V, module: &'a Mod) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_node

`function` · `ruff_python_ast::visitor::source_order::walk_node`

```rust
fn walk_node<'a, V>(visitor: &mut V, node: AnyNodeRef<'a>) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_operator

`function` · `ruff_python_ast::visitor::source_order::walk_operator`

```rust
fn walk_operator<'a, V>(_visitor: &mut V, _operator: &'a Operator) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_parameter

`function` · `ruff_python_ast::visitor::source_order::walk_parameter`

```rust
fn walk_parameter<'a, V>(visitor: &mut V, parameter: &'a Parameter) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_parameter_with_default

`function` · `ruff_python_ast::visitor::source_order::walk_parameter_with_default`

```rust
fn walk_parameter_with_default<'a, V>(visitor: &mut V, parameter_with_default: &'a ParameterWithDefault) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_parameters

`function` · `ruff_python_ast::visitor::source_order::walk_parameters`

```rust
fn walk_parameters<'a, V>(visitor: &mut V, parameters: &'a Parameters) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_pattern

`function` · `ruff_python_ast::visitor::source_order::walk_pattern`

```rust
fn walk_pattern<'a, V>(visitor: &mut V, pattern: &'a Pattern) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_pattern_arguments

`function` · `ruff_python_ast::visitor::source_order::walk_pattern_arguments`

```rust
fn walk_pattern_arguments<'a, V>(visitor: &mut V, pattern_arguments: &'a PatternArguments) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_pattern_keyword

`function` · `ruff_python_ast::visitor::source_order::walk_pattern_keyword`

```rust
fn walk_pattern_keyword<'a, V>(visitor: &mut V, pattern_keyword: &'a PatternKeyword) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_stmt

`function` · `ruff_python_ast::visitor::source_order::walk_stmt`

```rust
fn walk_stmt<'a, V>(visitor: &mut V, stmt: &'a Stmt) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_string_literal

`function` · `ruff_python_ast::visitor::source_order::walk_string_literal`

```rust
fn walk_string_literal<'a, V>(visitor: &mut V, string_literal: &'a StringLiteral) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_t_string

`function` · `ruff_python_ast::visitor::source_order::walk_t_string`

```rust
fn walk_t_string<'a, V>(visitor: &mut V, t_string: &'a TString) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_type_param

`function` · `ruff_python_ast::visitor::source_order::walk_type_param`

```rust
fn walk_type_param<'a, V>(visitor: &mut V, type_param: &'a TypeParam) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_type_params

`function` · `ruff_python_ast::visitor::source_order::walk_type_params`

```rust
fn walk_type_params<'a, V>(visitor: &mut V, type_params: &'a TypeParams) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_unary_op

`function` · `ruff_python_ast::visitor::source_order::walk_unary_op`

```rust
fn walk_unary_op<'a, V>(_visitor: &mut V, _unary_op: &'a UnaryOp) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## walk_with_item

`function` · `ruff_python_ast::visitor::source_order::walk_with_item`

```rust
fn walk_with_item<'a, V>(visitor: &mut V, with_item: &'a WithItem) where V: SourceOrderVisitor<'a> + ?Sized
```

---

## SourceOrderVisitor

`trait` · `ruff_python_ast::visitor::source_order::SourceOrderVisitor`

```rust
trait SourceOrderVisitor<'a>
```

**Implementors** (2)

- `pyrefly::binding::function::SuperMethodCallFinder`
- `pyrefly_python::ast::CoveringNodeVisitor`

**Methods** (35)

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

Visitor that traverses all nodes recursively in the order they appear in the source.

If you need a visitor that visits the nodes in the order they're evaluated at runtime,
use [`Visitor`](super::Visitor) instead.

---
