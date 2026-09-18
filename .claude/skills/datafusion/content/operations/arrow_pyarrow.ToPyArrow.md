# `arrow_pyarrow::ToPyArrow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.ToPyArrow.json).

<a id="op-00f921f5211cec7bf7633527"></a>
## ToPyArrow

`trait` · `arrow_pyarrow::ToPyArrow` · arrow-pyarrow 59.3.0

```rust
trait ToPyArrow
```

Source: `src/lib.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Create a new PyArrow object from a arrow-rs type.

<a id="op-d7e1a12c651a83ff62a44a2d"></a>
## to_pyarrow

`function` · `arrow_pyarrow::ToPyArrow::to_pyarrow` · arrow-pyarrow 59.3.0

```rust
fn to_pyarrow<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>
```

Source: `src/lib.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Convert the implemented type into a Python object without consuming it.
