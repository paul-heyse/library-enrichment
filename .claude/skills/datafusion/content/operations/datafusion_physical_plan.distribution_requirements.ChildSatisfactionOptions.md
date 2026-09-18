# `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.distribution_requirements.ChildSatisfactionOptions.json).

<a id="op-d30ccfcfffe773f667990248"></a>
## ChildSatisfactionOptions

`struct` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions` · datafusion-physical-plan 55.1.0

```rust
struct ChildSatisfactionOptions
```

Source: `src/distribution_requirements.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Options for checking child distribution satisfaction.

<a id="op-24c0759b58859d674f61b68f"></a>
## allow_subset

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::allow_subset` · datafusion-physical-plan 55.1.0

```rust
fn allow_subset(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [89, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether subset satisfaction is enabled.

<a id="op-108440a643ade5c69e0fe028"></a>
## clone

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ChildSatisfactionOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 22], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/distribution_requirements.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa6a12eb73dc24089f46a84f"></a>
## default

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> ChildSatisfactionOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 30], "end": [67, 37], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/distribution_requirements.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63bd8efc3ed970527f20b35b"></a>
## eq

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &ChildSatisfactionOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 39], "end": [67, 48], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/distribution_requirements.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-720e78cf1dddce2e144087ee"></a>
## fmt

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/distribution_requirements.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/distribution_requirements.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f69605fc968d1b26bf2daa7"></a>
## new

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [89, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create default satisfaction options.

<a id="op-1960da70220dd1bef0725459"></a>
## with_allow_subset

`function` · `datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions::with_allow_subset` · datafusion-physical-plan 55.1.0

```rust
fn with_allow_subset(self, allow_subset: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::distribution_requirements::ChildSatisfactionOptions", "path": "ChildSatisfactionOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [89, 2], "filename": "src/distribution_requirements.rs"}, "trait": null, "trait_path": null}`

Source: `src/distribution_requirements.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Allow a child partitioning whose key expressions are a subset of the
required key expressions to satisfy the requirement.
