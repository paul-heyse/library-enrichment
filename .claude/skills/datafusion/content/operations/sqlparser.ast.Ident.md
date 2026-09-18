# `sqlparser::ast::Ident`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Ident.json).

<a id="op-84514077950174f9df2ced53"></a>
## Ident

`struct` · `sqlparser::ast::Ident` · sqlparser 0.62.0

```rust
struct Ident
```

Source: `src/ast/mod.rs:246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An identifier, decomposed into its value or character data and the quote style.

<a id="op-2636eaeadb0d3bbc6e7a02fb"></a>
## clone

`function` · `sqlparser::ast::Ident::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Ident
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 17], "end": [243, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72c543b3ebe46b87620f277e"></a>
## cmp

`function` · `sqlparser::ast::Ident::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [312, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ec46632e9bdcf08cd4bedaf"></a>
## deserialize

`function` · `sqlparser::ast::Ident::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 49], "end": [244, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3018049108af8fa5c4301d51"></a>
## eq

`function` · `sqlparser::ast::Ident::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [267, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c87547e2b7b6013efa51db8"></a>
## fmt

`function` · `sqlparser::ast::Ident::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [377, 1], "end": [389, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f383356ae6490c203bc745a0"></a>
## fmt

`function` · `sqlparser::ast::Ident::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 10], "end": [243, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d50f1db0e2c7aaa21e26bcd9"></a>
## from

`function` · `sqlparser::ast::Ident::from` · sqlparser 0.62.0

```rust
fn from(value: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 1], "end": [375, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-791ab52baa913a789842ab49"></a>
## hash

`function` · `sqlparser::ast::Ident::hash` · sqlparser 0.62.0

```rust
fn hash<H: hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [269, 1], "end": [281, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e440920d0fcf971fdbd4a4f"></a>
## new

`function` · `sqlparser::ast::Ident::new` · sqlparser 0.62.0

```rust
fn new<S>(value: S) -> Self where S: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [365, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new identifier with the given value and no quotes and an empty span.

<a id="op-d71430d8b29afed990f9fa04"></a>
## partial_cmp

`function` · `sqlparser::ast::Ident::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [289, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b6d0f6a94b4808b0ea0f0e2"></a>
## quote_style

`struct_field` · `sqlparser::ast::Ident::quote_style` · sqlparser 0.62.0

```rust
quote_style: Option<char>
```

Source: `src/ast/mod.rs:251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The starting quote if any. Valid quote characters are the single quote,
double quote, backtick, and opening square bracket.

<a id="op-8cefbd2aaf20c9610e5a0cfc"></a>
## serialize

`function` · `sqlparser::ast::Ident::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 38], "end": [244, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf2949e49ac8155f1504b791"></a>
## span

`struct_field` · `sqlparser::ast::Ident::span` · sqlparser 0.62.0

```rust
span: tokenizer::Span
```

Source: `src/ast/mod.rs:253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The span of the identifier in the original SQL string.

<a id="op-f2980f33a782933aa9045dee"></a>
## value

`struct_field` · `sqlparser::ast::Ident::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/ast/mod.rs:248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value of the identifier without quotes.

<a id="op-09ff5c38881a68e8f05bfb45"></a>
## visit

`function` · `sqlparser::ast::Ident::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 47], "end": [245, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e0e5366b3b6ee78bd125619"></a>
## visit

`function` · `sqlparser::ast::Ident::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 40], "end": [245, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c08d06c5f863c8421e3d47d"></a>
## with_quote

`function` · `sqlparser::ast::Ident::with_quote` · sqlparser 0.62.0

```rust
fn with_quote<S>(quote: char, value: S) -> Self where S: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [365, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:329`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new quoted identifier with the given quote and value. This function
panics if the given quote is not a valid quote character.

<a id="op-ba3b443a2a571de62556eca0"></a>
## with_quote_and_span

`function` · `sqlparser::ast::Ident::with_quote_and_span` · sqlparser 0.62.0

```rust
fn with_quote_and_span<S>(quote: char, span: Span, value: S) -> Self where S: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [365, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a quoted `Ident` with the given `quote` and `span`.

<a id="op-1101a5c075625bcfbc2101d0"></a>
## with_span

`function` · `sqlparser::ast::Ident::with_span` · sqlparser 0.62.0

```rust
fn with_span<S>(span: Span, value: S) -> Self where S: Into<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Ident", "path": "Ident"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [365, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:342`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create an `Ident` with the given `span` and `value` (unquoted).
