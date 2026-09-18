# `datafusion_functions_window::nth_value::NthValueKind`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.nth_value.NthValueKind.json).

<a id="op-f0930fac63baddb73380fae7"></a>
## NthValueKind

`enum` · `datafusion_functions_window::nth_value::NthValueKind` · datafusion-functions-window 55.1.0

```rust
enum NthValueKind
```

Source: `src/nth_value.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Tag to differentiate special use cases of the NTH_VALUE built-in window function.

<a id="op-2c1dcb6c37feca3fc5ab7efc"></a>
## First

`variant` · `datafusion_functions_window::nth_value::NthValueKind::First` · datafusion-functions-window 55.1.0

```rust
First
```

Source: `src/nth_value.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb264e6f033590614ed990df"></a>
## Last

`variant` · `datafusion_functions_window::nth_value::NthValueKind::Last` · datafusion-functions-window 55.1.0

```rust
Last
```

Source: `src/nth_value.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4f4489e95e09e22de010b00"></a>
## Nth

`variant` · `datafusion_functions_window::nth_value::NthValueKind::Nth` · datafusion-functions-window 55.1.0

```rust
Nth
```

Source: `src/nth_value.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb3879bde5076861d354c454"></a>
## clone

`function` · `datafusion_functions_window::nth_value::NthValueKind::clone` · datafusion-functions-window 55.1.0

```rust
fn clone(&self) -> NthValueKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValueKind", "path": "NthValueKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 23], "end": [73, 28], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/nth_value.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d3500398ad2293f38ddffa6"></a>
## eq

`function` · `datafusion_functions_window::nth_value::NthValueKind::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &NthValueKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValueKind", "path": "NthValueKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 30], "end": [73, 39], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/nth_value.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dca42a8a7591f0afaa70472"></a>
## fmt

`function` · `datafusion_functions_window::nth_value::NthValueKind::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValueKind", "path": "NthValueKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/nth_value.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-146038dab9a1d6a1aaf336e2"></a>
## hash

`function` · `datafusion_functions_window::nth_value::NthValueKind::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::nth_value::NthValueKind", "path": "NthValueKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 45], "end": [73, 49], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/nth_value.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
