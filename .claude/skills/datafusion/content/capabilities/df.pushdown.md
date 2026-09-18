# Advertise only sound filter, projection and limit pushdown

Exact fully enforces SQL predicates; Inexact retains a conservative superset; Unsupported delegates evaluation. The logical scan order is filter, limit, then projection.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Exact | All accepted predicates are fully enforced | Planner can remove residual filters; incorrect claims silently change results. |
| Inexact | Sound candidate pruning retains every match | Residual evaluation removes false positives; limit cannot be pushed past inexact filtering. |
| Unsupported | No sound/useful pruning implementation | Correct general choice; do not invent approximate filtering merely to override a hook. |

## Contract

**inputs.** scan receives Session, optional ordered projection indices, a conjunction of filters, and optional limit. Filter inputs can be omitted from output projection; retain them internally until evaluation.
Claim `df.pushdown.inputs`; upstream_contract_interpretation; evidence: upstream.

**outputs.** Return a physical scan plan with the requested output schema/order. supports_filters_pushdown returns one classification for each input filter in matching order.
Claim `df.pushdown.outputs`; upstream_contract_interpretation; evidence: upstream.

**exactness.** Inexact may keep false positives but must not lose matches. Residual filtering cannot recover false negatives; SQL null/type semantics and duplicates must be retained.
Claim `df.pushdown.exactness`; upstream_contract_interpretation; evidence: upstream.

**limit.** The documented order is filters -> limit -> projection. A pushed limit requests at least that many qualifying rows when available; the scan may return more. Applying it to raw candidates can underproduce.
Claim `df.pushdown.limit`; upstream_contract_interpretation; evidence: upstream.

**new-entry.** scan_with_args takes ScanArgs and returns ScanResult, allowing additional requests. Inspect this pin's full member contract instead of transferring older signatures.
Claim `df.pushdown.new-entry`; upstream_contract_interpretation; evidence: upstream.

**upstream-wording-conflict.** The Exact enum variant documentation at this pin says a source can omit tuples which pass the predicate. That wording conflicts with TableProvider::scan and the observed removal of residual filtering. Follow the scan contract: keep qualifying rows and fully enforce the predicate. The variant sentence is treated as a documentation defect, not an alternative semantic contract.
Claim `df.pushdown.upstream-wording-conflict`; source_runtime_reconciliation; evidence: upstream, probe.

## Implementation

- Evaluate support per expression, not just by predicate column name.
- Keep predicate inputs available through filtering, then apply safe limit and output projection.
- Differentially compare Unsupported/reference execution with Exact/Inexact; inspect residual plans and scan arguments.
- Separate a scan limit hint (may overproduce) from final SQL LIMIT. Assert an actual Some(limit) scan call in a control without ORDER BY; an ordered query can keep LIMIT above a sort and miss this path.

## Limits and unknowns

- The test Inexact provider retains all candidates; it verifies planner/residual behavior, not a particular min/max algorithm.

## Exact contracts

- [`datafusion_session::table::TableProvider::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0) — `async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>`
- [`datafusion_session::table::TableProvider::supports_filters_pushdown`](../operations/datafusion_session.table.TableProvider.md#op-a929088704b5681a65528134) — `fn supports_filters_pushdown(&self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>`
- [`datafusion_session::table::TableProvider::scan_with_args`](../operations/datafusion_session.table.TableProvider.md#op-274938cecb923835571bdb77) — `async fn scan_with_args<'a>(&self, state: &dyn Session, args: ScanArgs<'a>) -> Result<ScanResult>`
- [`datafusion_expr::table_source::TableProviderFilterPushDown`](../operations/datafusion_expr.table_source.TableProviderFilterPushDown.md#op-4fd1b17084602ce2b715fb21) — `enum TableProviderFilterPushDown`
- [`datafusion_expr::table_source::TableProviderFilterPushDown::Exact`](../operations/datafusion_expr.table_source.TableProviderFilterPushDown.md#op-76dd051fb92b19d07ce613a6) — `Exact`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: provider_exact_inexact_projection_limit_match_reference
