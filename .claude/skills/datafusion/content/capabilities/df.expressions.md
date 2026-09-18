# Separate expression construction, coercion and evaluation

expr_fn helpers and ScalarUDF::call construct expressions. Ordinary analysis supplies type coercion; a direct simplifier call has type preconditions and may leave an incompatible expression unsimplified or fail.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| SQL / DataFrame analysis | Ordinary query construction | Uses the session analyzer/planner pipeline; still inspect inferred output fields. |
| ExprSimplifier::coerce then simplify | Direct expression processing outside analysis | Provide the correct DFSchema and SimplifyContext; not every incompatible pair has a coercion. |
| physical expression evaluation | Typed expression applied to batches | ColumnarValue may be scalar or array; field/type/length context matters. |

## Contract

**construction.** ScalarUDF::call(Vec<Expr>) -> Expr does not promise validation or casts. ExprSchemable can infer type/nullability against a schema; it is not execution.
Claim `df.expressions.construction`; upstream_contract_interpretation; evidence: upstream.

**simplification.** simplify expects types compatible with operator requirements. coerce(expr, schema) -> Result<Expr> is separate. An uncoerced call need not always error; it may remain unsimplified.
Claim `df.expressions.simplification`; upstream_contract_interpretation; evidence: upstream.

**evaluation.** Use planning context and registry appropriate to the query. Failures can occur during coercion, physical planning or evaluation; test all relevant boundaries.
Claim `df.expressions.evaluation`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Identify whether the caller enters normal analysis or bypasses it.
- For direct processing, coerce with the intended schema, then simplify and plan.
- Check both output datatype and value; include an unsupported pair control.

## Limits and unknowns

- Mixed integer addition is the runtime example; it is not a complete coercion matrix.

## Exact contracts

- [`datafusion_expr::udf::ScalarUDF::call`](../operations/datafusion_expr.udf.ScalarUDF.md#op-5bee7387f4bfbc22e08e3489) — `fn call(&self, args: Vec<Expr>) -> Expr`
- [`datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::coerce`](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-ab6282f359cd840ffaae3af5) — `fn coerce(&self, expr: Expr, schema: &DFSchema) -> Result<Expr>`
- [`datafusion_optimizer::simplify_expressions::expr_simplifier::ExprSimplifier::simplify`](../operations/datafusion_optimizer.simplify_expressions.expr_simplifier.ExprSimplifier.md#op-24b1e713502751d6ff36f0a4) — `fn simplify(&self, expr: Expr) -> Result<Expr>`
- [`datafusion::execution::context::SessionContext::create_physical_expr`](../operations/datafusion.execution.context.SessionContext.md#op-11fb97c3cf205078f236f9c2) — `fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: expressions_require_coercion_before_direct_simplification
