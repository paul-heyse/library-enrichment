# `tracing_subscriber::fmt::format::json`

Crate `tracing-subscriber` · 3 public items · structured records in [`model/tracing_subscriber.fmt.format.json.json`](../model/tracing_subscriber.fmt.format.json.json)

## Json

`struct` · `tracing_subscriber::fmt::format::json::Json`

Also reachable as `tracing_subscriber::fmt::format::Json`

```rust
struct Json
```

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn flatten_event(&mut self, flatten_event: bool)
fn with_current_span(&mut self, display_current_span: bool)
fn with_span_list(&mut self, display_span_list: bool)
```

Marker for [`Format`] that indicates that the newline-delimited JSON log
format should be used.

This formatter is intended for production use with systems where structured
logs are consumed as JSON by analysis and viewing tools. The JSON output is
not optimized for human readability; instead, it should be pretty-printed
using external JSON tools such as `jq`, or using a JSON log viewer.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-json
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-json`
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821315Z&quot;,&quot;level&quot;:&quot;INFO&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;preparing to shave yaks&quot;,&quot;number_of_yaks&quot;:3},&quot;target&quot;:&quot;fmt_json&quot;}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821422Z&quot;,&quot;level&quot;:&quot;INFO&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;shaving yaks&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821495Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;hello! I&apos;m gonna shave a yak&quot;,&quot;excitement&quot;:&quot;yay!&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:1,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821546Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;yak shaved successfully&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:1,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821598Z&quot;,&quot;level&quot;:&quot;DEBUG&quot;,&quot;fields&quot;:{&quot;yak&quot;:1,&quot;shaved&quot;:true},&quot;target&quot;:&quot;yak_events&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821637Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;yaks_shaved&quot;:1},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821684Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;hello! I&apos;m gonna shave a yak&quot;,&quot;excitement&quot;:&quot;yay!&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:2,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821727Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;yak shaved successfully&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:2,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821773Z&quot;,&quot;level&quot;:&quot;DEBUG&quot;,&quot;fields&quot;:{&quot;yak&quot;:2,&quot;shaved&quot;:true},&quot;target&quot;:&quot;yak_events&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821806Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;yaks_shaved&quot;:2},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821909Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;hello! I&apos;m gonna shave a yak&quot;,&quot;excitement&quot;:&quot;yay!&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:3,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.821956Z&quot;,&quot;level&quot;:&quot;WARN&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;could not locate yak&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;},{&quot;yak&quot;:3,&quot;name&quot;:&quot;shave&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.822006Z&quot;,&quot;level&quot;:&quot;DEBUG&quot;,&quot;fields&quot;:{&quot;yak&quot;:3,&quot;shaved&quot;:false},&quot;target&quot;:&quot;yak_events&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.822041Z&quot;,&quot;level&quot;:&quot;ERROR&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;failed to shave yak&quot;,&quot;yak&quot;:3,&quot;error&quot;:&quot;missing yak&quot;},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.822079Z&quot;,&quot;level&quot;:&quot;TRACE&quot;,&quot;fields&quot;:{&quot;yaks_shaved&quot;:2},&quot;target&quot;:&quot;fmt_json::yak_shave&quot;,&quot;spans&quot;:[{&quot;yaks&quot;:3,&quot;name&quot;:&quot;shaving_yaks&quot;}]}
{&quot;timestamp&quot;:&quot;2022-02-15T18:47:10.822117Z&quot;,&quot;level&quot;:&quot;INFO&quot;,&quot;fields&quot;:{&quot;message&quot;:&quot;yak shaving completed&quot;,&quot;all_yaks_shaved&quot;:false},&quot;target&quot;:&quot;fmt_json&quot;}
</pre>

# Options

This formatter exposes additional options to configure the structure of the
output JSON objects:

- [`Json::flatten_event`] can be used to enable flattening event fields into
  the root
- [`Json::with_current_span`] can be used to control logging of the current
  span
- [`Json::with_span_list`] can be used to control logging of the span list
  object.

By default, event fields are not flattened, and both current span and span
list are logged.

# Valuable Support

Experimental support is available for using the [`valuable`] crate to record
user-defined values as structured JSON. When the ["valuable" unstable
feature][unstable] is enabled, types implementing [`valuable::Valuable`] will
be recorded as structured JSON, rather than
using their [`std::fmt::Debug`] implementations.

**Note**: This is an experimental feature. [Unstable features][unstable]
must be enabled in order to use `valuable` support.

[`Json::flatten_event`]: Json::flatten_event()
[`Json::with_current_span`]: Json::with_current_span()
[`Json::with_span_list`]: Json::with_span_list()
[`valuable`]: https://crates.io/crates/valuable
[unstable]: crate#unstable-features
[`valuable::Valuable`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html

---

## JsonFields

`struct` · `tracing_subscriber::fmt::format::json::JsonFields`

Also reachable as `tracing_subscriber::fmt::format::JsonFields`

```rust
struct JsonFields
```

**Implements**: `tracing_subscriber::fmt::format::FormatFields`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `tracing_subscriber::fmt::format::FormatFields`**

```rust
fn add_fields(&self, current: &'a mut FormattedFields<Self>, fields: &Record<'_>) -> fmt::Result
fn format_fields<R: RecordFields>(&self, writer: Writer<'_>, fields: R) -> fmt::Result
```

The JSON [`FormatFields`] implementation.

---

## JsonVisitor

`struct` · `tracing_subscriber::fmt::format::json::JsonVisitor`

Also reachable as `tracing_subscriber::fmt::format::JsonVisitor`

```rust
struct JsonVisitor<'a>
```

**Implements**: `tracing_core::field::Visit`, `tracing_subscriber::field::VisitFmt`, `tracing_subscriber::field::VisitOutput`

**Derives**: Debug

**Methods** (1)

```rust
fn new(writer: &'a mut dyn Write) -> Self
```

**via `tracing_core::field::Visit`**

```rust
fn record_bool(&mut self, field: &Field, value: bool)
fn record_bytes(&mut self, field: &Field, value: &[u8])
fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)
fn record_f64(&mut self, field: &Field, value: f64)
fn record_i64(&mut self, field: &Field, value: i64)
fn record_str(&mut self, field: &Field, value: &str)
fn record_u64(&mut self, field: &Field, value: u64)
```

**via `tracing_subscriber::field::VisitFmt`**

```rust
fn writer(&mut self) -> &mut dyn fmt::Write
```

**via `tracing_subscriber::field::VisitOutput`**

```rust
fn finish(self) -> fmt::Result
```

The [visitor] produced by [`JsonFields`]'s [`MakeVisitor`] implementation.

[visitor]: crate::field::Visit
[`MakeVisitor`]: crate::field::MakeVisitor

---
