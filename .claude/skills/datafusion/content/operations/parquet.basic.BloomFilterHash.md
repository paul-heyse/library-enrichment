# `parquet::basic::BloomFilterHash`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.BloomFilterHash.json).

<a id="op-ed9a3f3da2bfea4aa6c7fac8"></a>
## BloomFilterHash

`enum` · `parquet::basic::BloomFilterHash` · parquet 59.3.0

```rust
enum BloomFilterHash
```

Source: `src/basic.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The hash function used in Bloom filter. This function takes the hash of a column value
using plain encoding.

<a id="op-1205d66e577cbbcec4663102"></a>
## XXHASH

`variant` · `parquet::basic::BloomFilterHash::XXHASH` · parquet 59.3.0

```rust
XXHASH
```

Source: `src/basic.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

xxHash Strategy.

<a id="op-3891628dd16117f59de944a4"></a>
## clone

`function` · `parquet::basic::BloomFilterHash::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterHash
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterHash", "path": "BloomFilterHash"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [949, 1], "end": [956, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3af63a3bdcfc813d90bb067"></a>
## eq

`function` · `parquet::basic::BloomFilterHash::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BloomFilterHash) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterHash", "path": "BloomFilterHash"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [949, 1], "end": [956, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e958362dcaa2f01d909d2308"></a>
## fmt

`function` · `parquet::basic::BloomFilterHash::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::BloomFilterHash", "path": "BloomFilterHash"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [949, 1], "end": [956, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:949`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
