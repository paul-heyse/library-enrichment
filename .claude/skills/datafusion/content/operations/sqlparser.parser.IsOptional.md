# `sqlparser::parser::IsOptional`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.parser.IsOptional.json).

<a id="op-b8f21fa1484e2b97758680dd"></a>
## IsOptional

`enum` · `sqlparser::parser::IsOptional` · sqlparser 0.62.0

```rust
enum IsOptional
```

Source: `src/parser/mod.rs:161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates whether a parser element is optional or mandatory.

<a id="op-804dc3ad47dafa5eb5d61772"></a>
## Mandatory

`variant` · `sqlparser::parser::IsOptional::Mandatory` · sqlparser 0.62.0

```rust
Mandatory
```

Source: `src/parser/mod.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The element is mandatory.

<a id="op-4e74511192c027e37ed4a6f5"></a>
## Optional

`variant` · `sqlparser::parser::IsOptional::Optional` · sqlparser 0.62.0

```rust
Optional
```

Source: `src/parser/mod.rs:163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The element is optional.

<a id="op-4c7c35afffa589b92b9f9bc1"></a>
## eq

`function` · `sqlparser::parser::IsOptional::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IsOptional) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::parser::IsOptional", "path": "IsOptional"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 10], "end": [159, 19], "filename": "src/parser/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser/mod.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
