# `arrow_ipc::gen::Schema::Timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Timestamp.json).

<a id="op-8688b397937d9b1199c7c53f"></a>
## Timestamp

`struct` · `arrow_ipc::gen::Schema::Timestamp` · arrow-ipc 59.3.0

```rust
struct Timestamp<'a>
```

Source: `src/gen/Schema.rs:3601`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Timestamp is a 64-bit signed integer representing an elapsed time since a
fixed epoch, stored in either of four units: seconds, milliseconds,
microseconds or nanoseconds, and is optionally annotated with a timezone.

Timestamp values do not include any leap seconds (in other words, all
days are considered 86400 seconds long).

Timestamps with a non-empty timezone
------------------------------------

If a Timestamp column has a non-empty timezone value, its epoch is
1970-01-01 00:00:00 (January 1st 1970, midnight) in the *UTC* timezone
(the Unix epoch), regardless of the Timestamp's own timezone.

Therefore, timestamp values with a non-empty timezone correspond to
physical points in time together with some additional information about
how the data was obtained and/or how to display it (the timezone).

  For example, the timestamp value 0 with the timezone string "Europe/Paris"
  corresponds to "January 1st 1970, 00h00" in the UTC timezone, but the
  application may prefer to display it as "January 1st 1970, 01h00" in
  the Europe/Paris timezone (which is the same physical point in time).

One consequence is that timestamp values with a non-empty timezone
can be compared and ordered directly, since they all share the same
well-known point of reference (the Unix epoch).

Timestamps with an unset / empty timezone
-----------------------------------------

If a Timestamp column has no timezone value, its epoch is
1970-01-01 00:00:00 (January 1st 1970, midnight) in an *unknown* timezone.

Therefore, timestamp values without a timezone cannot be meaningfully
interpreted as physical points in time, but only as calendar / clock
indications ("wall clock time") in an unspecified timezone.

  For example, the timestamp value 0 with an empty timezone string
  corresponds to "January 1st 1970, 00h00" in an unknown timezone: there
  is not enough information to interpret it as a well-defined physical
  point in time.

One consequence is that timestamp values without a timezone cannot
be reliably compared or ordered, since they may have different points of
reference.  In particular, it is *not* possible to interpret an unset
or empty timezone as the same as "UTC".

Conversion between timezones
----------------------------

If a Timestamp column has a non-empty timezone, changing the timezone
to a different non-empty value is a metadata-only operation:
the timestamp values need not change as their point of reference remains
the same (the Unix epoch).

However, if a Timestamp column has no timezone value, changing it to a
non-empty value requires to think about the desired semantics.
One possibility is to assume that the original timestamp values are
relative to the epoch of the timezone being set; timestamp values should
then adjusted to the Unix epoch (for example, changing the timezone from
empty to "Europe/Paris" would require converting the timestamp values
from "Europe/Paris" to "UTC", which seems counter-intuitive but is
nevertheless correct).

Guidelines for encoding data from external libraries
----------------------------------------------------

Date & time libraries often have multiple different data types for temporal
data. In order to ease interoperability between different implementations the
Arrow project has some recommendations for encoding these types into a Timestamp
column.

An "instant" represents a physical point in time that has no relevant timezone
(for example, astronomical data). To encode an instant, use a Timestamp with
the timezone string set to "UTC", and make sure the Timestamp values
are relative to the UTC epoch (January 1st 1970, midnight).

A "zoned date-time" represents a physical point in time annotated with an
informative timezone (for example, the timezone in which the data was
recorded).  To encode a zoned date-time, use a Timestamp with the timezone
string set to the name of the timezone, and make sure the Timestamp values
are relative to the UTC epoch (January 1st 1970, midnight).

 (There is some ambiguity between an instant and a zoned date-time with the
  UTC timezone.  Both of these are stored the same in Arrow.  Typically,
  this distinction does not matter.  If it does, then an application should
  use custom metadata or an extension type to distinguish between the two cases.)

An "offset date-time" represents a physical point in time combined with an
explicit offset from UTC.  To encode an offset date-time, use a Timestamp
with the timezone string set to the numeric timezone offset string
(e.g. "+03:00"), and make sure the Timestamp values are relative to
the UTC epoch (January 1st 1970, midnight).

A "naive date-time" (also called "local date-time" in some libraries)
represents a wall clock time combined with a calendar date, but with
no indication of how to map this information to a physical point in time.
Naive date-times must be handled with care because of this missing
information, and also because daylight saving time (DST) may make
some values ambiguous or nonexistent. A naive date-time may be
stored as a struct with Date and Time fields. However, it may also be
encoded into a Timestamp column with an empty timezone. The timestamp
values should be computed "as if" the timezone of the date-time values
was UTC; for example, the naive date-time "January 1st 1970, 00h00" would
be encoded as timestamp value 0.

<a id="op-aa15e3fd250a59310b8c00b3"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Timestamp::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3605, 1], "end": [3613, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3606`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e96fd1a211b855d8e215c9f"></a>
## VT_TIMEZONE

`assoc_const` · `arrow_ipc::gen::Schema::Timestamp::VT_TIMEZONE` · arrow-ipc 59.3.0

```rust
VT_TIMEZONE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3617`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e7bd093c11ecfef192bac4d"></a>
## VT_UNIT

`assoc_const` · `arrow_ipc::gen::Schema::Timestamp::VT_UNIT` · arrow-ipc 59.3.0

```rust
VT_UNIT
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3616`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28f875dffd5f104eb1f14df3"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::Timestamp::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:3602`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d5961b55eef793cb7112dd7"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Timestamp::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Timestamp<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 16], "end": [3494, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:3494`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1e53651e06cc3e959a74ff9"></a>
## create

`function` · `arrow_ipc::gen::Schema::Timestamp::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args TimestampArgs<'args>) -> flatbuffers::WIPOffset<Timestamp<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3624`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52895c5912b1638f286fd1dc"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Timestamp::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Timestamp<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3494, 23], "end": [3494, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:3494`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47fbb0465f7a716b26c24e62"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Timestamp::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3731, 1], "end": [3738, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:3732`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-593a36753faf7a1477de4715"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Timestamp::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3605, 1], "end": [3613, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3608`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8d839b1f492b78d2e3d90b"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::Timestamp::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3620`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fef024069fca8669025f5af7"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Timestamp::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3669, 1], "end": [3686, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:3671`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3035c3ae4ed0c70cc2ed6f64"></a>
## timezone

`function` · `arrow_ipc::gen::Schema::Timestamp::timezone` · arrow-ipc 59.3.0

```rust
fn timezone(&self) -> Option<&'a str>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3658`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

The timezone is an optional string indicating the name of a timezone,
one of:

* As used in the Olson timezone database (the "tz database" or
  "tzdata"), such as "America/New_York".
* An absolute timezone offset of the form "+XX:XX" or "-XX:XX",
  such as "+07:30".

Whether a timezone string is present indicates different semantics about
the data (see above).

<a id="op-53856511fb1e112859b06260"></a>
## unit

`function` · `arrow_ipc::gen::Schema::Timestamp::unit` · arrow-ipc 59.3.0

```rust
fn unit(&self) -> TimeUnit
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Timestamp", "path": "Timestamp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3615, 1], "end": [3667, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3637`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
