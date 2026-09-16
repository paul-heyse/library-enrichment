# `ruff_python_ast::helpers`

Crate `ruff_python_ast` · 45 public items · structured records in [`model/ruff_python_ast.helpers.json`](../model/ruff_python_ast.helpers.json)

## SideEffect

`enum` · `ruff_python_ast::helpers::SideEffect`

```rust
enum SideEffect
```

**Variants**: `Absent`, `Possible`, `Present`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn is_absent(self) -> bool
const fn is_present(self) -> bool
const fn merge(self, other: Self) -> Self
```

Whether an expression has no side effects, may have side effects,
or is assumed to have side effects.

---

## Truthiness

`enum` · `ruff_python_ast::helpers::Truthiness`

```rust
enum Truthiness
```

**Variants**: `True`, `False`, `Falsey`, `Truthy`, `None`, `Unknown`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn from_expr<F>(expr: &Expr, is_builtin: F) -> Self where F: Fn(&str) -> bool
fn into_bool(self) -> Option<bool>
const fn is_false(&self) -> bool
const fn is_falsey(&self) -> bool
const fn is_none(&self) -> bool
const fn is_true(&self) -> bool
const fn is_truthy(&self) -> bool
const fn is_unknown(&self) -> bool
```

---

## any_over_body

`function` · `ruff_python_ast::helpers::any_over_body`

```rust
fn any_over_body<F>(body: &[Stmt], func: F) -> bool where F: FnMut(&Expr) -> bool
```

---

## any_over_expr

`function` · `ruff_python_ast::helpers::any_over_expr`

```rust
fn any_over_expr<F>(expr: &Expr, func: F) -> bool where F: FnMut(&Expr) -> bool
```

Call `func` over every `Expr` in `expr`, returning `true` if any expression
returns `true`..

---

## body_without_leading_docstring

`function` · `ruff_python_ast::helpers::body_without_leading_docstring`

```rust
fn body_without_leading_docstring(body: &[Stmt]) -> &[Stmt]
```

Returns `body` without its leading docstring statement, if present.

---

## collect_import_from_member

`function` · `ruff_python_ast::helpers::collect_import_from_member`

```rust
fn collect_import_from_member<'a>(level: u32, module: Option<&'a str>, member: &'a str) -> name::QualifiedName<'a>
```

Format the call path for a relative import.

# Examples

```rust
# use ruff_python_ast::helpers::collect_import_from_member;

assert_eq!(collect_import_from_member(0, None, "bar").segments(), ["bar"]);
assert_eq!(collect_import_from_member(1, None, "bar").segments(), [".", "bar"]);
assert_eq!(collect_import_from_member(1, Some("foo"), "bar").segments(), [".", "foo", "bar"]);
```

---

## comment_indentation_after

`function` · `ruff_python_ast::helpers::comment_indentation_after`

```rust
fn comment_indentation_after(preceding: AnyNodeRef<'_>, comment_range: ruff_text_size::TextRange, source: &str) -> ruff_text_size::TextSize
```

Determine the indentation level of an own-line comment, defined as the minimum indentation of
all comments between the preceding node and the comment, including the comment itself. In
other words, we don't allow successive comments to ident _further_ than any preceding comments.

For example, given:
```python
if True:
    pass
    # comment
```

The indentation would be 4, as the comment is indented by 4 spaces.

Given:
```python
if True:
    pass
# comment
else:
    pass
```

The indentation would be 0, as the comment is not indented at all.

Given:
```python
if True:
    pass
    # comment
        # comment
```

Both comments would be marked as indented at 4 spaces, as the indentation of the first comment
is used for the second comment.

This logic avoids pathological cases like:
```python
try:
    if True:
        if True:
            pass

        # a
            # b
        # c
except Exception:
    pass
```

If we don't use the minimum indentation of any preceding comments, we would mark `# b` as
indented to the same depth as `pass`, which could in turn lead to us treating it as a trailing
comment of `pass`, despite there being a comment between them that "resets" the indentation.

---

## contains_effect

`function` · `ruff_python_ast::helpers::contains_effect`

```rust
fn contains_effect<F>(expr: &Expr, is_builtin: F) -> bool where F: Fn(&str) -> bool
```

Return `true` if the `Expr` contains an expression that appears to include a
side-effect (like a function call).

Accepts a closure that determines whether a given name (e.g., `"list"`) is a Python builtin.

---

## extract_handled_exceptions

`function` · `ruff_python_ast::helpers::extract_handled_exceptions`

```rust
fn extract_handled_exceptions(handlers: &[ExceptHandler]) -> Vec<&Expr>
```

Extract the names of all handled exceptions.

---

## format_import_from

`function` · `ruff_python_ast::helpers::format_import_from`

```rust
fn format_import_from(level: u32, module: Option<&str>) -> std::borrow::Cow<'_, str>
```

Format the module reference name for a relative import.

# Examples

```rust
# use ruff_python_ast::helpers::format_import_from;

assert_eq!(format_import_from(0, None), "".to_string());
assert_eq!(format_import_from(1, None), ".".to_string());
assert_eq!(format_import_from(1, Some("foo")), ".foo".to_string());
```

---

## format_import_from_member

`function` · `ruff_python_ast::helpers::format_import_from_member`

```rust
fn format_import_from_member(level: u32, module: Option<&str>, member: &str) -> String
```

Format the member reference name for a relative import.

# Examples

```rust
# use ruff_python_ast::helpers::format_import_from_member;

assert_eq!(format_import_from_member(0, None, "bar"), "bar".to_string());
assert_eq!(format_import_from_member(1, None, "bar"), ".bar".to_string());
assert_eq!(format_import_from_member(1, Some("foo"), "bar"), ".foo.bar".to_string());
```

---

## from_relative_import

`function` · `ruff_python_ast::helpers::from_relative_import`

```rust
fn from_relative_import<'a>(module: &'a [String], import: &[&'a str], tail: &[&'a str]) -> Option<name::QualifiedName<'a>>
```

Format the call path for a relative import, or `None` if the relative import extends beyond
the root module.

---

## generate_comparison

`function` · `ruff_python_ast::helpers::generate_comparison`

```rust
fn generate_comparison(left: &Expr, ops: &[CmpOp], comparators: &[Expr], parent: AnyNodeRef<'_>, tokens: &token::Tokens, source: &str) -> String
```

---

## in_nested_block

`function` · `ruff_python_ast::helpers::in_nested_block`

```rust
fn in_nested_block<'a>(parents: impl Iterator<Item = &'a Stmt>) -> bool
```

Check if a node is in a nested block.

---

## is_assignment_to_a_dunder

`function` · `ruff_python_ast::helpers::is_assignment_to_a_dunder`

```rust
fn is_assignment_to_a_dunder(stmt: &Stmt) -> bool
```

Return `true` if the [`Stmt`] is an assignment to a dunder (like `__all__`).

---

## is_compound_statement

`function` · `ruff_python_ast::helpers::is_compound_statement`

```rust
const fn is_compound_statement(stmt: &Stmt) -> bool
```

Return `true` if the `Stmt` is a compound statement (as opposed to a simple statement).

---

## is_const_false

`function` · `ruff_python_ast::helpers::is_const_false`

```rust
const fn is_const_false(expr: &Expr) -> bool
```

Return `true` if an [`Expr`] is a literal `False`.

---

## is_const_true

`function` · `ruff_python_ast::helpers::is_const_true`

```rust
const fn is_const_true(expr: &Expr) -> bool
```

Return `true` if an [`Expr`] is a literal `True`.

---

## is_constant

`function` · `ruff_python_ast::helpers::is_constant`

```rust
fn is_constant(expr: &Expr) -> bool
```

Return `true` if the [`Expr`] is a literal or tuple of literals.

---

## is_constant_non_singleton

`function` · `ruff_python_ast::helpers::is_constant_non_singleton`

```rust
fn is_constant_non_singleton(expr: &Expr) -> bool
```

Return `true` if the [`Expr`] is a non-singleton constant.

---

## is_docstring_stmt

`function` · `ruff_python_ast::helpers::is_docstring_stmt`

```rust
fn is_docstring_stmt(stmt: &Stmt) -> bool
```

Return `true` if a `Stmt` is a docstring.

---

## is_dotted_name

`function` · `ruff_python_ast::helpers::is_dotted_name`

```rust
fn is_dotted_name(expr: &ast::Expr) -> bool
```

---

## is_dunder

`function` · `ruff_python_ast::helpers::is_dunder`

```rust
fn is_dunder(id: &str) -> bool
```

---

## is_empty_f_string

`function` · `ruff_python_ast::helpers::is_empty_f_string`

```rust
fn is_empty_f_string(expr: &ast::ExprFString) -> bool
```

Returns `true` if the expression definitely resolves to the empty string, when used as an f-string
expression.

---

## is_mutable_iterable_initializer

`function` · `ruff_python_ast::helpers::is_mutable_iterable_initializer`

```rust
const fn is_mutable_iterable_initializer(expr: &Expr) -> bool
```

Return `true` if the [`Expr`] is a mutable iterable initializer, like `{}` or `[]`.

---

## is_stub_body

`function` · `ruff_python_ast::helpers::is_stub_body`

```rust
fn is_stub_body(body: &[Stmt]) -> bool
```

Returns `true` if all statements in `body` are `pass` or `...` (ellipsis)

An empty body (`[]`) returns `false`

---

## is_sunder

`function` · `ruff_python_ast::helpers::is_sunder`

```rust
fn is_sunder(id: &str) -> bool
```

Whether a name starts and ends with a single underscore.

`_a__` is considered neither a dunder nor a sunder name.

---

## is_unpacking_assignment

`function` · `ruff_python_ast::helpers::is_unpacking_assignment`

```rust
fn is_unpacking_assignment(parent: &Stmt, child: &Expr) -> bool
```

Check if a node represents an unpacking assignment.

---

## map_callable

`function` · `ruff_python_ast::helpers::map_callable`

```rust
fn map_callable(decorator: &Expr) -> &Expr
```

Given an [`Expr`] that can be callable or not (like a decorator, which could
be used with or without explicit call syntax), return the underlying
callable.

---

## map_starred

`function` · `ruff_python_ast::helpers::map_starred`

```rust
fn map_starred(expr: &Expr) -> &Expr
```

Given an [`Expr`] that can be starred, return the underlying starred expression.

---

## map_subscript

`function` · `ruff_python_ast::helpers::map_subscript`

```rust
fn map_subscript(expr: &Expr) -> &Expr
```

Given an [`Expr`] that can be a [`ExprSubscript`][ast::ExprSubscript] or not
(like an annotation that may be generic or not), return the underlying expr.

---

## on_conditional_branch

`function` · `ruff_python_ast::helpers::on_conditional_branch`

```rust
fn on_conditional_branch<'a>(parents: &mut impl Iterator<Item = &'a Stmt>) -> bool
```

Check if a node is part of a conditional branch.

---

## pep_604_optional

`function` · `ruff_python_ast::helpers::pep_604_optional`

```rust
fn pep_604_optional(expr: &Expr) -> Expr
```

Format the expression as a PEP 604-style optional.

---

## pep_604_union

`function` · `ruff_python_ast::helpers::pep_604_union`

```rust
fn pep_604_union(elts: &[Expr]) -> Expr
```

Format the expressions as a PEP 604-style union.

---

## resolve_imported_module_path

`function` · `ruff_python_ast::helpers::resolve_imported_module_path`

```rust
fn resolve_imported_module_path<'a>(level: u32, module: Option<&'a str>, module_path: Option<&[String]>) -> Option<std::borrow::Cow<'a, str>>
```

Given an imported module (based on its relative import level and module name), return the
fully-qualified module path.

---

## side_effect

`function` · `ruff_python_ast::helpers::side_effect`

```rust
fn side_effect<F>(expr: &Expr, is_builtin: F) -> SideEffect where F: Fn(&str) -> bool
```

Return whether `expr` has no side effects, maybe has side effects, or definitely
has side effects.

Unlike [`contains_effect`], which returns a simple `bool`, this function distinguishes
between expressions that are definitely side-effect-free, definitely side-effectful,
and those that may invoke user-defined code (e.g., formatting a non-literal f-string
interpolation can call `__format__` or `__str__`).

---

## to_module_path

`function` · `ruff_python_ast::helpers::to_module_path`

```rust
fn to_module_path(package: &std::path::Path, path: &std::path::Path) -> Option<Vec<String>>
```

Create a module path from a (package, path) pair.

For example, if the package is `foo/bar` and the path is `foo/bar/baz.py`,
the call path is `["baz"]`.

---

## typing_optional

`function` · `ruff_python_ast::helpers::typing_optional`

```rust
fn typing_optional(elt: Expr, binding: name::Name) -> Expr
```

Format the expression as a `typing.Optional`-style optional.

---

## typing_union

`function` · `ruff_python_ast::helpers::typing_union`

```rust
fn typing_union(elts: &[Expr], binding: name::Name) -> Expr
```

Format the expressions as a `typing.Union`-style union.

Note: It is a syntax error to have `Union[]` so the caller
should ensure that the `elts` argument is nonempty.

---

## uses_magic_variable_access

`function` · `ruff_python_ast::helpers::uses_magic_variable_access`

```rust
fn uses_magic_variable_access<F>(body: &[Stmt], is_builtin: F) -> bool where F: Fn(&str) -> bool
```

Return `true` if the body uses `locals()`, `globals()`, `vars()`, `eval()`.

Accepts a closure that determines whether a given name (e.g., `"list"`) is a Python builtin.

---

## AwaitVisitor

`struct` · `ruff_python_ast::helpers::AwaitVisitor`

```rust
struct AwaitVisitor
```

**Fields**: `seen_await`

**Implements**: `ruff_python_ast::visitor::Visitor`

**Derives**: Debug, Default

**via `ruff_python_ast::visitor::Visitor`**

```rust
fn visit_comprehension(&mut self, comprehension: &Comprehension)
fn visit_expr(&mut self, expr: &Expr)
fn visit_stmt(&mut self, stmt: &Stmt)
```

A [`Visitor`] that detects the presence of `await` expressions in the current scope.

---

## NameFinder

`struct` · `ruff_python_ast::helpers::NameFinder`

```rust
struct NameFinder<'a>
```

**Fields**: `names`

**Implements**: `ruff_python_ast::visitor::Visitor`

**Derives**: Debug, Default

**via `ruff_python_ast::visitor::Visitor`**

```rust
fn visit_expr(&mut self, expr: &'a Expr)
```

A [`Visitor`] to collect all [`Expr::Name`] nodes in an AST.

---

## RaiseStatementVisitor

`struct` · `ruff_python_ast::helpers::RaiseStatementVisitor`

```rust
struct RaiseStatementVisitor<'a>
```

**Fields**: `raises`

**Implements**: `ruff_python_ast::statement_visitor::StatementVisitor`

**Derives**: Default

**via `ruff_python_ast::statement_visitor::StatementVisitor`**

```rust
fn visit_stmt(&mut self, stmt: &'a Stmt)
```

A [`StatementVisitor`] that collects all `raise` statements in a function or method.

---

## ReturnStatementVisitor

`struct` · `ruff_python_ast::helpers::ReturnStatementVisitor`

```rust
struct ReturnStatementVisitor<'a>
```

**Fields**: `returns`, `is_generator`

**Implements**: `ruff_python_ast::visitor::Visitor`

**Derives**: Default

**via `ruff_python_ast::visitor::Visitor`**

```rust
fn visit_expr(&mut self, expr: &'a Expr)
fn visit_stmt(&mut self, stmt: &'a Stmt)
```

A [`Visitor`] that collects all `return` statements in a function or method.

---

## StoredNameFinder

`struct` · `ruff_python_ast::helpers::StoredNameFinder`

```rust
struct StoredNameFinder<'a>
```

**Fields**: `names`

**Implements**: `ruff_python_ast::visitor::Visitor`

**Derives**: Debug, Default

**via `ruff_python_ast::visitor::Visitor`**

```rust
fn visit_expr(&mut self, expr: &'a Expr)
```

A [`Visitor`] to collect all stored [`Expr::Name`] nodes in an AST.

---
