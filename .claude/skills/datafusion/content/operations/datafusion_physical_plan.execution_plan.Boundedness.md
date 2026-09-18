# `datafusion_physical_plan::execution_plan::Boundedness`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.Boundedness.json).

<a id="op-0bd02bea0eb0e3d05f074321"></a>
## Boundedness

`enum` · `datafusion_physical_plan::execution_plan::Boundedness` · datafusion-physical-plan 55.1.0

```rust
enum Boundedness
```

Source: `src/execution_plan.rs:1301`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represents whether a stream of data **generated** by an operator is bounded (finite)
or unbounded (infinite).

This is used to determine whether an execution plan will eventually complete
processing all its data (bounded) or could potentially run forever (unbounded).

For unbounded streams, it also tracks whether the operator requires finite memory
to process the stream or if memory usage could grow unbounded.

Boundedness of the output stream is based on the boundedness of the input stream and the nature of
the operator. For example, limit or topk with fetch operator can convert an unbounded stream to a bounded stream.

<a id="op-124ccedf9c9bae951929cf83"></a>
## Bounded

`variant` · `datafusion_physical_plan::execution_plan::Boundedness::Bounded` · datafusion-physical-plan 55.1.0

```rust
Bounded
```

Source: `src/execution_plan.rs:1303`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The data stream is bounded (finite) and will eventually complete

<a id="op-281fdeafbc2e42b0899ac012"></a>
## Unbounded

`variant` · `datafusion_physical_plan::execution_plan::Boundedness::Unbounded` · datafusion-physical-plan 55.1.0

```rust
Unbounded
```

Source: `src/execution_plan.rs:1305`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The data stream is unbounded (infinite) and could run forever

<a id="op-3c0df52460f1e62efe5054ca"></a>
## clone

`function` · `datafusion_physical_plan::execution_plan::Boundedness::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> Boundedness
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::Boundedness", "path": "Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1300, 17], "end": [1300, 22], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_plan.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b62c18b1fcec3dd25d0406b8"></a>
## eq

`function` · `datafusion_physical_plan::execution_plan::Boundedness::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &Boundedness) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::Boundedness", "path": "Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1300, 30], "end": [1300, 39], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/execution_plan.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d8d2dd706d560dbea25cbad"></a>
## fmt

`function` · `datafusion_physical_plan::execution_plan::Boundedness::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::Boundedness", "path": "Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1300, 10], "end": [1300, 15], "filename": "src/execution_plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_plan.rs:1300`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05f3074865dfeb2d0631f9f2"></a>
## is_unbounded

`function` · `datafusion_physical_plan::execution_plan::Boundedness::is_unbounded` · datafusion-physical-plan 55.1.0

```rust
fn is_unbounded(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::execution_plan::Boundedness", "path": "Boundedness"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1316, 1], "end": [1320, 2], "filename": "src/execution_plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_plan.rs:1317`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
