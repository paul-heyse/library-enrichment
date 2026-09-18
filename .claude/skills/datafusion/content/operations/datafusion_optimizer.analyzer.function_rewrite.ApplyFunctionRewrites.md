# `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.analyzer.function_rewrite.ApplyFunctionRewrites.json).

<a id="op-ffad1ccd15640beb11431eba"></a>
## ApplyFunctionRewrites

`struct` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites` · datafusion-optimizer 55.1.0

```rust
struct ApplyFunctionRewrites
```

Source: `src/analyzer/function_rewrite.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Analyzer rule that invokes [`FunctionRewrite`](../operations/datafusion_expr.expr_rewriter.FunctionRewrite.md#op-d16387f6b2d1bd76e4ec29b7)s on expressions

<a id="op-5a6d230f75ec3c8aed5dc230"></a>
## analyze

`function` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites::analyze` · datafusion-optimizer 55.1.0

```rust
fn analyze(&self, plan: LogicalPlan, options: &ConfigOptions) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites", "path": "ApplyFunctionRewrites"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [108, 2], "filename": "src/analyzer/function_rewrite.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::analyzer::AnalyzerRule", "path": "AnalyzerRule"}, "trait_path": "datafusion_optimizer::analyzer::AnalyzerRule"}`

Source: `src/analyzer/function_rewrite.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e0649b3b71f6fb7c1ea022"></a>
## default

`function` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> ApplyFunctionRewrites
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites", "path": "ApplyFunctionRewrites"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/analyzer/function_rewrite.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/analyzer/function_rewrite.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d5719c7e4d686d91cd28674"></a>
## fmt

`function` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites", "path": "ApplyFunctionRewrites"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 19], "end": [32, 24], "filename": "src/analyzer/function_rewrite.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analyzer/function_rewrite.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33a5fa22ead8c60d087e5fed"></a>
## name

`function` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites", "path": "ApplyFunctionRewrites"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [108, 2], "filename": "src/analyzer/function_rewrite.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::analyzer::AnalyzerRule", "path": "AnalyzerRule"}, "trait_path": "datafusion_optimizer::analyzer::AnalyzerRule"}`

Source: `src/analyzer/function_rewrite.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f91483aaa4571e1fad11b3eb"></a>
## new

`function` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites::new` · datafusion-optimizer 55.1.0

```rust
fn new(function_rewrites: Vec<Arc<dyn FunctionRewrite + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites", "path": "ApplyFunctionRewrites"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [97, 2], "filename": "src/analyzer/function_rewrite.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/function_rewrite.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
