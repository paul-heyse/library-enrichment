# `parquet::schema::parser`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.parser.json).

<a id="op-f510c61b33a756355f4dd0b5"></a>
## parser

`module` · `parquet::schema::parser` · parquet 59.3.0

```rust
mod parser
```

Source: `src/schema/parser.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Parquet schema parser.
Provides methods to parse and validate string message type into Parquet
[`Type`](../operations/parquet.schema.types.Type.md#op-e61bef0051911d1f490c45fc).

# Example

```rust
use parquet::schema::parser::parse_message_type;

let message_type = "
  message spark_schema {
    OPTIONAL BYTE_ARRAY a (UTF8);
    REQUIRED INT32 b;
    REQUIRED DOUBLE c;
    REQUIRED BOOLEAN d;
    OPTIONAL group e (LIST) {
      REPEATED group list {
        REQUIRED INT32 element;
      }
    }
  }
";

let schema = parse_message_type(message_type).expect("Expected valid schema");
println!("{:?}", schema);
```
