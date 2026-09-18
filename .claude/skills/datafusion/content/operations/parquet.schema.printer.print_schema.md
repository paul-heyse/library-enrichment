# `parquet::schema::printer::print_schema`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.printer.print_schema.json).

<a id="op-988b330e77470473699dd1f6"></a>
## print_schema

`function` · `parquet::schema::printer::print_schema` · parquet 59.3.0

```rust
fn print_schema(out: &mut dyn io::Write, tp: &schema::types::Type)
```

Source: `src/schema/printer.rs:147`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Prints Parquet [`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc) information.

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
