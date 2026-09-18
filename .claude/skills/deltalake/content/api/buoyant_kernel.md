# `buoyant_kernel`

Crate `buoyant_kernel` · 20 public items · structured records in [`model/buoyant_kernel.json`](../model/buoyant_kernel.json)

## transform_output_type

`macro` · `buoyant_kernel::transform_output_type`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transform_output_type.md)

Also reachable as `buoyant_kernel::transforms::transform_output_type`, `delta_kernel::transform_output_type`

```rust
macro_rules! transform_output_type
```

Defines a transform's `Output` and `Residual` associated types.

Example: fallible schema visitor
```rust,no_run
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transform_output_type;
# use delta_kernel::schema::StructField;
# use delta_kernel::transforms::SchemaTransform;
# use delta_kernel::DeltaResult;
struct Validate;

impl<'a> SchemaTransform<'a> for Validate {
    transform_output_type!(|'a, T| DeltaResult<()>);

    fn transform_struct_field(&mut self, _field: &'a StructField) -> DeltaResult<()> {
        todo!()
    }
}
```
# use buoyant_kernel as delta_kernel;

Example: infallible filtering expression transform
```rust,no_run
# use buoyant_kernel as delta_kernel;
# use std::borrow::Cow;
# use delta_kernel::transform_output_type;
# use delta_kernel::expressions::ColumnName;
# use delta_kernel::transforms::ExpressionTransform;
struct KeepSomeColumns;

impl<'a> ExpressionTransform<'a> for KeepSomeColumns {
    transform_output_type!(|'a, T| Option<Cow<'a, T>>);

    fn transform_expr_column(&mut self, _name: &'a ColumnName) -> Option<Cow<'a, ColumnName>> {
        todo!()
    }
}
```

---

## FileMeta

`struct` · `buoyant_kernel::FileMeta`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileMeta.md)

Also reachable as `delta_kernel::FileMeta`

```rust
struct FileMeta
```

**Fields**: `location`, `last_modified`, `size`

**Implements**: `buoyant_kernel::path::AsUrl`, `core::convert::TryFrom`, `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(location: Url, last_modified: i64, size: u64) -> Self
```

**via `buoyant_kernel::path::AsUrl`**

```rust
fn as_url(&self) -> &Url
```

**via `core::convert::TryFrom`**

```rust
fn try_from(ent: DirEntry) -> DeltaResult<FileMeta>
```

The metadata that describes an object.

---

## ParquetFooter

`struct` · `buoyant_kernel::ParquetFooter`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.ParquetFooter.md)

Also reachable as `delta_kernel::ParquetFooter`

```rust
struct ParquetFooter
```

**Fields**: `schema`

**Derives**: Clone, Debug

Metadata from a Parquet file footer.

This struct contains metadata extracted from a Parquet file's footer, including the schema.
It is designed to be extensible for future additions such as row group statistics.

---

## AsAny

`trait` · `buoyant_kernel::AsAny`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.AsAny.md)

Also reachable as `delta_kernel::AsAny`

```rust
trait AsAny: Any + Send + Sync
```

**Methods** (4)

```rust
fn any_ref(&self) -> &dyn Any + Send + Sync
fn as_any(Arc<self>) -> Arc<dyn Any + Send + Sync>
fn into_any(Box<self>) -> Box<dyn Any + Send + Sync>
fn type_name(&self) -> &'static str
```

Extension trait that makes it easier to work with traits objects that implement [`Any`],
implemented automatically for any type that satisfies `Any`, `Send`, and `Sync`. In particular,
given some `trait T: Any + Send + Sync`, it allows upcasting `T` to `dyn Any + Send + Sync`,
which in turn allows downcasting the result to a concrete type.

For example, the following code will compile:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::AsAny;
# use std::any::Any;
# use std::sync::Arc;
trait Foo : AsAny {}
struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let a: Arc<dyn Any + Send + Sync> = f.as_any();
let b: Arc<Bar> = a.downcast().unwrap();
```

In contrast, very similar code that relies only on `Any` would fail to compile:

```fail_compile
# use std::any::Any;
# use std::sync::Arc;
trait Foo: Any + Send + Sync {}

struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let b: Arc<Bar> = f.downcast().unwrap(); // `Arc::downcast` method not found
```

As would this:

```fail_compile
# use std::any::Any;
# use std::sync::Arc;
trait Foo: Any + Send + Sync {}

struct Bar;
impl Foo for Bar {}

let f: Arc<dyn Foo> = Arc::new(Bar);
let a: Arc<dyn Any + Send + Sync> = f; // trait upcasting coercion is not stable rust
let f: Arc<Bar> = a.downcast().unwrap();
```

NOTE: `AsAny` inherits the `Send + Sync` constraint from [`Arc::downcast`].

---

## DynPartialEq

`trait` · `buoyant_kernel::DynPartialEq`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.DynPartialEq.md)

Also reachable as `delta_kernel::DynPartialEq`

```rust
trait DynPartialEq: AsAny
```

**Methods** (1)

```rust
fn dyn_eq(&self, other: &dyn Any) -> bool
```

Extension trait that facilitates object-safe implementations of `PartialEq`.

---

## Engine

`trait` · `buoyant_kernel::Engine`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.Engine.md)

Also reachable as `delta_kernel::Engine`

```rust
trait Engine: AsAny
```

**Implementors** (3)

- `buoyant_kernel::metrics::metered_engine::MeteredDeltaEngine`
- `buoyant_kernel_engine::DefaultEngine`
- `deltalake_core::delta_datafusion::engine::DataFusionEngine`

**Methods** (4)

```rust
fn evaluation_handler(&self) -> Arc<dyn EvaluationHandler>
fn json_handler(&self) -> Arc<dyn JsonHandler>
fn parquet_handler(&self) -> Arc<dyn ParquetHandler>
fn storage_handler(&self) -> Arc<dyn StorageHandler>
```

The `Engine` trait encapsulates all the functionality an engine or connector needs to provide
to the Delta Kernel in order to read the Delta table.

Engines/Connectors are expected to pass an implementation of this trait when reading a Delta
table.

---

## EvaluationHandler

`trait` · `buoyant_kernel::EvaluationHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.EvaluationHandler.md)

Also reachable as `delta_kernel::EvaluationHandler`

```rust
trait EvaluationHandler: AsAny
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_expression::ArrowEvaluationHandler`

**Methods** (4)

```rust
fn create_many(&self, schema: SchemaRef, rows: &[&[Scalar]]) -> DeltaResult<Box<dyn EngineData>>
fn new_expression_evaluator(&self, input_schema: SchemaRef, expression: ExpressionRef, output_type: DataType) -> DeltaResult<Arc<dyn ExpressionEvaluator>>
fn new_predicate_evaluator(&self, input_schema: SchemaRef, predicate: PredicateRef) -> DeltaResult<Arc<dyn PredicateEvaluator>>
fn null_row(&self, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

Provides expression evaluation capability to Delta Kernel.

Delta Kernel can use this handler to evaluate a predicate on partition filters,
fill up partition column values, and any computation on data using Expressions.

---

## EvaluationHandlerExtension

`trait` · `buoyant_kernel::EvaluationHandlerExtension`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.EvaluationHandlerExtension.md)

Also reachable as `delta_kernel::EvaluationHandlerExtension`

```rust
trait EvaluationHandlerExtension: EvaluationHandler
```

**Methods** (1)

```rust
fn create_one(&self, schema: SchemaRef, values: &[Scalar]) -> DeltaResult<Box<dyn EngineData>>
```

Internal trait to allow us to have a private `create_one` API that's implemented for all
EvaluationHandlers.

---

## ExpressionEvaluator

`trait` · `buoyant_kernel::ExpressionEvaluator`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.ExpressionEvaluator.md)

Also reachable as `delta_kernel::ExpressionEvaluator`

```rust
trait ExpressionEvaluator: AsAny
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_expression::DefaultExpressionEvaluator`

**Methods** (1)

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

Trait for implementing an Expression evaluator.

It contains one Expression which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.

---

## IntoEngineData

`trait` · `buoyant_kernel::IntoEngineData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.IntoEngineData.md)

Also reachable as `delta_kernel::IntoEngineData`

```rust
trait IntoEngineData
```

**Implementors** (5)

- `buoyant_kernel::actions::CommitInfo`
- `buoyant_kernel::actions::DomainMetadata`
- `buoyant_kernel::actions::Metadata`
- `buoyant_kernel::actions::Protocol`
- `buoyant_kernel::actions::SetTransaction`

**Methods** (1)

```rust
fn into_engine_data(self, schema: SchemaRef, engine: &dyn Engine) -> DeltaResult<Box<dyn EngineData>>
```

A trait that allows converting a type into (single-row) EngineData

This is typically used with the `#[derive(IntoEngineData)]` macro
which leverages the traits `ToDataType` and `Into<Scalar>` for struct fields
to convert a struct into EngineData.

# Example
```ignore
# use buoyant_kernel as delta_kernel;
# use std::sync::Arc;
# use delta_kernel_derive::{Schema, IntoEngineData};

#[derive(Schema, IntoEngineData)]
struct MyStruct {
   a: i32,
   b: String,
}

let my_struct = MyStruct { a: 42, b: "Hello".to_string() };
// typically used with ToSchema
let schema = Arc::new(MyStruct::to_schema());
// single-row EngineData
let engine = todo!(); // create an engine
let engine_data = my_struct.into_engine_data(schema, engine);
```

---

## JsonHandler

`trait` · `buoyant_kernel::JsonHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.JsonHandler.md)

Also reachable as `delta_kernel::JsonHandler`

```rust
trait JsonHandler: AsAny
```

**Implementors** (3)

- `buoyant_kernel::metrics::metered_json::MeteredJsonHandler`
- `buoyant_kernel_engine::json::DefaultJsonHandler`
- `deltalake_core::delta_datafusion::engine::file_formats::DataFusionFileFormatHandler`

**Methods** (3)

```rust
fn parse_json(&self, json_strings: Box<dyn EngineData>, output_schema: SchemaRef) -> DeltaResult<Box<dyn EngineData>>
fn read_json_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn write_json_file(&self, path: &Url, data: DeltaResultIterator<'_, FilteredEngineData>, overwrite: bool) -> DeltaResult<()>
```

Provides JSON handling functionality to Delta Kernel.

Delta Kernel can use this handler to parse JSON strings into Row or read content from JSON
files. Connectors can leverage this trait to provide their best implementation of the JSON
parsing capability to Delta Kernel.

---

## ParquetHandler

`trait` · `buoyant_kernel::ParquetHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.ParquetHandler.md)

Also reachable as `delta_kernel::ParquetHandler`

```rust
trait ParquetHandler: AsAny
```

**Implementors** (3)

- `buoyant_kernel::metrics::metered_parquet::MeteredParquetHandler`
- `buoyant_kernel_engine::parquet::DefaultParquetHandler`
- `deltalake_core::delta_datafusion::engine::file_formats::DataFusionFileFormatHandler`

**Methods** (3)

```rust
fn read_parquet_files(&self, files: &[FileMeta], physical_schema: SchemaRef, predicate: Option<PredicateRef>) -> DeltaResult<FileDataReadResultIterator>
fn read_parquet_footer(&self, file: &FileMeta) -> DeltaResult<ParquetFooter>
fn write_parquet_file(&self, location: url::Url, data: DeltaResultIteratorStatic<Box<dyn EngineData>>) -> DeltaResult<()>
```

Provides Parquet file related functionalities to Delta Kernel.

Connectors can leverage this trait to provide their own custom
implementation of Parquet data file functionalities to Delta Kernel.

---

## PredicateEvaluator

`trait` · `buoyant_kernel::PredicateEvaluator`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.PredicateEvaluator.md)

Also reachable as `delta_kernel::PredicateEvaluator`

```rust
trait PredicateEvaluator: AsAny
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_expression::DefaultPredicateEvaluator`

**Methods** (1)

```rust
fn evaluate(&self, batch: &dyn EngineData) -> DeltaResult<Box<dyn EngineData>>
```

Trait for implementing a Predicate evaluator.

It contains one Predicate which can be evaluated on multiple ColumnarBatches.
Connectors can implement this trait to optimize the evaluation using the
connector specific capabilities.

---

## StorageHandler

`trait` · `buoyant_kernel::StorageHandler`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.StorageHandler.md)

Also reachable as `delta_kernel::StorageHandler`

```rust
trait StorageHandler: AsAny
```

**Implementors** (3)

- `buoyant_kernel::metrics::metered_storage::MeteredStorageHandler`
- `buoyant_kernel_engine::filesystem::ObjectStoreStorageHandler`
- `deltalake_core::delta_datafusion::engine::storage::DataFusionStorageHandler`

**Methods** (6)

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
fn delete(&self, path: &Url) -> DeltaResult<()>
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

Provides file system related functionalities to Delta Kernel.

Delta Kernel uses this handler whenever it needs to access the underlying
file system where the Delta table is present. Connector implementation of
this trait can hide filesystem specific details from Delta Kernel.

---

## FileDataReadResult

`type_alias` · `buoyant_kernel::FileDataReadResult`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileDataReadResult.md)

Also reachable as `delta_kernel::FileDataReadResult`

```rust
type FileDataReadResult = (FileMeta, Box<dyn EngineData>)
```

Data read from a Delta table file and the corresponding scan file information.

---

## FileDataReadResultIterator

`type_alias` · `buoyant_kernel::FileDataReadResultIterator`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileDataReadResultIterator.md)

Also reachable as `delta_kernel::FileDataReadResultIterator`

```rust
type FileDataReadResultIterator = DeltaResultIteratorStatic<Box<dyn EngineData>>
```

An iterator of data read from specified files

---

## FileIndex

`type_alias` · `buoyant_kernel::FileIndex`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileIndex.md)

Also reachable as `delta_kernel::FileIndex`

```rust
type FileIndex = u64
```

---

## FileSize

`type_alias` · `buoyant_kernel::FileSize`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileSize.md)

Also reachable as `delta_kernel::FileSize`

```rust
type FileSize = u64
```

---

## FileSlice

`type_alias` · `buoyant_kernel::FileSlice`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.FileSlice.md)

Also reachable as `delta_kernel::FileSlice`

```rust
type FileSlice = (url::Url, Option<std::ops::Range<FileIndex>>)
```

**Implements**: `deltalake_core::delta_datafusion::engine::storage::AsObjectStoreUrl`

A specification for a range of bytes to read from a file location

---

## Version

`type_alias` · `buoyant_kernel::Version`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.Version.md)

Also reachable as `delta_kernel::Version`, `deltalake::kernel::Version`, `deltalake_core::kernel::Version`

```rust
type Version = u64
```

Delta table version is 8 byte unsigned int

---
