# `datafusion_common::extensions::Extensions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.extensions.Extensions.json).

<a id="op-26fd22d4c6fccf540f6e2750"></a>
## Extensions

`struct` · `datafusion_common::extensions::Extensions` · datafusion-common 55.1.0

```rust
struct Extensions
```

Source: `src/extensions.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A type-keyed map of opaque `Arc`'d values. Each Rust type `T` occupies
its own slot, so independent components can each attach their own data
without conflict.

Cloning is cheap: the backing values are reference-counted.

# Example

```
# use std::sync::Arc;
# use datafusion_common::extensions::Extensions;
struct MyData(u32);
struct OtherData(&'static str);

let mut ext = Extensions::new();
ext.insert(MyData(42));
ext.insert_arc(Arc::new(OtherData("hello")));

assert_eq!(ext.get::<MyData>().unwrap().0, 42);
assert_eq!(ext.get::<OtherData>().unwrap().0, "hello");
```

<a id="op-b213a6ea9023ec25512efd2d"></a>
## clone

`function` · `datafusion_common::extensions::Extensions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 22], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extensions.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba26f23697f98043da4da847"></a>
## contains

`function` · `datafusion_common::extensions::Extensions::contains` · datafusion-common 55.1.0

```rust
fn contains<T: Any + Send + Sync>(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if an extension of type `T` is set.

<a id="op-7facccd91f6e78d2a6da5697"></a>
## default

`function` · `datafusion_common::extensions::Extensions::default` · datafusion-common 55.1.0

```rust
fn default() -> Extensions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 24], "end": [55, 31], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extensions.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0232ec2550389743fd7a089a"></a>
## fmt

`function` · `datafusion_common::extensions::Extensions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/extensions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extensions.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0ba25e80c5bfd20c52924a4"></a>
## get

`function` · `datafusion_common::extensions::Extensions::get` · datafusion-common 55.1.0

```rust
fn get<T: Any + Send + Sync>(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Borrow the extension of type `T`, if set.

<a id="op-e93be00ef8fdab30ef38db43"></a>
## get_arc

`function` · `datafusion_common::extensions::Extensions::get_arc` · datafusion-common 55.1.0

```rust
fn get_arc<T: Any + Send + Sync>(&self) -> Option<Arc<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get a cloned `Arc<T>` of the extension, if set.

<a id="op-c08c7308cfb263ebc11e65e0"></a>
## insert

`function` · `datafusion_common::extensions::Extensions::insert` · datafusion-common 55.1.0

```rust
fn insert<T: Any + Send + Sync>(&mut self, value: T) -> Option<Arc<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Insert an extension keyed by its concrete type `T`. Returns the
previous value of that type, if any.

The value is wrapped in an [`Arc`] internally. If the caller already
has an `Arc<T>` and wants to avoid an extra allocation, use
[`Self::insert_arc`](../operations/datafusion_common.extensions.Extensions.md#op-362ec8b4c50e61e1193db9f4).

Unresolved upstream links (retained, not inferred): ``Arc``.

<a id="op-362ec8b4c50e61e1193db9f4"></a>
## insert_arc

`function` · `datafusion_common::extensions::Extensions::insert_arc` · datafusion-common 55.1.0

```rust
fn insert_arc<T: Any + Send + Sync>(&mut self, value: Arc<T>) -> Option<Arc<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Insert an extension keyed by its concrete type `T`, taking an
already-allocated [`Arc<T>`]. Returns the previous value of that type,
if any.

Unresolved upstream links (retained, not inferred): ``Arc<T>``.

<a id="op-3f1e2c758867939a89e0940c"></a>
## insert_dyn

`function` · `datafusion_common::extensions::Extensions::insert_dyn` · datafusion-common 55.1.0

```rust
fn insert_dyn(&mut self, value: Arc<dyn Any + Send + Sync>) -> Option<Arc<dyn Any + Send + Sync>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Insert an already-type-erased value, keyed by its dynamic
[`TypeId`]. Used internally to support APIs that accept
`Arc<dyn Any + Send + Sync>` for backwards compatibility and need
to recover the concrete type for keying.

New code should use [`Self::insert`](../operations/datafusion_common.extensions.Extensions.md#op-c08c7308cfb263ebc11e65e0) or [`Self::insert_arc`](../operations/datafusion_common.extensions.Extensions.md#op-362ec8b4c50e61e1193db9f4), which
preserve the concrete type at the call site.

Unresolved upstream links (retained, not inferred): ``TypeId``.

<a id="op-928ab0ac215fe47647eb62eb"></a>
## is_empty

`function` · `datafusion_common::extensions::Extensions::is_empty` · datafusion-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if no extensions are set.

<a id="op-957b7e393ff4a73ee8027daf"></a>
## len

`function` · `datafusion_common::extensions::Extensions::len` · datafusion-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Number of extensions set.

<a id="op-154ff5bbda66ffd2c6507f91"></a>
## merge

`function` · `datafusion_common::extensions::Extensions::merge` · datafusion-common 55.1.0

```rust
fn merge(&mut self, other: &Extensions)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Merge entries from `other` into `self`. Entries in `other` take
precedence over existing entries with the same type.

<a id="op-32de8406b8be540e4be76686"></a>
## new

`function` · `datafusion_common::extensions::Extensions::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::extensions::Extensions", "path": "Extensions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [140, 2], "filename": "src/extensions.rs"}, "trait": null, "trait_path": null}`

Source: `src/extensions.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create an empty map.
