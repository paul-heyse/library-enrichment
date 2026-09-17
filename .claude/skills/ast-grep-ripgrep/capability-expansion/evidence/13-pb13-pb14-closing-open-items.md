# Evidence 13 — PB13 and PB14: the two cheap open items, closed

Plan v2 §12.3 carried four open items. Two of them were one probe each; both are answered here,
and both **strengthened** the design rather than forcing a change — which is the outcome worth
reporting honestly, because it is the less interesting one.

| Probe | Question | Verdict |
|---|---|---|
| **PB13** | Does `WriteBuilder::with_input_plan` share a transaction with `ConstraintBuilder`? | **No — always two commits.** And a failed constraint-add is non-destructive, which makes DDL-first strictly better rather than merely safer |
| **PB14** | Does `is_distinct=true` terminate on a cyclic graph? | **Only when the projection excludes `depth`** — so the two mitigations are mutually exclusive |

---

## PB13 — two commits, and a failed constraint-add leaves the bad data committed

- Source: [`probes/delta-arrow-probe/examples/pb13_write_constraint_txn.rs`](probes/delta-arrow-probe/examples/pb13_write_constraint_txn.rs)
- Capture: [`probes/PB13-write-constraint-txn.out.txt`](probes/PB13-write-constraint-txn.out.txt)
- Closes: evidence 05 §4 question 4, open since round 1.

Constraint `off >= 0`; conforming rows `off = 10, 20`; violating row `off = -5`. Every observation
is read from a **fresh load** of the table, so nothing in-process is trusted.

```
A  write CONFORMING then constraint
     after create             version=0  files=0  constraints=none
     after write (input_plan) version=1  files=1  constraints=none                OK
     after add constraint     version=2  files=1  constraints=off_nonneg          OK

B  write VIOLATING  then constraint
     after write (input_plan) version=1  files=1  constraints=none                OK
     after add constraint     version=1  files=1  constraints=none                ERR: 1 rows
                                                                                  failed validation

C  constraint then write VIOLATING           <- the plan v2 §6.4 order
     after add constraint     version=1  files=0  constraints=off_nonneg          OK
     after write (input_plan) version=1  files=0  constraints=off_nonneg          ERR: 1 rows
                                                                                  failed validation

D  CONTROL constraint then write CONFORMING
     after add constraint     version=1  files=0  constraints=off_nonneg          OK
     after write (input_plan) version=2  files=1  constraints=off_nonneg          OK
```

### 1. They are two commits, unconditionally

Arm A settles the literal question: create at `v0`, write at `v1`, constraint at `v2`. The write
and the constraint addition are separate transactions, and nothing about `with_input_plan` changes
that — it is a `LogicalPlan` source rather than materialised batches, and it still commits on its
own.

### 2. The failure is non-destructive — which is the part that matters

Arm B is the interesting one. The write **succeeded** and committed at `v1`. The constraint add
then **failed** with *"Invalid data found: 1 rows failed validation check"*, and the table remained
at `version=1`, `files=1`, `constraints=none`.

So a failed constraint-add rolls back **only itself**. It does not undo the preceding write, and it
does not leave a half-added constraint.

That is benign in isolation and dangerous as a habit, because of what arm B *leaves behind*: a
table containing a row that violates the invariant you wanted, and **no constraint recording that
you wanted it**. Nothing in the table's own metadata says the invariant was ever intended. A later
reader sees a table with no constraints and assumes none applies.

### 3. DDL-first is not a dodge — it is strictly better

Plan v2 §6.4 sidestepped this question by ordering: constraints are added at table creation, before
any data is written. Arms C and D show that ordering is not merely a way to avoid an unknown.

- **C** — with the constraint in place, the violating write **fails and nothing lands**:
  `files=0`, version unchanged. The bad data never enters the table.
- **D** — the control. With the same constraint in place, a *conforming* write succeeds to `v2`.

Arm D is load-bearing. Without it, C's failure could have meant "adding a constraint breaks writing
altogether" rather than "the constraint rejected these rows" — the same confound that made the
first PB07 run meaningless. It passes, so C's failure is attributable to the data.

**Comparing B against C is the whole result:**

| Order | Violating data ends up | Constraint ends up | Table self-describes the invariant? |
|---|---|---|---|
| write → constraint (B) | **committed, at v1** | **absent** | no |
| constraint → write (C) | rejected, never written | present | yes |

Same two operations, same failure, opposite outcomes. §6.4's ordering rule is upgraded from
"sidesteps an open question" to **"the measured-correct order"**, and it now has a reason a reader
can check rather than a deferral.

### Consequence for the plan

No change to the design — v2 §6.4 already mandates DDL-first. The justification changes from
*"the interleaving never occurs, so the question is moot"* to *"the interleaving was measured, and
the other order silently commits data that violates an invariant it then fails to record."*

One new mechanical check for §10.2: **a constraint may only be added to a table at creation.**
Previously this was a consequence of the DDL phase's existence; it is now an invariant with a
measured failure mode behind it.

---

## PB14 — `is_distinct` tames a cycle only if the tuple domain is finite

- Source: [`probes/delta-arrow-probe/examples/pb14_recursive_distinct.rs`](probes/delta-arrow-probe/examples/pb14_recursive_distinct.rs)
- Capture: [`probes/PB14-recursive-distinct.out.txt`](probes/PB14-recursive-distinct.out.txt)
- Closes: plan v2 §12.3 item 2. PB09 measured only `UNION ALL`.

### Predicted from source before running

`datafusion-physical-plan-55.1.0/src/recursive_query.rs:318` builds the deduplicator over the
**full output schema**:

```rust
let distinct_deduplicator = is_distinct
    .then(|| DistinctDeduplicator::new(Arc::clone(&schema), &task_context))
    .transpose()?;
```

Deduplication is therefore on the whole output tuple, not on a node identity. Which predicts: a
projection carrying `depth` makes every row unique by construction — depth increments each round —
so the deduplicator never fires and `UNION` is no safer than `UNION ALL`.

### Measured

Graph `a→b, b→c, c→a` (a cycle) plus `c→d`. Timeout 15 s per arm; **a timeout is the result**.

```
A CONTROL  UNION ALL, (node,depth), unbounded   is_distinct=false   TIMED OUT   -- UNBOUNDED
B          UNION,     (node,depth), unbounded   is_distinct=true    TIMED OUT   -- UNBOUNDED
C          UNION,     (node),       unbounded   is_distinct=true    TERMINATED  count=4, 25.8ms
D CONTROL  UNION ALL, (node,depth), BOUNDED     is_distinct=false   TERMINATED  count=4, 29.5ms
```

The prediction holds exactly. **`is_distinct` reached the plan in arms B and C** — the probe reads
`RecursiveQueryExec: is_distinct=true` from the rendered physical plan *before* any verdict is
taken, so "it behaved like `UNION ALL`" means the flag arrived and did not help, not that it never
arrived. That check exists because PB02b's first run confused exactly those two.

Arm A is the control proving the graph is genuinely cyclic; arm D is the control proving a bounded
query over that same graph terminates.

> The two `count=4` results answer **different questions** and their agreement is a coincidence of
> this graph, not a cross-check. C counts the 4 distinct reachable nodes `{b, c, a, d}`; D counts
> the 4 paths of length ≤ 3. Do not read them as confirming each other.

### The finding: the two mitigations are mutually exclusive

```
depth column present  ->  every tuple unique  ->  deduplicator never fires  ->  needs a BOUND
depth column absent   ->  finite tuple domain ->  deduplicator terminates   ->  no bound possible,
                                                                                 and no depth to
                                                                                 report
```

A query cannot have both. `depth` is precisely what a bound needs, and precisely what defeats
distinct-based termination.

### Consequence for the plan

**Round 2's decision stands, and the reason is now measured rather than cautious.** Plan v2 §7.2
and §7.3 bound every closure query with an explicit `depth` column inside the recursive term, and
§12.3 item 2 hoped that `is_distinct` might serve as "a safety net rather than the mechanism".
**It cannot be a safety net**, because it is inert in exactly the queries the plan writes — every
one of them carries `depth`.

So the rule sharpens from *"bound the recursion"* to:

> **Every recursive CTE in this design carries a `depth` column and is bounded inside the
> recursive term. `UNION` instead of `UNION ALL` buys nothing once `depth` is present, so it is
> not used, and its absence is not a latent risk.**

One exception is now available and worth recording: a closure query that needs **only the set of
reachable nodes** and not their depths may use `UNION` unbounded and will terminate. Of the six
graph questions in v2 §7.3, that shape fits *reachability* questions (call-graph reachability, CFG
reachability) when the caller does not ask for path witnesses. It does **not** fit the four that
require a path witness or a depth, and v2 §7.3 requires path witnesses for transitive results, so
the exception is narrow. It is recorded as available, not adopted.

---

## Net effect

| Document | Change |
|---|---|
| evidence 05 §4 | question 4 — **answered**; the dossier's last open question closes |
| PLAN-V2 §6.4 | justification upgraded from "sidestepped" to "measured-correct order", with the B-vs-C table |
| PLAN-V2 §7.2, §7.3 | the `is_distinct` exception recorded; bounding rule sharpened |
| PLAN-V2 §10.2 | new check: a constraint may only be added at table creation |
| PLAN-V2 §12.3 | items 1 and 2 removed; two remain, both needing a real workspace |
