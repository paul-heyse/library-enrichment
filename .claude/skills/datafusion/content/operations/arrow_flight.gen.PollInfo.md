# `arrow_flight::gen::PollInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.gen.PollInfo.json).

<a id="op-a5f3704ac23735382e4c2af6"></a>
## PollInfo

`struct` · `arrow_flight::gen::PollInfo` · arrow-flight 59.3.0

```rust
struct PollInfo
```

Source: `src/arrow.flight.protocol.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The information to process a long-running query.

<a id="op-1e78b9a8a741a68729a34e24"></a>
## clear

`function` · `arrow_flight::gen::PollInfo::clear` · arrow-flight 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 28], "end": [242, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be3d2a5274c6421e1bfa5ff5"></a>
## clone

`function` · `arrow_flight::gen::PollInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> PollInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 10], "end": [242, 15], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddc8dc7004f843d8f844e821"></a>
## default

`function` · `arrow_flight::gen::PollInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 28], "end": [242, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03f67d58874ec5832c4e4257"></a>
## encoded_len

`function` · `arrow_flight::gen::PollInfo::encoded_len` · arrow-flight 59.3.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 28], "end": [242, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e5b3dcef193c6cc583fa7d8"></a>
## eq

`function` · `arrow_flight::gen::PollInfo::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &PollInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 17], "end": [242, 26], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0bef64d9e7c6349327466f6"></a>
## expiration_time

`struct_field` · `arrow_flight::gen::PollInfo::expiration_time` · arrow-flight 59.3.0

```rust
expiration_time: ::core::option::Option<::prost_types::Timestamp>
```

Source: `src/arrow.flight.protocol.rs:278`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Expiration time for this request. After this passes, the server
might not accept the retry descriptor anymore (and the query may
be cancelled). This may be updated on a call to PollFlightInfo.

<a id="op-3af473d3c0068f811f32e360"></a>
## flight_descriptor

`struct_field` · `arrow_flight::gen::PollInfo::flight_descriptor` · arrow-flight 59.3.0

```rust
flight_descriptor: ::core::option::Option<FlightDescriptor>
```

Source: `src/arrow.flight.protocol.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The descriptor the client should use on the next try.
If unset, the query is complete.

<a id="op-67ca84998f27189fc91d3b69"></a>
## fmt

`function` · `arrow_flight::gen::PollInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 28], "end": [242, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d300fa8f1c0dbf866d31def1"></a>
## fmt

`function` · `arrow_flight::gen::PollInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [314, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/lib.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-558a03717a8dcf2daa9e7f43"></a>
## info

`struct_field` · `arrow_flight::gen::PollInfo::info` · arrow-flight 59.3.0

```rust
info: ::core::option::Option<FlightInfo>
```

Source: `src/arrow.flight.protocol.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


The currently available results.

If "flight_descriptor" is not specified, the query is complete
and "info" specifies all results. Otherwise, "info" contains
partial query results.

Note that each PollInfo response contains a complete
FlightInfo (not just the delta between the previous and current
FlightInfo).

Subsequent PollInfo responses may only append new endpoints to
info.

Clients can begin fetching results via DoGet(Ticket) with the
ticket in the info before the query is
completed. FlightInfo.ordered is also valid.

<a id="op-9fd5e049e5644450672a3195"></a>
## new

`function` · `arrow_flight::gen::PollInfo::new` · arrow-flight 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [701, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:662`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a new, empty [`PollInfo`](../operations/arrow_flight.gen.PollInfo.md#op-a5f3704ac23735382e4c2af6), providing information for a long-running query

# Example:
```
# use arrow_flight::{FlightInfo, PollInfo, FlightDescriptor};
# use prost_types::Timestamp;
// Create a new PollInfo
let poll_info = PollInfo::new()
  .with_info(FlightInfo::new())
  .with_descriptor(FlightDescriptor::new_cmd("RUN QUERY"))
  .try_with_progress(0.5)
  .expect("progress should've been valid")
  .with_expiration_time(
    "1970-01-01".parse().expect("invalid timestamp")
  );
```

<a id="op-5f3fa55575f25695713aaa08"></a>
## progress

`struct_field` · `arrow_flight::gen::PollInfo::progress` · arrow-flight 59.3.0

```rust
progress: ::core::option::Option<f64>
```

Source: `src/arrow.flight.protocol.rs:272`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Query progress. If known, must be in \[0.0, 1.0\] but need not be
monotonic or nondecreasing. If unknown, do not set.

<a id="op-e692bcfccf55815fd46e30e0"></a>
## progress

`function` · `arrow_flight::gen::PollInfo::progress` · arrow-flight 59.3.0

```rust
fn progress(&self) -> f64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 28], "end": [242, 44], "filename": "src/arrow.flight.protocol.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow.flight.protocol.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns the value of `progress`, or the default value if `progress` is unset.

<a id="op-07c20507c292fb300e75b5cb"></a>
## try_with_progress

`function` · `arrow_flight::gen::PollInfo::try_with_progress` · arrow-flight 59.3.0

```rust
fn try_with_progress(self, progress: f64) -> std::result::Result<Self, arrow_schema::ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [701, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:686`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Set the query progress if known. Must be in the range [0.0, 1.0] else this will
return an error

<a id="op-7debdd05d13a3c9a124760b3"></a>
## with_descriptor

`function` · `arrow_flight::gen::PollInfo::with_descriptor` · arrow-flight 59.3.0

```rust
fn with_descriptor(self, flight_descriptor: FlightDescriptor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [701, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:679`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add a [`FlightDescriptor`](../operations/arrow_flight.gen.FlightDescriptor.md#op-87317d059af665e5e2fe8326) that the client should use for the next poll call,
if the query is not yet complete

<a id="op-5d55a8a3e5c8045a1f6f3dff"></a>
## with_expiration_time

`function` · `arrow_flight::gen::PollInfo::with_expiration_time` · arrow-flight 59.3.0

```rust
fn with_expiration_time(self, expiration_time: Timestamp) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [701, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:697`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Specify expiration time for this request

<a id="op-384fe49442387604a493f97e"></a>
## with_info

`function` · `arrow_flight::gen::PollInfo::with_info` · arrow-flight 59.3.0

```rust
fn with_info(self, info: FlightInfo) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::gen::PollInfo", "path": "PollInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [701, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:672`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Add the current available results for the poll call as a [`FlightInfo`](../operations/arrow_flight.gen.FlightInfo.md#op-422a7589713733d24ed1403b)
