# `datafusion_common::null_equality`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.null_equality.json`](../model/datafusion_common.null_equality.json)

## NullEquality

`enum` · `datafusion_common::null_equality::NullEquality`

Also reachable as `datafusion::common::NullEquality`, `datafusion_common::NullEquality`

```rust
enum NullEquality
```

**Variants**: `NullEqualsNothing`, `NullEqualsNull`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Represents the behavior for null values when evaluating equality. Currently, its primary use
case is to define the behavior of joins for null values.

# Examples

The following table shows the expected equality behavior for `NullEquality`.

| A    | B    | NullEqualsNothing | NullEqualsNull |
|------|------|-------------------|----------------|
| NULL | NULL | false             | true           |
| NULL | 'b'  | false             | false          |
| 'a'  | NULL | false             | false          |
| 'a'  | 'b'  | false             | false          |

# Order

The order on this type represents the "restrictiveness" of the behavior. The more restrictive
a behavior is, the fewer elements are considered to be equal to null.
[NullEquality::NullEqualsNothing] represents the most restrictive behavior.

This mirrors the old order with `null_equals_null` booleans, as `false` indicated that
`null != null`.

---
