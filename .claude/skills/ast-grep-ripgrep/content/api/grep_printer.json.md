# `grep_printer::json`

Crate `grep-printer` · 3 public items · structured records in [`model/grep_printer.json.json`](../model/grep_printer.json.json)

## JSON

`struct` · `grep_printer::json::JSON`

Also reachable as `grep_printer::JSON`

```rust
struct JSON<W>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn get_mut(&mut self) -> &mut W
fn has_written(&self) -> bool
fn into_inner(self) -> W
fn new(wtr: W) -> JSON<W>
fn sink<'s, M: Matcher>(&'s mut self, matcher: M) -> JSONSink<'static, 's, M, W>
fn sink_with_path<'p, 's, M, P>(&'s mut self, matcher: M, path: &'p P) -> JSONSink<'p, 's, M, W> where M: Matcher, P: ?Sized + AsRef<Path>
```

The JSON printer, which emits results in a JSON lines format.

This type is generic over `W`, which represents any implementation of
the standard library `io::Write` trait.

# Format

This section describes the JSON format used by this printer.

To skip the rigamarole, take a look at the
[example](#example)
at the end.

## Overview

The format of this printer is the [JSON Lines](https://jsonlines.org/)
format. Specifically, this printer emits a sequence of messages, where
each message is encoded as a single JSON value on a single line. There are
four different types of messages (and this number may expand over time):

* **begin** - A message that indicates a file is being searched.
* **end** - A message the indicates a file is done being searched. This
  message also include summary statistics about the search.
* **match** - A message that indicates a match was found. This includes
  the text and offsets of the match.
* **context** - A message that indicates a contextual line was found.
  This includes the text of the line, along with any match information if
  the search was inverted.

Every message is encoded in the same envelope format, which includes a tag
indicating the message type along with an object for the payload:

```json
{
    "type": "{begin|end|match|context}",
    "data": { ... }
}
```

The message itself is encoded in the envelope's `data` key.

## Text encoding

Before describing each message format, we first must briefly discuss text
encoding, since it factors into every type of message. In particular, JSON
may only be encoded in UTF-8, UTF-16 or UTF-32. For the purposes of this
printer, we need only worry about UTF-8. The problem here is that searching
is not limited to UTF-8 exclusively, which in turn implies that matches
may be reported that contain invalid UTF-8. Moreover, this printer may
also print file paths, and the encoding of file paths is itself not
guaranteed to be valid UTF-8. Therefore, this printer must deal with the
presence of invalid UTF-8 somehow. The printer could silently ignore such
things completely, or even lossily transcode invalid UTF-8 to valid UTF-8
by replacing all invalid sequences with the Unicode replacement character.
However, this would prevent consumers of this format from accessing the
original data in a non-lossy way.

Therefore, this printer will emit valid UTF-8 encoded bytes as normal
JSON strings and otherwise base64 encode data that isn't valid UTF-8. To
communicate whether this process occurs or not, strings are keyed by the
name `text` where as arbitrary bytes are keyed by `bytes`.

For example, when a path is included in a message, it is formatted like so,
if and only if the path is valid UTF-8:

```json
{
    "path": {
        "text": "/home/ubuntu/lib.rs"
    }
}
```

If instead our path was `/home/ubuntu/lib\xFF.rs`, where the `\xFF` byte
makes it invalid UTF-8, the path would instead be encoded like so:

```json
{
    "path": {
        "bytes": "L2hvbWUvdWJ1bnR1L2xpYv8ucnM="
    }
}
```

This same representation is used for reporting matches as well.

The printer guarantees that the `text` field is used whenever the
underlying bytes are valid UTF-8.

## Wire format

This section documents the wire format emitted by this printer, starting
with the four types of messages.

Each message has its own format, and is contained inside an envelope that
indicates the type of message. The envelope has these fields:

* **type** - A string indicating the type of this message. It may be one
  of four possible strings: `begin`, `end`, `match` or `context`. This
  list may expand over time.
* **data** - The actual message data. The format of this field depends on
  the value of `type`. The possible message formats are
  [`begin`](#message-begin),
  [`end`](#message-end),
  [`match`](#message-match),
  [`context`](#message-context).

#### Message: **begin**

This message indicates that a search has begun. It has these fields:

* **path** - An
  [arbitrary data object](#object-arbitrary-data)
  representing the file path corresponding to the search, if one is
  present. If no file path is available, then this field is `null`.

#### Message: **end**

This message indicates that a search has finished. It has these fields:

* **path** - An
  [arbitrary data object](#object-arbitrary-data)
  representing the file path corresponding to the search, if one is
  present. If no file path is available, then this field is `null`.
* **binary_offset** - The absolute offset in the data searched
  corresponding to the place at which binary data was detected. If no
  binary data was detected (or if binary detection was disabled), then this
  field is `null`.
* **stats** - A [`stats` object](#object-stats) that contains summary
  statistics for the previous search.

#### Message: **match**

This message indicates that a match has been found. A match generally
corresponds to a single line of text, although it may correspond to
multiple lines if the search can emit matches over multiple lines. It
has these fields:

* **path** - An
  [arbitrary data object](#object-arbitrary-data)
  representing the file path corresponding to the search, if one is
  present. If no file path is available, then this field is `null`.
* **lines** - An
  [arbitrary data object](#object-arbitrary-data)
  representing one or more lines contained in this match.
* **line_number** - If the searcher has been configured to report line
  numbers, then this corresponds to the line number of the first line
  in `lines`. If no line numbers are available, then this is `null`.
* **absolute_offset** - The absolute byte offset corresponding to the start
  of `lines` in the data being searched.
* **submatches** - An array of [`submatch` objects](#object-submatch)
  corresponding to matches in `lines`. The offsets included in each
  `submatch` correspond to byte offsets into `lines`. (If `lines` is base64
  encoded, then the byte offsets correspond to the data after base64
  decoding.) The `submatch` objects are guaranteed to be sorted by their
  starting offsets. Note that it is possible for this array to be empty,
  for example, when searching reports inverted matches. If the configuration
  specifies a replacement, the resulting replacement text is also present.

#### Message: **context**

This message indicates that a contextual line has been found. A contextual
line is a line that doesn't contain a match, but is generally adjacent to
a line that does contain a match. The precise way in which contextual lines
are reported is determined by the searcher. It has these fields, which are
exactly the same fields found in a [`match`](#message-match):

* **path** - An
  [arbitrary data object](#object-arbitrary-data)
  representing the file path corresponding to the search, if one is
  present. If no file path is available, then this field is `null`.
* **lines** - An
  [arbitrary data object](#object-arbitrary-data)
  representing one or more lines contained in this context. This includes
  line terminators, if they're present.
* **line_number** - If the searcher has been configured to report line
  numbers, then this corresponds to the line number of the first line
  in `lines`. If no line numbers are available, then this is `null`.
* **absolute_offset** - The absolute byte offset corresponding to the start
  of `lines` in the data being searched.
* **submatches** - An array of [`submatch` objects](#object-submatch)
  corresponding to matches in `lines`. The offsets included in each
  `submatch` correspond to byte offsets into `lines`. (If `lines` is base64
  encoded, then the byte offsets correspond to the data after base64
  decoding.) The `submatch` objects are guaranteed to be sorted by
  their starting offsets. Note that it is possible for this array to be
  non-empty, for example, when searching reports inverted matches such that
  the original matcher could match things in the contextual lines. If the
  configuration specifies a replacemement, the resulting replacement text
  is also present.

#### Object: **submatch**

This object describes submatches found within `match` or `context`
messages. The `start` and `end` fields indicate the half-open interval on
which the match occurs (`start` is included, but `end` is not). It is
guaranteed that `start <= end`. It has these fields:

* **match** - An
  [arbitrary data object](#object-arbitrary-data)
  corresponding to the text in this submatch.
* **start** - A byte offset indicating the start of this match. This offset
  is generally reported in terms of the parent object's data. For example,
  the `lines` field in the
  [`match`](#message-match) or [`context`](#message-context)
  messages.
* **end** - A byte offset indicating the end of this match. This offset
  is generally reported in terms of the parent object's data. For example,
  the `lines` field in the
  [`match`](#message-match) or [`context`](#message-context)
  messages.
* **replacement** (optional) - An
  [arbitrary data object](#object-arbitrary-data) corresponding to the
  replacement text for this submatch, if the configuration specifies
  a replacement.

#### Object: **stats**

This object is included in messages and contains summary statistics about
a search. It has these fields:

* **elapsed** - A [`duration` object](#object-duration) describing the
  length of time that elapsed while performing the search.
* **searches** - The number of searches that have run. For this printer,
  this value is always `1`. (Implementations may emit additional message
  types that use this same `stats` object that represents summary
  statistics over multiple searches.)
* **searches_with_match** - The number of searches that have run that have
  found at least one match. This is never more than `searches`.
* **bytes_searched** - The total number of bytes that have been searched.
* **bytes_printed** - The total number of bytes that have been printed.
  This includes everything emitted by this printer.
* **matched_lines** - The total number of lines that participated in a
  match. When matches may contain multiple lines, then this includes every
  line that is part of every match.
* **matches** - The total number of matches. There may be multiple matches
  per line. When matches may contain multiple lines, each match is counted
  only once, regardless of how many lines it spans.

#### Object: **duration**

This object includes a few fields for describing a duration. Two of its
fields, `secs` and `nanos`, can be combined to give nanosecond precision
on systems that support it. It has these fields:

* **secs** - A whole number of seconds indicating the length of this
  duration.
* **nanos** - A fractional part of this duration represent by nanoseconds.
  If nanosecond precision isn't supported, then this is typically rounded
  up to the nearest number of nanoseconds.
* **human** - A human readable string describing the length of the
  duration. The format of the string is itself unspecified.

#### Object: **arbitrary data**

This object is used whenever arbitrary data needs to be represented as a
JSON value. This object contains two fields, where generally only one of
the fields is present:

* **text** - A normal JSON string that is UTF-8 encoded. This field is
  populated if and only if the underlying data is valid UTF-8.
* **bytes** - A normal JSON string that is a base64 encoding of the
  underlying bytes.

More information on the motivation for this representation can be seen in
the section [text encoding](#text-encoding) above.

## Example

This section shows a small example that includes all message types.

Here's the file we want to search, located at `/home/andrew/sherlock`:

```text
For the Doctor Watsons of this world, as opposed to the Sherlock
Holmeses, success in the province of detective work must always
be, to a very large extent, the result of luck. Sherlock Holmes
can extract a clew from a wisp of straw or a flake of cigar ash;
but Doctor Watson has to have it taken out for him and dusted,
and exhibited clearly, with a label attached.
```

Searching for `Watson` with a `before_context` of `1` with line numbers
enabled shows something like this using the standard printer:

```text
sherlock:1:For the Doctor Watsons of this world, as opposed to the Sherlock
--
sherlock-4-can extract a clew from a wisp of straw or a flake of cigar ash;
sherlock:5:but Doctor Watson has to have it taken out for him and dusted,
```

Here's what the same search looks like using the JSON wire format described
above, where in we show semi-prettified JSON (instead of a strict JSON
Lines format), for illustrative purposes:

```json
{
  "type": "begin",
  "data": {
    "path": {"text": "/home/andrew/sherlock"}}
  }
}
{
  "type": "match",
  "data": {
    "path": {"text": "/home/andrew/sherlock"},
    "lines": {"text": "For the Doctor Watsons of this world, as opposed to the Sherlock\n"},
    "line_number": 1,
    "absolute_offset": 0,
    "submatches": [
      {"match": {"text": "Watson"}, "start": 15, "end": 21}
    ]
  }
}
{
  "type": "context",
  "data": {
    "path": {"text": "/home/andrew/sherlock"},
    "lines": {"text": "can extract a clew from a wisp of straw or a flake of cigar ash;\n"},
    "line_number": 4,
    "absolute_offset": 193,
    "submatches": []
  }
}
{
  "type": "match",
  "data": {
    "path": {"text": "/home/andrew/sherlock"},
    "lines": {"text": "but Doctor Watson has to have it taken out for him and dusted,\n"},
    "line_number": 5,
    "absolute_offset": 258,
    "submatches": [
      {"match": {"text": "Watson"}, "start": 11, "end": 17}
    ]
  }
}
{
  "type": "end",
  "data": {
    "path": {"text": "/home/andrew/sherlock"},
    "binary_offset": null,
    "stats": {
      "elapsed": {"secs": 0, "nanos": 36296, "human": "0.0000s"},
      "searches": 1,
      "searches_with_match": 1,
      "bytes_searched": 367,
      "bytes_printed": 1151,
      "matched_lines": 2,
      "matches": 2
    }
  }
}
```
and here's what a match type item would looks like if a replacement text
of 'Moriarity' was given as a parameter:
```json
{
  "type": "match",
  "data": {
    "path": {"text": "/home/andrew/sherlock"},
    "lines": {"text": "For the Doctor Watsons of this world, as opposed to the Sherlock\n"},
    "line_number": 1,
    "absolute_offset": 0,
    "submatches": [
      {"match": {"text": "Watson"}, "replacement": {"text": "Moriarity"}, "start": 15, "end": 21}
    ]
  }
}
```

---

## JSONBuilder

`struct` · `grep_printer::json::JSONBuilder`

Also reachable as `grep_printer::JSONBuilder`

```rust
struct JSONBuilder
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn always_begin_end(&mut self, yes: bool) -> &mut JSONBuilder
fn build<W: io::Write>(&self, wtr: W) -> JSON<W>
fn new() -> JSONBuilder
fn pretty(&mut self, yes: bool) -> &mut JSONBuilder
fn replacement(&mut self, replacement: Option<Vec<u8>>) -> &mut JSONBuilder
```

A builder for a JSON lines printer.

The builder permits configuring how the printer behaves. The JSON printer
has fewer configuration options than the standard printer because it is
a structured format, and the printer always attempts to find the most
information possible.

Some configuration options, such as whether line numbers are included or
whether contextual lines are shown, are drawn directly from the
`grep_searcher::Searcher`'s configuration.

Once a `JSON` printer is built, its configuration cannot be changed.

---

## JSONSink

`struct` · `grep_printer::json::JSONSink`

Also reachable as `grep_printer::JSONSink`

```rust
struct JSONSink<'p, 's, M: Matcher, W>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Debug

**Methods** (4)

```rust
fn binary_byte_offset(&self) -> Option<u64>
fn has_match(&self) -> bool
fn match_count(&self) -> u64
fn stats(&self) -> &Stats
```

**via `grep_searcher::sink::Sink`**

```rust
fn begin(&mut self, _searcher: &Searcher) -> Result<bool, io::Error>
fn binary_data(&mut self, searcher: &Searcher, binary_byte_offset: u64) -> Result<bool, io::Error>
fn context(&mut self, searcher: &Searcher, ctx: &SinkContext<'_>) -> Result<bool, io::Error>
fn finish(&mut self, _searcher: &Searcher, finish: &SinkFinish) -> Result<(), io::Error>
fn matched(&mut self, searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

An implementation of `Sink` associated with a matcher and an optional file
path for the JSON printer.

This type is generic over a few type parameters:

* `'p` refers to the lifetime of the file path, if one is provided. When
no file path is given, then this is `'static`.
* `'s` refers to the lifetime of the [`JSON`] printer that this type
borrows.
* `M` refers to the type of matcher used by
`grep_searcher::Searcher` that is reporting results to this sink.
* `W` refers to the underlying writer that this printer is writing its
output to.

---
