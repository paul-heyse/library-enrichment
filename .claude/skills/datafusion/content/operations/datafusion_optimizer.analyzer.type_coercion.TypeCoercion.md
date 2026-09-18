# `datafusion_optimizer::analyzer::type_coercion::TypeCoercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.analyzer.type_coercion.TypeCoercion.json).

<a id="op-2f0b064d806007348d2b902c"></a>
## TypeCoercion

`struct` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion` · datafusion-optimizer 55.1.0

```rust
struct TypeCoercion
```

Source: `src/analyzer/type_coercion.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Performs type coercion by determining the schema
and performing the expression rewrites.

<a id="op-4335ad2313606ff6eaafa65f"></a>
## analyze

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion::analyze` · datafusion-optimizer 55.1.0

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercion", "path": "TypeCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [111, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::analyzer::AnalyzerRule", "path": "AnalyzerRule"}, "trait_path": "datafusion_optimizer::analyzer::AnalyzerRule"}`

Source: `src/analyzer/type_coercion.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1757e477235cf13a24fd6f71"></a>
## default

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> TypeCoercion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercion", "path": "TypeCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 17], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/analyzer/type_coercion.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7532c9ce468212ba099c63e0"></a>
## fmt

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercion", "path": "TypeCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 19], "end": [68, 24], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analyzer/type_coercion.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7153e558dbc4a309ecb12c94"></a>
## name

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercion", "path": "TypeCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [111, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::analyzer::AnalyzerRule", "path": "AnalyzerRule"}, "trait_path": "datafusion_optimizer::analyzer::AnalyzerRule"}`

Source: `src/analyzer/type_coercion.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-226dd10b044b1b476f870984"></a>
## new

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercion::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercion", "path": "TypeCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [75, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/type_coercion.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
