# `arrow_cast::display::DurationFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.display.DurationFormat.json).

<a id="op-ae18af0f29abc24d8d67ccc5"></a>
## DurationFormat

`enum` · `arrow_cast::display::DurationFormat` · arrow-cast 59.3.0

```rust
enum DurationFormat
```

Source: `src/display.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Format for displaying durations

<a id="op-c697aa6d5036a8badee42bc8"></a>
## ISO8601

`variant` · `arrow_cast::display::DurationFormat::ISO8601` · arrow-cast 59.3.0

```rust
ISO8601
```

Source: `src/display.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

ISO 8601 - `P198DT72932.972880S`

<a id="op-ab565c415450c645dfeca674"></a>
## Pretty

`variant` · `arrow_cast::display::DurationFormat::Pretty` · arrow-cast 59.3.0

```rust
Pretty
```

Source: `src/display.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A human readable representation - `198 days 16 hours 34 mins 15.407810000 secs`

<a id="op-69f779123448fd78df167e8e"></a>
## clone

`function` · `arrow_cast::display::DurationFormat::clone` · arrow-cast 59.3.0

```rust
fn clone(&self) -> DurationFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::DurationFormat", "path": "DurationFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 23], "end": [43, 28], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5da9928ac13c2c2aa3123b77"></a>
## eq

`function` · `arrow_cast::display::DurationFormat::eq` · arrow-cast 59.3.0

```rust
fn eq(&self, other: &DurationFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::DurationFormat", "path": "DurationFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 30], "end": [43, 39], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/display.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-357b08dd2a0d30183609e61c"></a>
## fmt

`function` · `arrow_cast::display::DurationFormat::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::DurationFormat", "path": "DurationFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-872779829eb0a49d2c6e6cd5"></a>
## hash

`function` · `arrow_cast::display::DurationFormat::hash` · arrow-cast 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::display::DurationFormat", "path": "DurationFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 45], "end": [43, 49], "filename": "src/display.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/display.rs:43`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
