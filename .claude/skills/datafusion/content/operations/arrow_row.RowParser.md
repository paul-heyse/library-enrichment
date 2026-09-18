# `arrow_row::RowParser`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.RowParser.json).

<a id="op-955916db32b3a0fe58460e3a"></a>
## RowParser

`struct` · `arrow_row::RowParser` · arrow-row 59.3.0

```rust
struct RowParser
```

Source: `src/lib.rs:1280`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

A [`RowParser`](../operations/arrow_row.RowParser.md#op-955916db32b3a0fe58460e3a) can be created from a [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) and used to parse bytes to [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966)

<a id="op-a15250bd8c0fe6082746190f"></a>
## fmt

`function` · `arrow_row::RowParser::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowParser", "path": "RowParser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1279, 10], "end": [1279, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1279`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b66e742de2b8312b677bac26"></a>
## parse

`function` · `arrow_row::RowParser::parse` · arrow-row 59.3.0

```rust
fn parse<'a>(&'a self, bytes: &'a [u8]) -> Row<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowParser", "path": "RowParser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1284, 1], "end": [1316, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1310`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Creates a [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) from the provided `bytes`.

`bytes` must be a [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) produced by the [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) associated with
this [`RowParser`](../operations/arrow_row.RowParser.md#op-955916db32b3a0fe58460e3a), otherwise subsequent operations with the produced [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) may panic
