# `sqlparser::ast::query::MatchRecognizePattern`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.MatchRecognizePattern.json).

<a id="op-1c7a719f8cdea40047117dfc"></a>
## MatchRecognizePattern

`enum` · `sqlparser::ast::query::MatchRecognizePattern` · sqlparser 0.62.0

```rust
enum MatchRecognizePattern
```

Source: `src/ast/query.rs:2131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The pattern in a `MATCH_RECOGNIZE` operation.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize#pattern-specifying-the-pattern-to-match>.

<a id="op-4ce2ee632f006f6054b868ff"></a>
## Alternation

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Alternation` · sqlparser 0.62.0

```rust
Alternation
```

Source: `src/ast/query.rs:2143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

pattern_1 | pattern_2 | ... | pattern_n

<a id="op-d2162dac6697e2502319e14d"></a>
## Concat

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Concat` · sqlparser 0.62.0

```rust
Concat
```

Source: `src/ast/query.rs:2139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

pattern_1 pattern_2 ... pattern_n

<a id="op-b764fbe67c08a3de5ebcf42e"></a>
## Exclude

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Exclude` · sqlparser 0.62.0

```rust
Exclude
```

Source: `src/ast/query.rs:2135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

{- symbol -}

<a id="op-29fde6e04456774ed34ce9b6"></a>
## Group

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Group` · sqlparser 0.62.0

```rust
Group
```

Source: `src/ast/query.rs:2141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

( pattern )

<a id="op-c8ac11a526455c0d7c118c7d"></a>
## Permute

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Permute` · sqlparser 0.62.0

```rust
Permute
```

Source: `src/ast/query.rs:2137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PERMUTE(symbol_1, ..., symbol_n)

<a id="op-d1fa8825aff0385163251e9c"></a>
## Repetition

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Repetition` · sqlparser 0.62.0

```rust
Repetition
```

Source: `src/ast/query.rs:2145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

e.g. pattern*

<a id="op-728d2e309f350e377e34f323"></a>
## Symbol

`variant` · `sqlparser::ast::query::MatchRecognizePattern::Symbol` · sqlparser 0.62.0

```rust
Symbol
```

Source: `src/ast/query.rs:2133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named symbol such as `S1` or a virtual symbol such as `^`.

<a id="op-caddc91af201616e55b1d148"></a>
## clone

`function` · `sqlparser::ast::query::MatchRecognizePattern::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MatchRecognizePattern
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 17], "end": [2128, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c14d937f9a3282b552419e54"></a>
## cmp

`function` · `sqlparser::ast::query::MatchRecognizePattern::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MatchRecognizePattern) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 51], "end": [2128, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36f410809cb1aa6891f0e825"></a>
## deserialize

`function` · `sqlparser::ast::query::MatchRecognizePattern::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2129, 49], "end": [2129, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11009665f413bcd8253fa37b"></a>
## eq

`function` · `sqlparser::ast::query::MatchRecognizePattern::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MatchRecognizePattern) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 24], "end": [2128, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67beaaf56bc0136d85820c94"></a>
## fmt

`function` · `sqlparser::ast::query::MatchRecognizePattern::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 10], "end": [2128, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a55cfa601d618a58bb3f98b6"></a>
## fmt

`function` · `sqlparser::ast::query::MatchRecognizePattern::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2148, 1], "end": [2161, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcbf1ea3ca6ec643606b5610"></a>
## hash

`function` · `sqlparser::ast::query::MatchRecognizePattern::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 56], "end": [2128, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c93c27392151e35d9e52c48"></a>
## partial_cmp

`function` · `sqlparser::ast::query::MatchRecognizePattern::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MatchRecognizePattern) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2128, 35], "end": [2128, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9255375557fad321629beb10"></a>
## serialize

`function` · `sqlparser::ast::query::MatchRecognizePattern::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2129, 38], "end": [2129, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77787b6a79b078978195332a"></a>
## span

`function` · `sqlparser::ast::query::MatchRecognizePattern::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "super::MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2100, 1], "end": [2104, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8e9cb27dda7d3f66507a53"></a>
## visit

`function` · `sqlparser::ast::query::MatchRecognizePattern::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2130, 40], "end": [2130, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f22acc314f6f0dd7b9fbdf57"></a>
## visit

`function` · `sqlparser::ast::query::MatchRecognizePattern::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::MatchRecognizePattern", "path": "MatchRecognizePattern"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2130, 47], "end": [2130, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
