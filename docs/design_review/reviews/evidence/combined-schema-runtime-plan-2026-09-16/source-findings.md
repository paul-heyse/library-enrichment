# Exact-source findings for Plan 17

Inspected **2026-09-16**. Paths below are relative to the named source root, not moving upstream HEAD.
Source hashes for the principal files are retained in [receipt.json](receipt.json). The
[original review evidence](../schema-engineering-typed-values-2026-09-16/) contains index lookups;
the additions here include exact source and the executed probe.

## Native API qualifications

Registry root: `/home/paul/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f`.
Delta root: `/home/paul/.cargo/git/checkouts/delta-rs-dcb716bfdc369320/58f07cd`.
Kernel root: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f`.
The full Git revisions are in the receipt and README.

| Canonical API / source location | Observed contract | Planning implication |
|---|---|---|
| `datafusion_common::types::extension::DFExtensionType`; `datafusion-common-55.1.0/src/types/extension.rs` | Methods expose storage type, serialized metadata and array formatter. The documentation names pretty-printing as the currently customizable operation | Registry adoption is useful but does not replace semantic rules or value validation |
| `datafusion_expr::udf::ScalarUDFImpl`; `datafusion-expr-55.1.0/src/udf.rs:647–699` | `return_field_from_args` can derive a Field from argument fields and known scalar arguments | Adopt full-field planning for the five product UDFs |
| `datafusion_expr::udf::StructFieldMapping`; same file `:41–58,1026–1044` | Maps each output field to an equivalent input argument for optimizer ordering through projections | URL components and parsed PEP 440 fields are not equal to the original whole input; leave this hook absent there |
| `datafusion_expr::expr_schema`; `datafusion-expr-55.1.0/src/expr_schema.rs:1321` onward | Cast field metadata behavior is tested explicitly; source extension metadata is removed when the target lacks it | Validate before erasure and use authoritative derived field contracts, not hoped-for propagation |
| `datafusion_expr_common::columnar_value::ColumnarValue::cast_to`; `datafusion-expr-common-55.1.0/src/columnar_value.rs:322` onward | Nested casts dispatch to DataFusion nested-struct casting | The project's native cast follows DataFusion's name-based path, not a blanket Arrow-cast limitation |
| `datafusion_physical_expr::expressions::cast::CastExpr`; `datafusion-physical-expr-55.1.0/src/expressions/cast.rs:331` | Execution delegates to `ColumnarValue::cast_to` | Probe both reorder and missing-child cases; missing-field NULL insertion is observable |
| `datafusion_common::functional_dependencies::Constraint`; `datafusion-common-55.1.0/src/functional_dependencies.rs:30–35` | Native declarations are PrimaryKey and Unique; no foreign-key declaration in this enum | Use native joins for reference admission; do not advertise native foreign-key enforcement |
| Delta internal `validation_predicates`; `crates/core/src/delta_datafusion/data_validation.rs:269–334` | Scalar source nullability can be trusted; CHECK/Invariants/GeneratedColumns processing depends on active table features | Validate candidate values and actual protocol features, not merely field/property labels |
| Delta internal requiredness helpers; same file `:340–430` | Scalar path traversal handles structs without guarding nullable parents. List/map element checks exist; paths crossing elements and FixedSizeList cases are skipped | Generate parent-aware and per-element native conditions from the actual contract; qualify used physical layouts |
| `deltalake_core::operations::constraints::ConstraintBuilder`; `crates/core/src/operations/constraints.rs:220–240` | Complete operation updates writer protocol/features and commits constraint actions | Use `DeltaTable::add_constraint`; raw CreateBuilder configuration is not equivalent |
| Delta file stats; `crates/core/src/writer/stats.rs:136–197` | Explicit names are parsed as multipart identifiers then matched to leaf `col.name()`; default budget counts distinct top-level non-partition fields | Explains the missing explicit nested stats in the probe and corrects the review's nested-leaf-budget assumption |
| Delta stats regression; same file, `test_nested_fields_do_not_consume_stats_budget` | Test names the top-level budgeting intent | Nested default stats and exposed DataFusion pruning statistics are separate questions |
| `arrow_json::writer::encoder::EncoderFactory`; `arrow-json-59.3.0/src/writer/encoder.rs:145` onward | Factory receives the field, array and encoder options; can supply a custom NullableEncoder or delegate | Implement a finite generated value codec for ID/domain/digest/precision formatting; no semantic policy in the serializer |

## Application opportunities grounded in current consumers

All application paths are relative to the workspace root. These are source observations, not
new behavioral test passes.

| Source | Current ownership / gap | Plan decision |
|---|---|---|
| `crates/enrichment-core/src/evidence/arrow_model/{encode,checks,decode}.rs` | Hand-maintained tagged shapes, rule tables and decoder allowlists | One enum/record declaration generates every variant consumer |
| `crates/enrichment-core/src/native_key.rs:234–268` | Canonical collection handling dispatches on specific identity/field names; result concatenates a prefix and hex digest | Move collection/domain meaning to the shared descriptor and keep IDs binary until declared wire rendering |
| `crates/enrichment-core/src/native_url.rs:17–98`; `native_version.rs` | Parser UDF outputs are transformed fields and return only DataType | Full-field semantics without unsound argument-equivalence hooks |
| `crates/enrichment-store/src/preparation.rs:466–495` | Compatibility filters semantic metadata mainly to role/function and permits native coercion | Compare all authoritative extension/field contract metadata; distinguish admitted and derived nullable fields |
| `crates/enrichment-store/src/provider.rs` | Projection aliases/casts carry rewritten search-candidate roles | Keep presentation role separate from stable identity domain and derived semantic validation |
| `crates/enrichment-store/src/admission.rs:261–285` | Worker/document artifact references include producer-binding joins and variant guards; some neighboring rules enforce profile/log semantics instead | Generate complete scoped references; preserve other native predicates as non-reference rules |
| `crates/enrichment-store/src/coverage.rs`, `query.rs`, `python_normalize.rs` | Correlated values travel in separately collected/aggregated arrays or vectors | Record-valued native aggregation/decode with explicit ordinal and NULL/empty semantics |
| `crates/enrichment-store/src/native_delta.rs` | Semantic/read mapping and contract registry already exist; datatype-only projection and nullable adaptation still need completion | Derive all mappings from one declaration and qualify exact fields/conditional requiredness |
| `crates/enrichment-core/src/wire/evidence.rs`; `crates/enrichment-store/src/projection/render.rs` | Typed evidence loses structure at the DTO/render boundary | Generated typed wire projection and field-aware batch encoding |
| `python/enrichment_worker/__main__.py:48–76` | Function/overload observations retain rendered signatures, bases are strings | Extract producer-native parameter/return facts, preserve rendered evidence independently |
| `crates/enrichment-core/src/producer/rustdoc/facts.rs:183` and `:367–375` | ItemFact lacks stability/callable structure; Signature carries ordinal/id/parent/text | Use the qualified format-61 model for complete producer facts, then native normalization |
| `crates/enrichment-store/src/comparison.rs:44–50` | API comparison includes signature text inside a native struct | Existing compare/inspect provides a concrete consumer for typed parameter/return changes |
| `python/enrichment_mcp/server.py` | Existing 4.0.3 registration/strict-schema boundary is the integration point | Generate bindings there; keep domain policy/result construction in Rust |

The installed Griffe model at `.venv/lib/python3.14/site-packages/griffe/_internal/models.py`
exposes `Function.parameters` and `Function.returns` (`:2567–2600`) with structured Parameter
annotation/default/kind fields. The locally inspected `rustdoc-types-0.61.0/src/lib.rs` supplies
Function generics and FunctionSignature inputs/output (`:1209–1213,1655–1662`). These are sources for
producer-specific facts, not evidence that the current worker emits them or that Rust and Python
types share an equivalence relation. Rustdoc 0.61 is a proposed target, not an adopted dependency.

FastMCP's pinned skill `content/api/fastmcp.tools.base.md` describes `ToolResult` structured_content
and Tool output_schema. The canonical definition is `fastmcp.tools.base.ToolResult`; the importable
`fastmcp.tools.ToolResult` is suitable for the existing adapter. The plan uses this native boundary
without adding a second provider/middleware stack. Arrow/Delta and MCP round-trip qualification is
still required after implementation.
