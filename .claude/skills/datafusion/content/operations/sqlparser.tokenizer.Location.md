# `sqlparser::tokenizer::Location`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Location.json).

<a id="op-e28189c4b3ca84cefc9c04c4"></a>
## Location

`struct` · `sqlparser::tokenizer::Location` · sqlparser 0.62.0

```rust
struct Location
```

Source: `src/tokenizer.rs:552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Location in input string

# Create an "empty" (unknown) `Location`
```
# use sqlparser::tokenizer::Location;
let location = Location::empty();
```

# Create a `Location` from a line and column
```
# use sqlparser::tokenizer::Location;
let location = Location::new(1, 1);
```

# Create a `Location` from a pair
```
# use sqlparser::tokenizer::Location;
let location = Location::from((1, 1));
```

<a id="op-efbece9b92f21c1319d7106c"></a>
## clone

`function` · `sqlparser::tokenizer::Location::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Location
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 31], "end": [549, 36], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d29ed9ae7374199abcb12d5"></a>
## cmp

`function` · `sqlparser::tokenizer::Location::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Location) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 44], "end": [549, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0857e54505b81ef65def68c"></a>
## column

`struct_field` · `sqlparser::tokenizer::Location::column` · sqlparser 0.62.0

```rust
column: u64
```

Source: `src/tokenizer.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Line column, starting from 1.

Note: Column 0 is used for empty spans

<a id="op-475966652643acc5682078b0"></a>
## deserialize

`function` · `sqlparser::tokenizer::Location::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 49], "end": [550, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-291765c3b33603432e5270bf"></a>
## empty

`function` · `sqlparser::tokenizer::Location::empty` · sqlparser 0.62.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [578, 1], "end": [601, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return an "empty" / unknown location

<a id="op-27c0dadd2e696e53e2b07379"></a>
## eq

`function` · `sqlparser::tokenizer::Location::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Location) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 14], "end": [549, 23], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dcab8808e764c15da5776c8"></a>
## fmt

`function` · `sqlparser::tokenizer::Location::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [570, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f0df3c0c51d24a4d2945d6b"></a>
## fmt

`function` · `sqlparser::tokenizer::Location::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [572, 1], "end": [576, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68e09aa359154d5724804500"></a>
## from

`function` · `sqlparser::tokenizer::Location::from` · sqlparser 0.62.0

```rust
fn from((line, column): (u64, u64)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 1], "end": [607, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "u64"}, {"primitive": "u64"}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/tokenizer.rs:604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0b7b8058fbc779fc100dd93"></a>
## hash

`function` · `sqlparser::tokenizer::Location::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 25], "end": [549, 29], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc1f06f2dd9d6df760349020"></a>
## line

`struct_field` · `sqlparser::tokenizer::Location::line` · sqlparser 0.62.0

```rust
line: u64
```

Source: `src/tokenizer.rs:556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Line number, starting from 1.

Note: Line 0 is used for empty spans

<a id="op-4d5817425943338871a55e3a"></a>
## new

`function` · `sqlparser::tokenizer::Location::new` · sqlparser 0.62.0

```rust
fn new(line: u64, column: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [578, 1], "end": [601, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new `Location` for a given line and column

<a id="op-218166f884398a44c85b781d"></a>
## of

`function` · `sqlparser::tokenizer::Location::of` · sqlparser 0.62.0

```rust
fn of(line: u64, column: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [578, 1], "end": [601, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new location for a given line and column

Alias for [`Self::new`](../operations/sqlparser.tokenizer.Location.md#op-4d5817425943338871a55e3a)

<a id="op-5d4f1964a962b6b3ec061faf"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::Location::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Location) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 49], "end": [549, 59], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65a73db4c93ffa87ac79b1c1"></a>
## serialize

`function` · `sqlparser::tokenizer::Location::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03d49b6d8ff1e153a8db3719"></a>
## span_to

`function` · `sqlparser::tokenizer::Location::span_to` · sqlparser 0.62.0

```rust
fn span_to(self, end: Self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [578, 1], "end": [601, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Combine self and `end` into a new `Span`

<a id="op-c69b46ed75669bd994e90cb2"></a>
## visit

`function` · `sqlparser::tokenizer::Location::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 40], "end": [551, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d95fab862af9bd1d34b7d3fe"></a>
## visit

`function` · `sqlparser::tokenizer::Location::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Location", "path": "Location"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 47], "end": [551, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
