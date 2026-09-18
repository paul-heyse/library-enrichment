# `arrow_ord::partition::Partitions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.partition.Partitions.json).

<a id="op-9548b71e532931bce2ff0343"></a>
## Partitions

`struct` · `arrow_ord::partition::Partitions` · arrow-ord 59.3.0

```rust
struct Partitions
```

Source: `src/partition.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

A computed set of partitions, see [`partition`](../operations/arrow_ord.partition.partition.md#op-bb0253e80df493c54b1c48d6)

<a id="op-6210c7b437ed835dd8dde57f"></a>
## clone

`function` · `arrow_ord::partition::Partitions::clone` · arrow-ord 59.3.0

```rust
fn clone(&self) -> Partitions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::partition::Partitions", "path": "Partitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/partition.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/partition.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4614835f174839a661bcfd50"></a>
## fmt

`function` · `arrow_ord::partition::Partitions::fmt` · arrow-ord 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::partition::Partitions", "path": "Partitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/partition.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partition.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1718053d8761f646a1f8ae78"></a>
## is_empty

`function` · `arrow_ord::partition::Partitions::is_empty` · arrow-ord 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::partition::Partitions", "path": "Partitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [70, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Returns true if this contains no partitions

<a id="op-fb575ea0459cb733f6525d27"></a>
## len

`function` · `arrow_ord::partition::Partitions::len` · arrow-ord 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::partition::Partitions", "path": "Partitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [70, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Returns the number of partitions

<a id="op-3bd2d24488b3f9c17b7149a2"></a>
## ranges

`function` · `arrow_ord::partition::Partitions::ranges` · arrow-ord 59.3.0

```rust
fn ranges(&self) -> Vec<Range<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ord::partition::Partitions", "path": "Partitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [70, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Returns the range of each partition

Consecutive ranges will be contiguous: i.e [`(a, b)` and `(b, c)`], and
`start = 0` and `end = self.len()` for the first and last range respectively
