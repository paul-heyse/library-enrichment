# `datafusion_common_runtime::trace_utils::JoinSetTracerError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common_runtime.trace_utils.JoinSetTracerError.json).

<a id="op-1370748744ac14f11eefb9e7"></a>
## JoinSetTracerError

`enum` · `datafusion_common_runtime::trace_utils::JoinSetTracerError` · datafusion-common-runtime 55.1.0

```rust
enum JoinSetTracerError
```

Source: `src/trace_utils.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

A custom error type for tracer injection failures.

<a id="op-7806f7f9e6e7509e84a80cd9"></a>
## AlreadySet

`variant` · `datafusion_common_runtime::trace_utils::JoinSetTracerError::AlreadySet` · datafusion-common-runtime 55.1.0

```rust
AlreadySet
```

Source: `src/trace_utils.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

The global tracer has already been set.

<a id="op-10b72cfcaac7090b56d07179"></a>
## fmt

`function` · `datafusion_common_runtime::trace_utils::JoinSetTracerError::fmt` · datafusion-common-runtime 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common_runtime::trace_utils::JoinSetTracerError", "path": "JoinSetTracerError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/trace_utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/trace_utils.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc5633751818651588071a55"></a>
## fmt

`function` · `datafusion_common_runtime::trace_utils::JoinSetTracerError::fmt` · datafusion-common-runtime 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common_runtime::trace_utils::JoinSetTracerError", "path": "JoinSetTracerError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [85, 2], "filename": "src/trace_utils.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/trace_utils.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common-runtime/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
