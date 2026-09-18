# Use coalesce through its planning contract

In DataFusion 55.1.0 coalesce is simplified to CASE; its invocation hook returns an internal error if called directly. Use the analyzer/simplifier/planner path, including custom coercion and return-field logic.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| SQL coalesce / expr_fn::coalesce | Return the first non-null compatible expression for each row | Enter normal planning so the built-in can coerce and rewrite to CASE. |
| Arrow array selection / explicit CASE | Work directly with arrays or need a different conditional policy | Match null, type, scalar/array and short-circuit semantics explicitly. |

## Contract

**inputs.** At least one argument is required. CoalesceFunc declares Signature::user_defined with Immutable volatility and coerce_types uses try_type_union_resolution. A generic constructor alone does not establish argument compatibility.
Claim `df.coalesce.inputs`; upstream_source; evidence: source.

**output-field.** return_field_from_args returns the first non-Null datatype after argument coercion (or the first datatype when all are Null). Output is nullable only when all argument fields are nullable. It constructs a new Field named coalesce; do not infer input metadata propagation.
Claim `df.coalesce.output-field`; upstream_source; evidence: source.

**rewrite.** simplify rejects zero arguments, returns the sole argument unchanged, or builds CASE WHEN arg IS NOT NULL THEN arg ... ELSE last_arg. invoke_with_args returns an internal error stating coalesce should have been simplified to case.
Claim `df.coalesce.rewrite`; upstream_source; evidence: source.

**conditional.** short_circuits is true. conditional_arguments identifies the first argument as eager and later arguments as lazy. Do not equate these hooks with arbitrary user-code side-effect guarantees.
Claim `df.coalesce.conditional`; upstream_source; evidence: source.

## Implementation

- Resolve the function in the intended session registry and feature profile.
- Construct expressions, then use normal analysis/simplification/physical planning; do not invoke CoalesceFunc as a standalone Arrow kernel.
- Assert both output field and values, including all-null, non-null fallback, mixed types and no-argument controls.

## Limits and unknowns

- The probe covers Int32 nullable/non-null fallback and Int32/Int64 coercion. Nested/extension metadata and arbitrary side effects are not tested.

## Exact contracts

- [`datafusion_functions::core::coalesce::CoalesceFunc`](../operations/datafusion_functions.core.coalesce.CoalesceFunc.md#op-d9d68100dd13733fb4da39cc) — `struct CoalesceFunc`
- [`datafusion_functions::core::coalesce`](../modules/datafusion_functions.core.coalesce.md#op-0f933f3a0204e95d603daacd) — `mod coalesce`
- [`datafusion_functions::core::coalesce`](../operations/datafusion_functions.core.coalesce.md#op-fa1b2bd53aae2f1f6f4000ff) — `fn coalesce() -> std::sync::Arc<datafusion_expr::ScalarUDF>`
- [`datafusion_functions::core::expr_fn::coalesce`](../operations/datafusion_functions.core.expr_fn.coalesce.md#op-939549e8bea4a39d41107f07) — `fn coalesce(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/implementation/source-exports/datafusion-functions-55.1.0/src/core/coalesce.rs): Published crate source; source-exports sibling SOURCE_MANIFEST.json records archive and file digests.
  Tests: 
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Named test assertions only, recorded feature profile.
  Tests: coalesce_uses_planning_and_preserves_nonnull_fallback
