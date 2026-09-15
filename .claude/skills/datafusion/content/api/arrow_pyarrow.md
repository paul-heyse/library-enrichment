# `arrow_pyarrow`

Crate `arrow-pyarrow` · 7 public items · structured records in [`model/arrow_pyarrow.json`](../model/arrow_pyarrow.json)

## ArrowException

`struct` · `arrow_pyarrow::ArrowException`

Also reachable as `arrow::pyarrow::ArrowException`

```rust
struct ArrowException
```

**Implements**: `pyo3::err::ToPyErr`, `pyo3::type_object::PyTypeInfo`, `pyo3::types::DerefToPyAny`

**Methods** (1)

```rust
fn new_err<A>(args: A) -> PyErr where A: PyErrArguments + ::core::marker::Send + ::core::marker::Sync + 'static
```

**via `pyo3::type_object::PyTypeInfo`**

```rust
fn type_object_raw(py: Python<'_>) -> *mut ffi::PyTypeObject
```

A Rust type representing an exception defined in Python code.

This type was created by the [`pyo3::import_exception!`] macro - see its documentation
for more information.

[`pyo3::import_exception!`]: https://docs.rs/pyo3/latest/pyo3/macro.import_exception.html "import_exception in pyo3"

---

## PyArrowType

`struct` · `arrow_pyarrow::PyArrowType`

Also reachable as `arrow::pyarrow::PyArrowType`

```rust
struct PyArrowType<T>
```

**Implements**: `core::convert::From`, `pyo3::conversion::FromPyObject`, `pyo3::conversion::IntoPyObject`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: T) -> Self
```

**via `pyo3::conversion::FromPyObject`**

```rust
fn extract(value: Borrowed<'_, '_, PyAny>) -> PyResult<Self>
```

**via `pyo3::conversion::IntoPyObject`**

```rust
fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output>
```

A newtype wrapper for types implementing [`FromPyArrow`] or [`IntoPyArrow`].

When wrapped around a type `T: FromPyArrow`, it
implements [`FromPyObject`] for the PyArrow objects. When wrapped around a
`T: IntoPyArrow`, it implements `IntoPy<PyObject>` for the wrapped type.

---

## Table

`struct` · `arrow_pyarrow::Table`

Also reachable as `arrow::pyarrow::Table`

```rust
struct Table
```

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::IntoPyArrow`, `core::convert::TryFrom`

**Derives**: Clone

**Methods** (4)

```rust
fn into_inner(self) -> (Vec<RecordBatch>, SchemaRef)
fn record_batches(&self) -> &[RecordBatch]
fn schema(&self) -> SchemaRef
fn try_new(record_batches: Vec<RecordBatch>, schema: SchemaRef) -> Result<Self, ArrowError>
```

**via `arrow_pyarrow::FromPyArrow`**

```rust
fn from_pyarrow_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self>
```

**via `arrow_pyarrow::IntoPyArrow`**

```rust
fn into_pyarrow(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>>
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: Box<dyn RecordBatchReader>) -> Result<Self, ArrowError>
```

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

---

## FromPyArrow

`trait` · `arrow_pyarrow::FromPyArrow`

Also reachable as `arrow::pyarrow::FromPyArrow`

```rust
trait FromPyArrow: Sized
```

**Implementors** (8)

- `alloc::vec::Vec`
- `arrow_array::ffi_stream::ArrowArrayStreamReader`
- `arrow_array::record_batch::RecordBatch`
- `arrow_data::data::ArrayData`
- `arrow_pyarrow::Table`
- `arrow_schema::datatype::DataType`
- `arrow_schema::field::Field`
- `arrow_schema::schema::Schema`

**Methods** (1)

```rust
fn from_pyarrow_bound(value: &Bound<'_, PyAny>) -> PyResult<Self>
```

Trait for converting Python objects to arrow-rs types.

---

## IntoPyArrow

`trait` · `arrow_pyarrow::IntoPyArrow`

Also reachable as `arrow::pyarrow::IntoPyArrow`

```rust
trait IntoPyArrow
```

**Implementors** (3)

- `alloc::boxed::Box`
- `arrow_array::ffi_stream::ArrowArrayStreamReader`
- `arrow_pyarrow::Table`

**Methods** (1)

```rust
fn into_pyarrow<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>
```

Convert an arrow-rs type into a PyArrow object.

---

## ToPyArrow

`trait` · `arrow_pyarrow::ToPyArrow`

Also reachable as `arrow::pyarrow::ToPyArrow`

```rust
trait ToPyArrow
```

**Implementors** (6)

- `alloc::vec::Vec`
- `arrow_array::record_batch::RecordBatch`
- `arrow_data::data::ArrayData`
- `arrow_schema::datatype::DataType`
- `arrow_schema::field::Field`
- `arrow_schema::schema::Schema`

**Methods** (1)

```rust
fn to_pyarrow<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>
```

Create a new PyArrow object from a arrow-rs type.

---

## PyArrowException

`type_alias` · `arrow_pyarrow::PyArrowException`

Also reachable as `arrow::pyarrow::PyArrowException`

```rust
type PyArrowException = ArrowException
```

Represents an exception raised by PyArrow.

---
