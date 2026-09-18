# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants`

Crate `datafusion-physical-optimizer` · 5 public items · structured records in [`model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.json`](../model/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.json)

## plan_with_order_breaking_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_breaking_variants`

```rust
fn plan_with_order_breaking_variants(sort_input: OrderPreservationContext) -> datafusion_common::Result<OrderPreservationContext>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.plan_with_order_breaking_variants.md).


Calculates the updated plan by replacing operators that preserve ordering
inside `sort_input` with their order-breaking variants. This will restore
the original plan modified by [`plan_with_order_preserving_variants`].

---

## plan_with_order_preserving_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_preserving_variants`

```rust
fn plan_with_order_preserving_variants(sort_input: OrderPreservationContext, is_spr_better: bool, is_spm_better: bool, fetch: Option<usize>) -> datafusion_common::Result<OrderPreservationContext>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.plan_with_order_preserving_variants.md).


Calculates the updated plan by replacing operators that lose ordering
inside `sort_input` with their order-preserving variants. This will
generate an alternative plan, which will be accepted or rejected later on
depending on whether it helps us remove a `SortExec`.

---

## replace_with_order_preserving_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::replace_with_order_preserving_variants`

```rust
fn replace_with_order_preserving_variants(requirements: OrderPreservationContext, is_spr_better: bool, is_spm_better: bool, config: &datafusion_common::config::ConfigOptions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<OrderPreservationContext>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.replace_with_order_preserving_variants.md).


The `replace_with_order_preserving_variants` optimizer sub-rule tries to
remove `SortExec`s from the physical plan by replacing operators that do
not preserve ordering with their order-preserving variants; i.e. by replacing
ordinary `RepartitionExec`s with their sort-preserving variants or by replacing
`CoalescePartitionsExec`s with `SortPreservingMergeExec`s.

If this replacement is helpful for removing a `SortExec`, it updates the plan.
Otherwise, it leaves the plan unchanged.

NOTE: This optimizer sub-rule will only produce sort-preserving `RepartitionExec`s
if the query is bounded or if the config option `prefer_existing_sort` is
set to `true`.

The algorithm flow is simply like this:
1. Visit nodes of the physical plan bottom-up and look for `SortExec` nodes.
   During the traversal, keep track of operators that maintain ordering (or
   can maintain ordering when replaced by an order-preserving variant) until
   a `SortExec` is found.
2. When a `SortExec` is found, update the child of the `SortExec` by replacing
   operators that do not preserve ordering in the tree with their order
   preserving variants.
3. Check if the `SortExec` is still necessary in the updated plan by comparing
   its input ordering with the output ordering it imposes. We do this because
   replacing operators that lose ordering with their order-preserving variants
   enables us to preserve the previously lost ordering at the input of `SortExec`.
4. If the `SortExec` in question turns out to be unnecessary, remove it and
   use updated plan. Otherwise, use the original plan.
5. Continue the bottom-up traversal until another `SortExec` is seen, or the
   traversal is complete.

---

## update_order_preservation_ctx_children_data

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::update_order_preservation_ctx_children_data`

```rust
fn update_order_preservation_ctx_children_data(opc: &mut OrderPreservationContext)
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.update_order_preservation_ctx_children_data.md).


Updates order-preservation data for all children of the given node.

---

## OrderPreservationContext

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::OrderPreservationContext`

```rust
type OrderPreservationContext = datafusion_physical_plan::tree_node::PlanContext<bool>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.OrderPreservationContext.md).


For a given `plan`, this object carries the information one needs from its
descendants to decide whether it is beneficial to replace order-losing (but
somewhat faster) variants of certain operators with their order-preserving
(but somewhat slower) cousins.

---
