# `ruff_python_semantic::analyze::imports`

Crate `ruff_python_semantic` · 5 public items · structured records in [`model/ruff_python_semantic.analyze.imports.json`](../model/ruff_python_semantic.analyze.imports.json)

## is_matplotlib_activation

`function` · `ruff_python_semantic::analyze::imports::is_matplotlib_activation`

```rust
fn is_matplotlib_activation(stmt: &ruff_python_ast::Stmt, semantic: &SemanticModel<'_>) -> bool
```

Returns `true` if a [`Stmt`] is a `matplotlib.use` activation, as in:
```python
import matplotlib

matplotlib.use("Agg")
```

---

## is_os_environ_modification

`function` · `ruff_python_semantic::analyze::imports::is_os_environ_modification`

```rust
fn is_os_environ_modification(stmt: &ruff_python_ast::Stmt, semantic: &SemanticModel<'_>) -> bool
```

Returns `true` if a [`Stmt`] is an `os.environ` modification, as in:
```python
import os

os.environ["CUDA_VISIBLE_DEVICES"] = "4"
```

---

## is_pytest_importorskip

`function` · `ruff_python_semantic::analyze::imports::is_pytest_importorskip`

```rust
fn is_pytest_importorskip(stmt: &ruff_python_ast::Stmt, semantic: &SemanticModel<'_>) -> bool
```

Returns `true` if a [`Stmt`] is a `pytest.importorskip()` call, as in:
```python
import pytest

pytest.importorskip("foo.bar")
```

---

## is_site_sys_path_modification

`function` · `ruff_python_semantic::analyze::imports::is_site_sys_path_modification`

```rust
fn is_site_sys_path_modification(stmt: &ruff_python_ast::Stmt, semantic: &SemanticModel<'_>) -> bool
```

Returns `true` if a [`Stmt`] is a dynamic modification of the Python
module search path, e.g.,
```python
import site

site.addsitedir(...)
```

---

## is_sys_path_modification

`function` · `ruff_python_semantic::analyze::imports::is_sys_path_modification`

```rust
fn is_sys_path_modification(stmt: &ruff_python_ast::Stmt, semantic: &SemanticModel<'_>) -> bool
```

Returns `true` if a [`Stmt`] is a `sys.path` modification, as in:
```python
import sys

sys.path.append("../")
sys.path += ["../"]
```

---
