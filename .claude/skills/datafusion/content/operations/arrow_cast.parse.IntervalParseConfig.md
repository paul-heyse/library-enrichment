# `arrow_cast::parse::IntervalParseConfig`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.IntervalParseConfig.json).

<a id="op-b58f87de9767e20d305af7fe"></a>
## IntervalParseConfig

`struct` · `arrow_cast::parse::IntervalParseConfig` · arrow-cast 59.3.0

```rust
struct IntervalParseConfig
```

Source: `src/parse.rs:1104`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Config to parse interval strings

Currently stores the `default_unit` to use if the string doesn't have one specified

<a id="op-2001a201a50729b444bbd402"></a>
## clone

`function` · `arrow_cast::parse::IntervalParseConfig::clone` · arrow-cast 59.3.0

```rust
fn clone(&self) -> IntervalParseConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalParseConfig", "path": "IntervalParseConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 17], "end": [1103, 22], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parse.rs:1103`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8ab2f939442e41752602af7"></a>
## fmt

`function` · `arrow_cast::parse::IntervalParseConfig::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalParseConfig", "path": "IntervalParseConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1103, 10], "end": [1103, 15], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parse.rs:1103`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ea325709e164284997de1ab"></a>
## new

`function` · `arrow_cast::parse::IntervalParseConfig::new` · arrow-cast 59.3.0

```rust
fn new(default_unit: IntervalUnit) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalParseConfig", "path": "IntervalParseConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 1], "end": [1115, 2], "filename": "src/parse.rs"}, "trait": null, "trait_path": null}`

Source: `src/parse.rs:1112`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Create a new [IntervalParseConfig](../operations/arrow_cast.parse.IntervalParseConfig.md#op-b58f87de9767e20d305af7fe) with the given default unit
