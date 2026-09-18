# `sqlparser::ast::Interval`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Interval.json).

<a id="op-3da8ca09b4cbf642122decbf"></a>
## Interval

`struct` · `sqlparser::ast::Interval` · sqlparser 0.62.0

```rust
struct Interval
```

Source: `src/ast/mod.rs:501`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an INTERVAL expression, roughly in the following format:
`INTERVAL '<value>' [ <leading_field> [ (<leading_precision>) ] ]
[ TO <last_field> [ (<fractional_seconds_precision>) ] ]`,
e.g. `INTERVAL '123:45.67' MINUTE(3) TO SECOND(2)`.

The parser does not validate the `<value>`, nor does it ensure
that the `<leading_field>` units >= the units in `<last_field>`,
so the user will have to reject intervals like `HOUR TO YEAR`.

<a id="op-e38dae5af1fe795a82f57297"></a>
## clone

`function` · `sqlparser::ast::Interval::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 17], "end": [498, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91c63d5c05085407c0a9c065"></a>
## cmp

`function` · `sqlparser::ast::Interval::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Interval) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 51], "end": [498, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ce050a4c3fab9858a14d558"></a>
## deserialize

`function` · `sqlparser::ast::Interval::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 49], "end": [499, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a00c13453fade97536e0164"></a>
## eq

`function` · `sqlparser::ast::Interval::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Interval) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 24], "end": [498, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17140d4988d15f1e09921321"></a>
## fmt

`function` · `sqlparser::ast::Interval::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [555, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7c5df194fcb786587b9db17"></a>
## fmt

`function` · `sqlparser::ast::Interval::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 10], "end": [498, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-156bbd150aff8251f0bb8846"></a>
## fractional_seconds_precision

`struct_field` · `sqlparser::ast::Interval::fractional_seconds_precision` · sqlparser 0.62.0

```rust
fractional_seconds_precision: Option<u64>
```

Source: `src/ast/mod.rs:513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The fractional seconds precision, when specified.

See SQL `SECOND(n)` or `SECOND(m, n)` forms.

<a id="op-ae33d62594bc95384d7b7e63"></a>
## hash

`function` · `sqlparser::ast::Interval::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 56], "end": [498, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a249149d20f1be0147f337fc"></a>
## last_field

`struct_field` · `sqlparser::ast::Interval::last_field` · sqlparser 0.62.0

```rust
last_field: Option<DateTimeField>
```

Source: `src/ast/mod.rs:509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional trailing time unit for a range (e.g., `SECOND`).

<a id="op-f032043de1ba92161896575e"></a>
## leading_field

`struct_field` · `sqlparser::ast::Interval::leading_field` · sqlparser 0.62.0

```rust
leading_field: Option<DateTimeField>
```

Source: `src/ast/mod.rs:505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional leading time unit (e.g., `HOUR`, `MINUTE`).

<a id="op-05c0205c98de5c6047a6118d"></a>
## leading_precision

`struct_field` · `sqlparser::ast::Interval::leading_precision` · sqlparser 0.62.0

```rust
leading_precision: Option<u64>
```

Source: `src/ast/mod.rs:507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional leading precision for the leading field.

<a id="op-0716f7dbf824b547245066fb"></a>
## partial_cmp

`function` · `sqlparser::ast::Interval::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Interval) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [498, 35], "end": [498, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5e25ce8bccb20bb600bf5c3"></a>
## serialize

`function` · `sqlparser::ast::Interval::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 38], "end": [499, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b61a385249c5411754c635"></a>
## value

`struct_field` · `sqlparser::ast::Interval::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:503`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The interval value expression (commonly a string literal).

<a id="op-66d63229a497b61629e4aee9"></a>
## visit

`function` · `sqlparser::ast::Interval::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 40], "end": [500, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd4eb2d066e6458eed2d2f97"></a>
## visit

`function` · `sqlparser::ast::Interval::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Interval", "path": "Interval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 47], "end": [500, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
