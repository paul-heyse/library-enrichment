# Crate roles

All pinned packages are routed; a role is not exhaustive semantic characterization.

| Crate | Role | Representation | Task vocabulary | Reviewed briefs |
|---|---|---|---|---|
| arrow | Facade and common re-exports | Array, Schema, kernels, IPC/FFI access | Reuse existing facade imports; inspect defining subcrate | Full API only |
| arrow-arith | Array arithmetic and Boolean/aggregate kernels | Arrays and scalars to arrays or aggregates | overflow; Kleene logic; numeric reductions | Full API only |
| arrow-array | Typed and dynamic arrays, builders, batches | Values/buffers to ArrayRef and RecordBatch | construct batches; access nested arrays; dictionaries; views | [arrow.schema](../capabilities/arrow.schema.md), [arrow.select](../capabilities/arrow.select.md) |
| arrow-avro | Avro decoding and encoding | Avro data and Arrow batches | read or write Avro without a relational plan | Full API only |
| arrow-buffer | Physical buffers, offsets, null bitmaps, allocation | Byte/value buffers and offset/null structures | buffer sharing; allocation; bitmap operations | Full API only |
| arrow-cast | Type conversion, parsing and formatting | Array plus target DataType to ArrayRef | cast failure policy; decimal/time conversion; formatting | [arrow.cast](../capabilities/arrow.cast.md) |
| arrow-csv | CSV reading and writing | Text records and RecordBatch | CSV codec; schema inference; projection | Full API only |
| arrow-data | ArrayData validation and transformation | Buffer/type trees to validated or transformed ArrayData | low-level array construction; copy selected ranges | Full API only |
| arrow-flight | Flight and Flight SQL protocol tooling | Batches and RPC protocol messages | remote batch exchange; transport integration | [df.interchange](../capabilities/df.interchange.md) |
| arrow-ipc | Arrow IPC readers, writers and metadata | Batches/schema and IPC messages/files | persist or exchange Arrow representations | [arrow.row-keys](../capabilities/arrow.row-keys.md), [df.interchange](../capabilities/df.interchange.md) |
| arrow-json | JSON reading, decoding and writing | JSON records and RecordBatch | incremental decode; schema; output encoding | Full API only |
| arrow-ord | Comparison, ranking and sorting kernels | Arrays to comparisons or sort indices | lexicographic sort; rank; min/max comparisons | [arrow.row-keys](../capabilities/arrow.row-keys.md) |
| arrow-pyarrow | Rust/PyArrow conversion helpers | Python Arrow objects and Rust Arrow values | Python interoperability; ownership boundaries | Full API only |
| arrow-row | Comparison-oriented row encoding | Columns to Rows, and Rows to arrays | composite keys; repeated comparison; dictionary hydration | [arrow.row-keys](../capabilities/arrow.row-keys.md) |
| arrow-schema | DataType, Field, Schema and extension metadata | Type/schema definitions and errors | schema merge; metadata; nullability; extension types | [arrow.cast](../capabilities/arrow.cast.md), [arrow.schema](../capabilities/arrow.schema.md) |
| arrow-select | Selection and arrangement kernels | Arrays/batches plus masks or indices to arrays/batches | filter; take; concat; interleave; coalesce | [arrow.filter-reuse](../capabilities/arrow.filter-reuse.md), [arrow.select](../capabilities/arrow.select.md) |
| arrow-string | String and pattern kernels | String/binary arrays to values or Boolean arrays | like; regexp; string predicates; vectorized text | Full API only |
| datafusion | Session and DataFrame facade/integration | SQL/expressions/sources to plans and result streams | application entry; session wiring; query execution | [df.consume](../capabilities/df.consume.md), [df.relations](../capabilities/df.relations.md), [df.source](../capabilities/df.source.md), [df.storage-reuse](../capabilities/df.storage-reuse.md) |
| datafusion-catalog | Catalog implementations and reusable providers | Tables/plans/batches to catalogs and providers | MemTable; ViewTable; streaming sources; registration | [df.source](../capabilities/df.source.md) |
| datafusion-catalog-listing | File listing and table discovery | File locations/configuration to providers | ListingTable; partitioned file datasets | [df.source](../capabilities/df.source.md) |
| datafusion-common | Shared expression/plan/schema primitives | DFSchema, ScalarValue, statistics, tree helpers | qualified schemas; tree rewrites; scalar conversion; errors | [arrow.schema](../capabilities/arrow.schema.md) |
| datafusion-common-runtime | Runtime/task helpers | Async tasks and execution support | task ownership; runtime integration | Full API only |
| datafusion-datasource | Reusable scan and sink infrastructure | File/source configuration to execution components | FileSource; FileOpener; projection; sinks | [df.source](../capabilities/df.source.md), [df.storage-reuse](../capabilities/df.storage-reuse.md) |
| datafusion-datasource-arrow | Arrow IPC file source integration | IPC files to DataFusion scans | query Arrow IPC files | Full API only |
| datafusion-datasource-avro | Avro file source integration | Avro files to DataFusion scans | query Avro files | Full API only |
| datafusion-datasource-csv | CSV file source integration | CSV files to DataFusion scans | query CSV; CSV file options | Full API only |
| datafusion-datasource-json | JSON file source integration | JSON files to DataFusion scans | query JSON; JSON file options | Full API only |
| datafusion-datasource-parquet | Parquet scan integration and tuning | Parquet metadata/files to execution sources | pruning; row filters; reader factories; schema adaptation | [df.source](../capabilities/df.source.md), [parquet.selection](../capabilities/parquet.selection.md) |
| datafusion-doc | Function documentation structures | Function documentation metadata | generate or inspect function descriptions | Full API only |
| datafusion-execution | Runtime, memory, streams and task state | Runtime/session configuration and execution resources | memory pools; caches; disk manager; stream contracts | [df.consume](../capabilities/df.consume.md), [df.storage-reuse](../capabilities/df.storage-reuse.md) |
| datafusion-expr | Logical expressions, plans and function interfaces | Typed expression/plan trees and function definitions | construct queries; function calls; logical extensions | [df.aggregate-window](../capabilities/df.aggregate-window.md), [df.coalesce](../capabilities/df.coalesce.md), [df.expressions](../capabilities/df.expressions.md), [df.functions](../capabilities/df.functions.md), [df.pushdown](../capabilities/df.pushdown.md), [df.relations](../capabilities/df.relations.md) |
| datafusion-expr-common | Shared expression contracts | Signatures, operators and accumulator interfaces | coercion vocabulary; volatility; aggregation contracts | [df.aggregate-window](../capabilities/df.aggregate-window.md) |
| datafusion-ffi | DataFusion-specific foreign interfaces | Providers/functions/plans across ABI boundary | plugin interoperability; foreign lifecycle | [df.interchange](../capabilities/df.interchange.md) |
| datafusion-functions | Built-in scalar functions | Scalar/array arguments via function objects or Expr helpers | find existing numeric/string/date/core functions | [df.coalesce](../capabilities/df.coalesce.md), [df.expressions](../capabilities/df.expressions.md), [df.functions](../capabilities/df.functions.md) |
| datafusion-functions-aggregate | Built-in aggregate functions | Grouped inputs and aggregate state to results | count; sum; approximate or statistical aggregates | [df.aggregate-window](../capabilities/df.aggregate-window.md), [df.functions](../capabilities/df.functions.md) |
| datafusion-functions-aggregate-common | Aggregate implementation helpers | Aggregate state and helper structures | reuse accumulator support; avoid duplicate mechanics | [df.aggregate-window](../capabilities/df.aggregate-window.md) |
| datafusion-functions-nested | Built-in nested-data functions | List/map/struct expressions and arrays | array transforms; nested access; map/list operations | [df.functions](../capabilities/df.functions.md), [df.relations](../capabilities/df.relations.md) |
| datafusion-functions-table | Built-in table-valued functions | Function arguments to relational sources | generate rows; series; table functions | [df.functions](../capabilities/df.functions.md) |
| datafusion-functions-window | Built-in window functions | Partition/frame/order context to window results | rank; lag/lead; positional functions | [df.aggregate-window](../capabilities/df.aggregate-window.md), [df.functions](../capabilities/df.functions.md) |
| datafusion-functions-window-common | Shared window-function contracts | Window expression and field metadata | window implementation support | Full API only |
| datafusion-macros | DataFusion procedural macro support | Rust definitions to generated support | function/implementation authoring support | Full API only |
| datafusion-optimizer | Logical analysis and optimization | LogicalPlan/Expr to analyzed or optimized trees | coercion; simplification; rewrite rules | [df.coalesce](../capabilities/df.coalesce.md), [df.expressions](../capabilities/df.expressions.md) |
| datafusion-physical-expr | Physical expression construction and helpers | Logical expressions/schema to evaluable expressions | batch evaluation; ordering/equivalence; intervals | [df.expressions](../capabilities/df.expressions.md) |
| datafusion-physical-expr-adapter | Physical expression/schema adaptation | Expressions and differing schemas to adapted expressions | schema evolution; file/table expression reconciliation | [arrow.schema](../capabilities/arrow.schema.md) |
| datafusion-physical-expr-common | Shared physical expression interfaces | Batches and expressions to ColumnarValue | PhysicalExpr; expression codecs; metrics primitives | Full API only |
| datafusion-physical-optimizer | Built-in physical optimization rules | Physical plan to transformed physical plan | partitioning; sorting; distribution; execution rewrites | Full API only |
| datafusion-physical-plan | Built-in execution operators and plan interfaces | Physical plan partitions to RecordBatch streams | joins; sorts; aggregates; limits; metrics | [df.consume](../capabilities/df.consume.md), [df.pushdown](../capabilities/df.pushdown.md), [df.relations](../capabilities/df.relations.md) |
| datafusion-proto | Logical/physical plan protobuf conversion | Plans to/from protobuf representations | plan serialization; custom extension codecs | [df.interchange](../capabilities/df.interchange.md) |
| datafusion-proto-common | Shared protobuf conversions | Common DataFusion/Arrow values and protobuf | common type conversion; serialization support | Full API only |
| datafusion-proto-models | Generated protobuf model types | Typed serialized plan messages | inspect wire model; use converters for executable objects | Full API only |
| datafusion-pruning | Predicate evaluation over statistics | Predicate/statistics to candidate selections | sound pruning; file/row-group candidate reduction | Full API only |
| datafusion-session | Shared session, provider and planner interfaces | Session/table/catalog contracts | TableProvider; scan arguments; planner integration | [df.pushdown](../capabilities/df.pushdown.md) |
| datafusion-spark | Spark-compatible functions and planning support | Spark-oriented syntax/functions to DataFusion plans | compatibility requirements; alternate semantics | Full API only |
| datafusion-sql | SQL planning and unparsing | SQL AST/context to logical expressions/plans and SQL | dialects; SQL translation; unparse | Full API only |
| datafusion-substrait | Substrait plan conversion | Logical plans to/from Substrait | plan interchange; extension mapping | [df.interchange](../capabilities/df.interchange.md) |
| object_store | Object-storage abstraction and implementations | Locations/ranges/bytes and object metadata | range reads; listings; conditional writes; storage backends | [df.source](../capabilities/df.source.md), [df.storage-reuse](../capabilities/df.storage-reuse.md) |
| parquet | Parquet metadata, readers and writers | Columnar file data and Arrow batches | row groups; page indexes; projection; row filters | [df.storage-reuse](../capabilities/df.storage-reuse.md), [parquet.selection](../capabilities/parquet.selection.md) |
| parquet-variant | Variant representation and builders | Variant bytes/metadata and values | semi-structured value representation | Full API only |
| parquet-variant-compute | Variant computation and shredding support | Variant/Arrow representations | semi-structured conversion and operations | Full API only |
| sqlparser | SQL syntax parsing and AST | SQL text to syntax tree | parse syntax; inspect AST; dialect; no query execution | Full API only |
