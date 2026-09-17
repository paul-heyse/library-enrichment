# PB19 — the CSV reader's arity and quoting behaviour

**Executed 2026-09-16.** `evidence/probes/delta-arrow-probe/examples/pb19_csv_arity.rs`,
captured at `evidence/probes/PB19-csv-arity.out.txt`. datafusion `=55.1.0`.

## Why

Step 3 replaces `index.rs`'s hand-rolled `split('\t')` reader with `CsvReadOptions`. That reader
exists to enforce one guarantee, stated at `index.rs:12-14`:

> "a silently shifted column is exactly the class of error this catalog exists to prevent: it would
> not fail, it would produce confidently wrong answers."

**Nothing in `evidence/` had ever exercised a DataFusion file reader** — no `read_csv`,
`register_csv`, `CsvReadOptions`, `ListingTable` or `FileFormat` appears in any of the seventeen
preceding dossiers or nine probes. The guarantee would have been kept or dropped by accident.

## Resolved defaults, read off the struct rather than inferred

```
has_header=true  delimiter=','  quote='"'  file_extension=".csv"  truncated_rows=false
```

## Results

| arm | input | observed |
|---|---|---|
| **A** control | three conforming 3-field rows | 3 rows, every cell intact |
| **A'** control | same bytes, `file_extension` default | **rejected**: *"File path '…/a2.tsv' does not match the expected extension '.csv'"* |
| **B** | a 2-field row | **rejected**: *"Csv error: incorrect number of fields for line 2, expected 3 got 2"* |
| **C** | a 4-field row | **rejected**: *"…expected 3 got 4"* |
| **D** | arm B with `truncated_rows(true)` | 3 rows, the short row null-padded — so the default B exercised is `false` |
| **E** | a cell *containing* `"` | `a"b` read verbatim; the quote is data |
| **F** | a cell *starting* with `"` | **`"quoted"` silently became `quoted`** |
| **F'** | arm F with `quote(b'\0')` | `"quoted"` read verbatim |

## What this settles

**The arity guarantee is kept, and it is kept by the default.** `truncated_rows` defaults to
`false`, so a short or long row fails the read with a message naming the line and both counts.
Arm D confirms the default was what arm B exercised rather than something else. No
`violations_staging_arity` view is needed — the guard is the reader, and it is louder than the one
it replaces, because it names the line number.

**Two options must be set explicitly, and one of them would have corrupted real data.**

`file_extension` defaults to `".csv"` and rejects every `.tsv` outright (arm A'). Loud, therefore
harmless, but it must be set.

`quote` is the dangerous one. Arm E passing proves nothing about arm F, because RFC4180 readers
treat a quote as special **only at field start** — so a cell *containing* a quote is safe while a
cell *beginning* with one is consumed. Arm F is not hypothetical:

```
$ awk -F'\t' '{for(i=1;i<=NF;i++) if(substr($i,1,1)=="\"") print FILENAME" field"i": "$i}' *.tsv
behaviors.tsv field8: "hello"
behaviors.tsv field8: "hello" 'goodbye' "said"
behaviors.tsv field8: "hello"
```

Three cells in the shipped `behaviors.tsv` begin with `"`. Read with the default options they lose
their quotes, with **no error and no warning** — the precise failure `index.rs` was written to
prevent, reintroduced by the mechanism meant to replace it. The skill's contract
(`index.rs:5-8`) says there is no quoting; `quote(b'\0')` is what tells the reader so.

## The reader Step 3 must write

```rust
CsvReadOptions::new()
    .has_header(false)
    .delimiter(b'\t')
    .file_extension(".tsv")   // default is ".csv" and rejects every input (arm A')
    .quote(b'\0')             // default is '"' and silently eats a leading one (arm F)
    .schema(&schema)          // borrows: the Schema must outlive the options
```

Leaving `truncated_rows` at its default is deliberate and is the arity guard.

## Lesson

The control that mattered was not A but **E against F**. E is the arm a person writes when they
think "does quoting bite?", it passes, and it licenses exactly the wrong conclusion. The two differ
only in where the quote sits, and only F is reachable from the shipped data. A probe whose arms do
not separate *contains* from *begins with* would have shipped the corruption and reported a clean
verdict — which is the same shape as §12.3's lesson about summarising a primary source.
