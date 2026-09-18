# `arrow_pyarrow::ArrowException`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.ArrowException.json).

<a id="op-711a06fd417a60d59c8dacbb"></a>
## ArrowException

`struct` · `arrow_pyarrow::ArrowException` · arrow-pyarrow 59.3.0

```rust
struct ArrowException
```

Source: `src/lib.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

A Rust type representing an exception defined in Python code.

This type was created by the [`pyo3::import_exception!`] macro - see its documentation
for more information.

[`pyo3::import_exception!`]: https://docs.rs/pyo3/latest/pyo3/macro.import_exception.html "import_exception in pyo3"

<a id="op-8d39b1e93ed9426c6b0972c7"></a>
## MODULE

`assoc_const` · `arrow_pyarrow::ArrowException::MODULE` · arrow-pyarrow 59.3.0

```rust
MODULE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::ArrowException", "path": "ArrowException"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [83, 43], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "pyo3::type_object::PyTypeInfo", "path": "PyTypeInfo"}, "trait_path": "pyo3::type_object::PyTypeInfo"}`

Source: `src/lib.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4e61d33cab502ac8ee3ae83"></a>
## NAME

`assoc_const` · `arrow_pyarrow::ArrowException::NAME` · arrow-pyarrow 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::ArrowException", "path": "ArrowException"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [83, 43], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "pyo3::type_object::PyTypeInfo", "path": "PyTypeInfo"}, "trait_path": "pyo3::type_object::PyTypeInfo"}`

Source: `src/lib.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88d8d95c9dbe43d7f07ee1eb"></a>
## new_err

`function` · `arrow_pyarrow::ArrowException::new_err` · arrow-pyarrow 59.3.0

```rust
fn new_err<A>(args: A) -> PyErr where A: PyErrArguments + ::core::marker::Send + ::core::marker::Sync + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::ArrowException", "path": "ArrowException"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [83, 43], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Creates a new [`PyErr`] of this type.

[`PyErr`]: https://docs.rs/pyo3/latest/pyo3/struct.PyErr.html "PyErr in pyo3"

<a id="op-02cbfae67ccfbcbb151215d3"></a>
## type_object_raw

`function` · `arrow_pyarrow::ArrowException::type_object_raw` · arrow-pyarrow 59.3.0

```rust
fn type_object_raw(py: Python<'_>) -> *mut ffi::PyTypeObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::ArrowException", "path": "ArrowException"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [83, 43], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "pyo3::type_object::PyTypeInfo", "path": "PyTypeInfo"}, "trait_path": "pyo3::type_object::PyTypeInfo"}`

Source: `src/lib.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
