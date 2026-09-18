# `arrow_pyarrow::IntoPyArrow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.IntoPyArrow.json).

<a id="op-ef53c9f088ce9b4a0ab948f0"></a>
## IntoPyArrow

`trait` · `arrow_pyarrow::IntoPyArrow` · arrow-pyarrow 59.3.0

```rust
trait IntoPyArrow
```

Source: `src/lib.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Convert an arrow-rs type into a PyArrow object.

<a id="op-c06444549df980e2d6082e05"></a>
## into_pyarrow

`function` · `arrow_pyarrow::IntoPyArrow::into_pyarrow` · arrow-pyarrow 59.3.0

```rust
fn into_pyarrow<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>
```

Source: `src/lib.rs:108`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Convert the implemented type into a Python object while consuming it.
