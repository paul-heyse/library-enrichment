# `pyrefly_types::meta_shape_dsl`

Crate `pyrefly_types` · 6 public items · structured records in [`model/pyrefly_types.meta_shape_dsl.json`](../model/pyrefly_types.meta_shape_dsl.json)

## convert_shape_dsl_function

`function` · `pyrefly_types::meta_shape_dsl::convert_shape_dsl_function`

```rust
fn convert_shape_dsl_function(func: &ruff_python_ast::StmtFunctionDef) -> Result<ShapeDslFunction, DslCompileError>
```

Convert a single Python function definition into a [`ShapeDslFunction`].

This is pure AST-to-IR lowering — it does not parse source text or run
the type checker. The output is a single opaque handle; the caller is
expected to combine handles from this function (and possibly other
modules) via [`validate_shape_dsl_functions`].

Returns `Err` with a terse description if the function body uses Python
syntax outside the DSL subset.

---

## validate_shape_dsl_functions

`function` · `pyrefly_types::meta_shape_dsl::validate_shape_dsl_functions`

```rust
fn validate_shape_dsl_functions(fns: &[std::sync::Arc<ShapeDslFunction>]) -> Result<(), Vec<DslCompileError>>
```

Validate a set of `ShapeDslFunction`s as a program.

Runs `type_check_program` on the inner `DslFnDef`s, verifying that
cross-function calls have consistent signatures. Returns collected
type error messages on failure.

Also rejects programs whose call graph (restricted to `fns`) contains
a cycle, since the DSL evaluator does not support recursion.

Intended to be called with a per-caller transitive closure (root +
its resolved helpers), not the full module.

---

## DslCompileError

`struct` · `pyrefly_types::meta_shape_dsl::DslCompileError`

```rust
struct DslCompileError
```

**Fields**: `range`, `message`

**Derives**: Clone, Debug

A structured error from DSL compilation (parsing or type-checking), carrying
the source range of the problematic construct so callers can emit precise
diagnostics without resorting to a function-wide fallback range.

---

## ShapeDslFunction

`struct` · `pyrefly_types::meta_shape_dsl::ShapeDslFunction`

```rust
struct ShapeDslFunction
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (2)

```rust
fn call_targets(&self) -> HashSet<String>
fn name(&self) -> &str
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, _ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

A single DSL function that has been lowered from its Python AST.

This is a cheap (one `Arc`) opaque handle produced by
[`convert_shape_dsl_function`] and consumed by [`validate_shape_dsl_functions`].

---

## ShapeTransform

`struct` · `pyrefly_types::meta_shape_dsl::ShapeTransform`

```rust
struct ShapeTransform
```

**Fields**: `dsl_fn`, `fn_closure`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn to_meta_shape_function(&self) -> Box<dyn MetaShapeFunction>
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, _ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

Reference to a shape-DSL function that refines a callable's return type.
Carried on `FuncFlags` for functions decorated with `@uses_shape_dsl`.

---

## MetaShapeFunction

`trait` · `pyrefly_types::meta_shape_dsl::MetaShapeFunction`

```rust
trait MetaShapeFunction: Debug + Send + Sync
```

**Implementors** (1)

- `pyrefly_types::meta_shape_dsl::DslMetaShapeFunction`

**Methods** (3)

```rust
fn evaluate(&self, bound_args: &HashMap<String, Type>, ret_type: &Type) -> Option<Result<Type, ShapeError>>
fn name(&self) -> &str
fn param_names(&self) -> Vec<&str>
```

A function that computes output shapes from input shapes.

The `evaluate` method takes bound arguments (from the call site) and the
fixture return type, and produces the refined return type directly.
This is symmetric with parameter binding: on the way in, `(Type, DslType) → Val`;
on the way out, `(Val, DslType, Type) → Type`.

---
