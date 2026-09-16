---
title: Typed struct schema design essay
status: proposed
pins: {datafusion: 55.1.0, arrow: 59.3.0, delta: 58f07cd62bfbce3649a7e1c87c696288068ae184, kernel: 8ba063f8f84fec222000f66d40d70911d7c79675}
regenerated: 2026-09-16
---

# Typed struct schema design essay

**Evidence boundary:** this is a design essay, not an executable capability inventory.
**Interface-checked:** full-field expression/UDF hooks, native struct/list operations,
and the distinction between `DataType::equals_datatype` and exact `Field` equality
were checked against the pinned skills and source in the
[schema review](../design_review/reviews/design_review_schema-engineering-typed-values_2026-09-16.md).
**Proposed:** application layouts, simplified consumers and performance benefits below.
No nested Parquet I/O savings or Delta nested expression pushdown have been measured.
The historical struct-filtering fix/backport claim below is unverified and supplies
no acceptance evidence. Plan 08 requires reproducible exact-pin tests and measurements.

Yes. There is a strong architectural case for this, but I would formulate the goal more precisely:

> **Represent coupled data as first-class, typed domain values, and make operations consume, preserve, or explicitly transform those values’ contracts.**

The main payoff is not nesting itself. It is **eliminating independent representation choices that downstream code would otherwise have to reconcile**. Arrow supplies structural invariants; DataFusion supplies schema-aware planning and extensible execution; your application supplies the domain semantics that neither system can infer automatically. DataFusion’s expression and UDF interfaces explicitly support full Arrow fields—including nullability and metadata—so this can extend substantially beyond column naming conventions. 

One qualification to “more strongly relational”: a flat Arrow `RecordBatch` already aligns its columns by row and validates matching lengths and types. A struct does not create that alignment, add foreign keys, or improve normalization automatically. What it adds is **a compositional value boundary**: a group of fields can have one identity in an expression, one presence/absence policy, and one input/output contract. 

## 1. Where richer schemas remove real implementation complexity

The largest gains come from replacing representations that admit unwanted independent variation.

| Existing representation | Stronger representation | Design benefit |
|---|---|---|
| Related top-level scalar columns | `Struct<...>` | Operations can accept, return, select, and pass the complete value rather than repeatedly assembling column groups. |
| Several independently nullable fields that must appear together | Nullable `Struct` with non-nullable children | Encodes “absent, or structurally complete” instead of requiring repeated all-or-none presence checks. |
| Parallel lists of identifiers, values, bounds, weights, etc. | `List<Struct<id, value, ...>>` | One list boundary and element ordering replace independent lists that must be checked for alignment. |
| Variable-length lists that must always have a fixed dimension | `FixedSizeList<T, N>` | Moves the uniform extent into the physical type. |
| JSON or a map whose keys are actually a fixed schema | `Struct` with named, typed children | Makes the fields available to schema analysis rather than rediscovering keys and types while processing values. |
| Independently produced values, diagnostics, and status | One struct-valued operation | Gives the operation one coherent result contract instead of several independently coordinated outputs. |

These are design consequences of Arrow’s layouts: structs contain typed child arrays; lists provide offsets into a child array; a list’s child can itself be a struct; fixed-size lists carry their extent in the datatype. They do **not** imply that arbitrary relationships between the children are automatically checked. 

### The strongest structural example: `List<Struct>` versus parallel lists

Compare:

```text
component_ids: List<UInt32>
fractions:     List<Float64>
uncertainties: List<Float64>
```

with:

```text
composition: List<
    Struct<
        component_id: UInt32,
        fraction: Float64,
        uncertainty: Float64
    >
>
```

In the first representation, code must establish that list lengths, element order, filtering, and null handling agree. In the second, there is **one sequence of component records**. Selecting or filtering a complete element carries its associated fields together.

The structural alignment checks disappear because there are no longer three independent list boundaries to compare. However, the constructor can still associate the wrong fraction with an identifier; duplicate identifiers, missing components, and normalization still require domain validation.

That distinction generalizes:

> **Schemas can eliminate invalid structures. Domain-aware constructors and operations must eliminate invalid meanings.**

## 2. A concrete domain-oriented representation

For a scientific or solver-oriented pipeline, an illustrative schema might look like this. Here `!` means required, `?` means nullable, and bracketed annotations represent application-defined field metadata—not literal Arrow schema syntax.

```text
scenario_id: UInt64!

state: Struct<
    temperature: Float64! [unit=K],
    pressure: Float64!    [unit=Pa],

    composition: List<
        Struct<
            component_id: UInt32!,
            fraction: Float64!
        >!
    >!
>! [domain=ThermodynamicState, version=1, basis=molar]

result: Struct<
    solution: Struct<
        enthalpy: Float64! [unit=J/mol],
        density: Float64!  [unit=mol/m3]
    >?,

    diagnostics: Struct<
        status: Utf8!,
        iterations: UInt32!,
        residual_norm: Float64?
    >!
>! [domain=ThermodynamicResult, version=1]
```

The corresponding application operation becomes:

```text
evaluate_state(ThermodynamicState/v1) -> ThermodynamicResult/v1
```

rather than a loosely coordinated collection of operations over temperature columns, pressure columns, anonymous component arrays, output arrays, and separate status tables.

This provides several useful boundaries:

**Input interpretation:** one contract specifies units, basis, child fields, and null semantics.

**Output interpretation:** the solution is absent or contains all required outputs; diagnostics remain available independently.

**Transformation:** an operation that changes composition explicitly produces another state rather than quietly changing one of several related arrays.

**Interoperability:** a consumer receives a structured schema instead of reconstructing domain objects from undocumented column-name prefixes.

But this schema still does not establish that fractions sum to one, identifiers are unique, temperatures are physically acceptable, or `status="success"` agrees with the presence of `solution`. Those remain explicit predicates.

DataFusion can construct and access nested values using facilities such as `named_struct` and `get_field`/bracket notation. **Constructing a struct is not itself a domain-validation operation**: an application-specific constructor must supply that additional meaning. 

## 3. Which validations disappear, move earlier, or remain?

This is the most important distinction for assessing the value proposition.

| Validation category | Examples | Appropriate enforcement point |
|---|---|---|
| **Physical/layout** | Child lengths, compatible array datatypes, validity-buffer consistency | Checked Arrow constructors and trusted array implementations. |
| **Structural schema** | Required children, nested types, fixed dimensions, aggregate optionality | Canonical schemas, schema-aware planning, and checked construction. |
| **Static domain compatibility** | Units, basis, domain version, semantic field roles | Your full-field contract checker, UDF planning hooks, and analyzer rules. |
| **Dynamic value constraints** | Finiteness, bounds, normalized fractions, tag/payload consistency | Vectorized validation at ingestion and after operations that can invalidate the predicate. |
| **Cross-row or cross-source constraints** | Uniqueness, referential integrity, join cardinality, component coverage | Explicit relational validation or datasource/application enforcement. |

For example, `StructArray::try_new` checks structural compatibility between fields and children, including length, datatype, and declared nullability requirements. It does not inspect a pair of numeric children and infer `lower <= upper`. 

Likewise, DataFusion’s table-constraint documentation distinguishes declared constraints from enforcement: primary-key, unique, foreign-key, and check constraints are not automatically enforced merely because they are present in provider metadata. Nesting does not change that responsibility. 

The opportunity is therefore substantial but specific:

> Replace repeated structural checks with representation guarantees; replace repeated compatibility checks with planning-time checks; retain runtime validation only where the answer genuinely depends on runtime values.

### Nullability becomes more expressive—but requires correct kernels

These are different contracts:

```text
Struct<a: Nullable<T>, b: Nullable<U>>
Nullable<Struct<a: T, b: U>>
```

The first permits independently absent children. The second represents an optional aggregate whose live values require both children.

Arrow maintains struct-parent validity separately from child validity. For a projected child, logical validity must incorporate the parent:

```text
effective_child_validity =
    parent_validity AND child_validity
```

For deeper nesting, all relevant ancestor masks matter. A null parent can mask physical child values. Consequently, directly accessing a raw child array is not equivalent to performing a null-aware field projection. Custom kernels must preserve those semantics. 

Also distinguish:

```text
null struct
struct containing null children
null list
empty list
list containing a null element
```

Treating these as explicit domain choices prevents another class of scattered “special case” checks.

## 4. The key DataFusion integration: operate on full fields, not just datatypes

A datatype such as `Float64` answers how values are represented. It does not answer whether they represent pressure, temperature, a residual, or a dimensionless fraction.

For your purpose, the useful unit of specification is closer to:

```text
Domain field =
    structural datatype
  + nullability
  + semantic metadata
  + domain identity/version
  + explicit compatibility rules
```

### Canonical schemas at the datasource boundary

Have ingestion adapters and `TableProvider::schema()` use the same canonical schema definitions. The provider supplies DataFusion’s table schema; its scan implementation must produce compatible physical results. Nested values fit inside that existing boundary—you do not need a new catalog abstraction merely to represent them. 

My recommendation is to normalize heterogeneous source representations **before presenting them as canonical domain values**. That is where unit conversion, field-name mapping, component alignment, and source-version adaptation belong.

### Planning-time domain checks in UDFs

DataFusion exposes:

```rust
fn return_field_from_args(
    &self,
    args: ReturnFieldArgs<'_>,
) -> Result<FieldRef>;
```

Unlike a return-type callback that sees only datatypes, this interface receives full input fields and can return a complete output field. `ReturnFieldArgs` also exposes constant argument values where available. This gives an application-defined function a place to reject incompatible domain inputs before execution and declare the precise nested result schema. 

For example, your implementation could enforce:

```text
evaluate_state:
    requires domain=ThermodynamicState
    accepts version=1
    requires basis=molar
    requires temperature unit=K
    requires pressure unit=Pa
    returns canonical ThermodynamicResult/v1 field
```

That checking policy is yours; DataFusion does not automatically interpret those metadata keys as physical laws.

The important result is that a large amount of compatibility checking can become **once-per-plan analysis**, rather than repeated checks in every batch or every downstream consumer.

### Whole-plan semantic policies

`ExprSchemable::to_field` derives an expression’s full field against an input schema. Custom `AnalyzerRule` implementations can inspect and transform logical plans. Together, these provide the building blocks for application-wide rules such as rejecting incompatible basis combinations or requiring explicit unit-conversion functions. 

This matters because checking only your custom functions leaves ordinary expressions as a possible bypass. A raw arithmetic expression over two `Float64` children does not necessarily pass through your domain checker.

A useful policy is:

```text
ordinary expression:
    permitted only where its semantic transfer rule is known

domain-changing expression:
    must use an explicit operation with an input/output contract
```

### Batch-level execution instead of repeated dynamic interpretation

At execution time, `ScalarFunctionArgs` carries arguments and field information. A typed implementation can establish the expected array representation at the batch boundary, then run the relevant array kernels without rediscovering field names and types for each row. 

The intended division is:

```text
planning: resolve meaning and compatibility
execution: process already-understood columnar values
```

This is a stronger simplification than merely replacing flat column names with dotted paths.

### A particularly important equality trap

Do not substitute an existing permissive datatype comparison for your domain-contract comparison.

Arrow’s `DataType::equals_datatype` deliberately ignores nested field names and metadata. DataFusion’s `DFSchema::has_equivalent_names_and_types` ignores nullability and metadata. Those methods serve their intended purposes, but neither establishes semantic domain equivalence. 

Define **directional contract compatibility** explicitly: required child names and structure, permitted physical coercions, nullability requirements, relevant semantic metadata, and supported domain versions. A harmless output alias should not change domain identity; changing a unit or basis should.

## 5. Carrying the structure through transformations is where the larger payoff appears

Simply adopting nested input schemas will not remove repeated validation if every transformation immediately flattens, modifies, and reconstructs the values without preserving their contracts.

I would give important operations explicit **invariant-preservation rules**. These would be application rules, not a claim that DataFusion already tracks arbitrary domain proofs.

| Transformation | Suggested contract treatment |
|---|---|
| Filter or reorder complete rows | Preserve predicates about each unchanged domain value; reconsider global ordering or coverage requirements. |
| Pass through a complete struct | Preserve its established value-level contract. |
| Extract one child | Preserve that child’s meaning, but do not attach the entire parent’s validation status to it. |
| Modify a child field | Invalidate predicates depending on that field; retain unrelated guarantees. |
| Filter elements within a composition list | Reconsider normalization and component-coverage predicates. |
| Join relations | Preserve unchanged values, but reconsider multiplicity, missing matches, and identity assumptions. |
| Aggregate or reconstruct a struct | Treat it as a new value unless the operation has a specific preservation rule. |

For the example state, filtering **rows** does not change the normalization of any retained composition. Filtering **components within a composition** generally does. The structural types alone cannot express that difference; the operation contracts can.

Similarly, selecting a complete “latest state” preserves the associations among its fields. Independently taking the maximum temperature, pressure, and composition-related values can manufacture a state that never existed. The desired operation is a row/value selection, not independent scalar aggregation.

DataFusion supports expanding list and struct columns through `unnest_columns` and related APIs. With `List<Struct<...>>`, that can expose element records for relational processing without first aligning separate lists. Nevertheless, unnesting changes cardinality, and null/empty-list behavior must remain part of the operation’s contract. 

### Validation can become dependency-driven

Suppose a domain contract declares:

```text
positive_pressure       depends on pressure
normalized_composition  depends on composition.fraction
unique_components       depends on composition.component_id
```

An operation that changes only pressure need not rerun component-uniqueness validation. An operation that permutes complete composition elements preserves pairing and normalization, but may violate a required canonical ordering.

This suggests a useful execution policy:

```text
trusted input
    -> establish invariants
    -> apply operations with known preservation rules
    -> revalidate only invalidated invariants
    -> validate again at untrusted boundaries
```

The correct objective is **not “validate once forever.”** It is “do not repeatedly validate facts that the intervening operations cannot have changed.”

Two safeguards are essential. First, a user-supplied `validated=true` metadata annotation is not trustworthy evidence. Second, mandatory validation must not be an unused expression that you merely expect a lazy query to execute. DataFusion DataFrames build logical plans rather than executing each construction step; validation that must cover all input rows belongs at an enforced ingestion, materialization, or execution boundary. 

## 6. Metadata and extension types help, but are not automatic semantic enforcement

Arrow extension types allow semantic identity and associated metadata to accompany a storage representation. Arrow’s extension-type API validates the relationship between datatype and extension metadata; that is not the same as validating every stored value against arbitrary domain predicates. 

For this architecture, three distinctions matter.

**Metadata describes a field, not each individual row.** A `unit=K` annotation is appropriate for a homogeneous temperature field. Where units or basis genuinely vary by row, normalize them or represent the varying identifier as data. Do not hide row-dependent meaning in one column-wide annotation.

**Annotation is not conversion.** DataFusion’s `with_metadata` attaches or overwrites field metadata. Labeling a number as Kelvin does not convert Celsius values or validate their origin. Semantic conversion must be an explicit, trusted operation. 

**Metadata preservation and correctness both need testing.** Dropping a unit annotation is bad; retaining an obsolete unit after a transformation is also bad. DataFusion has added field-aware casting and improved metadata handling, but that does not establish arbitrary domain-preservation rules for every expression. 

I would therefore use metadata as a **portable declaration of the contract**, while keeping authoritative interpretation, compatibility rules, and validation evidence in the application’s contract system.

### Interoperability and evolution

Arrow’s C Data Interface provides a standardized way to exchange nested schemas and arrays across language boundaries. This supports a Rust/Python architecture in which domain structure can travel with columnar values instead of being reconstructed through per-row objects. It does not guarantee that every consuming library understands your application metadata. 

For evolution, use explicit adapters such as:

```text
source schema/version
    -> structural adaptation
    -> semantic conversion
    -> canonical domain version
```

DataFusion has documented struct-coercion and field-matching behavior, but physical schema compatibility is not semantic compatibility. Adding a nullable child, renaming a field, changing a unit, and changing component ordering are not interchangeable migrations. 

A useful cross-language identity mechanism is a **canonical semantic schema fingerprint**, not raw serialized IPC-schema bytes. Include the contract-relevant structure and metadata; deliberately exclude irrelevant presentation details.

## 7. Execution value: credible opportunities, not automatic speedups

### Nested Arrow values remain columnar

An Arrow struct is not an array of heap-allocated row objects. Its children remain separate arrays. Consequently, wrapping existing compatible child arrays in a struct can retain their buffers rather than interleaving or serializing the values. This preserves the possibility of efficient columnar processing. 

However, an equivalent flat Arrow representation already has columnar buffers. **Nesting alone does not produce an inherent cache-locality or vectorization advantage over those same flat columns.**

The plausible performance benefits come from changes in the surrounding system: avoiding JSON/object conversion, repeated field discovery, list alignment, reconstruction joins for owned values, or repeated computation of correlated outputs. These are hypotheses to measure against your current implementation, not speedups guaranteed by the datatype.

### Struct-valued operations can simplify correlated computations

A domain function returning:

```text
Struct<solution, diagnostics, status>
```

can make a shared computation explicit instead of exposing separate functions that each reconstruct inputs or invoke the same underlying computation.

But a struct, alias, or CTE is not an exactly-once execution contract. For expensive solver calls, define the intended physical execution or materialization boundary and verify invocation counts. A result that is reused as data is different from a logical expression referenced several times.

### Preserve optimizer visibility

Do not place every elementary operation inside an opaque custom UDF. Prefer ordinary DataFusion expressions where their semantics are sufficient, and use domain UDFs where they provide a meaningful contract or specialized kernel.

Current UDF interfaces include `simplify` and `struct_field_mapping`; the latter can describe how output struct fields map to input arguments for optimizer reasoning such as ordering propagation. These are useful integration points, not a guarantee that arbitrary custom functions receive every optimization available to built-in field access. 

### Nested Parquet access is a concrete qualification

Field-level I/O savings must be verified for the actual datasource, expression shape, and release. Apache’s active struct-access work distinguishes ordinary `get_field` access from narrowed schemas expressed through casts, and tracks remaining projection, filter, statistics, and schema-evolution work. 

A related correctness fix for struct filtering combined with schema adaptation was merged on **August 20, 2026**, with a backport to branch 55. That is a reason to pin versions and test nested access paths—not a reason to reject nested schemas. 

For custom providers, the ordinary `TableProvider::scan` projection is a list of top-level column indices. Do not assume that declaring a nested schema automatically equips a custom scan with a nested leaf-projection protocol. 

## 8. The architecture I would recommend

I would build a **small canonical domain-contract layer that generates DataFusion-facing artifacts**, rather than scatter domain knowledge across providers, UDFs, validators, and serialization code.

```text
Canonical domain contract
    |
    +-- Arrow Field / Schema definitions
    +-- domain identity and compatibility rules
    +-- source/version adapters
    +-- function input/output contracts
    +-- runtime invariant definitions
    +-- invariant-preservation rules
    +-- introspection and conformance tests
                |
                v
      DataFusion planning and execution
```

The registry should be the source for field definitions, units, nullability, versioning, and predicate dependencies. Providers expose those schemas; UDFs validate against those same definitions; runtime validators execute predicates from the same contract.

The practical simplification is substantial: instead of maintaining several partially overlapping descriptions of “what a state is,” you maintain one definition and several implementations derived from it.

### Use a hybrid relational model

I would generally retain frequently joined, filtered, partitioned, or independently meaningful keys as top-level columns, and use nested values for cohesive payloads:

```text
entity_id | scenario_id | timestamp | state | result
```

Use a struct where fields have a shared lifecycle and are naturally consumed together. Use `List<Struct>` for owned repeated records. Keep independently identified, shared, independently updated entities in separate relations.

Likewise, prefer a map for a genuinely dynamic key set—not as a substitute for a known schema. A fixed-size list expresses a uniform extent, but does not establish which physical quantity or component occupies each position. Those semantic distinctions still belong in the contract.

The aim is not to make every table one enormous nested object. It is to put **the right boundary around each coherent value**.

## 9. How to establish the value in your implementation

The highest-value initial candidates are usually **parallel arrays/lists, optional multi-field aggregates, and correlated operation outputs**. They offer clear structural changes whose benefits can be demonstrated without redesigning the entire system.

For a pilot, measure two different outcomes.

**Design and correctness:** count alignment checks, field-discovery logic, reassembly code, duplicated domain definitions, and compatibility errors that can now be caught during planning. Test absent parents, nullable children, empty lists, duplicate identifiers, schema evolution, incompatible units/basis, and transformations that should invalidate established predicates.

**Execution:** compare the flat and nested versions using both whole-value and leaf-only workloads. Inspect plans and measure bytes read, allocations, peak memory, validation work, and expensive-function invocation counts. Test pushdown enabled versus disabled for result equivalence, and test Rust/Python plus IPC/Parquet round trips for both values and semantic fields.

The strongest justification is likely to be **a simpler correctness model and less duplicated semantic machinery**, with performance improvements where that machinery currently consumes execution time.

### Bottom line

Your intuition is strongest when interpreted as:

> **Move from loosely coordinated columns to typed domain values, and from repeated defensive checks to explicit construction, compatibility, and transformation contracts.**

Arrow structs and lists can directly eliminate structural inconsistencies. DataFusion’s full-field planning interfaces can move domain-compatibility errors ahead of execution. A thin application contract layer can then track which runtime guarantees remain valid as values move through the plan.

That combination can materially simplify representation, operations, transformations, and execution. **The leverage comes from preserving meaning through the pipeline—not merely from making the schema more nested.**