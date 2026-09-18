# `arrow_pyarrow::PyArrowType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.PyArrowType.json).

<a id="op-8e2dfc1f6334b7c75dc2db6f"></a>
## PyArrowType

`struct` · `arrow_pyarrow::PyArrowType` · arrow-pyarrow 59.3.0

```rust
struct PyArrowType<T>
```

Source: `src/lib.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

A newtype wrapper for types implementing [`FromPyArrow`](../operations/arrow_pyarrow.FromPyArrow.md#op-60d4c111111a10620d247475) or [`IntoPyArrow`](../operations/arrow_pyarrow.IntoPyArrow.md#op-ef53c9f088ce9b4a0ab948f0).

When wrapped around a type `T: FromPyArrow`, it
implements [`FromPyObject`] for the PyArrow objects. When wrapped around a
`T: IntoPyArrow`, it implements `IntoPy<PyObject>` for the wrapped type.

Unresolved upstream links (retained, not inferred): ``FromPyObject``.

<a id="op-1088d8fe15bc86628995cb3b"></a>
## 0

`struct_field` · `arrow_pyarrow::PyArrowType::0` · arrow-pyarrow 59.3.0

```rust
0: T
```

Source: `src/lib.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0472224cb87c033cce048a7a"></a>
## Error

`assoc_type` · `arrow_pyarrow::PyArrowType::Error` · arrow-pyarrow 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'py"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::IntoPyArrow", "path": "IntoPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [582, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'py"}], "constraints": []}}, "id": "pyo3::conversion::IntoPyObject", "path": "IntoPyObject"}, "trait_path": "pyo3::conversion::IntoPyObject"}`

Source: `src/lib.rs:577`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7b74c7b24c2c175445161b3"></a>
## Error

`assoc_type` · `arrow_pyarrow::PyArrowType::Error` · arrow-pyarrow 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::FromPyArrow", "path": "FromPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 1], "end": [570, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "pyo3::conversion::FromPyObject", "path": "FromPyObject"}, "trait_path": "pyo3::conversion::FromPyObject"}`

Source: `src/lib.rs:565`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9d4df896d047223465d3542"></a>
## Output

`assoc_type` · `arrow_pyarrow::PyArrowType::Output` · arrow-pyarrow 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'py"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::IntoPyArrow", "path": "IntoPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [582, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'py"}], "constraints": []}}, "id": "pyo3::conversion::IntoPyObject", "path": "IntoPyObject"}, "trait_path": "pyo3::conversion::IntoPyObject"}`

Source: `src/lib.rs:575`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f8e287c2c2d08212dbec72"></a>
## Target

`assoc_type` · `arrow_pyarrow::PyArrowType::Target` · arrow-pyarrow 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'py"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::IntoPyArrow", "path": "IntoPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [582, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'py"}], "constraints": []}}, "id": "pyo3::conversion::IntoPyObject", "path": "IntoPyObject"}, "trait_path": "pyo3::conversion::IntoPyObject"}`

Source: `src/lib.rs:573`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a9f9be889b97a6592a668ca"></a>
## extract

`function` · `arrow_pyarrow::PyArrowType::extract` · arrow-pyarrow 59.3.0

```rust
fn extract(value: Borrowed<'_, '_, PyAny>) -> PyResult<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::FromPyArrow", "path": "FromPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [564, 1], "end": [570, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}, {"lifetime": "'_"}], "constraints": []}}, "id": "pyo3::conversion::FromPyObject", "path": "FromPyObject"}, "trait_path": "pyo3::conversion::FromPyObject"}`

Source: `src/lib.rs:567`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-429310ff5efd43c81e364507"></a>
## fmt

`function` · `arrow_pyarrow::PyArrowType::fmt` · arrow-pyarrow 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [561, 10], "end": [561, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c97d369793c1f2979057109"></a>
## from

`function` · `arrow_pyarrow::PyArrowType::from` · arrow-pyarrow 59.3.0

```rust
fn from(s: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [584, 1], "end": [588, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:585`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01b3b4214d76a631cc1fcefc"></a>
## into_pyobject

`function` · `arrow_pyarrow::PyArrowType::into_pyobject` · arrow-pyarrow 59.3.0

```rust
fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_pyarrow::PyArrowType", "path": "PyArrowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'py"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_pyarrow::IntoPyArrow", "path": "IntoPyArrow"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [582, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'py"}], "constraints": []}}, "id": "pyo3::conversion::IntoPyObject", "path": "IntoPyObject"}, "trait_path": "pyo3::conversion::IntoPyObject"}`

Source: `src/lib.rs:579`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
