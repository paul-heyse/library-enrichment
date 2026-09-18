# `datafusion_physical_plan::ordering::InputOrderMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.ordering.InputOrderMode.json).

<a id="op-ef30a392d6cfc56b340387f0"></a>
## InputOrderMode

`enum` · `datafusion_physical_plan::ordering::InputOrderMode` · datafusion-physical-plan 55.1.0

```rust
enum InputOrderMode
```

Source: `src/ordering.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Specifies how the input to an aggregation or window operator is ordered
relative to their `GROUP BY` or  `PARTITION BY` expressions.

For example, if the existing ordering is `[a ASC, b ASC, c ASC]`

## Window Functions
- A `PARTITION BY b` clause can use `Linear` mode.
- A `PARTITION BY a, c` or a `PARTITION BY c, a` can use
  `PartiallySorted([0])` or `PartiallySorted([1])` modes, respectively.
  (The vector stores the index of `a` in the respective PARTITION BY expression.)
- A `PARTITION BY a, b` or a `PARTITION BY b, a` can use `Sorted` mode.

## Aggregations
- A `GROUP BY b` clause can use `Linear` mode, as the only one permutation `[b]`
  cannot satisfy the existing ordering.
- A `GROUP BY a, c` or a `GROUP BY c, a` can use
  `PartiallySorted([0])` or `PartiallySorted([1])` modes, respectively, as
  the permutation `[a]` satisfies the existing ordering.
  (The vector stores the index of `a` in the respective PARTITION BY expression.)
- A `GROUP BY a, b` or a `GROUP BY b, a` can use `Sorted` mode, as the
  full permutation `[a, b]` satisfies the existing ordering.

Note these are the same examples as above, but with `GROUP BY` instead of
`PARTITION BY` to make the examples easier to read.

<a id="op-880ff507000bed2710def34a"></a>
## Linear

`variant` · `datafusion_physical_plan::ordering::InputOrderMode::Linear` · datafusion-physical-plan 55.1.0

```rust
Linear
```

Source: `src/ordering.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

There is no partial permutation of the expressions satisfying the
existing ordering.

<a id="op-bddef6befaf36a09f0d83eca"></a>
## PartiallySorted

`variant` · `datafusion_physical_plan::ordering::InputOrderMode::PartiallySorted` · datafusion-physical-plan 55.1.0

```rust
PartiallySorted
```

Source: `src/ordering.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

There is a partial permutation of the expressions satisfying the
existing ordering. Indices describing the longest partial permutation
are stored in the vector.

<a id="op-6d9c636031897c22bdf8b10a"></a>
## Sorted

`variant` · `datafusion_physical_plan::ordering::InputOrderMode::Sorted` · datafusion-physical-plan 55.1.0

```rust
Sorted
```

Source: `src/ordering.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

There is a (full) permutation of the expressions satisfying the
existing ordering.

<a id="op-0aaf25df98a92c45e4d746a4"></a>
## clone

`function` · `datafusion_physical_plan::ordering::InputOrderMode::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> InputOrderMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::ordering::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/ordering.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ordering.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c9f973ce7a44597c856285e"></a>
## eq

`function` · `datafusion_physical_plan::ordering::InputOrderMode::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &InputOrderMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::ordering::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 24], "end": [42, 33], "filename": "src/ordering.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ordering.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b16a2a68d168edb3923396a"></a>
## fmt

`function` · `datafusion_physical_plan::ordering::InputOrderMode::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::ordering::InputOrderMode", "path": "InputOrderMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/ordering.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ordering.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
