# `datafusion_session::planner::UnsupportedQueryPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.planner.UnsupportedQueryPlanner.json).

<a id="op-3bc87131bbd1f7aa565e0e75"></a>
## UnsupportedQueryPlanner

`struct` · `datafusion_session::planner::UnsupportedQueryPlanner` · datafusion-session 55.1.0

```rust
struct UnsupportedQueryPlanner
```

Source: `src/planner.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A query planner that reports that planning is not implemented.

[`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093) implementations that do not expose a query planner can return
this planner explicitly.

<a id="op-8183c3a1388c66c5723aac1e"></a>
## create_physical_plan

`function` · `datafusion_session::planner::UnsupportedQueryPlanner::create_physical_plan` · datafusion-session 55.1.0

```rust
async fn create_physical_plan(&self, _logical_plan: &LogicalPlan, _session: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::planner::UnsupportedQueryPlanner", "path": "UnsupportedQueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [59, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::QueryPlanner", "path": "QueryPlanner"}, "trait_path": "datafusion_session::planner::QueryPlanner"}`

Source: `src/planner.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5fd4cddef969a0caf623afe"></a>
## default

`function` · `datafusion_session::planner::UnsupportedQueryPlanner::default` · datafusion-session 55.1.0

```rust
fn default() -> UnsupportedQueryPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::planner::UnsupportedQueryPlanner", "path": "UnsupportedQueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 24], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/planner.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c13a64fc962f54b82dcbc46"></a>
## fmt

`function` · `datafusion_session::planner::UnsupportedQueryPlanner::fmt` · datafusion-session 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_session::planner::UnsupportedQueryPlanner", "path": "UnsupportedQueryPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
