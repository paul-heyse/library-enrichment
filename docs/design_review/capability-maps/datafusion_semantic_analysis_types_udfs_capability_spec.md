# DataFusion semantic analysis, semantic types and optimizer-aware functions

**Scope:** dense capability and implementation reference for `AnalyzerRule`, Arrow semantic fields/extension types, and scalar/aggregate/window UDFs. Recommendations explain opportunities, conditions and tradeoffs; application architecture and acceptance policy remain codebase decisions. Brief examples are illustrative unless explicitly identified as source observations.

**Baseline and evidence:** DataFusion **55.1.0**, Arrow Rust **59.3.0**, matching the inspected `pse-arrow` manifest and lockfile on 2026-09-14. Library contracts below are **Interface-checked** against installed versioned source, with Context7 documentation used for discovery. Repository observations describe inspected source, not an executed qualification. The attachment's Arrow 59.2.x anchor is superseded here by the live repository pin. The related [logical-planning reference](</home/paul/Documents/Codex/2026-09-14/i-want-to-communicate-to-an/outputs/datafusion_logical_planning_capability_spec.md>) covers the full operator inventory and general validation strategies.

## 1. How the three capabilities combine

| Mechanism | What DataFusion supplies | Domain contribution and potential value |
|---|---|---|
| Logical plans and expressions | Typed relational structure, schemas, traversal, analysis and optimization. | A common computation vocabulary can reduce duplicate frontends and make transformations inspectable. |
| `AnalyzerRule` | An ordered, fallible plan-to-plan transformation before logical optimization. | Model-specific resolution, compatibility checks and explicit canonicalization can reveal mistakes before scanning data or assembling a solver problem. |
| Arrow fields and extension types | Storage shape, nullability, metadata and extension reconstruction. | Machine-readable quantity, domain and identity declarations can connect otherwise indistinguishable numerical columns to their meaning. |
| Field-aware UDFs | Input/output field derivation plus executable scalar, aggregate or window implementations. | Specialized operations can validate their semantic interface and produce correctly described results while retaining a native query representation. |
| Optimizer hooks | Simplification, predicate preimages, interval reasoning, ordering and placement interfaces. | Sound function-specific properties can expose optimizations without expanding every numerical implementation into primitive expressions. |
| Application semantic/solver layer | Integration points, not built-in physical science or nonlinear solution. | Quantity algebra, coordinate mappings, residual obligations, derivatives and numerical backend contracts connect the query engine to process-model meaning. |

These mechanisms are complementary, not a ranking from “good” to “bad” representations. Native algebra offers broad optimizer visibility, a UDF can encapsulate a useful numerical primitive, and a custom relational node can express multi-row semantics. Metadata contributes meaning only where consumers interpret it. A solver-oriented math IR can retain derivatives, implicit systems and domain operators that ordinary DataFusion `Expr` does not represent.

In `pse-arrow`, the relevant architecture is the native-plan target alongside the retained quantity/MathIR/kernel boundaries: [blueprint §3.3.3, D5–D6, D9–D11](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>) and [Plan 05](</home/paul/pse-arrow/docs/plans/05-native-logical-plan-hard-pivot.md>). This reference describes library mechanisms that can serve that design; it is not a replacement execution plan.

## 2. AnalyzerRule: semantic processing over plans

### Interface, lifecycle and registration

`AnalyzerRule` exposes `analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>` and `name() -> &str`. The trait requires `Debug`; configured rule collections hold `Arc<dyn AnalyzerRule + Send + Sync>`. The rule receives a complete plan and configuration, rather than a batch or automatic catalog callback. Additional domain registries/context can be retained in the rule object.

`Analyzer::execute_and_check` runs function rewrites and analyzer rules, reports errors with rule context, invokes an observer after each rule and checks implemented plan invariants. “Execute” here means executing analyzer passes, not evaluating data operators. In 55.1.0 the default rule list is `ResolveGroupingFunction` followed by `TypeCoercion`; configured `FunctionRewrite`s run before that list. `SessionState::add_analyzer_rule` and builder `with_analyzer_rule` append; `with_analyzer_rules`/`Analyzer::with_rules` select a complete list. Replacing the list changes native analysis as well as custom behavior.

Unlike an optimizer rule, an analyzer may resolve an incomplete representation into a valid one, including insertion of coercions or domain conversions. That freedom does not establish that a particular rewrite matches the application language: the language/registry still supplies the intended interpretation. The pipeline is ordered, and a single analyzer invocation runs its list sequentially rather than searching for a fixed point.

**Ordering tradeoff:** a pre-coercion rule can inspect original storage/metadata and prevent an inappropriate coercion; a post-coercion rule can rely on normalized physical types. A late rule that introduces new expressions may need subsequent coercion/reconstruction before execution. Because preparation can be repeated by different APIs, deterministic and repeat-safe transformations help avoid duplicated conversions and context-sensitive surprises. These are design properties of the custom rule, not automatic guarantees of the trait. [Analyzer source](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/analyzer/mod.rs.html), [session configuration](https://docs.rs/datafusion/55.1.0/src/datafusion/execution/session_state.rs.html).

### Information available and implementation shape

| Input to reasoning | Possible processing | Boundary |
|---|---|---|
| Plan topology and node expressions | Inspect/rewrite scans, joins, aggregates, windows, nested queries, writes and extensions. | Ordinary child traversal can omit embedded subqueries; scope-aware traversal matters for correlated references. |
| Input/output fields | Derive complete output contracts, detect incompatible quantity/shape/reference declarations, check field identity. | Fields describe columns; they do not expose every row's quantity ID or numerical state. |
| Literal arguments and typed parameters | Resolve constant selectors, package IDs, conversion coefficients or expected output shape. | Unbound or row-varying arguments may leave obligations for later phases. |
| Retained registry/source context | Resolve quantity kinds, phase/component domains, kernel signatures and named invariants. | A stable reference does not by itself prove that referenced data is admitted or unchanged. |
| Custom node/function contracts | Apply declared domain rules and diagnose unsupported operations. | Opaque bodies cannot be inspected automatically; their declarations and implementation evidence remain separate. |

`TreeNode` transformations, expression mapping and subquery-aware helpers support recursive passes; checked reconstruction and schema derivation help keep changes consistent. A structured error containing rule, expression, input field, expected/actual contract and source identifier can explain a problem closer to its origin than a numerical failure downstream. Focused rules can simplify diagnostics and testing; excessive fragmentation can multiply traversal cost and make dependencies harder to follow.

Analysis over schemas, literals and immutable registries is often inexpensive compared with data execution. The synchronous trait does not prohibit expensive work, and constant folding can later invoke numerical functions during planning. Bulk property evaluation or remote lookup inside a rule can therefore make preparation expensive or nondeterministic; resolving reusable context before constructing the rule is one possible separation.

### Process-semantic applications and their limits

| Concern | What can be established from sufficient static context | What may still require data or numerical work |
|---|---|---|
| Dimensions, kinds, bases and affine quantities | Admissible operation signatures and conversion edges; matching enthalpy/flow basis; point/difference rules. | Row-dependent conversion inputs, positive molecular weight, finite values and valid denominators. |
| Component/phase/domain alignment | Compatible named sets, ordered axes, state-definition families and declared mappings. | Membership/coverage, composition normalization and phase feasibility for actual states. |
| Property packages and reference states | Selected method availability, parameter/reference identity and compatible input/output contracts. | Correlation validity, EOS convergence and physical adequacy. |
| Time/space/scenario alignment | Required coordinates and explicit interpolation/lag/reindex operations. | Actual grid completeness, duplicate coordinates and interpolation accuracy. |
| Solver preparation | Declared variable/residual types, role compatibility and available derivative/backend bindings. | Binding coverage stored as rows, consistent equation system, finite evaluations and convergence. |

A registry-driven semantic program can separate facts already established by construction from residual checks over data. This can avoid repeating full scans while still explaining which facts remain unresolved. Centralizing the domain rules can reduce inconsistent checks across UI, query and solver paths; ingestion and output boundaries still need the parts of that program relevant to values and ownership.

**Brief applications:** (1) A heater energy-balance expression can resolve whether specific enthalpy and flow use compatible bases before residual assembly. (2) A species-indexed vector and a cell-indexed vector of equal length can be distinguished by shape/domain identity. (3) A property call whose temperature argument carries a pressure contract can fail field-level validation even if both use `Float64`. These examples depend on application-supplied quantity rules, not built-in DataFusion dimensional algebra.

## 3. Semantic fields and extension types

### Representation layers

An Arrow `Field` contains a name, physical `DataType`, nullability and a `HashMap<String, String>` of metadata. `DFSchema` adds relation qualification and functional dependencies around Arrow schema information. `ExprSchemable::{get_type, nullable, metadata, to_field}` derives expression information; `to_field` returns the optional qualifier and complete field. This enables field-aware reasoning without evaluating rows.

| Representation | Capabilities and value | Limits and tradeoffs |
|---|---|---|
| Plain field metadata | Namespaced semantic IDs, units, roles and provenance can accompany normal Arrow arrays without changing their scalar storage. | Strings are declarations, not enforced types; keys can be stale, conflicting or dropped by transformations. |
| Arrow `ExtensionType` | Logical identity over a standard storage type; typed metadata serialization/deserialization, storage compatibility checks and reconstruction. | Does not introduce automatic arithmetic, equality, unit conversion or value validation. An unaware consumer may operate only on storage. |
| DataFusion `DFExtensionType` and registry | Dynamic factory lookup from extension name/storage/metadata; resolved storage type and custom array formatting. | In 55.1.0 the trait's behavioral customization is formatting, not operator dispatch or a scientific type checker. |
| Typed registry relations | Rich reusable definitions, versioned identities, conversion graphs, component domains and method contracts. | Resolving a reference and proving a relationship may require context beyond a field. |
| Per-row semantic payload | Structs or companion columns can represent heterogeneous quantities, domains or outcomes. | Planning sees the common column schema; row-specific compatibility usually remains an execution/admission obligation. |

Arrow uses `ARROW:extension:name` and optional `ARROW:extension:metadata`. Its `ExtensionType` trait supplies `NAME`, associated `Metadata`, `metadata`, `serialize_metadata`, `deserialize_metadata`, `supports_data_type`, `try_new`, and provided `validate`/`try_new_from_field_metadata`. Field APIs include fallible extension attachment/reconstruction (`try_with_extension_type`, `try_extension_type`) and convenience variants that can panic on invalid input. Reconstruction checks metadata/storage compatibility; validating values or references inside arrays is a different operation.

DataFusion uses `ExtensionTypeRegistration::new_arc(name, factory)` with a factory `(storage_type, optional_metadata) -> Result<DFExtensionTypeRef>`. `ExtensionTypeRegistry` supports lookup/add/remove; `MemoryExtensionTypeRegistry` can be empty or include canonical Arrow registrations. A resolved `DFExtensionType` implements `storage_type`, `serialize_metadata` and optional `create_array_formatter`. This supports meaningful diagnostics for IDs, quantities and bounds without changing execution kernels. Arrow's typed trait and DataFusion's dynamic registry are distinct mechanisms. [Arrow extension source](https://docs.rs/arrow-schema/59.3.0/src/arrow_schema/extension/mod.rs.html), [Field](https://docs.rs/arrow-schema/59.3.0/arrow_schema/struct.Field.html), [DataFusion extensions](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/types/extension.rs.html), [registry](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/registry.rs.html).

### Column-wide versus row-dependent meaning

A homogeneous numerical column can name one complete quantity contract in field metadata. A heterogeneous column cannot truthfully claim that every row has that same contract: per-row IDs or a partition into homogeneous relations makes the distinction explicit. Nested list/struct child fields can carry their own metadata, but they describe the child column uniformly, not a separate schema for each element.

In `pse-arrow`, this distinction already has concrete representations: the homogeneous `pse.semantic.quantity_type` key and the `pse.quantity_value` extension with `value`, `quantity_type_id` and `unit_id` children. The latter does not make row-varying unit/basis validation a planning-only problem. Parent validity, child validity, list offsets, dictionary values, domain IDs and cross-column relationships can all affect the actual semantic interpretation. Structural Arrow validity and extension reconstruction address only part of that problem. [Blueprint §4.3–§4.6](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>), [extension implementations](</home/paul/pse-arrow/crates/pse-relations/src/ext/mod.rs>).

**Brief applications:** (1) Homogeneous canonical molar-flow output can carry one quantity-type ID. (2) A mixed case-input value relation can retain row-specific types through `pse.quantity_value`. (3) `pse.ordinal_ref` can describe a target relation in metadata, while actual ordinal bounds still depend on the bound artifact.

## 4. Meaning beyond storage: quantity, domain and model context

| Semantic dimension | Why it matters in complex computation |
|---|---|
| Physical dimension | Detects incompatible dimensional operations; alone it cannot distinguish all physical meanings. |
| Quantity kind | Separates dimensionless fractions, efficiencies and probabilities, or physically distinct quantities sharing dimensions. |
| Canonical unit and scale kind | Separates representation conversion from algebra; absolute points and differences have different affine behavior. |
| Basis and reference state | Determines whether mass/molar/volumetric properties combine and whether reference-dependent values can be compared. |
| Shape and domain identity | Distinguishes species, phases, cells and time coordinates independently of vector length; binds numerical positions to meaning. |
| Subject and material context | Identifies whether a value pertains to a phase, species, stream or mixture. |
| Package/parameter/method identity | Connects results to the selected constitutive model and its validity/derivative capabilities. |
| State definition and coordinate alignment | Identifies independent variables and intended temporal/spatial/scenario correspondence. |
| Bounds, missingness and solver role | Distinguishes unknowns, parameters, failures, unbounded limits and absent data; supplies numerical preparation context. |
| Version and provenance | Makes changes in interpretation distinguishable from unchanged numerical storage. |

These attributes need not all be metadata keys. Stable field-level references to typed definitions can avoid repeated, conflicting descriptions; explicit payloads are useful when attributes vary per row. A registry adds lookup and lifecycle complexity, while embedding full descriptions increases schema size and opportunities for drift.

Canonical units can reduce repeated conversions and simplify comparisons and solver packaging. They do not eliminate basis/reference transformations, guarantee reproducible floating-point results or justify arbitrary reassociation. A conversion such as `Int32 -> Float64` changes storage; `kg/s -> mol/s` depends on molecular weight and material context. Even a unit-only affine conversion distinguishes point offsets from difference scales. In `pse-arrow`, those semantics belong to the existing quantity/conversion definitions, not a newly invented `process.*` metadata vocabulary.

**Current source context:** `QuantityTypeKey` records kind, optional basis/reference state, scale kind, ordered shape and optional subject kind; `QuantityType` adds canonical unit and nominal magnitude. `OpRequest` covers arithmetic, affine expressions, gathers, reductions, broadcasts, conditionals, conversions, kernel calls, derivatives and integrals. These provide an existing semantic implementation boundary that analyzer/UDF adapters can call rather than duplicate. [Quantity types](</home/paul/pse-arrow/crates/pse-quantity/src/quantity_type.rs>), [operation inference](</home/paul/pse-arrow/crates/pse-quantity/src/infer.rs>), [unit conversions](</home/paul/pse-arrow/crates/pse-quantity/src/unit.rs>); architectural interpretation: [blueprint §6.2 and §8](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>).

**Brief applications:** temperature subtraction can derive a difference contract; a mass/molar enthalpy conversion can retain the molecular-weight dependency; component reindexing can retain the exact species mapping instead of relying on matching list lengths.

## 5. Metadata propagation and semantic canonicalization

### Actual 55.1.0 expression behavior

| Expression path | Field behavior relevant to semantic metadata |
|---|---|
| Column / negative expression | Uses the referenced/underlying field; domain-validity of negation is not implied. |
| Literal | Constructs a field from scalar storage and null state, with optional literal metadata. |
| Alias | Combines underlying metadata with alias metadata; alias values override duplicate keys. This can change declarations without changing values. |
| Binary expression | Derives physical result type/nullability and constructs a fresh field; arbitrary operand quantity metadata is not automatically derived. |
| CASE and several predicate expressions | Derive physical type/nullability through their paths; generic field construction does not infer a domain output contract. |
| Scalar / aggregate / window UDF | Uses the corresponding field-aware function hook after argument compatibility processing. |
| Cast / TryCast | Target-field-aware: type-only casts propagate non-extension source metadata while stripping extension identity; explicit target-field casts retain target metadata. TRY_CAST is nullable. Neither behavior establishes a physical-unit conversion. |
| Nested results / operator schema composition | Depends on expression, child fields and operator reconstruction; a valid outer schema is not evidence that all nested semantic relationships survived. |

This makes “preserve, derive, replace, intentionally drop, or reject” useful descriptions of application propagation behavior rather than universal engine rules. Copying a left operand's metadata onto every arithmetic result would preserve bytes but mislabel meanings such as energy flow. Conversely, dropping every annotation would make later domain checks impossible. Source-context-aware derivation can resolve the distinction.

Canonicalization can resolve aliases, normalize units/domains, insert conversion operations and produce consistent semantic expressions. It can improve comparison and reuse when the normalization respects NULL behavior, errors, affine rules and numerical evaluation order. It does not turn arbitrary equivalent expressions into identical plans or produce a canonical semantic fingerprint automatically. Arrow metadata maps, EXPLAIN strings and protobuf encodings are not substitutes for an application-defined identity contract.

Semantic declaration lifecycle matters independently of transport. Namespaced keys and a defined metadata version can make interpretation unambiguous; closed parsing can expose misspellings and unsupported fields early, while permissive parsing trades that detection for compatibility. A reference to a quantity or package definition needs the namespace/snapshot that gives the ID meaning. Changes to canonical units, reference states, parameter sets or output semantics can invalidate reuse even when the Arrow storage and function name stay unchanged. Explicit interpretation/version identities can support reproducibility without making every descriptive annotation part of a numerical cache key. The appropriate identity boundary depends on what the cache promises to reuse.

Interchange adds another boundary: schema-aware IPC, Parquet and FFI paths can carry metadata, but consumer registration, writer options, nesting and format adapters affect preservation. A raw top-level array does not carry its enclosing field metadata by itself. Paired field/schema transport and checks of the actual round trip are useful when semantic identity must survive another process or language. [Expression field derivation](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/expr_schema.rs.html).

**Brief applications:** a pressure comparison produces Boolean meaning, not pressure units; `enthalpy * molar_flow` needs a derived energy-flow contract; a formatted semantic ID can remain readable through the shared PSE formatter while equality and identity still follow their separate contracts.

## 6. Scalar UDFs: complete interface and execution contract

`ScalarUDF` holds an `Arc`-backed `ScalarUDFImpl`; `call(Vec<Expr>)` creates a logical function expression. The implementation requires `Debug + DynEq + DynHash + Send + Sync + Any`, so identity/equality and thread-safe retained state are part of integration, not just the numerical body. Registry lookup binds names; direct programmatic calls can carry a function object without looking it up by name. Serialization/rebinding is a separate concern.

### Core and field-aware hooks

| API | Contract, default and implementation significance |
|---|---|
| `name`, `aliases`, `schema_name` | Canonical invocation name, alternate names and generated expression/column naming. Names support discovery but do not uniquely identify parameterized implementations. |
| `signature` | Physical argument pattern, volatility and optional parameter names; named arguments can make multi-input APIs easier to inspect. |
| `return_type` | Required type-only method, after coercion to the declared signature. The standard field-aware route can bypass it when overridden; an erroring fallback is possible without a panic. |
| `return_field_from_args` | Receives `ReturnFieldArgs { arg_fields, scalar_arguments }`; can validate input metadata, derive nested output fields and nullability, and use available constant arguments. Default delegates to `return_type` and creates a nullable field without domain metadata. |
| `invoke_with_args` | Receives evaluated `ColumnarValue`s, input fields, `number_rows`, planned return field and runtime configuration; returns `ColumnarValue` or an error. |
| `is_strict` | Default false; true promises any NULL argument produces NULL. It does not itself implement null handling or mean every non-null input succeeds. |
| `with_updated_config` | Optional replacement instance when configuration changes; useful especially when output type depends on configuration. Default None. |
| `documentation` | Optional machine-readable docs for discovery and generated reference material. |
| Deprecated methods | Scalar `display_name` is unused/deprecated; scalar `is_nullable` is deprecated in favor of field-aware output derivation. |

`return_field_from_args` sees fields and known constants, **not arbitrary row data or the full plan**. In the checked logical path, literal expressions populate `scalar_arguments`; non-literal values may be unavailable until simplification/binding. A value-dependent return shape therefore needs enough planning-time information to remain stable for that logical input. The top-level returned field name is not the primary expression naming mechanism; nested struct child names remain meaningful. The output cannot vary its Arrow schema independently for each row.

A function can reject a pressure field in a temperature position despite equal storage, but that works only if upstream fields carry trustworthy semantics. Row-varying quantities, composition normalization and domain membership remain runtime or relational obligations. A UDF failure normally fails evaluation; producing a per-row outcome is an explicit alternate output design, not implicit NULL conversion.

At execution, scalar arguments can avoid broadcast allocations; arrays support batch evaluation, and a zero-argument function uses `number_rows` to determine cardinality. `ColumnarValue::values_to_arrays` simplifies implementations by broadcasting scalars, with a possible allocation cost. The physical function evaluator checks output array length, with a special one-element/all-scalar result case; numerical/layout correctness and the declared field contract remain implementation responsibilities. Send/Sync does not make external solver handles or hidden caches safe automatically.

**Current code examples:** `pse_exact_value_equal` compares complete declared input types during field derivation and returns a non-null Boolean including for two nulls, illustrating why strictness is not universal. The PSE diagnostic codec UDFs (`pse_rule_key`, `pse_literal`, `pse_named_id`, `pse_id_list`) demonstrate retained function instances, nested output fields and literal-aware arguments. These are source observations, not claims that process property UDFs are already implemented. [Scalar trait/source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udf.rs.html), [physical invocation](https://docs.rs/datafusion-physical-expr/55.1.0/src/datafusion_physical_expr/scalar_function.rs.html), [exact equality](</home/paul/pse-arrow/crates/pse-authoring/src/change_set/exact.rs>), [codec functions](</home/paul/pse-arrow/crates/pse-catalog/src/session/scalar.rs>).

### Signatures, coercion and volatility

`TypeSignature` variants are `Exact`, `Uniform`, `Variadic`, `VariadicAny`, `Any`, `OneOf`, `Coercible`, `Comparable`, `ArraySignature`, `Numeric`, `String`, `Nullary` and `UserDefined`. They express storage-type/arity alternatives, not process quantity compatibility. `coerce_types` is called for `UserDefined`, returning one target type per argument; the default errors. Broader signatures increase flexibility but move more compatibility reasoning into field-aware or runtime code.

| Signature form | Argument/coercion behavior |
|---|---|
| `Exact(types)` | Ordered target types; input expressions can still be coerced to them. “Exact” does not mean no implicit casts. |
| `Uniform(n, types)` / `Variadic(types)` | Fixed count / one-or-more arguments sharing an accepted type. |
| `Any(n)` / `VariadicAny` | Fixed count / one-or-more arbitrary argument types; does not establish quantity compatibility. |
| `OneOf(signatures)` | Tries alternatives in order and stops at the first successful coercion, making alternative order meaningful. |
| `Coercible(coercions)` | Per-position type classes with declared acceptable casts, allowing more control than a single target storage type. |
| `Comparable(n)` | Coerces arguments to one comparable type through comparison coercion rules. |
| `Numeric(n)` / `String(n)` | Common numeric / common string storage; string precedence includes Utf8View, LargeUtf8 and Utf8. |
| `ArraySignature(...)` | Specialized array/element argument relationships, including recursive-array and map-array forms. |
| `Nullary` | No arguments; execution cardinality comes from the invocation context. |
| `UserDefined` | Delegates selection of target argument types to the implementation's `coerce_types`. |

`TypeSignatureClass` supports `Any`, `Timestamp`, `Time`, `Interval`, `Duration`, `Native`, `Integer`, `Float`, `Decimal`, `Numeric` and `Binary`. These classes allow flexibility in timezone, decimal precision/scale or binary width where applicable; accepting a class still does not establish semantic identity. Signature selection and semantic field derivation therefore answer different compatibility questions. For example, a property adapter may accept several numeric storage types while requiring one particular quantity kind, basis and component domain.

| Volatility | Engine-facing meaning | Relevance to model calculations |
|---|---|---|
| `Immutable` | Same arguments imply the same result; constant folding can evaluate the function during planning. | Fits deterministic kernels with fixed captured model/parameter context; a changing hidden package registry would undermine that assumption. |
| `Stable` | Same arguments are stable within a query but may differ across queries; folding is context-dependent, including different treatment of prepared/view definitions. | Can describe query-bound context, but does not automatically invalidate application caches across queries. |
| `Volatile` | Evaluations may differ; restricts folding and other transformations. | Fits truly changing observations/randomness, not a substitute for explicit failure, lifecycle or effect handling. |

Immutable does not mean cheap, globally memoized, pure in every other sense or exactly once. Constant folding may perform expensive property work before row execution, and function identity, captured parameters and session configuration affect reuse. Implemented equality/hash semantics that omit meaningful captured state can make distinct functions appear equivalent. [Signature/volatility source](https://docs.rs/datafusion-expr-common/55.1.0/src/datafusion_expr_common/signature.rs.html).

## 7. Scalar optimizer hooks: exact capabilities and conditions

| Hook | Engine contract / default | Potential value and applicability |
|---|---|---|
| `simplify(args, SimplifyContext)` | Returns `ExprSimplifyResult`; default `Original(args)`. An unchanged result returns original arguments. A replacement preserves schema, including type/nullability; application metadata semantics need equivalent care. | Can expose native algebra or remove a genuinely redundant conversion. Built-in argument simplification/folding already handles ordinary constants. |
| `preimage(args, literal_expr, context)` | Returns `PreimageResult::None` or `Range { expr, interval }`; the interval is interpreted **half-open**. Default supplies no rewrite. | Can replace a predicate on a function result with predicates on an input expression, exposing pruning and pushdown. See §8. |
| `evaluate_bounds(input_intervals)` | Forward interval estimate; default is an unbounded interval of Null type. | A sound implementation can provide useful ranges for physical expression analysis; it is not automatically an analyzer or solver-bound pass. |
| `propagate_constraints(output_interval, inputs)` | `Some(updated_intervals)` tightens inputs; `Some(empty_vec)` means unchanged; `None` means infeasible; `Err` is an error. Default unchanged. | Can narrow feasible inputs when called by interval propagation; it does not solve a coupled model. |
| `output_ordering(ExprProperties)` | Derives `SortProperties`; default generally unordered unless lexicographic preservation can establish more. | Can retain order knowledge through projections and avoid some supporting sorts. |
| `preserves_lex_ordering` | Default false; declares preservation of relevant lexicographic input ordering. | Supports multi-key ordering reasoning, not arbitrary “monotonic-looking” functions. |
| `strictly_order_preserving` | Default false; preserves comparison outcomes with respect to ordered inputs. | Stronger than nondecreasing behavior: collapsing distinct values is not strict preservation. |
| `placement` | Default `KeepInPlace`; can return placement information such as `MoveTowardsLeafNodes` based on argument placement. | Used by expression extraction/pushdown rules; can expose early inexpensive work but neither promises remote provider execution nor grants unrestricted movement across all operators. |
| `short_circuits` | Default false; flags potentially unevaluated children for optimizer restrictions. | Helps prevent rewrites such as unsafe common-expression extraction. It does not implement runtime laziness. |
| `conditional_arguments` | Describes disjoint exhaustive eager/lazy argument groups. Default None for ordinary calls; when short-circuiting, default treats all arguments as potentially lazy. | Gives more precise optimizer information than the Boolean flag; requires corresponding actual execution semantics. |
| `struct_field_mapping` | Default None; maps output field-access expressions to input argument indexes. | Preserves direct correspondence/ordering through struct construction, not arbitrary derived-property lineage or automatic partial evaluation. |

These hooks are trusted semantic declarations with distinct consumers. Their implementation does not prove that the selected plan will use them; optimizer rule configuration, expression shape, statistics and physical properties determine applicability. Conservative defaults sacrifice opportunities without inventing knowledge. Incorrect positive claims can change values, NULL behavior, errors or row selection, so implementation effort is often better concentrated on hooks with both a sound contract and a meaningful consumer.

**Short-circuit boundary:** the checked ordinary `ScalarFunctionExpr::evaluate` evaluates its child expressions before calling `invoke_with_args`. Marking a UDF short-circuiting cannot prevent a failing unused child from being evaluated on that path. Native CASE/conditional lowering or a specialized physical expression can supply genuinely conditional evaluation. This is directly relevant to phase-dependent property branches and domain-limited logarithm/square-root calculations.

**Brief applications:** an identity unit conversion may simplify to its argument if the complete output contract and failure behavior agree; a cheap canonical-key expression may be useful near a scan; a guarded vapor/liquid calculation can use CASE to select the valid branch rather than relying on a scalar UDF flag. [Scalar hook contracts](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udf.rs.html), [physical evaluator](https://docs.rs/datafusion-physical-expr/55.1.0/src/datafusion_physical_expr/scalar_function.rs.html), [placement consumer](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/extract_leaf_expressions.rs.html).

## 8. Predicate preimages, intervals and ordering in scientific computation

### Preimage is a particular rewrite protocol

In 55.1.0 the simplifier requests a function's preimage for a non-null literal and generates comparisons from its returned interval `[l,u)`: equality becomes `x >= l AND x < u`; `<` uses `l`; `>=` uses `l`; `>` uses `u`; `<=` uses `u`. Inequality and null-safe distinctness have their own Boolean expansions; literal IN-list handling also uses preimages. The callback receives the arguments and literal, **not the comparison operator**.

Consequently, a contiguous equality preimage alone is insufficient justification for every generated comparison. The expression returned by the hook needs compatible order and null semantics across the supported rewrites. A decreasing function or a nonmonotonic correlation cannot simply return an inverse interval and expect comparison direction to be corrected automatically. Unsupported/no-useful-preimage (`None`) is not a proof that the original predicate is unsatisfiable.

This protocol is well suited to increasing bucket/extraction functions with exact boundaries. Scientific affine conversions may also be candidates, but IEEE rounding, unattainable outputs, overflow and half-open endpoints complicate a mathematically simple inverse. An interval enclosure suitable for pruning is not necessarily an exact predicate replacement. For example, a declared increasing integer time-bin function can map a bin equality to its exact coordinate range; a rounded floating unit conversion needs an argument about representable values before the analogous rewrite is sound. [Preimage rewrite implementation](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/simplify_expressions/udf_preimage.rs.html), [simplifier consumer](https://docs.rs/datafusion-optimizer/55.1.0/src/datafusion_optimizer/simplify_expressions/expr_simplifier.rs.html).

### Interval soundness and actual integration

Forward intervals enclose possible output values; backward propagation retains every input still capable of satisfying the output restriction. Wider intervals may miss optimization but remain conservative; excluding possible values can incorrectly prune data or declare infeasibility. Defaults do not supply a thermodynamic range oracle. DataFusion's physical scalar expression delegates interval hooks to the UDF, but application model bounds do not automatically become these inputs or flow back into solver variable bounds.

A process-model integration can map admitted variable bounds into an expression graph, invoke supported propagation, then translate accepted refinements into the existing bounds/scaling contract. This is additional integration work, including units, domain validity and sound treatment of numerical rounding. Coupled dependencies can make interval boxes loose; a nonmonotonic or disconnected feasible set may require conservative enclosure rather than exact inversion. NULL/missingness and NaN/error semantics also require treatment beyond ordinary numeric interval endpoints.

A correlation validity interval is not automatically a proof of model infeasibility. If the requested temperature range overlaps its domain only partly, the result can be “validity unresolved outside this region”; intersecting away the rest silently changes the admissible model unless an independent constraint or chosen policy justifies it. Phase transitions and discontinuities similarly limit global bounds and monotonicity claims.

**Brief applications:** a declared positive flow/density/geometry domain may permit conservative pressure-drop screening; an affine canonical-unit bound conversion can prepare solver limits if endpoints are rounded conservatively; a property correlation outside its documented domain can produce a localized applicability diagnostic rather than an unsupported infeasibility conclusion.

### Ordering is a separate claim

Ordering hooks reason about sorted/singleton/unordered inputs and ranges, potentially preserving useful order for downstream windows. They do not sort data or certify thermodynamic monotonicity. Positive scaling is increasing over real arithmetic, but finite precision can collapse distinct inputs, making strict-order preservation false even when nondecreasing behavior holds. A property may be monotone within one phase/domain and not globally. The distinction matters because sort removal relies on actual ordering guarantees, not approximate numerical trends. [Scalar physical properties](https://docs.rs/datafusion-physical-expr/55.1.0/src/datafusion_physical_expr/scalar_function.rs.html).

## 9. Struct outputs and shared numerical work

A scalar UDF may return `Struct` or other nested Arrow storage with semantically typed child fields. A single `state_properties(T,P,z)` result can share an EOS evaluation across density, enthalpy and entropy outputs. Separate UDFs expose more granular demand, but may repeat shared calculations. A shared internal kernel, an explicit intermediate relation or a struct-valued call are alternative reuse boundaries, each with different batching, ownership and optimization costs.

`StructFieldMapping` contains a `field_accessor` UDF and a list of accessor literal arguments paired with source argument indexes. It is appropriate when a struct field directly represents an input argument, such as a state-bundle constructor retaining its pressure field. Density computed from temperature/pressure/composition is not a direct alias to any one argument. The hook therefore does not tell DataFusion how to skip unrequested derived properties, share EOS state, or recover an arbitrary dependency graph. Such capabilities need other planning/kernel contracts.

**Brief applications:** a canonical state struct can retain field correspondence for its original coordinates; a batched multi-property kernel can amortize expensive intermediates when several outputs are demanded; an individually requested viscosity result may favor a narrower call when unrelated outputs would be costly. These property-function shapes are opportunities, not observed completed implementations in `pse-kernels`. [StructFieldMapping](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udf.rs.html).

## 10. Aggregate UDFs: stateful grouped computation

`AggregateUDFImpl` has the same object-level `Debug + DynEq + DynHash + Send + Sync + Any` requirements as scalar UDF implementations. Its result is a reduction over a group; its partial state is a separate typed interface used to combine work across execution partitions. Both native `Accumulator` and optional specialized paths remain relevant.

| API family | Capability and implementation conditions |
|---|---|
| `name`, `aliases`, `signature`, `coerce_types` | Identity, argument/volatility contract and UserDefined coercion, analogous to scalar functions. |
| `return_type`, `return_field`, `is_nullable` | Type-only or full field-aware result derivation. `return_field` receives input fields, not the scalar constant array supplied to scalar `ReturnFieldArgs`; default delegates to type/nullability behavior. Aggregate `is_nullable` is an active API. |
| `accumulator` | Creates per-group state implementing `update_batch`, `state`, `merge_batch`, `evaluate` and memory `size`; optional retraction support addresses sliding use. |
| `state_fields` | Defines partial-state fields and ordering fields. Default starts from the result field; multi-state calculations can declare sums, counts, extrema or other state with distinct names. State layout must agree with emitted/merged state. |
| `groups_accumulator_supported`, `create_groups_accumulator` | Optional shared storage for many groups, reducing per-group allocation/dynamic dispatch. Generic accumulator paths still occur for some queries and window use. |
| `create_sliding_accumulator` | Optional retractable state; default creates the ordinary accumulator, which does not automatically gain a valid inverse update. |
| `order_sensitivity`, `with_beneficial_ordering` | Classifies insensitive, beneficial or required ordering; default is `HardRequirement`. A beneficial-order implementation can adapt to existing order while remaining correct without it. |
| `simplify`, `simplify_expr_op_literal` | Function-level and aggregate-comparison simplifications; default no special rewrite. A replacement has to retain the semantics of grouping, distinctness, filters and ordering. |
| `reverse_expr` | No support by default; can identify equivalent or replacement behavior under reversed order. |
| `is_descending` | Specialized min/max-like indicator, not a generic declaration that an arbitrary aggregate decreases. Default None. |
| `value_from_stats` | Optional direct aggregate result when statistics and arguments determine it exactly; otherwise None. Conservative min/max bounds alone need not identify an actual aggregate value. |
| `default_value` | Result policy for all-null input; default typed NULL. Non-null result declarations must agree with empty/all-null behavior on the execution path. |
| `supports_null_handling_clause` | Enables IGNORE/RESPECT NULLS syntax; implementation handles the resulting accumulator arguments. Default false. |
| `supports_within_group_clause` | Enables ordered-set syntax; default false. For this syntax the implementation handles its ordering internally rather than assuming DataFusion inserted a sort. |
| `set_monotonicity` | Result behavior as the input set grows; default NotMonotonic. Distinct from scalar input-order preservation. |
| Naming and documentation | `schema_name`, `human_display`, `window_function_schema_name`, `display_name`, `window_function_display_name`, `documentation` describe aggregate and aggregate-as-window expressions. |

Partial-state combination is where scientific numerical semantics meet parallel execution. A weighted mean state can hold weighted sum and total weight, but zero/negative weights, missing pairs, normalization and units determine its meaning. Floating-point sums are not associative; valid partitioning changes may alter rounding, so reproducibility/tolerance choices belong to the numerical contract rather than the aggregate interface. Retraction can accumulate additional numerical error and may be unavailable for some statistics.

**Brief applications:** component inventory grouped by scenario/unit; a weighted stream-property average with declared basis/weight semantics; a rolling closure metric using a retractable accumulator when its state supports it. Built-in SUM/AVG plus native expressions may already cover simpler versions, avoiding a custom state protocol. [Aggregate trait](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udaf.rs.html), [Accumulator](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/trait.Accumulator.html), [GroupsAccumulator](https://docs.rs/datafusion-expr/55.1.0/datafusion_expr/trait.GroupsAccumulator.html).

## 11. Window UDFs: partition context and bounded execution

`WindowUDFImpl` describes one result per input row under partition/order/frame context. `field(WindowUDFFieldArgs)` derives the output field; `partition_evaluator(PartitionEvaluatorArgs)` constructs the stateful implementation. `expressions(ExpressionArgs)` controls physical expressions sent to that evaluator, defaulting to the input expressions. These contracts differ from scalar invocation and aggregate partial-state merging.

| Hook | Meaning / default |
|---|---|
| `name`, `aliases`, `signature`, `coerce_types`, `documentation` | Function identity, storage/coercion contract and discoverability; coercion hook is for UserDefined signatures. |
| `simplify` | Optional window-specific replacement preserving its result contract; default None. |
| `sort_options` | Optional ordering introduced by the result; updates ordering knowledge, not the input ORDER BY clause. Default None. |
| `reverse_expr` | Default NotSupported; may describe identical or replacement behavior under reversal. |
| `limit_effect` | `Unknown` by default; `None` means no additional rows, `Relative(n)` grows the limit by n, and `Absolute(n)` requires at least n rows. These describe requirements relevant to limit pushdown, not a request to truncate the evaluator arbitrarily. |

`PartitionEvaluator` exposes three evaluation routes: `evaluate` over a row range, `evaluate_all` over a partition, and `evaluate_all_with_rank` over peer/rank ranges. `uses_window_frame` and `include_rank` select applicable context; `get_range` describes required input when the SQL frame is not used. `supports_bounded_execution` declares incremental bounded-memory support; `is_causal` distinguishes dependence on future rows; `memoize` can retain sufficient state while allowing old input to be pruned. Defaults are conservative and do not implement a complete evaluator; an implementation's flags have to match the routes it supports.

These hooks can improve trajectory analytics without treating a window as a DAE integrator. Sampling gaps, coordinate units, irregular time steps, ties and scenario/phase partitions remain application semantics. A cumulative row sum is not automatically a time integral; a rate-of-change estimate needs the actual coordinate difference and a zero-gap policy. Causal functions can emit earlier than future-dependent functions, but bounded memory still depends on the complete physical plan and partition behavior.

**Brief applications:** a fouling trend per unit/scenario; a coordinate-aware finite-difference diagnostic; a moving closure statistic. Native LAG/LEAD and aggregate windows may suffice, with custom evaluators valuable where repeated frame work or specialized state is material. [Window trait](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udwf.rs.html), [PartitionEvaluator](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/partition_evaluator.rs.html).

## 12. Connecting to the process compiler and numerical backends

DataFusion can organize scenario expansion, source alignment, property demand, batched calculations, residual inputs, bounds diagnostics and result analysis. It does not automatically provide autodifferentiation of a UDF, sparse Jacobians, Newton/Krylov iteration, nonlinear optimization, DAE integration or convergence guarantees. A function's input/output signature and interval hooks cannot manufacture derivative bindings.

In `pse-arrow`, [blueprint D6, §7 and §18](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>) assigns richer mathematical semantics and numerical layouts to MathIR/backend contracts. The same numerical kernel can have separately declared scalar, dual/hyper-dual, Arrow, DataFusion or external-function adapters when those implementations exist. An inspectable query-level residual does not by itself make a fused solver kernel equivalent; ordering, units, parameters, failure and derivative semantics connect the two.

A solver-interface relation can associate semantic variable IDs, quantity/basis/reference contracts, coordinate mappings, fixed/free roles, lower/upper/nominal values, scaling and backend indexes. Some facts are derivable from fields, others require relational coverage checks or numerical evaluation. Packing and unpacking vectors through these mappings can preserve the meaning of otherwise anonymous `f64` buffers. This describes a possible use of existing compiled/runtime relations, not a recommendation to introduce a second authoritative `SolverContract` store.

**Failure semantics:** missing inputs, unbounded bounds, domain errors and unconverged solver results are distinct states. Returning NULL for a failed property calculation can silently affect aggregates that ignore nulls. Explicit outcome relations or typed errors can retain the distinction, depending on whether the task is fail-fast simulation or tolerant analysis. A solver result may be an approximate accepted iterate, a failure or a cancellation; semantic field validity does not establish convergence or engineering validity.

**Brief applications:** heater energy-flow preparation can retain quantity IDs through vector packing; scenario screening can report an unsupported property domain separately from nonlinear infeasibility; post-solve relations can join residual magnitudes to semantic equation/source identities for diagnosis. Existing architectural context: [blueprint §7.6 and §18.5](</home/paul/pse-arrow/docs/authoritative_design/blueprint.md>).

## 13. Evidence and implementation opportunities in pse-arrow

The following are **source observations at review time**, not a declaration that Plan 05 or process-simulation acceptance is complete. The repository is undergoing a native-plan transition, so these paths are integration anchors rather than a prescribed implementation sequence.

| Inspected source | Relevance to these capabilities |
|---|---|
| [Quantity types/inference](</home/paul/pse-arrow/crates/pse-quantity/src/infer.rs>) and [unit conversion](</home/paul/pse-arrow/crates/pse-quantity/src/unit.rs>) | Existing quantity semantics and explicit conversions can inform field-aware functions and semantic plan derivation. |
| [PSE extension reconstruction](</home/paul/pse-arrow/crates/pse-relations/src/ext/mod.rs>) | Eleven named `pse.*` extensions with typed metadata parsing, version/shape checks and storage validation; stronger than unvalidated string tags, but not full row/reference validity. |
| [DataFusion extension registry](</home/paul/pse-arrow/crates/pse-catalog/src/session/registry.rs>) | Registers platform factories against shared field validation and supplies PSE formatting through `DFExtensionType`. |
| [Preparation](</home/paul/pse-arrow/crates/pse-catalog/src/session/preparation.rs>) and [admission](</home/paul/pse-arrow/crates/pse-catalog/src/session/admission.rs>) | Retain original/analyzed/optimized plans; inspect actual sources/functions and field declarations; restore derived semantic fields after optimization. Prepared/completed computation is explicitly distinct from discharging all stage obligations. |
| [Engine rule profile](</home/paul/pse-arrow/crates/pse-catalog/src/session/profile.rs>) | Retains actual ordered analyzer/logical/physical optimizer objects and descriptive profile identity; useful context for reproducible hook behavior. |
| [Function binding inventory](</home/paul/pse-arrow/crates/pse-catalog/src/session/functions.rs>) | Checks retained implementation object identity across scalar/aggregate/window/higher-order calls, illustrating the distinction between a function name and a bound implementation. |
| [Exact equality](</home/paul/pse-arrow/crates/pse-authoring/src/change_set/exact.rs>) and [codec UDFs](</home/paul/pse-arrow/crates/pse-catalog/src/session/scalar.rs>) | Concrete field-aware numerical/storage interfaces already exist for authoring and diagnostics. |
| [Kernel boundary](</home/paul/pse-arrow/crates/pse-kernels/src/lib.rs>) | The inspected file declares a boundary with no kernel implementation yet. Property/EOS examples in this document are therefore illustrative, not claims of existing UDF coverage. |

The source attachment's proposed generic semantic tags, centralized analyzers and solver-contract pattern are useful conceptual prompts, but the live registry, quantity system and numerical contract already determine the codebase-specific vocabulary. That is why examples here connect to those existing boundaries rather than introduce parallel declarations.

## 14. Validating semantic hooks and their value

| Evidence technique | What it can establish / why it is useful |
|---|---|
| Analyzer acceptance/rejection and ordered-pass cases | Shows which static contracts are checked and whether coercion or repeated analysis changes them; localized diagnostics help separate domain errors from physical type failures. |
| Field propagation cases | Aliases, arithmetic, casts, CASE, aggregation, nested access and interchange can expose lost or stale metadata; storage equality alone misses these defects. |
| Scalar/array/null/boundary execution | Reveals broadcasting, shape, missingness and value-dependent errors; a field-aware signature cannot validate all runtime values. |
| Baseline versus simplified/pushed execution | Exercises actual optimizer consumers, comparing full values, multiplicities, NULLs and failure behavior rather than only the expected tree shape. |
| Preimage endpoint and ordering checks | Detects off-by-one, rounding, direction and NULL errors at the exact domain boundaries where unsound pruning occurs. |
| Interval reference checks | Known solutions, exhaustive small domains or mathematical arguments can support enclosure/propagation claims; sampled success alone cannot prove global soundness for an arbitrary correlation. |
| Aggregate partition/merge/retract cases | Reveals state-layout, empty-group, weighting and numerical-order issues hidden by single-batch execution. |
| Window frame/partition/causality cases | Tests irregular coordinates, ties, future requirements and batch boundaries; bounded-execution flags are claims to exercise, not proofs by declaration. |
| Solver mapping/adapter comparisons | Connects semantic identities, natural units, variable order, outcomes and derivative routes across the numerical boundary. |
| Targeted measurement | Establishes whether a hook actually reduces scans, sorts, evaluations, allocations or planning time for the relevant workload. |

A useful record associates a claimed property with its domain, implementation and consumer, plus the evidence and untested cases. Some properties can be established by construction; others need residual relation checks, execution tests or stronger mathematical justification. This enables selective, explainable validation rather than turning every available hook into a mandatory feature. The value proposition is shared semantic information across planning, execution, diagnostics and numerical integration.
