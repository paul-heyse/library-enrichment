# `datafusion_datasource::source::OpenArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.source.OpenArgs.json).

<a id="op-7c9938e1c34cfb36a80a4e92"></a>
## OpenArgs

`struct` · `datafusion_datasource::source::OpenArgs` · datafusion-datasource 55.1.0

```rust
struct OpenArgs
```

Source: `src/source.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Arguments for [`DataSource::open_with_args`](../operations/datafusion_datasource.source.DataSource.md#op-f4650a7c687d1d6b4d5c6906)

<a id="op-2e78060b2c2ab122f72a30a0"></a>
## clone

`function` · `datafusion_datasource::source::OpenArgs::clone` · datafusion-datasource 55.1.0

```rust
fn clone(&self) -> OpenArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::OpenArgs", "path": "OpenArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 17], "end": [312, 22], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/source.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-713000160539bb6877d06e0f"></a>
## context

`struct_field` · `datafusion_datasource::source::OpenArgs::context` · datafusion-datasource 55.1.0

```rust
context: std::sync::Arc<datafusion_execution::TaskContext>
```

Source: `src/source.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

The task context for execution

<a id="op-0f99193f9d64968db66c6525"></a>
## fmt

`function` · `datafusion_datasource::source::OpenArgs::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::OpenArgs", "path": "OpenArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 10], "end": [312, 15], "filename": "src/source.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/source.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65dc5ee53c4102305ef7bf1d"></a>
## new

`function` · `datafusion_datasource::source::OpenArgs::new` · datafusion-datasource 55.1.0

```rust
fn new(partition: usize, context: Arc<TaskContext>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::OpenArgs", "path": "OpenArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 1], "end": [341, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create a new OpenArgs with required arguments

<a id="op-3a677d84e94d329b9db5607e"></a>
## partition

`struct_field` · `datafusion_datasource::source::OpenArgs::partition` · datafusion-datasource 55.1.0

```rust
partition: usize
```

Source: `src/source.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Which partition to open

<a id="op-e4e56a0f6bd97dd622124cde"></a>
## sibling_state

`struct_field` · `datafusion_datasource::source::OpenArgs::sibling_state` · datafusion-datasource 55.1.0

```rust
sibling_state: Option<std::sync::Arc<dyn Any + Send + Sync>>
```

Source: `src/source.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Optional sibling-shared execution state, see
[`DataSource::create_sibling_state`](../operations/datafusion_datasource.source.DataSource.md#op-b6f45e11fb485500af46863a) for details.

<a id="op-2ef1c1e0076663d581a30509"></a>
## with_shared_state

`function` · `datafusion_datasource::source::OpenArgs::with_shared_state` · datafusion-datasource 55.1.0

```rust
fn with_shared_state(self, sibling_state: Option<Arc<dyn Any + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::source::OpenArgs", "path": "OpenArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 1], "end": [341, 2], "filename": "src/source.rs"}, "trait": null, "trait_path": null}`

Source: `src/source.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set sibling shared state
