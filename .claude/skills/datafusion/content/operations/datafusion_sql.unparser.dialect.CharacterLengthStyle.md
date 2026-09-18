# `datafusion_sql::unparser::dialect::CharacterLengthStyle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.CharacterLengthStyle.json).

<a id="op-10e123a0c1c71f47a8a187e4"></a>
## CharacterLengthStyle

`enum` · `datafusion_sql::unparser::dialect::CharacterLengthStyle` · datafusion-sql 55.1.0

```rust
enum CharacterLengthStyle
```

Source: `src/unparser/dialect.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

`CharacterLengthStyle` to use for unparsing

Different DBMSs uses different names for function calculating the number of characters in the string
`Length` style uses length(x)
`SQLStandard` style uses character_length(x)

<a id="op-18afab6c2e77c9a9489aaba1"></a>
## CharacterLength

`variant` · `datafusion_sql::unparser::dialect::CharacterLengthStyle::CharacterLength` · datafusion-sql 55.1.0

```rust
CharacterLength
```

Source: `src/unparser/dialect.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e568ffb9cb475fad31b0a557"></a>
## Length

`variant` · `datafusion_sql::unparser::dialect::CharacterLengthStyle::Length` · datafusion-sql 55.1.0

```rust
Length
```

Source: `src/unparser/dialect.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b048c2e7ca78101f87494356"></a>
## clone

`function` · `datafusion_sql::unparser::dialect::CharacterLengthStyle::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> CharacterLengthStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CharacterLengthStyle", "path": "CharacterLengthStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 10], "end": [335, 15], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/dialect.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7528507c534fa50a336d5705"></a>
## eq

`function` · `datafusion_sql::unparser::dialect::CharacterLengthStyle::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &CharacterLengthStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CharacterLengthStyle", "path": "CharacterLengthStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 23], "end": [335, 32], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unparser/dialect.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
