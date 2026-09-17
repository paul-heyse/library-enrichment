# Evidence 15 — PB17: the scoped overwrite, and a question asked about the wrong type

Plan v2 §12.3 carried two open items into phase 7. This closes the second, and closing it required
first noticing that **the question named a method that does not exist**.

| | |
|---|---|
| Question as asked | Does `MergeBuilder::with_replace_where` commit in one transaction at the pinned delta-rs rev, and what does a failed merge leave behind? |
| Answer | **`MergeBuilder` has no `with_replace_where`.** `with_replace_where` is `WriteBuilder`'s alone. The question was about the wrong type, and the design built on it was too |
| Question worth asking instead | Does a run-scoped `WriteBuilder` overwrite behave on a table shaped like a canonical one? |
| Verdict | **CONFIRMED** — preserves other runs, rejects a misattributed batch, idempotent in content, and the control destroyed what the treatment preserved |

- Source: [`probes/delta-arrow-probe/examples/pb17_replace_where.rs`](probes/delta-arrow-probe/examples/pb17_replace_where.rs)
- Capture: [`probes/PB17-replace-where.out.txt`](probes/PB17-replace-where.out.txt)
- Closes: plan v2 §12.3 open item 2, open since round 2.
- Read at rev `58f07cd62bfbce3649a7e1c87c696288068ae184` (`crates/core/Cargo.toml: version = "1.0.0"`).

---

## 1. The reading, and where the drift came from

`MergeBuilder` has sixteen public methods (`crates/core/src/operations/merge/mod.rs:196-476`).
None of them narrows the target. Its `predicate` argument is documented at `merge/mod.rs:152` as
`/// The join predicate`. A repo-wide search for `replace_where` finds exactly one definition:

```rust
// crates/core/src/operations/write/mod.rs:233-237
/// When using `Overwrite` mode, replace data that matches a predicate
pub fn with_replace_where(mut self, predicate: impl Into<Expression>) -> Self {
    self.predicate = Some(predicate.into());
    self
}
```

**Evidence 05 §2 recorded this correctly in round 1.** It lists `with_replace_where` under
`WriteBuilder` and says *"`with_replace_where` gives idempotent per-run reload: re-running
extraction family `F` for snapshot `S` replaces exactly `family = 'F' AND snapshot_id = 'S'`"*.
The attribution to `MergeBuilder` appeared later, when PLAN-V2 §12.2/§12.3 and IMPLEMENTATION.md
§4e compressed that note. So this is not a correction to the dossier — it is **drift between the
dossier and the plan**, and the plan was the thing that was wrong. Worth recording as a failure
mode in its own right: a summary of a primary source is not a primary source.

The same compression produced a predicate nobody could have written. `family` is not a column on
any canonical table; it exists only on `snapshot.extraction_run`. The column carrying exactly
`family ∧ snapshot` is `run_id`, which is `run:<snapshot>/<family>/0` by construction.

### Three further facts established by reading, so the probe did not re-ask them

| Fact | Where |
|---|---|
| A data-column predicate is accepted on an **unpartitioned** table — no flag, no gate, unlike Spark's `replaceWhere.dataColumns.enabled`. A non-partition column merely flips `partition_only` | `delta_datafusion/find_files.rs:152-159`; upstream test `test_replace_where` at `write/mod.rs:1959-2020` |
| `with_replace_where` is **silently ignored** unless the save mode is `Overwrite` — a passthrough plan, no error | `write/plan.rs:441-445` |
| One commit, always. Removes and adds land in one `Vec<Action>`, and unlike merge there is no empty-actions guard, so a no-op write still bumps the version | `write/mod.rs:588-634` vs `merge/mod.rs:1732-1734` |

The second of those is why the probe needs a control at all, and the third is measured below in
arm D.

---

## 2. What reading could not settle

The scoped path does not simply drop matched files. For an unpartitioned table it takes
`RewriteKind::DataRescue`, which **unions the caller's batch with a scan of the existing Delta
files** and realigns the two schemas (`write/plan.rs:500-517`, `align_plan_to_schema` at `:513`).

Delta's read path returns `Utf8View` where the stored type is `Utf8` — PB05's
`schema_force_view_types` finding, which has already cost this codebase two cycles and is the
reason `writer::read_table` carries a `cast_to` helper. Whether that union plans against a table
carrying Arrow extension metadata and a CHECK constraint is not answerable from the source without
executing it.

So the probe's table is a canonical table in miniature: `entity_id`, `entity_key` and
`precision_rank` all carry `ARROW:extension:name`, `precision_rank` is under the same
`BETWEEN 0 AND 3` CHECK that `derived_constraints` puts on every lattice-bearing table, and the
strings are stored `Utf8`. A `(id, value)` table would have answered a question nobody asked.

---

## 3. What ran

```
  A  run1 scoped -> run2 scoped        run1 must SURVIVE
      write run1 (into EMPTY)   version=2  run1=2                        OK
      write run2 scoped         version=3  run1=2 run2=2                 OK

  B  run1 scoped -> run2 UNSCOPED      CONTROL: run1 must be DESTROYED
      write run2 unscoped       version=3  run2=2                        OK

  C  batch carries a foreign run_id    must be REJECTED
      write mixed run_ids       version=2  run1=2   ERR: Invalid data found: 1 rows
                                                         failed validation check.

  D  re-run arm A's second write       rows unchanged, version advances
      before re-run             version=3  run1=2 run2=2
      after re-run              version=4  run1=2 run2=2                 OK
```

Every row count is read from a **fresh load** through a real scan, so nothing in-process is
trusted and the `Utf8View` read path is exercised on every observation rather than assumed.

### Arm A — and the case every fresh catalog hits first

The `Utf8View`/`Utf8` union plans and executes. No cast was needed on the write side; the rescue
branch and the insert batch reconcile without help.

A second thing arm A establishes that was not the headline: **a scoped write into an empty table
works.** `version=2` after the first write is create at `v0`, constraint at `v1`, write at `v2` —
PB13's granularity, unchanged. This matters because the real writer will scope *every* write,
including the first one into a fresh catalog, and a predicate matching no existing file is the
ordinary case there rather than an edge.

### Arm B — the control, and why it is not decoration

`with_replace_where` fails open: outside `Overwrite` mode it is ignored with no error. A green arm
A on its own is therefore consistent with a predicate that did nothing whatsoever. The control
removes `run1` entirely, which is what makes arm A's survival attributable to the predicate rather
than to the second write happening to be small.

### Arm C — the writer proves its own scope, for free

A batch whose rows do not all satisfy the predicate is rejected, per row, at stream time:

```
Invalid data found: 1 rows failed validation check.
```

`DeltaTableError::InvalidData` (`errors.rs:101-106`), raised from
`delta_datafusion/data_validation.rs:737-759`. NULL counts as a violation there, not as SQL
`WHERE` semantics — so this is a genuine "every row satisfies the predicate" guard.

**The table was left at `version=2` with `run1=2` intact**, so the rejection is clean at the log
level. This is a guard worth leaning on: a build that misattributes a row fails loudly instead of
silently widening what it replaces.

### Arm D — idempotent in content, not in version

Re-running the identical scoped write leaves the rows untouched and advances the table from `v3`
to `v4`. `WriteBuilder` has no empty-actions guard, so **a no-op scoped write is still a real
commit.** Anything watching the Delta log to decide whether a build did something must compare
rows, not versions — the same shape as the correction that cost a run of the phase-4 crash test.

---

## 4. Consequences

1. **The pass is one builder call, not a merge.** `writer::write_table` already uses `WriteBuilder`
   with `SaveMode::Overwrite`; it gains `.with_replace_where(format!("run_id = '{run_id}'"))`.
2. **The predicate must be plain column algebra.** Not a style preference. PB07 (evidence 12)
   measured that any Delta expression naming a *relation* — "a CHECK constraint, a merge predicate,
   a `replaceWhere`" — reaches `DeltaContextProvider::get_table_source`, which is
   `unimplemented!()`, and **aborts the process**. §9.1 states this rule for constraints; it is a
   rule about Delta expressions generally.
3. **Two tables stay whole-table overwrites, each for a stated reason.** `catalog.lattice` has no
   `run_id` to scope by — rank-to-label is not an observation about a snapshot.
   `snapshot.extraction_run` is rewritten across runs by reaping, so a read-modify-write of the
   whole table is the operation there rather than a shortcut.
4. **Do not enable Deletion Vectors or Row Tracking on these tables.** Parquet pushdown — which is
   what collapses the rescue scan to footer reads — is disabled when either feature is on
   (`delta_datafusion/table_provider/next/scan/plan.rs:492-494`). Evidence 04's feature table
   recommends both on other grounds; this is the cost that recommendation does not mention.
5. **A failed write leaves orphan parquet.** Data files are written before the commit and nothing
   removes them on failure; only `VacuumMode::Full` reclaims them (`operations/vacuum.rs:356`).
   Atomic at the log level, not at the storage level. The catalog is derived and rebuildable, so
   this is a disk-space cost rather than a correctness one — but it is not nothing, and arm C
   exercises the failure path that produces it.
