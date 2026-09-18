# `datafusion_ffi::session::ForeignSession`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.session.ForeignSession.json).

<a id="op-c4ae24e7c1cf2f8d62a48425"></a>
## ForeignSession

`struct` · `datafusion_ffi::session::ForeignSession` · datafusion-ffi 55.1.0

```rust
struct ForeignSession
```

Source: `src/session/mod.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must use only the stable function pointers in
`FFI_SessionRef` to interact with the foreign session.

# Query planner delegation

If the session owner installed the current foreign query planner,
[`Session::create_physical_plan`](../operations/datafusion_session.session.Session.md#op-f8a00c1a238fc9102cc1e8b7) dispatches back to that planner and
[`Session::query_planner`](../operations/datafusion_session.session.Session.md#op-d9a5770e1d1190279fe94203) returns that planner. The planner must retain and
invoke the session owner's previous planner instead of using either method to
delegate back to the session. Otherwise, repeated delegation exhausts the
stack. See [`crate::query_planner`](../modules/datafusion_ffi.query_planner.md#op-3caae2f40567c01aae388544) for details.

<a id="op-ad77c7bf2736453b8d7c78dc"></a>
## aggregate_functions

`function` · `datafusion_ffi::session::ForeignSession::aggregate_functions` · datafusion-ffi 55.1.0

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b25ee8bc88a4ae2ff4d92b1"></a>
## as_any

`function` · `datafusion_ffi::session::ForeignSession::as_any` · datafusion-ffi 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:828`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28c6912fb83cebf2edf3d79d"></a>
## catalog_list

`function` · `datafusion_ffi::session::ForeignSession::catalog_list` · datafusion-ffi 55.1.0

```rust
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91b1bbfa6a187f726aec4891"></a>
## config

`function` · `datafusion_ffi::session::ForeignSession::config` · datafusion-ffi 55.1.0

```rust
fn config(&self) -> &SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ade810e0df7aee81ef9ea35d"></a>
## config_options

`function` · `datafusion_ffi::session::ForeignSession::config_options` · datafusion-ffi 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbd8724367c062cec36e39bb"></a>
## create_physical_expr

`function` · `datafusion_ffi::session::ForeignSession::create_physical_expr` · datafusion-ffi 55.1.0

```rust
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:770`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3fa7bdb67ffa8abd4a99b78"></a>
## create_physical_plan

`function` · `datafusion_ffi::session::ForeignSession::create_physical_plan` · datafusion-ffi 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:748`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03167744c557784e3dab8e18"></a>
## default_table_options

`function` · `datafusion_ffi::session::ForeignSession::default_table_options` · datafusion-ffi 55.1.0

```rust
fn default_table_options(&self) -> TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:836`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc7ff0a63b9a61ae2c218e1d"></a>
## execution_props

`function` · `datafusion_ffi::session::ForeignSession::execution_props` · datafusion-ffi 55.1.0

```rust
fn execution_props(&self) -> &ExecutionProps
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:824`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70bb479929342979e891112d"></a>
## extension_type_registry

`function` · `datafusion_ffi::session::ForeignSession::extension_type_registry` · datafusion-ffi 55.1.0

```rust
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:816`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-789d7018fee9a4d0277e2045"></a>
## fmt

`function` · `datafusion_ffi::session::ForeignSession::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [536, 10], "end": [536, 15], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/session/mod.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de8e56234880b3f9697127b5"></a>
## higher_order_functions

`function` · `datafusion_ffi::session::ForeignSession::higher_order_functions` · datafusion-ffi 55.1.0

```rust
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:804`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-027faeb65452f685c5d08b79"></a>
## optimize

`function` · `datafusion_ffi::session::ForeignSession::optimize` · datafusion-ffi 55.1.0

```rust
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:730`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ffeee20e9c42cdd62020d0"></a>
## physical_optimizers

`function` · `datafusion_ffi::session::ForeignSession::physical_optimizers` · datafusion-ffi 55.1.0

```rust
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:791`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e8ec6d47a5050d8bd11c711"></a>
## query_planner

`function` · `datafusion_ffi::session::ForeignSession::query_planner` · datafusion-ffi 55.1.0

```rust
fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc717c5d75880b91b24e8c30"></a>
## runtime_env

`function` · `datafusion_ffi::session::ForeignSession::runtime_env` · datafusion-ffi 55.1.0

```rust
fn runtime_env(&self) -> &Arc<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:820`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eed0dd7e20ecc20602e55f9d"></a>
## scalar_functions

`function` · `datafusion_ffi::session::ForeignSession::scalar_functions` · datafusion-ffi 55.1.0

```rust
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:800`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-466dfeea1758449f4be01c4d"></a>
## session_id

`function` · `datafusion_ffi::session::ForeignSession::session_id` · datafusion-ffi 55.1.0

```rust
fn session_id(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:707`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbc984b06b399762e3cf98db"></a>
## table_options

`function` · `datafusion_ffi::session::ForeignSession::table_options` · datafusion-ffi 55.1.0

```rust
fn table_options(&self) -> &TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:832`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af9b1df8e9de82e50e8733ea"></a>
## table_options_mut

`function` · `datafusion_ffi::session::ForeignSession::table_options_mut` · datafusion-ffi 55.1.0

```rust
fn table_options_mut(&mut self) -> &mut TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:844`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84b3c5039ae61a746ae0a4d0"></a>
## task_ctx

`function` · `datafusion_ffi::session::ForeignSession::task_ctx` · datafusion-ffi 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:851`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35fd70ddd5d3338abb672f3e"></a>
## window_functions

`function` · `datafusion_ffi::session::ForeignSession::window_functions` · datafusion-ffi 55.1.0

```rust
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::session::ForeignSession", "path": "ForeignSession"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [854, 2], "filename": "src/session/mod.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/session/mod.rs:812`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
