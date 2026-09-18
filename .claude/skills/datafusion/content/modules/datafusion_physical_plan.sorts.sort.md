# `datafusion_physical_plan::sorts::sort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.sort.json).

<a id="op-fe2c173b4bbd76b2e551e6da"></a>
## sort

`module` · `datafusion_physical_plan::sorts::sort` · datafusion-physical-plan 55.1.0

```rust
mod sort
```

Source: `src/sorts/sort.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort that deals with an arbitrary size of the input.
It will do in-memory sorting if it has enough memory budget
but spills to disk if needed.
