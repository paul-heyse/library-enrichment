# `datafusion_optimizer::analyzer::Analyzer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.analyzer.Analyzer.json).

<a id="op-f305ce03fd0d8b4faab403e4"></a>
## Analyzer

`struct` · `datafusion_optimizer::analyzer::Analyzer` · datafusion-optimizer 55.1.0

```rust
struct Analyzer
```

Source: `src/analyzer/mod.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rule-based Analyzer.

Applies [`FunctionRewrite`](../operations/datafusion_expr.expr_rewriter.FunctionRewrite.md#op-d16387f6b2d1bd76e4ec29b7)s and [`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s to transform a
[`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) in preparation for execution.

For example, the `Analyzer` applies type coercion to ensure the types of
operands match the types required by functions.

<a id="op-e1304a3a084e75d4455db19d"></a>
## add_function_rewrite

`function` · `datafusion_optimizer::analyzer::Analyzer::add_function_rewrite` · datafusion-optimizer 55.1.0

```rust
fn add_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [168, 2], "filename": "src/analyzer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/mod.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Add a function rewrite rule

<a id="op-e87a7186ee5997e189649be6"></a>
## clone

`function` · `datafusion_optimizer::analyzer::Analyzer::clone` · datafusion-optimizer 55.1.0

```rust
fn clone(&self) -> Analyzer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/analyzer/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/analyzer/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca283a2f38670cecbe8f609d"></a>
## default

`function` · `datafusion_optimizer::analyzer::Analyzer::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [83, 2], "filename": "src/analyzer/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/analyzer/mod.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b620e3f43c2b5872a12e79c"></a>
## execute_and_check

`function` · `datafusion_optimizer::analyzer::Analyzer::execute_and_check` · datafusion-optimizer 55.1.0

```rust
fn execute_and_check<F>(&self, plan: LogicalPlan, config: &ConfigOptions, observer: F) -> Result<LogicalPlan> where F: FnMut(&LogicalPlan, &dyn AnalyzerRule)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [168, 2], "filename": "src/analyzer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Analyze the logical plan by applying analyzer rules, and
do necessary check and fail the invalid plans

<a id="op-701c27e629d5840c769763b5"></a>
## fmt

`function` · `datafusion_optimizer::analyzer::Analyzer::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 17], "end": [71, 22], "filename": "src/analyzer/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analyzer/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80796f0b9b6a86fda0f1cbe5"></a>
## function_rewrites

`function` · `datafusion_optimizer::analyzer::Analyzer::function_rewrites` · datafusion-optimizer 55.1.0

```rust
fn function_rewrites(&self) -> &[Arc<dyn FunctionRewrite + Send + Sync>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [168, 2], "filename": "src/analyzer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/mod.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

return the list of function rewrites in this analyzer

<a id="op-e765bb1f9054dfa3c8ba82d5"></a>
## function_rewrites

`struct_field` · `datafusion_optimizer::analyzer::Analyzer::function_rewrites` · datafusion-optimizer 55.1.0

```rust
function_rewrites: Vec<std::sync::Arc<dyn FunctionRewrite + Send + Sync>>
```

Source: `src/analyzer/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Expr --> Function writes to apply prior to analysis passes

<a id="op-c1f5ae4720a057e6eedef9b2"></a>
## new

`function` · `datafusion_optimizer::analyzer::Analyzer::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [168, 2], "filename": "src/analyzer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/mod.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new analyzer using the recommended list of rules

<a id="op-b1aa596ce448e10fe2d22e55"></a>
## rules

`struct_field` · `datafusion_optimizer::analyzer::Analyzer::rules` · datafusion-optimizer 55.1.0

```rust
rules: Vec<std::sync::Arc<dyn AnalyzerRule + Send + Sync>>
```

Source: `src/analyzer/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

All rules to apply

<a id="op-e0a7625405e62adc5de8cbcd"></a>
## with_rules

`function` · `datafusion_optimizer::analyzer::Analyzer::with_rules` · datafusion-optimizer 55.1.0

```rust
fn with_rules(rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::Analyzer", "path": "Analyzer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [168, 2], "filename": "src/analyzer/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new analyzer with the given rules
