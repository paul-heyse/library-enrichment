# `sqlparser::ast::value::NormalizationForm`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.NormalizationForm.json).

<a id="op-323044e65293a9549db747cd"></a>
## NormalizationForm

`enum` · `sqlparser::ast::value::NormalizationForm` · sqlparser 0.62.0

```rust
enum NormalizationForm
```

Source: `src/ast/value.rs:511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The Unicode Standard defines four normalization forms, which are intended to eliminate
certain distinctions between visually or functionally identical characters.

See [Unicode Normalization Forms](https://unicode.org/reports/tr15/) for details.

<a id="op-f41ae1092b1e6b91d351e754"></a>
## NFC

`variant` · `sqlparser::ast::value::NormalizationForm::NFC` · sqlparser 0.62.0

```rust
NFC
```

Source: `src/ast/value.rs:513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Canonical Decomposition, followed by Canonical Composition.

<a id="op-855b9acc0e688013e6952778"></a>
## NFD

`variant` · `sqlparser::ast::value::NormalizationForm::NFD` · sqlparser 0.62.0

```rust
NFD
```

Source: `src/ast/value.rs:515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Canonical Decomposition.

<a id="op-f4623b23c7d6ea6846d9c019"></a>
## NFKC

`variant` · `sqlparser::ast::value::NormalizationForm::NFKC` · sqlparser 0.62.0

```rust
NFKC
```

Source: `src/ast/value.rs:517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Compatibility Decomposition, followed by Canonical Composition.

<a id="op-e8fe0653d9472ed6db3529c9"></a>
## NFKD

`variant` · `sqlparser::ast::value::NormalizationForm::NFKD` · sqlparser 0.62.0

```rust
NFKD
```

Source: `src/ast/value.rs:519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Compatibility Decomposition.

<a id="op-6ae1b9e738b7b0127ea016b2"></a>
## clone

`function` · `sqlparser::ast::value::NormalizationForm::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NormalizationForm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 17], "end": [504, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d65e528fd726f195eb7fa5f"></a>
## cmp

`function` · `sqlparser::ast::value::NormalizationForm::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NormalizationForm) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 45], "end": [504, 48], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-361109e91c8bd23c2be7e0d0"></a>
## deserialize

`function` · `sqlparser::ast::value::NormalizationForm::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 49], "end": [505, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6135e04bc2917502d6ea5c03"></a>
## eq

`function` · `sqlparser::ast::value::NormalizationForm::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NormalizationForm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 30], "end": [504, 39], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18a4ca6a23385f20bbecf00f"></a>
## fmt

`function` · `sqlparser::ast::value::NormalizationForm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 10], "end": [504, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31cb166cbbc56612449898bd"></a>
## fmt

`function` · `sqlparser::ast::value::NormalizationForm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [531, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24cb64c394f70cf998126e1b"></a>
## hash

`function` · `sqlparser::ast::value::NormalizationForm::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 62], "end": [504, 66], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aa9ce753f88b19f05405d09"></a>
## partial_cmp

`function` · `sqlparser::ast::value::NormalizationForm::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NormalizationForm) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 50], "end": [504, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91dd8e5a50b91221f332adf2"></a>
## serialize

`function` · `sqlparser::ast::value::NormalizationForm::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 38], "end": [505, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10c3abe253c1520a7b74ec32"></a>
## visit

`function` · `sqlparser::ast::value::NormalizationForm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 40], "end": [506, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6288692c8e8925d521d8df12"></a>
## visit

`function` · `sqlparser::ast::value::NormalizationForm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::NormalizationForm", "path": "NormalizationForm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 47], "end": [506, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
