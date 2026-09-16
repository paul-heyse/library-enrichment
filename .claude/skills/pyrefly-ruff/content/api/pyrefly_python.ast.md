# `pyrefly_python::ast`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.ast.json`](../model/pyrefly_python.ast.json)

## Ast

`struct` · `pyrefly_python::ast::Ast`

```rust
struct Ast
```

**Methods** (32)

```rust
fn annotation_syntax_problem(x: &Expr) -> Option<&'static str>
fn body_contains_yield(stmts: &[Stmt]) -> bool
fn contains_await(expr: &Expr) -> bool
fn decorator_trailing_name(decorator: &Expr) -> Option<&str>
fn expr_contains_yield(expr: &Expr) -> bool
fn expr_lvalue<'a>(x: &'a Expr, f: &mut impl FnMut(&'a ExprName))
fn expr_name_identifier(x: ExprName) -> Identifier
fn flatten_dict_items<'b>(x: &'b [DictItem]) -> Vec<&'b DictItem>
fn has_docstring(x: &ModModule) -> bool
fn if_branches(x: &StmtIf) -> impl Iterator<Item = (Option<&Expr>, &[Stmt])>
fn if_branches_owned(x: StmtIf) -> impl Iterator<Item = (TextRange, Option<Expr>, ThinVec<Stmt>)>
fn is_intentionally_unused(name: &str) -> bool
fn is_list_literal_or_comprehension(expr: &Expr) -> bool
fn is_literal(x: &Expr) -> bool
fn is_main_guard(test: &Expr) -> bool
fn is_mangled_attr(name: &Name) -> bool
fn is_pytest_tracebackhide(name: &str) -> bool
fn is_synthesized_empty_identifier(x: &Identifier) -> bool
fn is_synthesized_empty_name(x: &ExprName) -> bool
fn locate_node<'a>(module: &'a ModModule, position: TextSize) -> Vec<AnyNodeRef<'a>>
fn needs_brackets(parent: Option<AnyNodeRef<'_>>, node: &Expr) -> bool
fn parameters_iter_mut(x: &mut Parameters) -> impl Iterator<Item = (&mut Parameter, Option<&mut Option<Box<Expr>>>)>
fn parent_node(module: &ModModule, target: TextRange) -> Option<AnyNodeRef<'_>>
fn parse(contents: &str, source_type: PySourceType) -> (ModModule, Vec<ParseError>, Vec<UnsupportedSyntaxError>)
fn parse_expr(contents: &str, pos: TextSize) -> anyhow::Result<Expr>
fn parse_type_literal(x: &ExprStringLiteral, source: &str) -> anyhow::Result<Expr>
fn parse_with_version(contents: &str, version: PythonVersion, source_type: PySourceType) -> (Parsed<ModModule>, Vec<ParseError>, Vec<UnsupportedSyntaxError>)
fn pattern_is_irrefutable_for_subject(pattern: &Pattern, subject: &Expr) -> bool
fn pattern_lvalue<'a>(x: &'a Pattern, f: &mut impl FnMut(&'a Identifier))
fn pattern_match_singleton_to_expr(x: &PatternMatchSingleton) -> Expr
fn str_expr(s: &str, range: TextRange) -> Expr
fn unpack_slice(x: &Expr) -> &[Expr]
```

Just used for convenient namespacing - not a real type

---
