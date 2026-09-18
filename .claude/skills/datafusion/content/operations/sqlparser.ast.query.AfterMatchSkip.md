# `sqlparser::ast::query::AfterMatchSkip`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.AfterMatchSkip.json).

<a id="op-1b2d9fb4e13bc145c4c480d4"></a>
## AfterMatchSkip

`enum` · `sqlparser::ast::query::AfterMatchSkip` · sqlparser 0.62.0

```rust
enum AfterMatchSkip
```

Source: `src/ast/query.rs:2036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The after match skip option in a `MATCH_RECOGNIZE` operation.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize#after-match-skip-specifying-where-to-continue-after-a-match>.

<a id="op-d5ef84bb231e8cf5e6950235"></a>
## PastLastRow

`variant` · `sqlparser::ast::query::AfterMatchSkip::PastLastRow` · sqlparser 0.62.0

```rust
PastLastRow
```

Source: `src/ast/query.rs:2038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PAST LAST ROW`

<a id="op-1de66edb4c0ab15056e4809e"></a>
## ToFirst

`variant` · `sqlparser::ast::query::AfterMatchSkip::ToFirst` · sqlparser 0.62.0

```rust
ToFirst
```

Source: `src/ast/query.rs:2042`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TO FIRST <symbol>`

<a id="op-0ea1236266fb5a2d23bd2c2e"></a>
## ToLast

`variant` · `sqlparser::ast::query::AfterMatchSkip::ToLast` · sqlparser 0.62.0

```rust
ToLast
```

Source: `src/ast/query.rs:2044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TO LAST <symbol>`

<a id="op-20aaca0dab201df06aa630ae"></a>
## ToNextRow

`variant` · `sqlparser::ast::query::AfterMatchSkip::ToNextRow` · sqlparser 0.62.0

```rust
ToNextRow
```

Source: `src/ast/query.rs:2040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TO NEXT ROW`

<a id="op-7e93686460af57452b28228e"></a>
## clone

`function` · `sqlparser::ast::query::AfterMatchSkip::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AfterMatchSkip
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 17], "end": [2033, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da03cc43751731b33105c8d6"></a>
## cmp

`function` · `sqlparser::ast::query::AfterMatchSkip::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AfterMatchSkip) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 51], "end": [2033, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41aa84583a03535ae2f11e9b"></a>
## deserialize

`function` · `sqlparser::ast::query::AfterMatchSkip::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2034, 49], "end": [2034, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2034`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8bcdfc92e185ae2ba173906"></a>
## eq

`function` · `sqlparser::ast::query::AfterMatchSkip::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AfterMatchSkip) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 24], "end": [2033, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-780a92749ffe84572f118da1"></a>
## fmt

`function` · `sqlparser::ast::query::AfterMatchSkip::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2047, 1], "end": [2057, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-914336c0d3cdf422227dd19b"></a>
## fmt

`function` · `sqlparser::ast::query::AfterMatchSkip::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 10], "end": [2033, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a250d71e127cb726063b3203"></a>
## hash

`function` · `sqlparser::ast::query::AfterMatchSkip::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 56], "end": [2033, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb2e9d029b250cd5ecd1adc7"></a>
## partial_cmp

`function` · `sqlparser::ast::query::AfterMatchSkip::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AfterMatchSkip) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2033, 35], "end": [2033, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff89a21f47284e7001637fe2"></a>
## serialize

`function` · `sqlparser::ast::query::AfterMatchSkip::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2034, 38], "end": [2034, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2034`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c05301409ff94bb5f8eced6f"></a>
## visit

`function` · `sqlparser::ast::query::AfterMatchSkip::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2035, 40], "end": [2035, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6f6345311507ffd1f71d7f3"></a>
## visit

`function` · `sqlparser::ast::query::AfterMatchSkip::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::AfterMatchSkip", "path": "AfterMatchSkip"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2035, 47], "end": [2035, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
