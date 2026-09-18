# `datafusion_session::table::ScanResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.ScanResult.json).

<a id="op-c55b4a2c2cef2e5d170d09c4"></a>
## ScanResult

`struct` · `datafusion_session::table::ScanResult` · datafusion-session 55.1.0

```rust
struct ScanResult
```

Source: `src/table.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Result of a table scan operation from [`TableProvider::scan_with_args`](../operations/datafusion_session.table.TableProvider.md#op-274938cecb923835571bdb77).

<a id="op-a83389c492267116fdaf3794"></a>
## clone

`function` · `datafusion_session::table::ScanResult::clone` · datafusion-session 55.1.0

```rust
fn clone(&self) -> ScanResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 17], "end": [523, 22], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c36742eb4bd93ce378e2342"></a>
## fmt

`function` · `datafusion_session::table::ScanResult::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [523, 10], "end": [523, 15], "filename": "src/table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb71291de802ed8ee3aa865"></a>
## from

`function` · `datafusion_session::table::ScanResult::from` · datafusion-session 55.1.0

```rust
fn from(plan: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [555, 1], "end": [559, 2], "filename": "src/table.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table.rs:556`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef57eb5b24881fbc5d75b085"></a>
## into_inner

`function` · `datafusion_session::table::ScanResult::into_inner` · datafusion-session 55.1.0

```rust
fn into_inner(self) -> Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [553, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Consume this ScanResult and return the execution plan.

Returns the owned [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) that will perform
the actual table scanning and data retrieval.

<a id="op-9b33b67a1e4a70a393fe65df"></a>
## new

`function` · `datafusion_session::table::ScanResult::new` · datafusion-session 55.1.0

```rust
fn new(plan: Arc<dyn ExecutionPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [553, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a new `ScanResult` with the given execution plan.

# Arguments
* `plan` - The execution plan that will perform the table scan

<a id="op-acfc3bf3458a295e7139213d"></a>
## plan

`function` · `datafusion_session::table::ScanResult::plan` · datafusion-session 55.1.0

```rust
fn plan(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::table::ScanResult", "path": "ScanResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [553, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:542`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get a reference to the execution plan for this scan result.

Returns a reference to the [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) that will perform
the actual table scanning and data retrieval.
