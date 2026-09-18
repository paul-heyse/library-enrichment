# `arrow_json::reader::schema::infer_json_schema_from_iterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.schema.infer_json_schema_from_iterator.json).

<a id="op-02a2b11a363a3691a2aba52d"></a>
## infer_json_schema_from_iterator

`function` · `arrow_json::reader::schema::infer_json_schema_from_iterator` · arrow-json 59.3.0

```rust
fn infer_json_schema_from_iterator<I, V>(value_iter: I) -> Result<arrow_schema::Schema, arrow_schema::ArrowError> where I: Iterator<Item = Result<V, arrow_schema::ArrowError>>, V: Borrow<serde_json::Value>
```

Source: `src/reader/schema.rs:424`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Infer the fields of a JSON file by reading all items from the JSON Value Iterator.

The following type coercion logic is implemented:
* `Int64` and `Float64` are converted to `Float64`
* Lists and scalars are coerced to a list of a compatible scalar
* All other cases are coerced to `Utf8` (String)

Note that the above coercion logic is different from what Spark has, where it would default to
String type in case of List and Scalar values appeared in the same field.

The reason we diverge here is because we don't have utilities to deal with JSON data once it's
interpreted as Strings. We should match Spark's behavior once we added more JSON parsing
kernels in the future.
