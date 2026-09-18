# `datafusion_common::stats::NdvFallback`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.stats.NdvFallback.json).

<a id="op-99558ddcae7487bd8a35c4e0"></a>
## NdvFallback

`enum` · `datafusion_common::stats::NdvFallback` · datafusion-common 55.1.0

```rust
enum NdvFallback
```

Source: `src/stats.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Fallback to use when NDV overlap can not be estimated from column bounds.

<a id="op-894f13b7f6fa8b6caeb809f8"></a>
## Max

`variant` · `datafusion_common::stats::NdvFallback::Max` · datafusion-common 55.1.0

```rust
Max
```

Source: `src/stats.rs:391`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Use the larger input NDV. This is the conservative default for
related fragments such as files from the same table.

<a id="op-7ce39289fcbe776544fc76a5"></a>
## Sum

`variant` · `datafusion_common::stats::NdvFallback::Sum` · datafusion-common 55.1.0

```rust
Sum
```

Source: `src/stats.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sum the input NDVs. This is a conservative upper bound for
independent inputs such as `UNION ALL`.

<a id="op-241c04efb5cde9e36df84b87"></a>
## clone

`function` · `datafusion_common::stats::NdvFallback::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> NdvFallback
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::NdvFallback", "path": "NdvFallback"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 17], "end": [386, 22], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stats.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-047a070d2ad5194f8c3efbcd"></a>
## default

`function` · `datafusion_common::stats::NdvFallback::default` · datafusion-common 55.1.0

```rust
fn default() -> NdvFallback
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::NdvFallback", "path": "NdvFallback"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 45], "end": [386, 52], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stats.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5174f7b719ff8f38629fc2ff"></a>
## eq

`function` · `datafusion_common::stats::NdvFallback::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &NdvFallback) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::NdvFallback", "path": "NdvFallback"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 30], "end": [386, 39], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stats.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d059b3235e7023a6c82f435"></a>
## fmt

`function` · `datafusion_common::stats::NdvFallback::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::stats::NdvFallback", "path": "NdvFallback"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 10], "end": [386, 15], "filename": "src/stats.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stats.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
