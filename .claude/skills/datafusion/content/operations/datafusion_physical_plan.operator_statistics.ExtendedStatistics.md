# `datafusion_physical_plan::operator_statistics::ExtendedStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.ExtendedStatistics.json).

<a id="op-b8de662ae83c9d7215d8ad00"></a>
## ExtendedStatistics

`struct` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics` · datafusion-physical-plan 55.1.0

```rust
struct ExtendedStatistics
```

Source: `src/operator_statistics/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics with support for custom extensions.

Wraps the standard [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6) and adds a type-erased extension map
for custom statistics like histograms, sketches, or domain-specific metadata.

# Example

```ignore
// Define a custom statistics extension
#[derive(Debug, Clone)]
struct HistogramStats {
    buckets: Vec<(i64, i64, usize)>, // (min, max, count)
}

// Set extension in a planner
let mut stats = ExtendedStatistics::from(base_stats);
stats.set_extension(HistogramStats { buckets: vec![] });

// Retrieve in a consumer
if let Some(hist) = stats.get_extension::<HistogramStats>() {
    // Use histogram for better estimation
}
```

<a id="op-5fb8616238e86236f483ec3f"></a>
## base

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::base` · datafusion-physical-plan 55.1.0

```rust
fn base(&self) -> &Statistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a reference to the base [`Statistics`](../operations/datafusion_common.stats.Statistics.md#op-06d2580fbefe2068a74aafb6).

<a id="op-25f710dedc9e1cc6cfc9b447"></a>
## base_arc

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::base_arc` · datafusion-physical-plan 55.1.0

```rust
fn base_arc(&self) -> &Arc<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a reference to the underlying [`Arc<Statistics>`].

Unresolved upstream links (retained, not inferred): ``Arc<Statistics>``.

<a id="op-41591281e43ba02a5e6868aa"></a>
## clone

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ExtendedStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 17], "end": [126, 22], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/operator_statistics/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21a244852d95b51b447ede6f"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> ExtendedStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 24], "end": [126, 31], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64c209fedeca64b8df2d6ca2"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 10], "end": [126, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bae81f9a1013904cd18f7ae"></a>
## from

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::from` · datafusion-physical-plan 55.1.0

```rust
fn from(base: Arc<Statistics>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [192, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/operator_statistics/mod.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83d4c40bd459987ab8e63d61"></a>
## from

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::from` · datafusion-physical-plan 55.1.0

```rust
fn from(base: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [186, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::stats::Statistics", "path": "Statistics"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/operator_statistics/mod.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a0d2338e9707eeae986c2a0"></a>
## get_extension

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::get_extension` · datafusion-physical-plan 55.1.0

```rust
fn get_extension<T: 'static + Send + Sync>(&self) -> Option<&T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get a reference to a custom statistics extension by type.

<a id="op-47c207f9f08acf4d98e78e37"></a>
## has_extension

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::has_extension` · datafusion-physical-plan 55.1.0

```rust
fn has_extension<T: 'static + Send + Sync>(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Check if an extension of the given type exists.

<a id="op-e6049ee8a171b5eab132acad"></a>
## merge_extensions

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::merge_extensions` · datafusion-physical-plan 55.1.0

```rust
fn merge_extensions(&mut self, other: &ExtendedStatistics)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Merge extensions from another ExtendedStatistics (other's extensions take precedence).

<a id="op-20fde235ea7845a76f826c9b"></a>
## new

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::new` · datafusion-physical-plan 55.1.0

```rust
fn new(base: Statistics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new ExtendedStatistics wrapping owned statistics.

<a id="op-eadd4dc32e2e16b150ca2fcd"></a>
## new_arc

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::new_arc` · datafusion-physical-plan 55.1.0

```rust
fn new_arc(base: Arc<Statistics>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create new ExtendedStatistics from an [`Arc<Statistics>`].

Unresolved upstream links (retained, not inferred): ``Arc<Statistics>``.

<a id="op-868508849484fc38119a8a8c"></a>
## set_extension

`function` · `datafusion_physical_plan::operator_statistics::ExtendedStatistics::set_extension` · datafusion-physical-plan 55.1.0

```rust
fn set_extension<T: 'static + Send + Sync>(&mut self, value: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::ExtendedStatistics", "path": "ExtendedStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [180, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set a custom statistics extension.
