# Choose relational cardinality, null-key and ordering semantics

Join keys can multiply rows; equality and null-safe equality differ. Bag/set operations differ on duplicates, and neither partitioning nor streaming promises global result order.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| join / join_on | Combine matching rows with chosen join type/predicate | Check multiplicity and null-safe versus ordinary equality. |
| union / UNION ALL | Concatenate bags | Preserves duplicate multiplicity, subject to compatible schemas. |
| union_distinct / UNION | Set-like duplicate elimination | Adds equality/dedup work; output order is unspecified without sort. |
| unnest | Expand nested values into rows | Row cardinality and null/empty-list options require explicit review. |

## Contract

**cardinality.** Many-to-many key matches multiply rows. Ordinary SQL equality does not match null keys; IS NOT DISTINCT FROM has different null equality. Select the join predicate deliberately.
Claim `df.relations.cardinality`; upstream_contract_interpretation; evidence: upstream.

**sets.** DataFrame union keeps duplicates; union_distinct removes them. Intersect/except expose distinct/all semantics via their APIs; inspect their flag and schema requirements rather than assuming SQL default multiplicities.
Claim `df.relations.sets`; upstream_contract_interpretation; evidence: upstream.

**ordering.** Explicit ORDER BY establishes requested ordering. LIMIT without a total ordering does not identify deterministic rows among ties; partitioned execution changes arrival order.
Claim `df.relations.ordering`; upstream_contract_interpretation; evidence: upstream.

**schema.** Align widths, types, column identity and metadata before relational combination. Compatible logical schema does not establish every physical metadata propagation path.
Claim `df.relations.schema`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Write down expected multiplicity, null equality and tie behavior.
- Use duplicate/null/empty inputs in a differential fixture.
- Inspect explain output for sort/top-k and cardinality effects; validate values independently of plan shape.

## Limits and unknowns

- The SQL probe covers ordinary/null-safe inner joins and distinct UNION/INTERSECT/EXCEPT plus UNION ALL; all join types and DataFrame all-flags are not runtime-qualified.

## Exact contracts

- [`datafusion::dataframe::DataFrame::join`](../operations/datafusion.dataframe.DataFrame.md#op-8991ac8bbaecfd6a6972ed32) — `fn join(self, right: DataFrame, join_type: JoinType, left_cols: &[&str], right_cols: &[&str], filter: Option<Expr>) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::join_on`](../operations/datafusion.dataframe.DataFrame.md#op-bf2e40975b5857c34b3f7997) — `fn join_on(self, right: DataFrame, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::union`](../operations/datafusion.dataframe.DataFrame.md#op-ef61c631381b9b5946811db6) — `fn union(self, dataframe: DataFrame) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::union_distinct`](../operations/datafusion.dataframe.DataFrame.md#op-26f79a38ddbe6096a2d66b5b) — `fn union_distinct(self, dataframe: DataFrame) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::intersect`](../operations/datafusion.dataframe.DataFrame.md#op-6711a3af98d7542d1cec7903) — `fn intersect(self, dataframe: DataFrame) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::except`](../operations/datafusion.dataframe.DataFrame.md#op-dbd92e8ee9b929d68d189748) — `fn except(self, dataframe: DataFrame) -> Result<DataFrame>`
- [`datafusion::dataframe::DataFrame::unnest_columns`](../operations/datafusion.dataframe.DataFrame.md#op-a62714b7d02a6f39683e377c) — `fn unnest_columns(self, columns: &[&str]) -> Result<DataFrame>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: joins_sets_and_window_frames_have_distinct_cardinality
