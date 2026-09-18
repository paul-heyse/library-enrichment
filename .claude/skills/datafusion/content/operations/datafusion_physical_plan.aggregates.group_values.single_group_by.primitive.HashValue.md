# `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.single_group_by.primitive.HashValue.json).

<a id="op-dc03a62549096ef90e89ddec"></a>
## HashValue

`trait` · `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue` · datafusion-physical-plan 55.1.0

```rust
trait HashValue
```

Source: `src/aggregates/group_values/single_group_by/primitive.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A trait to allow hashing of floating point numbers

<a id="op-0f2d19f3dfffb7f5083da681"></a>
## canonicalize

`function` · `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue::canonicalize` · datafusion-physical-plan 55.1.0

```rust
fn canonicalize(self) -> Self where Self: Sized
```

Source: `src/aggregates/group_values/single_group_by/primitive.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a canonical representative whose bit pattern is identical for
all values that should be grouped together. Default is the identity;
floats override this to fold `-0.0` into `+0.0` so the bit-equal
`is_eq` check used during insertion treats them as the same group.
NaN payload bits are preserved.

<a id="op-5469ed64dfa7bd8deff5b1a4"></a>
## hash

`function` · `datafusion_physical_plan::aggregates::group_values::single_group_by::primitive::HashValue::hash` · datafusion-physical-plan 55.1.0

```rust
fn hash(&self, state: &RandomState) -> u64
```

Source: `src/aggregates/group_values/single_group_by/primitive.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
