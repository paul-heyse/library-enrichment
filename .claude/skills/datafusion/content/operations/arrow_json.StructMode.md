# `arrow_json::StructMode`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.StructMode.json).

<a id="op-a862c9daf0b023e9659efe1e"></a>
## StructMode

`enum` · `arrow_json::StructMode` · arrow-json 59.3.0

```rust
enum StructMode
```

Source: `src/lib.rs:118`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Specifies what is considered valid JSON when reading or writing
RecordBatches or StructArrays.

This enum controls which form(s) the Reader will accept and which form the
Writer will produce. For example, if the RecordBatch Schema is
`[("a", Int32), ("r", Struct("b": Boolean, "c" Utf8))]`
then a Reader with [`StructMode::ObjectOnly`](../operations/arrow_json.StructMode.md#op-ef7ccec4bd270fb91965fa3a) would read rows of the form
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

<a id="op-0f3601420c924b23097dfcd2"></a>
## ListOnly

`variant` · `arrow_json::StructMode::ListOnly` · arrow-json 59.3.0

```rust
ListOnly
```

Source: `src/lib.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Encode/decode structs as lists (e.g., [1, "c"])

<a id="op-ef7ccec4bd270fb91965fa3a"></a>
## ObjectOnly

`variant` · `arrow_json::StructMode::ObjectOnly` · arrow-json 59.3.0

```rust
ObjectOnly
```

Source: `src/lib.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Encode/decode structs as objects (e.g., {"a": 1, "b": "c"})

<a id="op-7f4332efa3b38ff7a9d39720"></a>
## clone

`function` · `arrow_json::StructMode::clone` · arrow-json 59.3.0

```rust
fn clone(&self) -> StructMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::StructMode", "path": "StructMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 16], "end": [117, 21], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be5ba5976cbbd805b983e002"></a>
## default

`function` · `arrow_json::StructMode::default` · arrow-json 59.3.0

```rust
fn default() -> StructMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::StructMode", "path": "StructMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 30], "end": [117, 37], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769a5f3217f2d892b825e64c"></a>
## eq

`function` · `arrow_json::StructMode::eq` · arrow-json 59.3.0

```rust
fn eq(&self, other: &StructMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::StructMode", "path": "StructMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 39], "end": [117, 48], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28023b8980472cdc27dc621e"></a>
## fmt

`function` · `arrow_json::StructMode::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::StructMode", "path": "StructMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 23], "end": [117, 28], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
