# `datafusion_common::display::StringifiedPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.display.StringifiedPlan.json).

<a id="op-e8da7e4a4d346e3f88fefb98"></a>
## StringifiedPlan

`struct` · `datafusion_common::display::StringifiedPlan` · datafusion-common 55.1.0

```rust
struct StringifiedPlan
```

Source: `src/display/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents some sort of execution plan, in String form

<a id="op-0cbf174ba306674e3b596130"></a>
## clone

`function` · `datafusion_common::display::StringifiedPlan::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> StringifiedPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 17], "end": [103, 22], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26ac75c2201872613d0635a8"></a>
## eq

`function` · `datafusion_common::display::StringifiedPlan::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &StringifiedPlan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 24], "end": [103, 33], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/display/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f107fa347d0e537c96c9f91"></a>
## fmt

`function` · `datafusion_common::display::StringifiedPlan::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 10], "end": [103, 15], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1998fcef96dbe56c2d585323"></a>
## hash

`function` · `datafusion_common::display::StringifiedPlan::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 51], "end": [103, 55], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/display/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2b512c0af405856508f034e"></a>
## new

`function` · `datafusion_common::display::StringifiedPlan::new` · datafusion-common 55.1.0

```rust
fn new(plan_type: PlanType, plan: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [131, 2], "filename": "src/display/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new Stringified plan of `plan_type` with string
representation `plan`

<a id="op-6ed96156088b386133ec7f2b"></a>
## partial_cmp

`function` · `datafusion_common::display::StringifiedPlan::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &StringifiedPlan) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 39], "end": [103, 49], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/display/mod.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59804c7bb1476c5cb6e993fa"></a>
## plan

`struct_field` · `datafusion_common::display::StringifiedPlan::plan` · datafusion-common 55.1.0

```rust
plan: std::sync::Arc<String>
```

Source: `src/display/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The string representation of the plan

<a id="op-c8fe32e0ac9deb121a1ffcd6"></a>
## plan_type

`struct_field` · `datafusion_common::display::StringifiedPlan::plan_type` · datafusion-common 55.1.0

```rust
plan_type: PlanType
```

Source: `src/display/mod.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An identifier of what type of plan this string represents

<a id="op-8bda3473c14970e3bf7aca6f"></a>
## should_display

`function` · `datafusion_common::display::StringifiedPlan::should_display` · datafusion-common 55.1.0

```rust
fn should_display(&self, verbose_mode: bool) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::StringifiedPlan", "path": "StringifiedPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [131, 2], "filename": "src/display/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if this plan should be displayed. Generally
`verbose_mode = true` will display all available plans
