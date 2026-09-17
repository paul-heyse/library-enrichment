# `tracing_subscriber::fmt::format`

Crate `tracing-subscriber` · 14 public items · structured records in [`model/tracing_subscriber.fmt.format.json`](../model/tracing_subscriber.fmt.format.json)

## debug_fn

`function` · `tracing_subscriber::fmt::format::debug_fn`

```rust
fn debug_fn<F>(f: F) -> FieldFn<F> where F: Fn(&mut Writer<'_>, &tracing_core::field::Field, &dyn fmt::Debug) -> fmt::Result + Clone
```

Returns a [`FormatFields`] implementation that formats fields using the
provided function or closure.

---

## format

`function` · `tracing_subscriber::fmt::format::format`

Also reachable as `tracing_subscriber::fmt::format`

```rust
fn format() -> Format
```

Returns the default configuration for an event formatter.

Methods on the returned event formatter can be used for further
configuration. For example:

```rust
let format = tracing_subscriber::fmt::format()
    .without_time()         // Don't include timestamps
    .with_target(false)     // Don't include event targets.
    .with_level(false)      // Don't include event levels.
    .compact();             // Use a more compact, abbreviated format.

// Use the configured formatter when building a new subscriber.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

---

## json

`function` · `tracing_subscriber::fmt::format::json`

```rust
fn json() -> Format<Json>
```

Returns the default configuration for a JSON event formatter.

---

## Compact

`struct` · `tracing_subscriber::fmt::format::Compact`

```rust
struct Compact
```

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Marker for [`Format`] that indicates that the compact log format should be used.

The compact format includes fields from all currently entered spans, after
the event's fields. Span fields are ordered (but not grouped) by
span, and span names are not shown. A more compact representation of the
event's [`Level`] is used, and additional information—such as the event's
target—is disabled by default.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-compact
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-compact`
<font color="#AAAAAA">2022-02-17T19:51:05.809287Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: preparing to shave yaks </font><i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809367Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: shaving yaks </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809414Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809443Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809477Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809500Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=1 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809531Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809554Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809581Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809606Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809635Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809664Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: could not locate yak </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809693Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809717Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash] </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809743Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809768Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: yak shaving completed </font><i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>

</pre>

---

## DefaultFields

`struct` · `tracing_subscriber::fmt::format::DefaultFields`

```rust
struct DefaultFields
```

**Implements**: `tracing_subscriber::field::MakeVisitor`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `tracing_subscriber::field::MakeVisitor`**

```rust
fn make_visitor(&self, target: Writer<'a>) -> Self::Visitor
```

The default [`FormatFields`] implementation.

---

## DefaultVisitor

`struct` · `tracing_subscriber::fmt::format::DefaultVisitor`

```rust
struct DefaultVisitor<'a>
```

**Implements**: `tracing_core::field::Visit`, `tracing_subscriber::field::VisitFmt`, `tracing_subscriber::field::VisitOutput`

**Derives**: Debug

**Methods** (1)

```rust
fn new(writer: Writer<'a>, is_empty: bool) -> Self
```

**via `tracing_core::field::Visit`**

```rust
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
fn record_error(&mut self, field: &Field, value: &dyn std::error::Error + 'static)
fn record_str(&mut self, field: &Field, value: &str)
```

**via `tracing_subscriber::field::VisitFmt`**

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

**via `tracing_subscriber::field::VisitOutput`**

```rust
fn finish(self) -> fmt::Result
```

The [visitor] produced by [`DefaultFields`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

---

## FieldFn

`struct` · `tracing_subscriber::fmt::format::FieldFn`

```rust
struct FieldFn<F>
```

**Implements**: `tracing_subscriber::field::MakeVisitor`

**Derives**: Clone, Debug

**via `tracing_subscriber::field::MakeVisitor`**

```rust
fn make_visitor(&self, writer: Writer<'a>) -> Self::Visitor
```

A [`FormatFields`] implementation that formats fields by calling a function
or closure.

---

## FieldFnVisitor

`struct` · `tracing_subscriber::fmt::format::FieldFnVisitor`

```rust
struct FieldFnVisitor<'a, F>
```

**Implements**: `tracing_core::field::Visit`, `tracing_subscriber::field::VisitFmt`, `tracing_subscriber::field::VisitOutput`

**Derives**: Debug

**via `tracing_core::field::Visit`**

```rust
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
```

**via `tracing_subscriber::field::VisitFmt`**

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

**via `tracing_subscriber::field::VisitOutput`**

```rust
fn finish(self) -> fmt::Result
```

The [visitor] produced by [`FieldFn`]'s [`MakeVisitor`] implementation.

[visitor]: super::super::field::Visit
[`MakeVisitor`]: super::super::field::MakeVisitor

---

## FmtSpan

`struct` · `tracing_subscriber::fmt::format::FmtSpan`

```rust
struct FmtSpan
```

**Implements**: `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, rhs: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, rhs: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, rhs: Self)
```

Configures what points in the span lifecycle are logged as events.

See also [`with_span_events`].

[`with_span_events`]: super::SubscriberBuilder::with_span_events

---

## Format

`struct` · `tracing_subscriber::fmt::format::Format`

```rust
struct Format<F = Full, T = super::time::SystemTime>
```

**Implements**: `tracing_subscriber::fmt::format::FormatEvent`

**Derives**: Clone, Debug, Default

**Methods** (16)

```rust
fn compact(self) -> Format<Compact, T>
fn flatten_event(self, flatten_event: bool) -> Format<Json, T>
fn json(self) -> Format<Json, T>
fn pretty(self) -> Format<Pretty, T>
fn with_ansi(self, ansi: bool) -> Format<F, T>
fn with_current_span(self, display_current_span: bool) -> Format<Json, T>
fn with_file(self, display_filename: bool) -> Format<F, T>
fn with_level(self, display_level: bool) -> Format<F, T>
fn with_line_number(self, display_line_number: bool) -> Format<F, T>
fn with_source_location(self, display_location: bool) -> Self
fn with_span_list(self, display_span_list: bool) -> Format<Json, T>
fn with_target(self, display_target: bool) -> Format<F, T>
fn with_thread_ids(self, display_thread_id: bool) -> Format<F, T>
fn with_thread_names(self, display_thread_name: bool) -> Format<F, T>
fn with_timer<T2>(self, timer: T2) -> Format<F, T2>
fn without_time(self) -> Format<F, ()>
```

**via `tracing_subscriber::fmt::format::FormatEvent`**

```rust
fn format_event(&self, ctx: &FmtContext<'_, S, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result
fn format_event(&self, ctx: &FmtContext<'_, C, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result
fn format_event(&self, ctx: &FmtContext<'_, S, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result
fn format_event(&self, ctx: &FmtContext<'_, S, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result where S: Subscriber + for<'a> LookupSpan<'a>
```

A pre-configured event formatter.

You will usually want to use this as the [`FormatEvent`] for a [`FmtSubscriber`].

The default logging format, [`Full`] includes all fields in each event and its containing
spans. The [`Compact`] logging format is intended to produce shorter log
lines; it displays each event's fields, along with fields from the current
span context, but other information is abbreviated. The [`Pretty`] logging
format is an extra-verbose, multi-line human-readable logging format
intended for use in development.

[`FmtSubscriber`]: super::Subscriber

---

## Full

`struct` · `tracing_subscriber::fmt::format::Full`

```rust
struct Full
```

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

Marker for [`Format`] that indicates that the default log format should be used.

This formatter shows the span context before printing event data. Spans are
displayed including their names and fields.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt`
<font color="#AAAAAA">2022-02-15T18:40:14.289898Z </font><font color="#4E9A06"> INFO</font> fmt: preparing to shave yaks <i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-15T18:40:14.289974Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: shaving yaks</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290011Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290038Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290070Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290089Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290114Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290134Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290157Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290174Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290198Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290222Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: could not locate yak</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290247Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290268Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash]</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290287Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290309Z </font><font color="#4E9A06"> INFO</font> fmt: yak shaving completed. <i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>
</pre>

---

## Writer

`struct` · `tracing_subscriber::fmt::format::Writer`

```rust
struct Writer<'writer>
```

**Implements**: `core::fmt::Write`

**Derives**: Debug

**Methods** (7)

```rust
fn by_ref(&mut self) -> Writer<'_>
fn has_ansi_escapes(&self) -> bool
fn new(writer: &'writer mut impl fmt::Write) -> Self
fn sanitizes_ansi_escapes(&self) -> bool
fn write_char(&mut self, c: char) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
```

**via `core::fmt::Write`**

```rust
fn write_char(&mut self, c: char) -> fmt::Result
fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result
fn write_str(&mut self, s: &str) -> fmt::Result
```

A writer to which formatted representations of spans and events are written.

This type is provided as input to the [`FormatEvent::format_event`] and
[`FormatFields::format_fields`] methods, which will write formatted
representations of [`Event`]s and [fields] to the [`Writer`].

This type implements the [`std::fmt::Write`] trait, allowing it to be used
with any function that takes an instance of [`std::fmt::Write`].
Additionally, it can be used with the standard library's [`std::write!`] and
[`std::writeln!`] macros.

Additionally, a [`Writer`] may expose additional [`tracing`]-specific
information to the formatter implementation.

[fields]: tracing_core::field

---

## FormatEvent

`trait` · `tracing_subscriber::fmt::format::FormatEvent`

Also reachable as `tracing_subscriber::fmt::FormatEvent`

```rust
trait FormatEvent<S, N> where S: Subscriber + for<'a> LookupSpan<'a>, N: for<'a> FormatFields<'a> + 'static
```

**Implementors** (1)

- `tracing_subscriber::fmt::format::Format`

**Methods** (1)

```rust
fn format_event(&self, ctx: &FmtContext<'_, S, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result
```

A type that can format a tracing [`Event`] to a [`Writer`].

[`FormatEvent`] is primarily used in the context of [`fmt::Subscriber`] or
[`fmt::Layer`]. Each time an event is dispatched to [`fmt::Subscriber`] or
[`fmt::Layer`], the subscriber or layer
forwards it to its associated [`FormatEvent`] to emit a log message.

This trait is already implemented for function pointers with the same
signature as `format_event`.

# Arguments

The following arguments are passed to [`FormatEvent::format_event`]:

* A [`FmtContext`]. This is an extension of the [`layer::Context`] type,
  which can be used for accessing stored information such as the current
  span context an event occurred in.

  In addition, [`FmtContext`] exposes access to the [`FormatFields`]
  implementation that the subscriber was configured to use via the
  [`FmtContext::field_format`] method. This can be used when the
  [`FormatEvent`] implementation needs to format the event's fields.

  For convenience, [`FmtContext`] also implements [`FormatFields`],
  forwarding to the configured [`FormatFields`] type.

* A [`Writer`] to which the formatted representation of the event is
  written. This type implements the [`std::fmt::Write`] trait, and therefore
  can be used with the [`std::write!`] and [`std::writeln!`] macros, as well
  as calling [`std::fmt::Write`] methods directly.

  The [`Writer`] type also implements additional methods that provide
  information about how the event should be formatted. The
  [`Writer::has_ansi_escapes`] method indicates whether [ANSI terminal
  escape codes] are supported by the underlying I/O writer that the event
  will be written to. If this returns `true`, the formatter is permitted to
  use ANSI escape codes to add colors and other text formatting to its
  output. If it returns `false`, the event will be written to an output that
  does not support ANSI escape codes (such as a log file), and they should
  not be emitted.

  Crates like [`nu_ansi_term`] and [`owo-colors`] can be used to add ANSI
  escape codes to formatted output.

* The actual [`Event`] to be formatted.

# Examples

This example re-implements a simplified version of this crate's [default
formatter]:

```rust
use std::fmt;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

let _subscriber = tracing_subscriber::fmt()
    .event_format(MyFormatter)
    .init();

let _span = tracing::info_span!("my_span", answer = 42).entered();
tracing::info!(question = "life, the universe, and everything", "hello world");
```

This formatter will print events like this:

```text
DEBUG yak_shaving::shaver: some-span{field-on-span=foo}: started shaving yak
```

[`layer::Context`]: crate::layer::Context
[`fmt::Layer`]: super::Layer
[`fmt::Subscriber`]: super::Subscriber
[`Event`]: tracing::Event
[implements `FormatFields`]: super::FmtContext#impl-FormatFields<'writer>
[ANSI terminal escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code
[`Writer::has_ansi_escapes`]: Writer::has_ansi_escapes
[`nu_ansi_term`]: https://crates.io/crates/nu_ansi_term
[`owo-colors`]: https://crates.io/crates/owo-colors
[default formatter]: Full

---

## FormatFields

`trait` · `tracing_subscriber::fmt::format::FormatFields`

Also reachable as `tracing_subscriber::fmt::FormatFields`

```rust
trait FormatFields<'writer>
```

**Implementors** (3)

- `tracing_subscriber::fmt::fmt_layer::FmtContext`
- `tracing_subscriber::fmt::format::json::JsonFields`
- `tracing_subscriber::fmt::format::pretty::Pretty`

**Methods** (2)

```rust
fn add_fields(&self, current: &'writer mut FormattedFields<Self>, fields: &span::Record<'_>) -> fmt::Result
fn format_fields<R: RecordFields>(&self, writer: Writer<'writer>, fields: R) -> fmt::Result
```

A type that can format a [set of fields] to a [`Writer`].

[`FormatFields`] is primarily used in the context of [`FmtSubscriber`]. Each
time a span or event with fields is recorded, the subscriber will format
those fields with its associated [`FormatFields`] implementation.

[set of fields]: crate::field::RecordFields
[`FmtSubscriber`]: super::Subscriber

---
