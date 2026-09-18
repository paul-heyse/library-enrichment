# `datafusion_common::utils::combine_limit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.combine_limit.json).

<a id="op-58dfc62d30351a5aa880f28e"></a>
## combine_limit

`function` · `datafusion_common::utils::combine_limit` · datafusion-common 55.1.0

```rust
fn combine_limit(parent_skip: usize, parent_fetch: Option<usize>, child_skip: usize, child_fetch: Option<usize>) -> (usize, Option<usize>)
```

Source: `src/utils/mod.rs:1125`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Computes the `skip` and `fetch` parameters of a single limit that would be
equivalent to two consecutive limits with the given `skip`/`fetch` parameters.

There are multiple cases to consider:

# Case 0: Parent and child are disjoint (`child_fetch <= skip`).

```text
  Before merging:
                    |........skip........|---fetch-->|     Parent limit
   |...child_skip...|---child_fetch-->|                    Child limit
```

  After merging:
```text
   |.........(child_skip + skip).........|
```

# Case 1: Parent is beyond child's range (`skip < child_fetch <= skip + fetch`).

  Before merging:
```text
                    |...skip...|------------fetch------------>|   Parent limit
   |...child_skip...|-------------child_fetch------------>|       Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---(child_fetch - skip)-->|
```

 # Case 2: Parent is within child's range (`skip + fetch < child_fetch`).

  Before merging:
```text
                    |...skip...|---fetch-->|                   Parent limit
   |...child_skip...|-------------child_fetch------------>|    Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---fetch-->|
```
