# `parquet::basic::BloomFilterAlgorithm`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.BloomFilterAlgorithm.json).

<a id="op-adb6a5e7aba52b0cdc0ceb7c"></a>
## BloomFilterAlgorithm

`enum` · `parquet::basic::BloomFilterAlgorithm` · parquet 59.3.0

```rust
enum BloomFilterAlgorithm
```

Source: `src/basic.rs:938`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The algorithm used in Bloom filter.

<a id="op-9d36d78c56460e690b5d01e8"></a>
## BLOCK

`variant` · `parquet::basic::BloomFilterAlgorithm::BLOCK` · parquet 59.3.0

```rust
BLOCK
```

Source: `src/basic.rs:938`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Block-based Bloom filter.

<a id="op-c376baf6f2f6993580742125"></a>
## clone

`function` · `parquet::basic::BloomFilterAlgorithm::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterAlgorithm", "path": "BloomFilterAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [944, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:938`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6457cd950a058dbb3c509069"></a>
## eq

`function` · `parquet::basic::BloomFilterAlgorithm::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BloomFilterAlgorithm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterAlgorithm", "path": "BloomFilterAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [944, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:938`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e599e2a4907970ca4d318b3f"></a>
## fmt

`function` · `parquet::basic::BloomFilterAlgorithm::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterAlgorithm", "path": "BloomFilterAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [938, 1], "end": [944, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:938`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
