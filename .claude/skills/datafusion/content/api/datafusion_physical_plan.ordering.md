# `datafusion_physical_plan::ordering`

Crate `datafusion-physical-plan` · 1 public items · structured records in [`model/datafusion_physical_plan.ordering.json`](../model/datafusion_physical_plan.ordering.json)

## InputOrderMode

`enum` · `datafusion_physical_plan::ordering::InputOrderMode`

Also reachable as `datafusion::physical_plan::InputOrderMode`, `datafusion_physical_plan::InputOrderMode`, `datafusion_physical_plan::execution_plan::InputOrderMode`

```rust
enum InputOrderMode
```

**Variants**: `Linear`, `PartiallySorted`, `Sorted`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

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

---
