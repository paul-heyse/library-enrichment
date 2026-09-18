# `datafusion_physical_plan::test::exec::ErrorExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.ErrorExec.json).

<a id="op-1ab9c5b1eb6e7ed3d0eca7f1"></a>
## ErrorExec

`struct` · `datafusion_physical_plan::test::exec::ErrorExec` · datafusion-physical-plan 55.1.0

```rust
struct ErrorExec
```

Source: `src/test/exec.rs:553`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A mock execution plan that errors on a call to execute

<a id="op-2332195f71ce3a6ea6b54f49"></a>
## apply_expressions

`function` · `datafusion_physical_plan::test::exec::ErrorExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82ad0b68a11a40e1566cc80d"></a>
## children

`function` · `datafusion_physical_plan::test::exec::ErrorExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:614`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb312d808186b7a94a9a9b70"></a>
## default

`function` · `datafusion_physical_plan::test::exec::ErrorExec::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 1], "end": [561, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/exec.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f66d87fc4dda991af3d388de"></a>
## execute

`function` · `datafusion_physical_plan::test::exec::ErrorExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:644`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a stream which yields data

<a id="op-2550973beb09f6583c64e8c4"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::ErrorExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [552, 10], "end": [552, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef2a9f50b8eb0fda2bc76308"></a>
## fmt_as

`function` · `datafusion_physical_plan::test::exec::ErrorExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [603, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/test/exec.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45fa8592b74b42ade154241a"></a>
## name

`function` · `datafusion_physical_plan::test::exec::ErrorExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef1a3cb98f5ead2f052a7f08"></a>
## new

`function` · `datafusion_physical_plan::test::exec::ErrorExec::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [563, 1], "end": [585, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e8bad978c4c1bfe4ae9f12d"></a>
## properties

`function` · `datafusion_physical_plan::test::exec::ErrorExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b06bf2e56c15ed755e1a257d"></a>
## replace_children

`function` · `datafusion_physical_plan::test::exec::ErrorExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, _: Vec<Arc<dyn ExecutionPlan>>, _: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:618`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-940e938c9889a9a98560c81f"></a>
## with_new_children

`function` · `datafusion_physical_plan::test::exec::ErrorExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::ErrorExec", "path": "ErrorExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [605, 1], "end": [651, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/test/exec.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
