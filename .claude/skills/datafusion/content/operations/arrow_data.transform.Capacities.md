# `arrow_data::transform::Capacities`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.transform.Capacities.json).

<a id="op-85e7a2e1566fea07900143a1"></a>
## Capacities

`enum` · `arrow_data::transform::Capacities` · arrow-data 59.3.0

```rust
enum Capacities
```

Source: `src/transform/mod.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Define capacities to pre-allocate for child data or data buffers.

<a id="op-350aad34918f06dc4329c5fb"></a>
## Array

`variant` · `arrow_data::transform::Capacities::Array` · arrow-data 59.3.0

```rust
Array
```

Source: `src/transform/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Don't preallocate inner buffers and rely on array growth strategy

<a id="op-75baf814d13e80aa46b9bd97"></a>
## Binary

`variant` · `arrow_data::transform::Capacities::Binary` · arrow-data 59.3.0

```rust
Binary
```

Source: `src/transform/mod.rs:374`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Binary, Utf8 and LargeUtf8 data types

Defines
* the capacity of the array offsets
* the capacity of the binary/ str buffer

<a id="op-3c3a7a534953d94d7610a3db"></a>
## Dictionary

`variant` · `arrow_data::transform::Capacities::Dictionary` · arrow-data 59.3.0

```rust
Dictionary
```

Source: `src/transform/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Dictionary type

Defines
* the capacity of the array/keys
* the capacity of the values

<a id="op-e2baa3e1869a5c6d9035aeae"></a>
## List

`variant` · `arrow_data::transform::Capacities::List` · arrow-data 59.3.0

```rust
List
```

Source: `src/transform/mod.rs:380`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

List and LargeList data types

Defines
* the capacity of the array offsets
* the capacity of the child data

<a id="op-5efd1dc10d030b71bec240c6"></a>
## Struct

`variant` · `arrow_data::transform::Capacities::Struct` · arrow-data 59.3.0

```rust
Struct
```

Source: `src/transform/mod.rs:386`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Struct type

Defines
* the capacity of the array
* the capacities of the fields

<a id="op-1d449f8d4f46058e5b1d8f70"></a>
## clone

`function` · `arrow_data::transform::Capacities::clone` · arrow-data 59.3.0

```rust
fn clone(&self) -> Capacities
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::transform::Capacities", "path": "Capacities"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 17], "end": [367, 22], "filename": "src/transform/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/transform/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b385ff8c4f35bc49494e626c"></a>
## fmt

`function` · `arrow_data::transform::Capacities::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::transform::Capacities", "path": "Capacities"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [367, 10], "end": [367, 15], "filename": "src/transform/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/transform/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
