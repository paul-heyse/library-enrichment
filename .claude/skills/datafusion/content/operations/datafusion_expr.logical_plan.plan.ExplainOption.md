# `datafusion_expr::logical_plan::plan::ExplainOption`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.ExplainOption.json).

<a id="op-784cf352b14885c813b5fb5d"></a>
## ExplainOption

`struct` · `datafusion_expr::logical_plan::plan::ExplainOption` · datafusion-expr 55.1.0

```rust
struct ExplainOption
```

Source: `src/logical_plan/plan.rs:3469`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Options for EXPLAIN

<a id="op-35d6d39331187b385eb1d28f"></a>
## analyze

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::analyze` · datafusion-expr 55.1.0

```rust
analyze: bool
```

Source: `src/logical_plan/plan.rs:3473`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Actually execute the plan and report metrics

<a id="op-9f42d6516c88e1fa0996dd2b"></a>
## analyze_categories

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::analyze_categories` · datafusion-expr 55.1.0

```rust
analyze_categories: Option<datafusion_common::format::ExplainAnalyzeCategories>
```

Source: `src/logical_plan/plan.rs:3484`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_categories`.
`None` means "fall back to session config".

<a id="op-fc2d137924ee47369cbeb5a5"></a>
## analyze_level

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::analyze_level` · datafusion-expr 55.1.0

```rust
analyze_level: Option<datafusion_common::format::MetricType>
```

Source: `src/logical_plan/plan.rs:3481`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.analyze_level`.
`None` means "fall back to session config".

<a id="op-39632d81ac6c81b9b8e7b779"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ExplainOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3468, 17], "end": [3468, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04c92b4aadb337473858aa04"></a>
## default

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3487, 1], "end": [3498, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/logical_plan/plan.rs:3488`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75cad23dc19faae96a472991"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ExplainOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3468, 24], "end": [3468, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85c6761be78162870fda2909"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3468, 10], "end": [3468, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-126c3337513de7a756a3ae72"></a>
## format

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::format` · datafusion-expr 55.1.0

```rust
format: datafusion_common::format::ExplainFormat
```

Source: `src/logical_plan/plan.rs:3475`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Output syntax/format

<a id="op-e6737a15a751607a34495299"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3468, 39], "end": [3468, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f029af185264a0044dc351c5"></a>
## show_statistics

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::show_statistics` · datafusion-expr 55.1.0

```rust
show_statistics: Option<bool>
```

Source: `src/logical_plan/plan.rs:3478`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statement-level override for `datafusion.explain.show_statistics`.
`None` means "fall back to session config".

<a id="op-426bd08b4499610f55481fc4"></a>
## verbose

`struct_field` · `datafusion_expr::logical_plan::plan::ExplainOption::verbose` · datafusion-expr 55.1.0

```rust
verbose: bool
```

Source: `src/logical_plan/plan.rs:3471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Include detailed debug info

<a id="op-4c9e95975f331b4c7dcd92ec"></a>
## with_analyze

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_analyze` · datafusion-expr 55.1.0

```rust
fn with_analyze(self, analyze: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3508`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder‐style setter for `analyze`

<a id="op-b3c0eebcdbc0937f33371da9"></a>
## with_analyze_categories

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_analyze_categories` · datafusion-expr 55.1.0

```rust
fn with_analyze_categories(self, analyze_categories: Option<ExplainAnalyzeCategories>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3535`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder-style setter for a statement-level override of
`datafusion.explain.analyze_categories`.

<a id="op-919d4cf78f73e048e3416952"></a>
## with_analyze_level

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_analyze_level` · datafusion-expr 55.1.0

```rust
fn with_analyze_level(self, analyze_level: Option<MetricType>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3528`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder-style setter for a statement-level override of
`datafusion.explain.analyze_level`.

<a id="op-59be6d67aa10700c47511ec8"></a>
## with_format

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_format` · datafusion-expr 55.1.0

```rust
fn with_format(self, format: ExplainFormat) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3514`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder‐style setter for `format`

<a id="op-57661cc707406d88f41aba44"></a>
## with_show_statistics

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_show_statistics` · datafusion-expr 55.1.0

```rust
fn with_show_statistics(self, show_statistics: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3521`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder-style setter for a statement-level override of
`datafusion.explain.show_statistics`.

<a id="op-86a37489b6dc81f5819886d2"></a>
## with_verbose

`function` · `datafusion_expr::logical_plan::plan::ExplainOption::with_verbose` · datafusion-expr 55.1.0

```rust
fn with_verbose(self, verbose: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::ExplainOption", "path": "ExplainOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3500, 1], "end": [3542, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3502`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder‐style setter for `verbose`
