# `datafusion_execution::cache::CacheEntryInfo`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.CacheEntryInfo.json).

<a id="op-8c45de8579e838b42fccb594"></a>
## CacheEntryInfo

`struct` · `datafusion_execution::cache::CacheEntryInfo` · datafusion-execution 55.1.0

```rust
struct CacheEntryInfo<V>
```

Source: `src/cache/mod.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22818b10e6a64ef3d8d927a4"></a>
## clone

`function` · `datafusion_execution::cache::CacheEntryInfo::clone` · datafusion-execution 55.1.0

```rust
fn clone(&self) -> CacheEntryInfo<V>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::CacheEntryInfo", "path": "CacheEntryInfo"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 10], "end": [112, 15], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cache/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b54e115a2e8118367fb206d5"></a>
## eq

`function` · `datafusion_execution::cache::CacheEntryInfo::eq` · datafusion-execution 55.1.0

```rust
fn eq(&self, other: &CacheEntryInfo<V>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::CacheEntryInfo", "path": "CacheEntryInfo"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 24], "end": [112, 33], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cache/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d89135ec939102eab2400f4c"></a>
## expires

`struct_field` · `datafusion_execution::cache::CacheEntryInfo::expires` · datafusion-execution 55.1.0

```rust
expires: Option<datafusion_common::instant::Instant>
```

Source: `src/cache/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b1cbca7a37370253f746b4"></a>
## fmt

`function` · `datafusion_execution::cache::CacheEntryInfo::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "datafusion_execution::cache::CacheEntryInfo", "path": "CacheEntryInfo"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 17], "end": [112, 22], "filename": "src/cache/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb2df62e457bc219b9fc661"></a>
## hits

`struct_field` · `datafusion_execution::cache::CacheEntryInfo::hits` · datafusion-execution 55.1.0

```rust
hits: usize
```

Source: `src/cache/mod.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-237413cff70928b0163a9138"></a>
## size_bytes

`struct_field` · `datafusion_execution::cache::CacheEntryInfo::size_bytes` · datafusion-execution 55.1.0

```rust
size_bytes: usize
```

Source: `src/cache/mod.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96bdb2e3944ef66f143b4e8f"></a>
## value

`struct_field` · `datafusion_execution::cache::CacheEntryInfo::value` · datafusion-execution 55.1.0

```rust
value: V
```

Source: `src/cache/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
