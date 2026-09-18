# `datafusion_common::display::PlanType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.display.PlanType.json).

<a id="op-4f0b495a30f294ba4ce9383a"></a>
## PlanType

`enum` · `datafusion_common::display::PlanType` · datafusion-common 55.1.0

```rust
enum PlanType
```

Source: `src/display/mod.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents which type of plan, when storing multiple
for use in EXPLAIN plans

<a id="op-178ae2b9802ede529831cd98"></a>
## AnalyzedLogicalPlan

`variant` · `datafusion_common::display::PlanType::AnalyzedLogicalPlan` · datafusion-common 55.1.0

```rust
AnalyzedLogicalPlan
```

Source: `src/display/mod.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The LogicalPlan which results from applying an analyzer pass

<a id="op-ba3b60d9623070ae8084ec1d"></a>
## FinalAnalyzedLogicalPlan

`variant` · `datafusion_common::display::PlanType::FinalAnalyzedLogicalPlan` · datafusion-common 55.1.0

```rust
FinalAnalyzedLogicalPlan
```

Source: `src/display/mod.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The LogicalPlan after all analyzer passes have been applied

<a id="op-e385538b6bff0bbd089c0eb2"></a>
## FinalLogicalPlan

`variant` · `datafusion_common::display::PlanType::FinalLogicalPlan` · datafusion-common 55.1.0

```rust
FinalLogicalPlan
```

Source: `src/display/mod.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The final, fully optimized LogicalPlan that was converted to a physical plan

<a id="op-09f7376aaeb4afa8d7f506ca"></a>
## FinalPhysicalPlan

`variant` · `datafusion_common::display::PlanType::FinalPhysicalPlan` · datafusion-common 55.1.0

```rust
FinalPhysicalPlan
```

Source: `src/display/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The final, fully optimized physical plan which would be executed

<a id="op-fd97fc518989401f3744f2c9"></a>
## FinalPhysicalPlanWithSchema

`variant` · `datafusion_common::display::PlanType::FinalPhysicalPlanWithSchema` · datafusion-common 55.1.0

```rust
FinalPhysicalPlanWithSchema
```

Source: `src/display/mod.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The final with schema, fully optimized physical plan which would be executed

<a id="op-455d4e278000c11206abf086"></a>
## FinalPhysicalPlanWithStats

`variant` · `datafusion_common::display::PlanType::FinalPhysicalPlanWithStats` · datafusion-common 55.1.0

```rust
FinalPhysicalPlanWithStats
```

Source: `src/display/mod.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The final with stats, fully optimized physical plan which would be executed

<a id="op-74f3d3f80d05afc6a44d4558"></a>
## InitialLogicalPlan

`variant` · `datafusion_common::display::PlanType::InitialLogicalPlan` · datafusion-common 55.1.0

```rust
InitialLogicalPlan
```

Source: `src/display/mod.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The initial LogicalPlan provided to DataFusion

<a id="op-36c2b576a430d99f3f899ff9"></a>
## InitialPhysicalPlan

`variant` · `datafusion_common::display::PlanType::InitialPhysicalPlan` · datafusion-common 55.1.0

```rust
InitialPhysicalPlan
```

Source: `src/display/mod.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The initial physical plan, prepared for execution

<a id="op-e28c7dd5d2f9020db79a3777"></a>
## InitialPhysicalPlanWithSchema

`variant` · `datafusion_common::display::PlanType::InitialPhysicalPlanWithSchema` · datafusion-common 55.1.0

```rust
InitialPhysicalPlanWithSchema
```

Source: `src/display/mod.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The initial physical plan with schema, prepared for execution

<a id="op-093d65191728fd5f3c34076a"></a>
## InitialPhysicalPlanWithStats

`variant` · `datafusion_common::display::PlanType::InitialPhysicalPlanWithStats` · datafusion-common 55.1.0

```rust
InitialPhysicalPlanWithStats
```

Source: `src/display/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The initial physical plan with stats, prepared for execution

<a id="op-caeba30cbf1b84b51bd87da2"></a>
## OptimizedLogicalPlan

`variant` · `datafusion_common::display::PlanType::OptimizedLogicalPlan` · datafusion-common 55.1.0

```rust
OptimizedLogicalPlan
```

Source: `src/display/mod.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The LogicalPlan which results from applying an optimizer pass

<a id="op-5c2d01ed6b74bd5bfc3d085d"></a>
## OptimizedPhysicalPlan

`variant` · `datafusion_common::display::PlanType::OptimizedPhysicalPlan` · datafusion-common 55.1.0

```rust
OptimizedPhysicalPlan
```

Source: `src/display/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The ExecutionPlan which results from applying an optimizer pass

<a id="op-6483272560cf2b6ccf1c599e"></a>
## PhysicalPlanError

`variant` · `datafusion_common::display::PlanType::PhysicalPlanError` · datafusion-common 55.1.0

```rust
PhysicalPlanError
```

Source: `src/display/mod.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An error creating the physical plan

<a id="op-cb4409c65e854434d17f8782"></a>
## clone

`function` · `datafusion_common::display::PlanType::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> PlanType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 22], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/display/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f184f08e6a6eae5b8fe0f72d"></a>
## eq

`function` · `datafusion_common::display::PlanType::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &PlanType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 24], "end": [31, 33], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/display/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-320f2ebab1e01579129b372e"></a>
## fmt

`function` · `datafusion_common::display::PlanType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/display/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-543d1ff00b9871bf06465efe"></a>
## fmt

`function` · `datafusion_common::display::PlanType::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [100, 2], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/display/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7b5250adda4e206f121c4f"></a>
## hash

`function` · `datafusion_common::display::PlanType::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 51], "end": [31, 55], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/display/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11768534d2cfb0ae8d7b37ae"></a>
## partial_cmp

`function` · `datafusion_common::display::PlanType::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &PlanType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::PlanType", "path": "PlanType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 39], "end": [31, 49], "filename": "src/display/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/display/mod.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
