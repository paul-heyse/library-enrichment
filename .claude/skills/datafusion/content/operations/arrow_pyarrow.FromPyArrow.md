# `arrow_pyarrow::FromPyArrow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.FromPyArrow.json).

<a id="op-60d4c111111a10620d247475"></a>
## FromPyArrow

`trait` · `arrow_pyarrow::FromPyArrow` · arrow-pyarrow 59.3.0

```rust
trait FromPyArrow: Sized
```

Source: `src/lib.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Trait for converting Python objects to arrow-rs types.

<a id="op-bf19176c5268818661560e2d"></a>
## from_pyarrow_bound

`function` · `arrow_pyarrow::FromPyArrow::from_pyarrow_bound` · arrow-pyarrow 59.3.0

```rust
fn from_pyarrow_bound(value: &Bound<'_, PyAny>) -> PyResult<Self>
```

Source: `src/lib.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Convert a Python object to an arrow-rs type.

Takes a GIL-bound value from Python and returns a result with the arrow-rs type.
