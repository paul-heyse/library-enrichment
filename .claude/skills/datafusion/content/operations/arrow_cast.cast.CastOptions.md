# `arrow_cast::cast::CastOptions`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.CastOptions.json).

<a id="op-befce2bf693cce8eadd84fda"></a>
## CastOptions

`struct` · `arrow_cast::cast::CastOptions` · arrow-cast 59.3.0

```rust
struct CastOptions<'a>
```

Source: `src/cast/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

CastOptions provides a way to override the default cast behaviors

<a id="op-8e223b9f65d198a63b0147a6"></a>
## clone

`function` · `arrow_cast::cast::CastOptions::clone` · arrow-cast 59.3.0

```rust
fn clone(&self) -> CastOptions<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::cast::CastOptions", "path": "CastOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 22], "filename": "src/cast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/cast/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9358fd8910f1bdbd448a00c6"></a>
## default

`function` · `arrow_cast::cast::CastOptions::default` · arrow-cast 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_cast::cast::CastOptions", "path": "CastOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [110, 2], "filename": "src/cast/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/cast/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5853848e3d5059b59580c333"></a>
## eq

`function` · `arrow_cast::cast::CastOptions::eq` · arrow-cast 59.3.0

```rust
fn eq(&self, other: &CastOptions<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::cast::CastOptions", "path": "CastOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 24], "end": [95, 33], "filename": "src/cast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cast/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-286b3dcd6595d9487f084c08"></a>
## fmt

`function` · `arrow_cast::cast::CastOptions::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::cast::CastOptions", "path": "CastOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/cast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cast/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37a48cf3ef17300799dd699f"></a>
## format_options

`struct_field` · `arrow_cast::cast::CastOptions::format_options` · arrow-cast 59.3.0

```rust
format_options: display::FormatOptions<'a>
```

Source: `src/cast/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Formatting options when casting from temporal types to string

<a id="op-4b293a71877d54dcab3fc1d0"></a>
## hash

`function` · `arrow_cast::cast::CastOptions::hash` · arrow-cast 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_cast::cast::CastOptions", "path": "CastOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 39], "end": [95, 43], "filename": "src/cast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/cast/mod.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e866b77c8af6ede1f3f743eb"></a>
## safe

`struct_field` · `arrow_cast::cast::CastOptions::safe` · arrow-cast 59.3.0

```rust
safe: bool
```

Source: `src/cast/mod.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

how to handle cast failures, either return NULL (safe=true) or return ERR (safe=false)
