# Evidence 11 — PB02 executed: does the provider wrapper earn its keep?

The plan's §2.3 proposes `bridge::provider::CanonicalTable`, a wrapper over delta-rs's
`DeltaScan` that fills in the `TableProvider` methods delta-rs leaves at trait defaults
(evidence 05 §1). Two of those justify the effort: `constraints()` and `statistics()`. If the
optimizer plans identically with and without them, the wrapper is ceremony.

- Probe: [`probes/delta-arrow-probe/examples/pb02_constraints.rs`](probes/delta-arrow-probe/examples/pb02_constraints.rs)
- Capture: [`probes/PB02-constraints.out.txt`](probes/PB02-constraints.out.txt)
- **Control:** the wrapper is used in *both* arms, differing only in what `constraints()` returns.
  A bare `DeltaScan` in one arm would have confounded "declaring a key" with "being wrapped".

---

## PB02a — `constraints()` **changes the plan. Build the wrapper.**

Two of seven queries plan differently, and both changes are structural rather than cosmetic.

### 1. `SELECT DISTINCT entity_id FROM t` — the aggregation disappears

```
A (constraints() -> None)          B (constraints() -> PrimaryKey([0]))
  Aggregate: groupBy=[[t.entity_id]], aggr=[[]]      TableScan: t projection=[entity_id]
    TableScan: t projection=[entity_id]
```

The entire `Aggregate` node is gone. `ReplaceDistinctWithAggregate` recognises that a column
declared unique and non-null is already distinct, so the DISTINCT is a no-op. On the canonical
model — where `entity_id` is the primary key of every `snapshot.*` table and most retrieval
queries select distinct entities — this removes a hash aggregation from the hot path.

### 2. A self-join on the key becomes a **semi-join**

```
A:  Projection: a.entity_id                  B:  LeftSemi Join: a.entity_id = b.entity_id
      Inner Join: a.entity_id = b.entity_id        SubqueryAlias: a
        SubqueryAlias: a                             TableScan: t projection=[entity_id]
          TableScan: t projection=[entity_id]      SubqueryAlias: b
        SubqueryAlias: b                             TableScan: t projection=[entity_id]
          TableScan: t projection=[entity_id]
```

`Inner Join` + `Projection` collapses to `LeftSemi Join`. A semi-join stops at the first match
and cannot duplicate left rows, so it is both cheaper and *semantically* narrower. This matters
directly for the merge passes (plan §6.2), which join staging against canonical on the key, and
for `NativeBinding` crosswalk lookups.

### What reached the plan

```
arm B: FunctionalDependencies { deps: [ FunctionalDependence {
           source_indices: [0], target_indices: [0, 1, 2], nullable: false, mode: Single } ] }
arm A: FunctionalDependencies { deps: [] }
```

The declared primary key propagates into the scan schema as a functional dependence covering
**all three columns** — `entity_id` determines `kind` and `n`. That is the mechanism behind both
rewrites, and it is exactly the property evidence 01 §3 predicted would be useful.

Unchanged (correctly): distinct on a non-key column, distinct on key+dependent, `GROUP BY` the
key, `count(DISTINCT key)`, and a plain filter. The last is the negative control — a constraint
declaration should not perturb an unrelated plan, and it does not.

### Verdict

**Build the wrapper, and declare keys on every canonical table.** The cost is one file; the
return is an eliminated aggregation and a narrowed join on the two query shapes the retrieval
layer uses most. `Constraints::new_unverified` is a declaration DataFusion trusts — so the
matching Delta `CheckConstraints` (plan §4.2) is not redundant with it, it is what makes the
declaration true.

---

## PB02b — `statistics()` **changes nothing, because delta-rs already supplies them**

The interesting part is not the verdict but where the statistics come from.

```
scan       plan: same   root num_rows  A(None)=Absent  B(Exact 3)=Absent
self-join  plan: same   root num_rows  A(None)=Absent  B(Exact 3)=Absent
```

and the rendered physical plan, **identical in both arms**:

```
DeltaScanExec, statistics=[Rows=Exact(3), Bytes=Inexact(1051), [(Col[0]:)]]
  DataSourceExec: … file_type=parquet, statistics=[Rows=Exact(3), Bytes=Inexact(1051),
     [(Col[0]:),(Col[1]: Min=Exact(…) Max=Exact(…) Null=Exact(0) Distinct=Exact(1))]]
```

**`TableProvider::statistics()` returning `None` does not mean the optimizer is blind.**
delta-rs supplies accurate statistics at the `ExecutionPlan` level instead — row counts from the
transaction log and per-column min/max/null/distinct from Parquet, already `Precision`-typed.
Supplying `statistics()` on the provider adds nothing and is ignored.

### Verdict

**Drop `statistics()` from the wrapper.** Evidence 05 §1 listed it as one of the two methods
justifying the wrapper; that was wrong. Keep `constraints()`, `get_column_default()`,
`scan_with_args()` and the `merge_into()` delegation; `statistics()` is already handled.

This also closes evidence 01 §6 question 3 — whether a Delta provider can answer statistics from
the log cheaply. It can, and does, without the wrapper.

### Two corrections to this probe, both recorded rather than hidden

1. **The first version supplied `Statistics::new_unknown`**, whose every field is
   `Precision::Absent`. That carries no information, so "the plan did not change" was the
   expected answer and the probe could not have failed. Replaced with `Precision::Exact(3)`.
2. **The first observation channel was broken.** `datafusion.explain.show_statistics` governs
   `EXPLAIN` output only; the programmatic display needs
   `displayable(plan).set_show_statistics(true)`. Without it the rendered plan showed no
   statistics at all, and "same" meant *not observed* rather than *not used* — two results that
   look identical and mean opposite things. The probe now asserts the channel is live
   (`statistics rendered in plan text: true`) before trusting the verdict.

### One unexplained inconsistency, flagged not resolved

`ExecutionPlan::partition_statistics(None)` and `partition_statistics(Some(0))` both return
`num_rows: Absent` at the plan root, while the rendered plan for the same root reports
`Rows=Exact(3)`. Two DataFusion channels for "what are this plan's statistics" disagree.

Not pursued here — it does not affect the PB02 verdict, which rests on the rendered plan being
identical across both arms. **Recorded as a round-2 question**, because a design that reasons
about cardinality needs to know which channel is authoritative.

---

## Net effect on the plan

| Plan section | Change |
|---|---|
| §2.3 wrapper table | `statistics()` row removed; `constraints()` row promoted with the measured result |
| §2.3 closing line | PB02 answered: the wrapper is justified, on constraints alone |
| §11 risk register | PB02 row closed |
| §3.4 "Keys" | strengthened — declaring `PrimaryKey` is an optimisation, not only documentation |
