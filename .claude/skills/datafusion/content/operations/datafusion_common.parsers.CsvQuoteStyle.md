# `datafusion_common::parsers::CsvQuoteStyle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.parsers.CsvQuoteStyle.json).

<a id="op-ace35ea38075b344649b9b86"></a>
## CsvQuoteStyle

`enum` · `datafusion_common::parsers::CsvQuoteStyle` · datafusion-common 55.1.0

```rust
enum CsvQuoteStyle
```

Source: `src/parsers.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

CSV quote style

Controls when fields are quoted when writing CSV files.
Corresponds to [`arrow::csv::QuoteStyle`].

Unresolved upstream links (retained, not inferred): ``arrow::csv::QuoteStyle``.

<a id="op-d43f812bf761c6e2ff84df38"></a>
## Always

`variant` · `datafusion_common::parsers::CsvQuoteStyle::Always` · datafusion-common 55.1.0

```rust
Always
```

Source: `src/parsers.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Quote all fields

<a id="op-0377106a6a42148fecf1c159"></a>
## Err

`assoc_type` · `datafusion_common::parsers::CsvQuoteStyle::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [108, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parsers.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-624b2adf7f8dc3aba6e8fb74"></a>
## Necessary

`variant` · `datafusion_common::parsers::CsvQuoteStyle::Necessary` · datafusion-common 55.1.0

```rust
Necessary
```

Source: `src/parsers.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Only quote fields when necessary (default)

<a id="op-4655c112d4fd0f0302b6c2d7"></a>
## Never

`variant` · `datafusion_common::parsers::CsvQuoteStyle::Never` · datafusion-common 55.1.0

```rust
Never
```

Source: `src/parsers.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Never quote fields

<a id="op-3deb52c6ce35cf749ca7d72d"></a>
## NonNumeric

`variant` · `datafusion_common::parsers::CsvQuoteStyle::NonNumeric` · datafusion-common 55.1.0

```rust
NonNumeric
```

Source: `src/parsers.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Quote all non-numeric fields

<a id="op-3014aa8697ff35b60d0ba97e"></a>
## clone

`function` · `datafusion_common::parsers::CsvQuoteStyle::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 17], "end": [81, 22], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parsers.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d14fd12aafbb2c0cc787080"></a>
## default

`function` · `datafusion_common::parsers::CsvQuoteStyle::default` · datafusion-common 55.1.0

```rust
fn default() -> CsvQuoteStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 51], "end": [81, 58], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/parsers.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b86b9eaa1bd9919d0c3aadc8"></a>
## eq

`function` · `datafusion_common::parsers::CsvQuoteStyle::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &CsvQuoteStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 30], "end": [81, 39], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parsers.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-486b6c0bbb7a7e35da9d2971"></a>
## fmt

`function` · `datafusion_common::parsers::CsvQuoteStyle::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 10], "end": [81, 15], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parsers.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc6f9d38a66702a7a47de0db"></a>
## fmt

`function` · `datafusion_common::parsers::CsvQuoteStyle::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [131, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parsers.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31564d3605dfdf54f409a381"></a>
## from_str

`function` · `datafusion_common::parsers::CsvQuoteStyle::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [108, 2], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parsers.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62294281d1e6eeff412c1b66"></a>
## hash

`function` · `datafusion_common::parsers::CsvQuoteStyle::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 45], "end": [81, 49], "filename": "src/parsers.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/parsers.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-170f75f57f39f6587ae7f79f"></a>
## set

`function` · `datafusion_common::parsers::CsvQuoteStyle::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "crate::parsers::CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2486, 1], "end": [2495, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2491`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-009b76cac55b795ce9cc754f"></a>
## visit

`function` · `datafusion_common::parsers::CsvQuoteStyle::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parsers::CsvQuoteStyle", "path": "crate::parsers::CsvQuoteStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2486, 1], "end": [2495, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:2487`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
