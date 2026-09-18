# `datafusion_physical_plan::sorts::partial_sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.partial_sort.json).

<a id="op-827445bc43e7e4ca6da82a82"></a>
## partial_sort

`module` · `datafusion_physical_plan::sorts::partial_sort` · datafusion-physical-plan 55.1.0

```rust
mod partial_sort
```

Source: `src/sorts/partial_sort.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Partial Sort deals with input data that partially
satisfies the required sort order. Such an input data can be
partitioned into segments where each segment already has the
required information for lexicographic sorting so sorting
can be done without loading the entire dataset.

Consider a sort plan having an input with ordering `a ASC, b ASC`

```text
+---+---+---+
| a | b | d |
+---+---+---+
| 0 | 0 | 3 |
| 0 | 0 | 2 |
| 0 | 1 | 1 |
| 0 | 2 | 0 |
+---+---+---+
```

and required ordering for the plan is `a ASC, b ASC, d ASC`.
The first 3 rows(segment) can be sorted as the segment already
has the required information for the sort, but the last row
requires further information as the input can continue with a
batch with a starting row where a and b does not change as below

```text
+---+---+---+
| a | b | d |
+---+---+---+
| 0 | 2 | 4 |
+---+---+---+
```

The plan concats incoming data with such last rows of previous input
and continues partial sorting of the segments.
