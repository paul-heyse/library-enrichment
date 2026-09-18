# `datafusion_datasource::morsel::PendingMorselPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.PendingMorselPlanner.json).

<a id="op-d93f38c08ec554842f365ea0"></a>
## PendingMorselPlanner

`struct` · `datafusion_datasource::morsel::PendingMorselPlanner` · datafusion-datasource 55.1.0

```rust
struct PendingMorselPlanner
```

Source: `src/morsel/mod.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Wrapper for I/O that must complete before planning can continue.

<a id="op-b9f6b9891db9f1b651c6964d"></a>
## Output

`assoc_type` · `datafusion_datasource::morsel::PendingMorselPlanner::Output` · datafusion-datasource 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::PendingMorselPlanner", "path": "PendingMorselPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [234, 2], "filename": "src/morsel/mod.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/morsel/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e582f71acf27690c5a2c3a3"></a>
## into_future

`function` · `datafusion_datasource::morsel::PendingMorselPlanner::into_future` · datafusion-datasource 55.1.0

```rust
fn into_future(self) -> BoxFuture<'static, Result<Box<dyn MorselPlanner>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::PendingMorselPlanner", "path": "PendingMorselPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [225, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Consume this wrapper and return the underlying future.

<a id="op-c8649b5664d04e03e003eb5f"></a>
## new

`function` · `datafusion_datasource::morsel::PendingMorselPlanner::new` · datafusion-datasource 55.1.0

```rust
fn new<F>(future: F) -> Self where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::PendingMorselPlanner", "path": "PendingMorselPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [225, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new pending planner future.

Example
```
# use datafusion_common::DataFusionError;
# use datafusion_datasource::morsel::{MorselPlanner, PendingMorselPlanner};
let work = async move {
 let planner: Box<dyn MorselPlanner> = {
  // Do I/O work here, then return the next planner to run.
 # unimplemented!();
  };
  Ok(planner) as Result<_, DataFusionError>;
};
let pending_io = PendingMorselPlanner::new(work);
```

<a id="op-0745c3a4f812ebd55e415c31"></a>
## poll

`function` · `datafusion_datasource::morsel::PendingMorselPlanner::poll` · datafusion-datasource 55.1.0

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::PendingMorselPlanner", "path": "PendingMorselPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [234, 2], "filename": "src/morsel/mod.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/morsel/mod.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
