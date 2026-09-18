# `sqlparser::ast::value::ValueWithSpan`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.ValueWithSpan.json).

<a id="op-0bfe0aa12515e78fed80065b"></a>
## ValueWithSpan

`struct` · `sqlparser::ast::value::ValueWithSpan` · sqlparser 0.62.0

```rust
struct ValueWithSpan
```

Source: `src/ast/value.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Wraps a primitive SQL [`Value`](../operations/sqlparser.ast.value.Value.md#op-bc448bc8bde6975348b4ad8b)  with its [`Span`](../operations/sqlparser.tokenizer.Span.md#op-c744756cf28c8276d407a53a) location

# Example: create a `ValueWithSpan` from a `Value`
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
// from line 1, column 1 to line 1, column 7
let span = Span::new(Location::new(1, 1), Location::new(1, 7));
let value_with_span = value.with_span(span);
```

# Example: create a `ValueWithSpan` from a `Value` with an empty span

You can call [`Value::with_empty_span`](../operations/sqlparser.ast.value.Value.md#op-debb7e074f0dadbf3d8c1278) to create a `ValueWithSpan` with an empty span
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
let value_with_span = value.with_empty_span();
assert_eq!(value_with_span.span, Span::empty());
```

You can also use the [`From`] trait to convert  `ValueWithSpan` to/from `Value`s
```
# use sqlparser::ast::{Value, ValueWithSpan};
# use sqlparser::tokenizer::{Location, Span};
let value = Value::SingleQuotedString(String::from("endpoint"));
// converting `Value` to `ValueWithSpan` results in an empty span
let value_with_span: ValueWithSpan = value.into();
assert_eq!(value_with_span.span, Span::empty());
// convert back to `Value`
let value: Value = value_with_span.into();
```
A `Value` paired with its source `Span` location.

Unresolved upstream links (retained, not inferred): ``From``.

<a id="op-6da87d1cd9ac012bc804d88f"></a>
## Target

`assoc_type` · `sqlparser::ast::value::ValueWithSpan::Target` · sqlparser 0.62.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [127, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/value.rs:122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d47968e13f2de8666ca0b494"></a>
## clone

`function` · `sqlparser::ast::value::ValueWithSpan::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ValueWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 17], "end": [71, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9b22dbca89122e815a3e96a"></a>
## cmp

`function` · `sqlparser::ast::value::ValueWithSpan::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Self) -> core::cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [95, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:92`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f993d24af5e34b3de8a0d6f7"></a>
## deref

`function` · `sqlparser::ast::value::ValueWithSpan::deref` · sqlparser 0.62.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [127, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/value.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48de57cae0c9f5ca4b7d0f5d"></a>
## deref_mut

`function` · `sqlparser::ast::value::ValueWithSpan::deref_mut` · sqlparser 0.62.0

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [133, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::ops::deref::DerefMut", "path": "DerefMut"}, "trait_path": "core::ops::deref::DerefMut"}`

Source: `src/ast/value.rs:130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12ca79413c2dbc2712f702ce"></a>
## deserialize

`function` · `sqlparser::ast::value::ValueWithSpan::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 49], "end": [72, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32501d722980e1933b855bf3"></a>
## eq

`function` · `sqlparser::ast::value::ValueWithSpan::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [89, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1af78c5ec59ad0904faa4ded"></a>
## fmt

`function` · `sqlparser::ast::value::ValueWithSpan::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [259, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a14bb6127a35865d2987c29f"></a>
## fmt

`function` · `sqlparser::ast::value::ValueWithSpan::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c995dec9159438b0f6919f37"></a>
## from

`function` · `sqlparser::ast::value::ValueWithSpan::from` · sqlparser 0.62.0

```rust
fn from(value: Value) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [113, 2], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/value.rs:110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78deb6b6aef767e079564d4c"></a>
## hash

`function` · `sqlparser::ast::value::ValueWithSpan::hash` · sqlparser 0.62.0

```rust
fn hash<H: core::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [107, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adbaf98e36dced3551c0339c"></a>
## into_string

`function` · `sqlparser::ast::value::ValueWithSpan::into_string` · sqlparser 0.62.0

```rust
fn into_string(self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [215, 2], "filename": "src/ast/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/value.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the underlying literal is a string, regardless of quote style, returns the associated string value

<a id="op-eeb345970b47b62c4983c04a"></a>
## partial_cmp

`function` · `sqlparser::ast::value::ValueWithSpan::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [101, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-882f543e8e7577116505e5c6"></a>
## serialize

`function` · `sqlparser::ast::value::ValueWithSpan::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 38], "end": [72, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24dceccc03535cb7aa817c16"></a>
## span

`struct_field` · `sqlparser::ast::value::ValueWithSpan::span` · sqlparser 0.62.0

```rust
span: tokenizer::Span
```

Source: `src/ast/value.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source `Span` covering the token(s) that produced the value.

<a id="op-66a2b23ce0a232a148132de1"></a>
## span

`function` · `sqlparser::ast::value::ValueWithSpan::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "super::value::ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2207, 1], "end": [2211, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0d7b4b00b18f66566c0f5e8"></a>
## value

`struct_field` · `sqlparser::ast::value::ValueWithSpan::value` · sqlparser 0.62.0

```rust
value: Value
```

Source: `src/ast/value.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The wrapped `Value`.

<a id="op-603234387abee2a397574e15"></a>
## visit

`function` · `sqlparser::ast::value::ValueWithSpan::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 12], "end": [75, 17], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-940bb012600d317a2949294e"></a>
## visit

`function` · `sqlparser::ast::value::ValueWithSpan::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 19], "end": [75, 27], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
