# `arrow_pyarrow::Table`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.Table.json).

<a id="op-758e308e4d06e3cbb65d6e7f"></a>
## Table

`struct` · `arrow_pyarrow::Table` · arrow-pyarrow 59.3.0

```rust
struct Table
```

Source: `src/lib.rs:450`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

This is a convenience wrapper around `Vec<RecordBatch>` that tries to simplify conversion from
and to `pyarrow.Table`.

This could be used in circumstances where you either want to consume a `pyarrow.Table` directly
(although technically, since `pyarrow.Table` implements the ArrayStreamReader PyCapsule
interface, one could also consume a `PyArrowType<ArrowArrayStreamReader>` instead) or, more
importantly, where one wants to export a `pyarrow.Table` from a `Vec<RecordBatch>` from the Rust
side.

```ignore
#[pyfunction]
fn return_table(...) -> PyResult<PyArrowType<Table>> {
    let batches: Vec<RecordBatch>;
    let schema: SchemaRef;
    PyArrowType(Table::try_new(batches, schema).map_err(|err| err.into_py_err(py))?)
}
```

<a id="op-c2c90f3098c07d98257b144e"></a>
## Error

`assoc_type` · `arrow_pyarrow::Table::Error` · arrow-pyarrow 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [497, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:490`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6237921790a88418f6aac02f"></a>
## clone

`function` · `arrow_pyarrow::Table::clone` · arrow-pyarrow 59.3.0

```rust
fn clone(&self) -> Table
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [449, 10], "end": [449, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb2502d2da3ca48f56098edd"></a>
## from_pyarrow_bound

`function` · `arrow_pyarrow::Table::from_pyarrow_bound` · arrow-pyarrow 59.3.0

```rust
fn from_pyarrow_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [506, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "arrow_pyarrow::FromPyArrow", "path": "FromPyArrow"}, "trait_path": "arrow_pyarrow::FromPyArrow"}`

Source: `src/lib.rs:501`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15efa8022f0b8c477f208057"></a>
## into_inner

`function` · `arrow_pyarrow::Table::into_inner` · arrow-pyarrow 59.3.0

```rust
fn into_inner(self) -> (Vec<RecordBatch>, SchemaRef)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [487, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:484`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1396bcae5b270a03d8b9c0d2"></a>
## into_pyarrow

`function` · `arrow_pyarrow::Table::into_pyarrow` · arrow-pyarrow 59.3.0

```rust
fn into_pyarrow(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [519, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "arrow_pyarrow::IntoPyArrow", "path": "IntoPyArrow"}, "trait_path": "arrow_pyarrow::IntoPyArrow"}`

Source: `src/lib.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62d4b102bca963b7d279c83d"></a>
## record_batches

`function` · `arrow_pyarrow::Table::record_batches` · arrow-pyarrow 59.3.0

```rust
fn record_batches(&self) -> &[RecordBatch]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [487, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:476`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0e04fec87ba1858099fa984"></a>
## schema

`function` · `arrow_pyarrow::Table::schema` · arrow-pyarrow 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [487, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:480`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d881bd272e3712f6bad56116"></a>
## try_from

`function` · `arrow_pyarrow::Table::try_from` · arrow-pyarrow 59.3.0

```rust
fn try_from(value: Box<dyn RecordBatchReader>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [489, 1], "end": [497, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/lib.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce6df25d20b5d2c67af6271"></a>
## try_new

`function` · `arrow_pyarrow::Table::try_new` · arrow-pyarrow 59.3.0

```rust
fn try_new(record_batches: Vec<RecordBatch>, schema: SchemaRef) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_pyarrow::Table", "path": "Table"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [487, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:456`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
