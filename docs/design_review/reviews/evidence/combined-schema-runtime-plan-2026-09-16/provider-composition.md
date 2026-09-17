# FP05 native provider composition verification

Retrieved 2026-09-16. Read-only verification against DataFusion 55.1.0,
delta-rs `58f07cd62bfbce3649a7e1c87c696288068ae184`, and the pinned kernel
`8ba063f8f84fec222000f66d40d70911d7c79675`. No builds or tests were run.
The DataFusion and Delta capability indexes were used to locate public seams;
the source below determines the verdict. This note does not change dependencies.

## Verdict

The required combination needs a narrow upstream patch if both actual
`ListingSchemaProvider::refresh` and `DeltaTableFactory::create` must own their
normal routes. Their existing injection points do not meet all requirements.
Custom replacement providers could avoid a patch but would replace the requested
upstream behavior. A bounded ObjectStore wrapper can cap inventory, but does not
turn the upstream collection into streaming consumption or fix collision handling.

## Exact sources

Local DataFusion source root is
`/home/paul/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`.
Local Delta source root is
`/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd/`.

| Capability | Exact file and lines | Finding |
|---|---|---|
| Listing injection | `datafusion-catalog-55.1.0/src/listing_schema.rs:72–87` | `new(authority, path, factory, store, format)` already accepts the shared ObjectStore and any `TableProviderFactory`. Retain these seams. |
| Inventory | Same file, `refresh:91–112` | `let entries: Vec<_> = self.store.list(Some(&self.path)).try_collect().await?;` materializes every object before discovery. Candidate roots are then accumulated in a HashSet. |
| Names/collisions | Same file, `refresh:114–145` | Name uses `file_name.split('.').collect_vec()[0]`; `if !self.table_exist(table_name)` silently skips colliding roots. HashSet iteration makes the winner unspecified. Registration occurs incrementally, so a later failure leaves partial refresh state. |
| Mutability | Same file, `register_table:178–188` | Registration inserts into the shared map, replacing existing entries; it returns the inserted provider rather than rejecting duplicates. |
| Delta factory opening | `crates/core/src/delta_datafusion/mod.rs:538–575` | Empty factory struct; `create` uses `open_table` or `open_table_with_storage_options`, then creates a provider. No ObjectStore, opener, pinned-version or semantic-schema argument exists. Session resolution uses `DeriveFromTrait`. |
| Exact lower-level opener | `crates/core/src/table/builder.rs:154,193–199,243–244,260–296` | Builder supports explicit version, custom root storage and IO runtime. The factory does not expose them. Custom storage is rooted at storage-system root, with table prefix applied by logstore construction. |
| Provider capture | `crates/core/src/delta_datafusion/table_provider.rs:299–322,377–429,471–477` | Builder can receive exact snapshots and a session; loaded `DeltaTable::table_provider()` captures its current snapshot. No public schema override setter is exposed on this builder. |
| Schema override/ownership | Same file `DeltaScanConfig:230–235`; `table_provider/next/mod.rs:518–543,594–598` | Direct `DeltaScanNext::new(snapshot, config.with_schema(...))` exists, but attaching the retained log store is crate-private. Its snapshot field is private. No public capture callback exists on the factory result. |

Primary links:
[ListingSchemaProvider](https://github.com/apache/datafusion/blob/55.1.0/datafusion/catalog/src/listing_schema.rs),
[DeltaTableFactory](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L538),
[DeltaTableBuilder](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs),
[provider builder](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider.rs),
[native scan](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs).

## Minimal patch boundary

1. In ListingSchemaProvider, consume listing with `try_next` and keep only bounded,
   owned distinct candidate roots. Bound objects examined and candidate count; reject
   limit exhaustion. Validate normalized name-to-root uniqueness before any provider
   publication. Construct the candidate map privately and publish it atomically.
   Keep the native name policy initially if desired, but reject its collisions.
2. Add an optional typed opening hook to DeltaTableFactory that receives the session
   and command and returns the opened `DeltaTable` plus optional read schema. The
   default path retains current behavior. The application hook uses existing
   DeltaTableBuilder APIs, validates table ID/version/storage schema, and records
   immutable capture from that same table before returning it. This avoids loading a
   second table to infer the factory result's identity. The factory continues to own
   construction of the actual native provider.
3. Add the missing optional schema setter to TableProviderBuilder and pass it into
   DeltaScanConfig inside `build`. This retains the existing log-store attachment,
   exact snapshot and session instead of copying private scan code. Require a concrete
   supplied SessionState for the application route; do not silently derive defaults.

The hook is an application-owned admission seam, not a new transaction/catalog engine.
No kernel patch is required by the inspected paths. Global factory registry replacement
is not an equivalent per-session immutable ownership guarantee. Preserve captured
providers behind the existing immutable session catalog boundary; do not attach a
refreshable shared listing map as the operation's snapshot authority.

## Nested CHECK IN formatting

`crates/core/src/delta_datafusion/expr.rs:632–641` formats the left side of InList
using ordinary Expr Display (`write!(f, "{expr} IN ...")`) rather than `SqlFormat`.
`fmt_expr_to_sql` at lines 649–655 promises a parsable SQL expression, but this arm
can leak DataFusion display literals such as `Utf8(...)` inside nested get_field.
The list entries use `expr_vec_fmt`, which applies SqlFormat; the defect is the left
expression in both positive and negated branches. BinaryExpr formatting uses the
SQL formatter recursively, explaining why equivalent OR comparisons avoid this path.
`operations/constraints.rs:166` persists the formatted constraint expression.

Small fix: format the InList left operand with `SqlFormat { expr }` in both branches.
This is a formatter correction, not a reason to disable constraints or optimizers.
Source: [expression formatter](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/expr.rs#L632),
[constraint persistence](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs#L166).
The reported failing IN and successful OR executions are parent-task observations;
this review independently verified their source-level explanation, not their runtime.

## Required route-specific oracles

- Actual native listing with a counting streaming store: bounded consumption,
  complete final inventory, collision rejection for `a`/`a.b`, and no partial map
  on list/factory failure. Retained operation catalogs remain unchanged on refresh.
- Actual DeltaTableFactory hook: supplied store identity and owned session observed,
  exact version/table ID/schema match captured provider; advancing/replacing the
  table after capture cannot silently change the operation's provider.
- Schema override: projected returned batches preserve application types and metadata;
  do not infer nested-leaf pruning from this override (the separately verified native
  scan only projects kernel top-level names).
- CHECK round trip: nested positive/negative IN, escaped field names, strings/nulls,
  then real constraint creation, reload, accepted rows and rejected rows. Assert that
  persisted SQL reparses and contains no display-only `Utf8(...)` syntax.

These remain qualification obligations; this note records no new passing gate.
