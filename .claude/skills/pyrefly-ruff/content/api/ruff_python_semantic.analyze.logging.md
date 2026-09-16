# `ruff_python_semantic::analyze::logging`

Crate `ruff_python_semantic` · 2 public items · structured records in [`model/ruff_python_semantic.analyze.logging.json`](../model/ruff_python_semantic.analyze.logging.json)

## exc_info

`function` · `ruff_python_semantic::analyze::logging::exc_info`

```rust
fn exc_info<'a>(arguments: &'a ruff_python_ast::Arguments, semantic: &model::SemanticModel<'_>) -> Option<&'a ruff_python_ast::Keyword>
```

If the keywords to a logging call contain `exc_info=True` or `exc_info=sys.exc_info()`,
return the `Keyword` for `exc_info`.

---

## is_logger_candidate

`function` · `ruff_python_semantic::analyze::logging::is_logger_candidate`

```rust
fn is_logger_candidate(func: &ruff_python_ast::Expr, semantic: &model::SemanticModel<'_>, logger_objects: &[String]) -> bool
```

Return `true` if the given `Expr` is a potential logging call. Matches
`logging.error`, `logger.error`, `self.logger.error`, etc., but not
arbitrary `foo.error` calls.

It also matches direct `logging.error` calls when the `logging` module
is aliased. Example:
```python
import logging as bar

# This is detected to be a logger candidate.
bar.error()
```

---
