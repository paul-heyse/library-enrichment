# `parquet::schema::printer`

Crate `parquet` · 3 public items · structured records in [`model/parquet.schema.printer.json`](../model/parquet.schema.printer.json)

## print_file_metadata

`function` · `parquet::schema::printer::print_file_metadata`

```rust
fn print_file_metadata(out: &mut dyn io::Write, file_metadata: &file::metadata::FileMetaData)
```

Prints file metadata [`FileMetaData`] information.

---

## print_parquet_metadata

`function` · `parquet::schema::printer::print_parquet_metadata`

```rust
fn print_parquet_metadata(out: &mut dyn io::Write, metadata: &file::metadata::ParquetMetaData)
```

Prints Parquet metadata [`ParquetMetaData`] information.

---

## print_schema

`function` · `parquet::schema::printer::print_schema`

```rust
fn print_schema(out: &mut dyn io::Write, tp: &schema::types::Type)
```

Prints Parquet [`Type`] information.

# Example

```rust
use parquet::{
    basic::{ConvertedType, Repetition, Type as PhysicalType},
    schema::{printer::print_schema, types::Type},
};
use std::sync::Arc;

let field_a = Type::primitive_type_builder("a", PhysicalType::BYTE_ARRAY)
    .with_id(Some(42))
    .with_converted_type(ConvertedType::UTF8)
    .build()
    .unwrap();

let field_b = Type::primitive_type_builder("b", PhysicalType::INT32)
    .with_repetition(Repetition::REQUIRED)
    .build()
    .unwrap();

let field_d = Type::primitive_type_builder("d", PhysicalType::INT64)
    .with_id(Some(99))
    .build()
    .unwrap();

let field_c = Type::group_type_builder("c")
    .with_id(Some(43))
    .with_fields(vec![Arc::new(field_d)])
    .build()
    .unwrap();

let schema = Type::group_type_builder("schema")
    .with_fields(vec![Arc::new(field_a), Arc::new(field_b), Arc::new(field_c)])
    .build()
    .unwrap();

print_schema(&mut std::io::stdout(), &schema);
```

outputs

```text
message schema {
  OPTIONAL BYTE_ARRAY a [42] (UTF8);
  REQUIRED INT32 b;
  message c [43] {
    OPTIONAL INT64 d [99];
  }
}
```

---
