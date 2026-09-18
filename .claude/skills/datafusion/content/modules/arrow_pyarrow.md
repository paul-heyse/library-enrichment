# `arrow_pyarrow`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_pyarrow.json).

<a id="op-c44ece0b78b0605a609c7981"></a>
## arrow_pyarrow

`module` · `arrow_pyarrow` · arrow-pyarrow 59.3.0

```rust
mod arrow_pyarrow
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-pyarrow/59.3.0/json).

Pass Arrow objects from and to PyArrow, using Arrow's
[C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html)
and [pyo3](https://docs.rs/pyo3/latest/pyo3/).

For underlying implementation, see the [ffi](../modules/arrow_array.ffi.md#op-b725a1de54eced6ce05f9387) module.

One can use these to write Python functions that take and return PyArrow
objects, with automatic conversion to corresponding arrow-rs types.

```ignore
#[pyfunction]
fn double_array(array: PyArrowType<ArrayData>) -> PyResult<PyArrowType<ArrayData>> {
    let array = array.0; // Extract from PyArrowType wrapper
    let array: Arc<dyn Array> = make_array(array); // Convert ArrayData to ArrayRef
    let array: &Int32Array = array.as_any().downcast_ref()
        .ok_or_else(|| PyValueError::new_err("expected int32 array"))?;
    let array: Int32Array = array.iter().map(|x| x.map(|x| x * 2)).collect();
    Ok(PyArrowType(array.into_data()))
}
```

| pyarrow type                | arrow-rs type                                                      |
|-----------------------------|--------------------------------------------------------------------|
| `pyarrow.DataType`          | [DataType](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)                                                         |
| `pyarrow.Field`             | [Field](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)                                                            |
| `pyarrow.Schema`            | [Schema](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050)                                                           |
| `pyarrow.Array`             | [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)                                                        |
| `pyarrow.RecordBatch`       | [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)                                                      |
| `pyarrow.RecordBatchReader` | [ArrowArrayStreamReader](../operations/arrow_array.ffi_stream.ArrowArrayStreamReader.md#op-0390051e3be11e60c2b7f3d0) / `Box<dyn RecordBatchReader + Send>` (1) |
| `pyarrow.Table`             | [Table](../operations/arrow_pyarrow.Table.md#op-758e308e4d06e3cbb65d6e7f) (2)                                                        |

(1) `pyarrow.RecordBatchReader` can be imported as [ArrowArrayStreamReader](../operations/arrow_array.ffi_stream.ArrowArrayStreamReader.md#op-0390051e3be11e60c2b7f3d0). Either
[ArrowArrayStreamReader](../operations/arrow_array.ffi_stream.ArrowArrayStreamReader.md#op-0390051e3be11e60c2b7f3d0) or `Box<dyn RecordBatchReader + Send>` can be exported
as `pyarrow.RecordBatchReader`. (`Box<dyn RecordBatchReader + Send>` is typically
easier to create.)

(2) Although arrow-rs offers [Table](../operations/arrow_pyarrow.Table.md#op-758e308e4d06e3cbb65d6e7f), a convenience wrapper for [pyarrow.Table](https://arrow.apache.org/docs/python/generated/pyarrow.Table)
that internally holds `Vec<RecordBatch>`, it is meant primarily for use cases where you already
have `Vec<RecordBatch>` on the Rust side and want to export that in bulk as a `pyarrow.Table`.
In general, it is recommended to use streaming approaches instead of dealing with data in bulk.
For example, a `pyarrow.Table` (or any other object that implements the ArrayStream PyCapsule
interface) can be imported to Rust through `PyArrowType<ArrowArrayStreamReader>` instead of
forcing eager reading into `Vec<RecordBatch>`.
