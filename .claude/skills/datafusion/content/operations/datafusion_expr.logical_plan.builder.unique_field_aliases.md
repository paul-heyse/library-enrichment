# `datafusion_expr::logical_plan::builder::unique_field_aliases`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.unique_field_aliases.json).

<a id="op-a36ca8d379a128d12e2b6cf6"></a>
## unique_field_aliases

`function` · `datafusion_expr::logical_plan::builder::unique_field_aliases` · datafusion-expr 55.1.0

```rust
fn unique_field_aliases(fields: &arrow::datatypes::Fields) -> Vec<Option<String>>
```

Source: `src/logical_plan/builder.rs:1619`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns aliases to make field names unique.

Returns a vector of optional aliases, one per input field. `None` means keep the original name,
`Some(alias)` means rename to the alias to ensure uniqueness.

Used when creating [`SubqueryAlias`](../operations/datafusion_expr.logical_plan.plan.SubqueryAlias.md#op-552fb54c96bb55e58c6409d2) or similar operations that strip table qualifiers but need
to maintain unique column names.

# Example
Input fields: `[a, a, b, b, a, a:1]` ([`DFSchema`](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98) valid when duplicate fields have different qualifiers)
Returns: `[None, Some("a:1"), None, Some("b:1"), Some("a:2"), Some("a:1:1")]`
