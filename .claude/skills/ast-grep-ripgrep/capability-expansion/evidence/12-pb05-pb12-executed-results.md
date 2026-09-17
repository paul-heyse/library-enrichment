# Evidence 12 — PB05–PB12 executed: the rest of the probe backlog

Round 1 left six probes open (evidence 09) plus two questions raised by PB02 and never settled.
All are answered here. Four of the eight changed a design decision; three of them are hazards
that produce **wrong answers silently**, with no error and no warning.

| Probe | Question | Verdict |
|---|---|---|
| **PB05** | Do `MergeBuilder` predicates see session UDFs? | **YES**, in both the join predicate and match-clause predicates |
| **PB06** | Does `preimage` fire for a Delta-backed scan? | **YES, and it prunes** — under three conditions, two of them silent |
| **PB07** | Can a CHECK constraint reference a UDF? Another table? | **UDF yes, but it permanently couples every future writer. Another table PANICS.** |
| **PB08** | Does `Syntax` have a regex lexer / markup-parse content? | **No lexer; does NOT markup-parse.** Safe renderable, no highlighting |
| **PB09** | Are outer predicates pushed into the recursive term? | **NO.** On a cyclic graph the query never terminates |
| **PB10** | Does `DeltaCdfTableProvider` filter pushdown narrow? | **Accepts everything; narrows only on partition columns** |
| **PB11** | Which statistics channel is authoritative? | **Resolved** — evidence 11 read a deprecated one |
| **PB12** | Can the wrapper derive `Constraints` from Delta's own? | **No, not in general** — stored as SQL text, no slot in the type |

Sources and captures in [`probes/`](probes/). Every Rust arm ran against the pinned stack
(`deltalake` at git rev `58f07cd6…`, `datafusion =55.1.0`, `arrow 59.3.0`, rustc 1.98.1).

---

## PB06 — `preimage` fires and prunes, under three conditions

Capture: [`probes/PB06-preimage.out.txt`](probes/PB06-preimage.out.txt) ·
source: [`probes/delta-arrow-probe/examples/pb06_preimage.rs`](probes/delta-arrow-probe/examples/pb06_preimage.rs)

Four Delta files, one per bucket of a `4096`-byte offset space. A correct rewrite prunes to one.

```
1 UDF preimage HALF-OPEN   count=100   Filter: t.off >= Int64(8192) AND t.off < Int64(12288)
                                       partial_filters=[...]   Rows=Inexact(100)   <- ONE file
2 UDF preimage CLOSED      count=99    Filter: t.off >= Int64(8192) AND t.off < Int64(12287)
                                       partial_filters=[...]   Rows=Inexact(100)
3 CONTROL no preimage      count=100   Filter: offset_bucket_np(t.off) = Int64(2)
                                       partial_filters=[UDF]   Rows=Inexact(400)   <- all four
4 CONTROL volatile         count=100   Filter: offset_bucket_vol(t.off) = Int64(2)
                                       NO partial_filters      Rows=Exact(400)
5 GOLD hand-written range  count=100   Filter: t.off >= 8192 AND t.off <= 12287
                                       partial_filters=[...]   Rows=Inexact(100)
6 CONTROL built-in floor   count=100   Filter: __common_expr_3 >= 2 AND __common_expr_3 < 3
                                       partial_filters=[...]   Rows=Inexact(400)   <- NOT pruned
```

**The answer to the headline question is yes**: arm 1 matches the gold standard exactly, on both
the rewritten predicate and the file pruning. Domain UDFs are therefore usable in hot filters and
the fallback to `GeneratedColumns` is not needed. But three conditions came out of the arms, and
two of them fail silently.

### 1. The UDF must be `Volatility::Immutable`

Arm 4 is not merely un-rewritten — it loses its `partial_filters` entirely, so the predicate does
not reach the scan in any form. The gate is in
`datafusion-optimizer-55.1.0/src/simplify_expressions/expr_simplifier.rs:2111`:

```rust
if func.signature().volatility != Volatility::Immutable {
    return Ok(PreimageResult::None);
}
```

No diagnostic is emitted. A UDF declared `Volatile` out of caution is silently excluded from
every optimisation in this family.

### 2. The interval is consumed as **half-open**, and getting it wrong loses rows silently

This is the finding worth the most. `PreimageResult::Range { expr, interval }` carries an
`Interval`, which is nominally a *closed*-interval type. The rewrite
(`simplify_expressions/udf_preimage.rs:47`) treats it as half-open:

```rust
// <expr> = x ==> (<expr> >= lower) and (<expr> < upper)
Operator::Eq => and(expr.clone().gt_eq(lower), expr.lt(upper)),
```

So a `preimage` returning the inclusive upper bound produces a predicate one value short. Arm 2
does exactly that and **returns 99 rows where the correct answer is 100** — same plan shape, same
pruning, same absence of any error. Nothing distinguishes it from arm 1 except the count.

The first run of this probe did not catch it, because the test data had no row at the top of a
bucket: both arms returned 100 and the off-by-one was invisible. The data now carries one row at
`off = i*4096 + 4095` precisely so the arms diverge. **A probe that cannot fail proves nothing**,
and this one could not fail until that row existed.

### 3. `preimage` must return a predicate on a **bare stored column**

Arm 6 is the instructive control. The built-in `floor` implements `preimage` and *is* rewritten —
but nothing is pruned, `Rows=Inexact(400)`. Its preimage returns a bound on
`CAST(t.off AS Float64) / Float64(4096)`, a derived expression, and Delta's min/max statistics are
keyed on stored columns. Arm 1 returns `args[0]`, the bare column, and prunes.

**Rewriting and pruning are two different achievements.** A UDF can pass the first and fail the
second, and the plan will look optimised either way.

### Consequence for the plan

Plan §5.1 already required `preimage` on every filter-eligible UDF. It now needs three more
clauses, each mechanically checkable:

1. every `bridge::udf::scalar` UDF is `Volatility::Immutable` unless it genuinely is not;
2. every `preimage` returns a **half-open** upper bound, and carries a boundary-value test;
3. every `preimage` returns a predicate on a bare stored column, not a derived expression.

---

## PB11 — the two disagreeing channels were not peers

Evidence 11 recorded `partition_statistics` returning `Absent` at a plan root whose rendered text
said `Rows=Exact(3)`, and flagged it unresolved. **The compiler answered it.** Building PB06 emits:

```
warning: use of deprecated method `ExecutionPlan::partition_statistics`:
         Use StatisticsContext::compute instead
```

Asked of all three channels, with a MemTable arm to separate "delta-rs does not populate it" from
"nobody populates it":

```
delta scan       DEPRECATED partition_statistics  None=Absent      Some(0)=Absent
                 StatisticsContext::compute       None=Exact(400)  Some(0)=Exact(400)
                 rendered root                    Rows=Exact(400)

delta filtered   DEPRECATED partition_statistics  None=Absent      Some(0)=Absent
                 StatisticsContext::compute       None=Inexact(100) Some(0)=Inexact(3)
                 rendered root                    Rows=Inexact(100)

memtable scan    DEPRECATED partition_statistics  None=Absent      Some(0)=Absent
                 StatisticsContext::compute       None=Exact(3)    Some(0)=Exact(3)
                 rendered root                    Rows=Exact(3)
```

The deprecated method returns `Absent` for **every** plan including the MemTable, so it was never
a delta-rs gap. `StatisticsContext::compute` agrees with the rendered text in all three cases.

**Resolved: `StatisticsContext::compute` is the programmatic channel; the rendered plan text is
its display.** The deprecation warning is deliberately left unsuppressed in the probe — it is the
evidence.

One residual: on the filtered Delta scan, `Some(0)` reports `Inexact(3)` against `Inexact(100)`
for the whole plan. Per-partition estimates are not the whole divided by partitions. Noted, not
load-bearing — the design reasons about whole-plan cardinality.

---

## PB05 — session UDFs reach merge predicates

Capture: [`probes/PB05-PB07-PB12-session.out.txt`](probes/PB05-PB07-PB12-session.out.txt)

```
a CONTROL plain predicate, session WITH udf      OK
b UDF in merge predicate, session WITH udf       OK
c CONTROL UDF in predicate, session WITHOUT udf  ERR: Invalid function 'is_valid_offset'
d UDF in when_matched_update predicate           OK
e UDF predicate + RequireSessionState            OK
```

Arm b answers the question; arm c is what makes it an answer rather than a coincidence — the same
merge with the same predicate fails when the supplied session lacks the UDF. Arm d matters
independently: match-clause predicates are resolved separately from the join predicate, and that
is where the merge passes of plan §6.2 would actually use a domain predicate.

**Plan §6.2's six merge passes may use domain predicates in both positions.**

### The fallback policy should be set, not defaulted

`delta_datafusion/session.rs:125` defines three policies, and the **default is the dangerous one**:

```rust
pub enum SessionFallbackPolicy {
    #[default]
    InternalDefaults,      // "log a warning and use internal defaults"
    DeriveFromTrait,       // derive a SessionState from the Session trait
    RequireSessionState,   // error if the provided session is not a SessionState
}
```

Under `InternalDefaults` a caller's session — and therefore its entire UDF registry — can be
discarded with nothing but a log line. Arm e confirms `RequireSessionState` works with the plan's
intended session. **Every delta-rs builder call site in the plan sets
`with_session_fallback_policy(RequireSessionState)`**, so a dropped session is an error rather
than a silently different query.

---

## PB07 — a UDF in a CHECK constraint couples every future writer

```
a CONTROL plain constraint `off >= 0`            OK
b UDF constraint, session WITH udf (add)         OK
c write by a session WITHOUT the udf             ERR: Invalid function 'is_valid_offset'
d CONTROL write by a session WITH the udf        OK
e constraint with a subquery on another table    PANIC (process would have aborted)
```

The mechanism, read from source before the probe was written, is an **asymmetry between add time
and write time**:

- `operations/constraints.rs:164` resolves the expression against the supplied session, then
  serialises it to a **SQL string** with `fmt_expr_to_sql` and stores it in table metadata under
  `delta.constraints.<name>`;
- `delta_datafusion/data_validation.rs:319` re-parses that string at **write** time with
  `parse_predicate_expression(schema, sql, session)` against **the writer's** session.

So the constraint outlives the session that created it, and arms c/d measure the consequence: a
table carrying a UDF-bearing constraint **cannot be written by any session that does not register
that UDF**. That is durable, table-level coupling to a piece of this tool's code — not a session
concern. A future version of the CLI that renames a UDF locks itself out of its own tables.

### Arm d is the reason to trust arms c and d at all

The first run of this probe had arms c **and d** failing identically with

```
No installed planner was able to convert the custom node to an execution plan: MetricObserver
```

The control failing the same way as the treatment means the probe was measuring neither. Delta
writes plan through delta-rs's own `MetricObserver` node, so a session built with
`SessionContext::new()` cannot execute a Delta write **at all** and the UDF never entered the
question. Both sessions are now built from `deltalake::delta_datafusion::create_session()`, which
is what delta-rs uses internally, and the control passes. This is a general fact worth carrying
into the implementation: **the plan's single `SessionState` must be built from delta-rs's
`create_session()`, not from a bare DataFusion `SessionContext`.**

### A constraint cannot reference another table — it aborts the process

`delta_datafusion/expr.rs:237`:

```rust
impl ContextProvider for DeltaContextProvider<'_> {
    fn get_table_source(&self, _name: TableReference) -> DFResult<Arc<dyn TableSource>> {
        unimplemented!()
    }
}
```

Any Delta expression that names a relation — a CHECK constraint, a merge predicate, a
`replaceWhere` — panics with `not implemented` rather than returning an error. The probe runs this
arm inside `tokio::spawn` so the panic surfaces as a `JoinError` instead of killing the process.

**Referential invariants cannot be table constraints.** The plan's risk-register mitigation
(`projection.violations_*` views, not write-blocking) is now the only option, and the reason is
stronger than "unsupported": attempting it is a process abort, so it must not be reachable from
input.

---

## PB12 — constraints are SQL text; `Constraints` has no slot for them

```
plain `off >= 0`              delta.constraints.off_nonneg = "off >= 0"
UDF `is_valid_offset(off)`    delta.constraints.off_valid  = "is_valid_offset(off)"
```

A table does expose its own constraints, readable through
`table.snapshot()?.metadata().configuration()`, as **SQL text**. But DataFusion's `Constraints`
models only `PrimaryKey(Vec<usize>)` and `Unique(Vec<usize>)` over column indices — an arbitrary
CHECK expression has no representation in that type.

**So the wrapper cannot derive its `Constraints` from Delta's `CheckConstraints` in general.** It
could recognise the narrow case of a constraint whose text is exactly a key shape, but that is a
parser for a convention this codebase would be inventing on both sides.

Closes evidence 05 §4 question 1's follow-up: **declare the key on the wrapper and write the
matching CHECK separately.** The duplication is real, and the honest mitigation is a build-time
consistency check between the two declarations rather than deriving one from the other.

---

## PB09 — outer predicates are not pushed into the recursive term

Capture: [`probes/PB09-PB10-recursion-cdf.out.txt`](probes/PB09-PB10-recursion-cdf.out.txt)

The graph is `a→b, b→c, c→a` (a **cycle**) plus `c→d`, so an unbounded closure has no fixpoint and
"not pushed" is observable as non-termination rather than as an opinion about plan shape.

```
A bound OUTSIDE the CTE     depth bound inside recursive term: false
                            executed: TIMED OUT after 20s -- UNBOUNDED
B bound INSIDE  (CONTROL)   depth bound inside recursive term: true
                            executed: count=4 in 23.9ms
```

The optimised plan for arm A places `Filter: depth <= Int64(3)` **above** `RecursiveQuery`; arm B
places `Filter: closure.depth < Int64(3)` inside the recursive term, under the `SubqueryAlias: c`
that reads the working table.

**Plan §6.4's mitigation is mandatory, not optional.** Every one of the six closure queries
carries an explicit `depth` column and bounds it *inside* the recursive term. `is_distinct=true`
is not a substitute — arm A's plan shows `is_distinct=false` for a `UNION ALL` recursion, and the
plan's claim that `is_distinct` "gives cycle tolerance without a written visited-set" is not
supported by this probe and should not be relied on until separately measured.

### A correction to this probe, recorded rather than hidden

The first version reported `filter inside recursive term: true` for **both** arms, contradicting
its own execution results. It was matching the base term's `Filter: edge.parent = 'a'` — a filter
that is inside the recursive query node but is not the bound. A plan-shape observable that
disagrees with execution is worse than no observable; it now tests specifically for a filter on
the depth column.

---

## PB10 — the CDF provider accepts every filter and narrows on partition columns only

Predicted from source before running, because a naive reading of the result would be a false
positive: `delta_datafusion/cdf/scan.rs:116` returns
`TableProviderFilterPushDown::Exact` for **every** filter unconditionally (source comment:
`// maybe exact`), so DataFusion removes the filter from above the scan in all cases. "The filter
disappeared" is therefore not evidence of anything. `operations/load_cdf.rs:68` says the predicate
is "used ONLY to prune files by their partition values… any non-partition conjuncts are ignored
here", with a `FilterExec` restoring row-level correctness.

Three commits across two partitions, so there are three files to narrow among:

| Filter | count | files scanned | narrowed? |
|---|---|---|---|
| none | 6 | `{3 groups: [hir, mir, hir]}` | — |
| `family = 'hir'` (**partition**) | 4 | `{2 groups: [hir, hir]}` | **yes** |
| `n >= 20` (data column) | 2 | `{3 groups: […]}` | no |
| `_commit_version >= 2` (CDF metadata) | 4 | `{3 groups: […]}` | no |

Every count is correct — the `FilterExec` does its job. Only the partition filter reduces the file
list, and it carries a real `pruning_predicate` on `family_min`/`family_max` with
`required_guarantees=[family in (hir)]`.

The data-column and `_commit_version` arms do reach `DataSourceExec` as a parquet `predicate` with
a `pruning_predicate`, so row-group pruning inside a file remains possible; what does not happen is
file elimination.

### Two consequences for plan §6.3

1. **Partition the canonical tables on the column incremental re-derivation filters by** — the
   extraction `family`, which is what a re-run narrows to. Partitioning is what makes the change
   feed cheap; a predicate alone is not.
2. **Narrow the change range with `with_starting_version` / `with_ending_version`, never with a
   predicate on `_commit_version`.** The predicate is correct and reads the entire range anyway.
   The CDF schema is
   `[entity_id, n, family, _change_type, _commit_version, _commit_timestamp]`, so the metadata
   columns are available for projection and joining — just not for narrowing what is read.

---

## PB08 — `Syntax` is markup-safe, and there is no regex lexer

Capture: [`probes/PB08-syntax-regex.out.txt`](probes/PB08-syntax-regex.out.txt) ·
source: [`probes/PB08-rich-syntax-regex.py`](probes/PB08-rich-syntax-regex.py)

```
regex                        -> None         <- same as a deliberately fake lexer name
yaml                         -> YamlLexer
rust                         -> RustLexer
text                         -> TextLexer
definitely-not-a-lexer-9f3a  -> None
```

Asking for a `regex` lexer returns `None` — **identically to a nonsense name**, with no error.
Checked against the full roster rather than inferred from one lookup: of **602 lexers in
pygments 2.21.0**, none is a regex lexer (the eight matching `/regex|pcre|ragel/` are all Ragel, a
state-machine language).

The markup question came out the other way, and it is the one that matters:

```
[a-z]      Syntax=kept   print(markup)=LOST   print(no markup)=kept
[pattern]  Syntax=kept   print(markup)=LOST   print(no markup)=kept
...
Syntax lost 0/8
CONTROL print(markup=True)  lost 8/8   (P01: expect all)
CONTROL print(markup=False) lost 0/8   (expect 0)
```

**`Syntax` does not markup-parse its content.** Both controls behaved as P01 measured, so the
harness is sound: the lossy control lost everything, the lossless control lost nothing.

**Consequence for plan §8.3:** `Syntax` is the correct renderable for patterns — it is safe by
construction, where `Console.print` needs `escape()` or `markup=False`. It simply will not
highlight regex. Writing a small pygments `RegexLexer` is the only route to highlighting, and is
deferred: correctness is free, colour is not.

---

## API facts established in passing

Each of these cost a compile cycle or a wrong result, and none is in the skill indexes.

- **`ScalarUDFImpl` has no `as_any`** in DataFusion 55.1 — the same removal already recorded for
  `TableProvider`.
- **`ScalarUDFImpl` requires `DynEq + DynHash`**, satisfied by deriving `PartialEq, Eq, Hash` on
  the impl struct. `Signature` already derives all three.
- **`ExecutionPlan::partition_statistics` is deprecated** in favour of
  `StatisticsContext::compute(plan, &StatisticsArgs::new())`. `StatisticsContext` holds an
  `Rc<RefCell<_>>` and is therefore `!Send`; confine each use to a block with no `await` in it.
- **`DeltaTable` carries a 20-method fluent operation surface** — `create`, `restore`, `vacuum`,
  `filesystem_check`, `add_feature`, `set_tbl_properties`, `add_columns`,
  `update_field_metadata`, `drop_column_not_null`, `update_table_metadata`, `generate`,
  `scan_table`, `scan_cdf`, `write`, `optimize`, `delete`, `update`, `merge`, `add_constraint`,
  `drop_constraints`. This **refines PB04**: `DeltaOps` does not exist, but "the individual
  builders are the only entry point" was also not right — `DeltaTable::<op>()` is the idiomatic
  seam and is what the plan's call sites should use.
- **`CdfLoadBuilder::new` is `pub(crate)`.** The public route is
  `DeltaCdfTableProvider::try_new(table.scan_cdf())`.
- **Delta writes require delta-rs's own session.** `create_session().into_inner()` gives a
  `SessionContext` on which UDFs can be registered before taking `.state()`.
- `table.get_file_uris()?.count()` for a file count; there is no `get_files_count`.

---

## Net effect on the plan

| Plan section | Change |
|---|---|
| §2.4 session assembly | built from delta-rs `create_session()`; `RequireSessionState` on every builder |
| §4.2 validation in the table | CHECK constraints must not reference UDFs (writer coupling) or tables (panic) |
| §5.1 UDF requirements | three new mandatory clauses: Immutable, half-open, bare column |
| §6.2 merge passes | domain predicates confirmed usable in join and match-clause positions |
| §6.3 CDF | partition on `family`; narrow by version bounds, not by `_commit_version` predicate |
| §6.4 recursive CTEs | depth bound **inside** the recursive term is mandatory; `is_distinct` claim withdrawn |
| §8.3 renderables | `Syntax` for patterns — safe, unhighlighted; no regex lexer exists |
| §11 risk register | PB05–PB10 rows closed; two new rows for the silent-failure modes |
| §2.3 wrapper | `constraints()` declared, not derived (PB12) |
