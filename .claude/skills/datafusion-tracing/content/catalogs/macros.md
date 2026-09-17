# Macro invocation grammar

31 macros, 49 accepted invocation forms. Read an arm as the literal shape you type: `options:` is a **keyword**, not a positional argument, and omitting it is a macro expansion error that names nothing useful.

## Two families

| Family | Macros | Wraps | Returns |
|---|---:|---|---|
| `instrument_with_*_spans!` | 25 | every `ExecutionPlan` node, at run time | a `PhysicalOptimizerRule` you register |
| `instrument_rules_with_*_spans!` | 6 | the analyzer, optimizer and physical-optimizer passes, at plan time | a `SessionState` |

The second takes `state:` as well as `options:`, which is the shape difference: one produces a rule you add to a builder, the other consumes and returns the state.

## Arms

### `debug!`

Constructs an event at the debug level.

Level: **caller-supplied**


### `debug_span!`

Constructs a span at the debug level.

Level: **caller-supplied**


### `enabled!`

Checks whether a span or event is [enabled] based on the provided [metadata].

Level: **caller-supplied**


### `error!`

Constructs an event at the error level.

Level: **caller-supplied**


### `error_span!`

Constructs a span at the error level.

Level: **caller-supplied**


### `event!`

Constructs a new `Event`.

Level: **caller-supplied**


### `event_enabled!`

Tests whether an event with the specified level and target would be enabled.

Level: **caller-supplied**


### `identify_callsite!`

Statically constructs an [`Identifier`] for the provided [`Callsite`].

Level: **caller-supplied**


### `info!`

Constructs an event at the info level.

Level: **caller-supplied**


### `info_span!`

Constructs a span at the info level.

Level: **caller-supplied**


### `instrument_rules_with_debug_spans!`

Instruments a `SessionState` with DEBUG-level tracing spans.

Level: **DEBUG**

- `instrument_rules_with_debug_spans!(target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_debug_spans!(options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_debug_spans!(target: $target:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_debug_spans!(options: $options:expr, state: $state:expr)`

### `instrument_rules_with_error_spans!`

Instruments a `SessionState` with ERROR-level tracing spans.

Level: **ERROR**

- `instrument_rules_with_error_spans!(target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_error_spans!(options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_error_spans!(target: $target:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_error_spans!(options: $options:expr, state: $state:expr)`

### `instrument_rules_with_info_spans!`

Instruments a `SessionState` with INFO-level tracing spans.

Level: **INFO**

- `instrument_rules_with_info_spans!(target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_info_spans!(options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_info_spans!(target: $target:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_info_spans!(options: $options:expr, state: $state:expr)`

### `instrument_rules_with_spans!`

Instruments a `SessionState` with tracing spans for all rule phases.

Level: **caller-supplied**

- `instrument_rules_with_spans!(target: $target:expr, $lvl:expr, options: $options:expr, state: $state:expr, $($fields:tt)*)`
- `instrument_rules_with_spans!(target: $target:expr, $lvl:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_spans!($lvl:expr, options: $options:expr, state: $state:expr, $($fields:tt)*)`
- `instrument_rules_with_spans!($lvl:expr, options: $options:expr, state: $state:expr)`

### `instrument_rules_with_trace_spans!`

Instruments a `SessionState` with TRACE-level tracing spans.

Level: **TRACE**

- `instrument_rules_with_trace_spans!(target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_trace_spans!(options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_trace_spans!(target: $target:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_trace_spans!(options: $options:expr, state: $state:expr)`

### `instrument_rules_with_warn_spans!`

Instruments a `SessionState` with WARN-level tracing spans.

Level: **WARN**

- `instrument_rules_with_warn_spans!(target: $target:expr, options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_warn_spans!(options: $options:expr, state: $state:expr, $($field:tt)*)`
- `instrument_rules_with_warn_spans!(target: $target:expr, options: $options:expr, state: $state:expr)`
- `instrument_rules_with_warn_spans!(options: $options:expr, state: $state:expr)`

### `instrument_with_debug_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan` at the debug level.

Level: **DEBUG**

- `instrument_with_debug_spans!(target: $target:expr, options: $options:expr, $($field:tt)*)`
- `instrument_with_debug_spans!(options: $options:expr, $($field:tt)*)`
- `instrument_with_debug_spans!(target: $target:expr, options: $options:expr)`
- `instrument_with_debug_spans!(options: $options:expr)`

### `instrument_with_error_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan` at the error level.

Level: **ERROR**

- `instrument_with_error_spans!(target: $target:expr, options: $options:expr, $($field:tt)*)`
- `instrument_with_error_spans!(options: $options:expr, $($field:tt)*)`
- `instrument_with_error_spans!(target: $target:expr, options: $options:expr)`
- `instrument_with_error_spans!(options: $options:expr)`

### `instrument_with_info_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan` at the info level.

Level: **INFO**

- `instrument_with_info_spans!(target: $target:expr, options: $options:expr, $($field:tt)*)`
- `instrument_with_info_spans!(options: $options:expr, $($field:tt)*)`
- `instrument_with_info_spans!(target: $target:expr, options: $options:expr)`
- `instrument_with_info_spans!(options: $options:expr)`

### `instrument_with_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan`.

Level: **caller-supplied**

- `instrument_with_spans!(target: $target:expr, $lvl:expr, options: $options:expr, $($fields:tt)*)`
- `instrument_with_spans!(target: $target:expr, $lvl:expr, options: $options:expr)`
- `instrument_with_spans!($lvl:expr, options: $options:expr, $($fields:tt)*)`
- `instrument_with_spans!($lvl:expr, options: $options:expr)`

### `instrument_with_trace_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan` at the trace level.

Level: **TRACE**

- `instrument_with_trace_spans!(target: $target:expr, options: $options:expr, $($field:tt)*)`
- `instrument_with_trace_spans!(options: $options:expr, $($field:tt)*)`
- `instrument_with_trace_spans!(target: $target:expr, options: $options:expr)`
- `instrument_with_trace_spans!(options: $options:expr)`

### `instrument_with_warn_spans!`

Constructs a new instrumentation `PhysicalOptimizerRule` for a DataFusion `ExecutionPlan` at the warn level.

Level: **WARN**

- `instrument_with_warn_spans!(target: $target:expr, options: $options:expr, $($field:tt)*)`
- `instrument_with_warn_spans!(options: $options:expr, $($field:tt)*)`
- `instrument_with_warn_spans!(target: $target:expr, options: $options:expr)`
- `instrument_with_warn_spans!(options: $options:expr)`

### `metadata!`

Statically constructs new span [metadata].

Level: **caller-supplied**


### `record_all!`

Records multiple values on a span in a single call. As with recording individual values, all fields must be declared when the span is created.

Level: **caller-supplied**


### `span!`

Constructs a new span.

Level: **caller-supplied**


### `span_at_level!`

Creates a span at the specified tracing level with the given name and fields.

Level: **caller-supplied**

- `span_at_level!($level:expr, $name:expr, $($field:tt)*)`

### `span_enabled!`

Tests whether a span with the specified level and target would be enabled.

Level: **caller-supplied**


### `trace!`

Constructs an event at the trace level.

Level: **caller-supplied**


### `trace_span!`

Constructs a span at the trace level.

Level: **caller-supplied**


### `warn!`

Constructs an event at the warn level.

Level: **caller-supplied**


### `warn_span!`

Constructs a span at the warn level.

Level: **caller-supplied**


## The custom-field coupling

`InstrumentationOptions::builder().add_custom_field("env", "production")` sets a value. It does **not** create a field. A `tracing` span's field set is fixed when the span is created, so the macro must declare the key too:

```rust
let rule = instrument_with_info_spans!(
    options: options,
    env = field::Empty,      // declared here
    region = field::Empty,
);
```

Upstream says the same thing in `corpus/examples/integration-utils/src/lib.rs`, in a comment beside those two lines: *custom fields keys must be defined at compile time*. Declare without setting and the field is absent from the span; set without declaring and the value goes nowhere. Neither case is an error at compile time or at run time.

## What an arm expands to is not in this index

rustdoc emits no macro bodies, at any format version -- every arm above ends `=> { ... }` in the source document too. The expansion is in `corpus/source/exec_instrument_macros.rs` and `corpus/source/rule_instrumentation_macros.rs`, and what it does at run time is in `index/behaviors.tsv`. Do not infer it from the arm.
