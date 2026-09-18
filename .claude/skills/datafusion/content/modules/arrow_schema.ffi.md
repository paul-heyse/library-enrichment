# `arrow_schema::ffi`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.ffi.json).

<a id="op-24c30c251a9b95c32edc2b8a"></a>
## ffi

`module` · `arrow_schema::ffi` · arrow-schema 59.3.0

```rust
mod ffi
```

Source: `src/ffi.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

```
# use arrow_schema::{DataType, Field, Schema};
# use arrow_schema::ffi::FFI_ArrowSchema;

// Create from data type
let ffi_data_type = FFI_ArrowSchema::try_from(&DataType::LargeUtf8).unwrap();
let back = DataType::try_from(&ffi_data_type).unwrap();
assert_eq!(back, DataType::LargeUtf8);

// Create from schema
let schema = Schema::new(vec![Field::new("foo", DataType::Int64, false)]);
let ffi_schema = FFI_ArrowSchema::try_from(&schema).unwrap();
let back = Schema::try_from(&ffi_schema).unwrap();

assert_eq!(schema, back);
```
