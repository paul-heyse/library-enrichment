# Discover built-ins in the actual session before writing a UDF

Search the SQL catalog, runtime registry and defining function crate. An indexed constructor is not proof of registration under a consumer's feature profile; names/aliases, signature and return-field logic are separate facts.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| built-in expr_fn / SQL function | Existing semantics fit the query | Keep optimizer visibility; inspect null/coercion/volatility contracts. |
| Arrow kernel | Already operating on arrays inside an execution boundary | Avoid unnecessary SQL/UDF registration but manage schema and scalar broadcasting. |
| custom ScalarUDFImpl | Semantics genuinely differ | Declare signature, volatility, invocation and output-field behavior; prove any optimizer hooks. |
| table function | Arguments produce a relation rather than a scalar column | Registration and output TableProvider differ from scalar functions. |

## Contract

**discovery.** SessionState exposes scalar/aggregate/window/table function maps. The captured registry records registered names, canonical names, aliases and signature Debug text for one explicit profile.
Claim `df.functions.discovery`; upstream_contract_interpretation; evidence: upstream.

**shape.** ScalarUDFImpl receives invocation arguments including ColumnarValue inputs and row-count/field context. Inspect invoke_with_args and return_field_from_args; scalar versus array and nullable output cannot be inferred from the SQL name.
Claim `df.functions.shape`; upstream_contract_interpretation; evidence: upstream.

**optimizer.** Volatility, simplification, ordering/bounds and short-circuit hooks affect planning. Defaults may be appropriate; incorrect advertised properties can change results.
Claim `df.functions.optimizer`; upstream_contract_interpretation; evidence: upstream.

**availability.** Cargo features, default function installation and session registrations all matter. Runtime registry capture is not a list of every function constructible from the 60 indexed crates.
Claim `df.functions.availability`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Find a SQL name or task synonym, resolve its defining implementation and expr_fn helper.
- Check actual registry membership and Cargo features.
- Read the specific function's signature, coercion and return-field/null contract before composing.

## Limits and unknowns

- The registry signature is Debug output, not a stable signature serialization. Return-field behavior still requires per-function reading/probing.

## Exact contracts

- [`datafusion::execution::session_state::SessionState::scalar_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-25a7ddf8e699878347177717) — `fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>`
- [`datafusion::execution::session_state::SessionState::scalar_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-7217e9deef5ad1504d02344c) — `fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>`
- [`datafusion::execution::session_state::SessionState::aggregate_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-4124c4769553c5205807a198) — `fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>`
- [`datafusion::execution::session_state::SessionState::aggregate_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-9fdfaa976ad7dd6e78a31ea2) — `fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>`
- [`datafusion::execution::session_state::SessionState::window_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-6a9570c68f50f57459d0efea) — `fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>`
- [`datafusion::execution::session_state::SessionState::window_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-bb943909cf0840cc57392a9a) — `fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>`
- [`datafusion::execution::session_state::SessionState::table_functions`](../operations/datafusion.execution.session_state.SessionState.md#op-fd1ec9d036c9069c93dd438f) — `fn table_functions(&self) -> &HashMap<String, Arc<TableFunction>>`
- [`datafusion_expr::udf::ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) — `trait ScalarUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
