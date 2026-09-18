# `datafusion_common::unnest::NullHandling`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.unnest.NullHandling.json).

<a id="op-68e5cdc7da449744d6708464"></a>
## NullHandling

`enum` · `datafusion_common::unnest::NullHandling` · datafusion-common 55.1.0

```rust
enum NullHandling
```

Source: `src/unnest.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

How [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) handles `NULL` and empty list values in the input column.

The variants enumerate the three observable behaviors so that callers do
not have to compose multiple boolean flags to express what they want.

<a id="op-f4589eddcff39f31e7244379"></a>
## Drop

`variant` · `datafusion_common::unnest::NullHandling::Drop` · datafusion-common 55.1.0

```rust
Drop
```

Source: `src/unnest.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Drop rows where the input list is `NULL` or empty. Matches the
default behavior of systems such as DuckDB and ClickHouse.

<a id="op-4dedf59fe69b83f1dd1f200f"></a>
## Preserve

`variant` · `datafusion_common::unnest::NullHandling::Preserve` · datafusion-common 55.1.0

```rust
Preserve
```

Source: `src/unnest.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Preserve `NULL` input rows as a single output row containing `NULL`.
Empty lists still produce zero output rows. This is the default and
matches DataFusion's historical `preserve_nulls = true` behavior.

<a id="op-c44c790e1010575c59d513ff"></a>
## PreserveAndExpandEmpty

`variant` · `datafusion_common::unnest::NullHandling::PreserveAndExpandEmpty` · datafusion-common 55.1.0

```rust
PreserveAndExpandEmpty
```

Source: `src/unnest.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Like [`Self::Preserve`](../operations/datafusion_common.unnest.NullHandling.md#op-4dedf59fe69b83f1dd1f200f), and additionally treat an empty list
identically to a `NULL` list, producing a single output row
containing `NULL`.

<a id="op-3c6a762e88de91ad1dd9f73f"></a>
## clone

`function` · `datafusion_common::unnest::NullHandling::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> NullHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b6c39cb3967c660176fa3c"></a>
## default

`function` · `datafusion_common::unnest::NullHandling::default` · datafusion-common 55.1.0

```rust
fn default() -> NullHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 30], "end": [26, 37], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55f92dc2808202d87adbd6da"></a>
## eq

`function` · `datafusion_common::unnest::NullHandling::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &NullHandling) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 39], "end": [26, 48], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00641a8502f1dd2f1c80a33"></a>
## fmt

`function` · `datafusion_common::unnest::NullHandling::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60f4f609495e5c34e783839b"></a>
## hash

`function` · `datafusion_common::unnest::NullHandling::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 66], "end": [26, 70], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6178b8cdd820fccd4a4e158a"></a>
## partial_cmp

`function` · `datafusion_common::unnest::NullHandling::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &NullHandling) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::unnest::NullHandling", "path": "NullHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 54], "end": [26, 64], "filename": "src/unnest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/unnest.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
