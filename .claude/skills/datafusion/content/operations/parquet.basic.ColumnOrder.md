# `parquet::basic::ColumnOrder`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.ColumnOrder.json).

<a id="op-8e08b3a6efbe2c28608469c3"></a>
## ColumnOrder

`enum` · `parquet::basic::ColumnOrder` · parquet 59.3.0

```rust
enum ColumnOrder
```

Source: `src/basic.rs:1004`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column order that specifies what method was used to aggregate min/max values for
statistics.

If column order is undefined, then it is the legacy behaviour and all values should
be compared as signed values/bytes.

<a id="op-31f85c759b972fe19ae5caad"></a>
## TYPE_DEFINED_ORDER

`variant` · `parquet::basic::ColumnOrder::TYPE_DEFINED_ORDER` · parquet 59.3.0

```rust
TYPE_DEFINED_ORDER
```

Source: `src/basic.rs:1007`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Column uses the order defined by its logical or physical type
(if there is no logical type), parquet-format 2.4.0+.

<a id="op-ebb6e1afbeed40d5e00dc734"></a>
## UNDEFINED

`variant` · `parquet::basic::ColumnOrder::UNDEFINED` · parquet 59.3.0

```rust
UNDEFINED
```

Source: `src/basic.rs:1011`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Undefined column order, means legacy behaviour before parquet-format 2.4.0.
Sort order is always SIGNED.

<a id="op-c5d3b94756a146e94976e19a"></a>
## UNKNOWN

`variant` · `parquet::basic::ColumnOrder::UNKNOWN` · parquet 59.3.0

```rust
UNKNOWN
```

Source: `src/basic.rs:1014`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An unknown but present ColumnOrder. Statistics with an unknown `ColumnOrder`
will be ignored.

<a id="op-fb4b7b9119fd06490ff096df"></a>
## clone

`function` · `parquet::basic::ColumnOrder::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1002, 17], "end": [1002, 22], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:1002`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8739628f0df3b4a67fe30435"></a>
## eq

`function` · `parquet::basic::ColumnOrder::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnOrder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1002, 30], "end": [1002, 39], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:1002`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a07a7e8f9c1f4efa64b1f1b0"></a>
## fmt

`function` · `parquet::basic::ColumnOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1002, 10], "end": [1002, 15], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:1002`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a389dd4bb20587be7a52ece8"></a>
## fmt

`function` · `parquet::basic::ColumnOrder::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1190, 1], "end": [1194, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:1191`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd583149e36ab78a075c1cf7"></a>
## get_sort_order

`function` · `parquet::basic::ColumnOrder::get_sort_order` · parquet 59.3.0

```rust
fn get_sort_order(logical_type: Option<LogicalType>, converted_type: ConvertedType, physical_type: Type) -> SortOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1130, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:1023`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns sort order for a physical/logical type.

<a id="op-a2c82e2843f4f4c435ab329e"></a>
## sort_order

`function` · `parquet::basic::ColumnOrder::sort_order` · parquet 59.3.0

```rust
fn sort_order(&self) -> SortOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1130, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:1123`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns sort order associated with this column order.

<a id="op-4f15c3a3b46554f503443d60"></a>
## sort_order_for_type

`function` · `parquet::basic::ColumnOrder::sort_order_for_type` · parquet 59.3.0

```rust
fn sort_order_for_type(logical_type: Option<&LogicalType>, converted_type: ConvertedType, physical_type: Type) -> SortOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::ColumnOrder", "path": "ColumnOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1017, 1], "end": [1130, 2], "filename": "src/basic.rs"}, "trait": null, "trait_path": null}`

Source: `src/basic.rs:1032`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns sort order for a physical/logical type.
