# `sqlparser::ast::ConditionalStatements`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ConditionalStatements.json).

<a id="op-2dffc726bc8756035eb9864d"></a>
## ConditionalStatements

`enum` · `sqlparser::ast::ConditionalStatements` · sqlparser 0.62.0

```rust
enum ConditionalStatements
```

Source: `src/ast/mod.rs:2794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A list of statements in a [ConditionalStatementBlock](../operations/sqlparser.ast.ConditionalStatementBlock.md#op-6d364edf1d9ed5606c36d180).
Statements used inside conditional blocks (`IF`, `WHEN`, `WHILE`).

<a id="op-ee713a9eb3ced69169695de6"></a>
## BeginEnd

`variant` · `sqlparser::ast::ConditionalStatements::BeginEnd` · sqlparser 0.62.0

```rust
BeginEnd
```

Source: `src/ast/mod.rs:2801`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Block enclosed by `BEGIN` and `END`.

<a id="op-872e2a411238cea50079bcb7"></a>
## Sequence

`variant` · `sqlparser::ast::ConditionalStatements::Sequence` · sqlparser 0.62.0

```rust
Sequence
```

Source: `src/ast/mod.rs:2796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Simple sequence of statements (no `BEGIN`/`END`).

<a id="op-7712b39735964f3f355ce090"></a>
## clone

`function` · `sqlparser::ast::ConditionalStatements::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConditionalStatements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 17], "end": [2790, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5115e1b9bd5f205b1dfc946f"></a>
## cmp

`function` · `sqlparser::ast::ConditionalStatements::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConditionalStatements) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 51], "end": [2790, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ee97aa2f3975b9a00407a84"></a>
## deserialize

`function` · `sqlparser::ast::ConditionalStatements::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2791, 49], "end": [2791, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34718667e6eacc5f8bdf3f60"></a>
## eq

`function` · `sqlparser::ast::ConditionalStatements::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConditionalStatements) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 24], "end": [2790, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92cb9512c773351639671148"></a>
## fmt

`function` · `sqlparser::ast::ConditionalStatements::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 10], "end": [2790, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac068591bb6ae56e5bae0ae5"></a>
## fmt

`function` · `sqlparser::ast::ConditionalStatements::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2814, 1], "end": [2826, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c1efe99348c338918ce8bc9"></a>
## hash

`function` · `sqlparser::ast::ConditionalStatements::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 56], "end": [2790, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8ec44c5063de38ed116943"></a>
## partial_cmp

`function` · `sqlparser::ast::ConditionalStatements::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConditionalStatements) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2790, 35], "end": [2790, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48715a4235fcf9712ba8882d"></a>
## serialize

`function` · `sqlparser::ast::ConditionalStatements::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2791, 38], "end": [2791, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ee6ed35c02f259b7aa4be71"></a>
## span

`function` · `sqlparser::ast::ConditionalStatements::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "super::ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [761, 1], "end": [770, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6acbb55b6ad590891410bb8"></a>
## statements

`function` · `sqlparser::ast::ConditionalStatements::statements` · sqlparser 0.62.0

```rust
fn statements(&self) -> &Vec<Statement>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2804, 1], "end": [2812, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:2806`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Get the statements in this conditional statements block.

<a id="op-7e41126b17d1bbc43241ff66"></a>
## visit

`function` · `sqlparser::ast::ConditionalStatements::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2792, 47], "end": [2792, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-effa1c88c29862e1d601ce37"></a>
## visit

`function` · `sqlparser::ast::ConditionalStatements::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ConditionalStatements", "path": "ConditionalStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2792, 40], "end": [2792, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
