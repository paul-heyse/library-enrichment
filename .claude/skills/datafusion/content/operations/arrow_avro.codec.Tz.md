# `arrow_avro::codec::Tz`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.codec.Tz.json).

<a id="op-ceb44017f5bbf57a91377796"></a>
## Tz

`enum` · `arrow_avro::codec::Tz` · arrow-avro 59.3.0

```rust
enum Tz
```

Source: `src/codec.rs:716`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Timezone representation for timestamps.

Avro only distinguishes between UTC and local time (no timezone), but Arrow supports
any of the two identifiers of the UTC timezone: "+00:00" and "UTC".
The data types using these time zone IDs behave identically, but are not logically equal.

<a id="op-e73de2901fe33cccaa507f16"></a>
## OffsetZero

`variant` · `arrow_avro::codec::Tz::OffsetZero` · arrow-avro 59.3.0

```rust
OffsetZero
```

Source: `src/codec.rs:719`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Represent Avro `timestamp-*` logical types with "+00:00" timezone ID

<a id="op-34118d3b6995e76eb01522e7"></a>
## Utc

`variant` · `arrow_avro::codec::Tz::Utc` · arrow-avro 59.3.0

```rust
Utc
```

Source: `src/codec.rs:721`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Represent Avro `timestamp-*` logical types with "UTC" timezone ID

<a id="op-c11c59de103faef8adf5c10a"></a>
## as_str

`function` · `arrow_avro::codec::Tz::as_str` · arrow-avro 59.3.0

```rust
fn as_str(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [724, 1], "end": [732, 2], "filename": "src/codec.rs"}, "trait": null, "trait_path": null}`

Source: `src/codec.rs:726`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

Returns the string identifier for this timezone representation

<a id="op-94f254266404dcf3ad470b5a"></a>
## clone

`function` · `arrow_avro::codec::Tz::clone` · arrow-avro 59.3.0

```rust
fn clone(&self) -> Tz
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 23], "end": [715, 28], "filename": "src/codec.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/codec.rs:715`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8215e33f22ee264eef22fd22"></a>
## default

`function` · `arrow_avro::codec::Tz::default` · arrow-avro 59.3.0

```rust
fn default() -> Tz
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 41], "end": [715, 48], "filename": "src/codec.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/codec.rs:715`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1fce8abc782252e0e2d02ae"></a>
## eq

`function` · `arrow_avro::codec::Tz::eq` · arrow-avro 59.3.0

```rust
fn eq(&self, other: &Tz) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 30], "end": [715, 39], "filename": "src/codec.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/codec.rs:715`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c259863c3646b54f8b9f303"></a>
## fmt

`function` · `arrow_avro::codec::Tz::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [734, 1], "end": [738, 2], "filename": "src/codec.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/codec.rs:735`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c672edfefbacc9a0115e81dd"></a>
## fmt

`function` · `arrow_avro::codec::Tz::fmt` · arrow-avro 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_avro::codec::Tz", "path": "Tz"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 10], "end": [715, 15], "filename": "src/codec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/codec.rs:715`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
