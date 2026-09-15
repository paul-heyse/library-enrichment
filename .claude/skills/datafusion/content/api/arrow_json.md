# `arrow_json`

Crate `arrow-json` · 2 public items · structured records in [`model/arrow_json.json`](../model/arrow_json.json)

## StructMode

`enum` · `arrow_json::StructMode`

Also reachable as `arrow::json::StructMode`

```rust
enum StructMode
```

**Variants**: `ObjectOnly`, `ListOnly`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Specifies what is considered valid JSON when reading or writing
RecordBatches or StructArrays.

This enum controls which form(s) the Reader will accept and which form the
Writer will produce. For example, if the RecordBatch Schema is
`[("a", Int32), ("r", Struct("b": Boolean, "c" Utf8))]`
then a Reader with [`StructMode::ObjectOnly`] would read rows of the form
`{"a": 1, "r": {"b": true, "c": "cat"}}` while with ['StructMode::ListOnly']
would read rows of the form `[1, [true, "cat"]]`. A Writer would produce
rows formatted similarly.

The list encoding is more compact if the schema is known, and is used by
tools such as [Presto] and [Trino].

When reading objects, the order of the key does not matter. When reading
lists, the entries must be the same number and in the same order as the
struct fields. Map columns are not affected by this option.

[Presto]: https://prestodb.io/docs/current/develop/client-protocol.html#important-queryresults-attributes
[Trino]: https://trino.io/docs/current/develop/client-protocol.html#important-queryresults-attributes

---

## JsonSerializable

`trait` · `arrow_json::JsonSerializable`

Also reachable as `arrow::json::JsonSerializable`

```rust
trait JsonSerializable: 'static
```

**Implementors** (1)

- `half::binary16::f16`

**Methods** (1)

```rust
fn into_json_value(self) -> Option<Value>
```

Trait declaring any type that is serializable to JSON. This includes all primitive types (bool, i32, etc.).

---
