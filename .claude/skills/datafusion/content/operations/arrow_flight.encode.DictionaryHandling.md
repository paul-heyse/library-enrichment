# `arrow_flight::encode::DictionaryHandling`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.encode.DictionaryHandling.json).

<a id="op-3909e2ed806a45e433fc92ad"></a>
## DictionaryHandling

`enum` · `arrow_flight::encode::DictionaryHandling` · arrow-flight 59.3.0

```rust
enum DictionaryHandling
```

Source: `src/encode.rs:469`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Defines how a [`FlightDataEncoder`](../operations/arrow_flight.encode.FlightDataEncoder.md#op-e4d1d137c901fe86782ae657) encodes [`DictionaryArray`]s

[`DictionaryArray`]: arrow_array::DictionaryArray

In the arrow flight protocol dictionary values and keys are sent as two separate messages.
When a sender is encoding a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) containing ['DictionaryArray'] columns, it will
first send a dictionary batch (a batch with header `MessageHeader::DictionaryBatch`) containing
the dictionary values. The receiver is responsible for reading this batch and maintaining state that associates
those dictionary values with the corresponding array using the `dict_id` as a key.

After sending the dictionary batch the sender will send the array data in a batch with header `MessageHeader::RecordBatch`.
For any dictionary array batches in this message, the encoded flight message will only contain the dictionary keys. The receiver
is then responsible for rebuilding the `DictionaryArray` on the client side using the dictionary values from the DictionaryBatch message
and the keys from the RecordBatch message.

For example, if we have a batch with a `TypedDictionaryArray<'_, UInt32Type, Utf8Type>` (a dictionary array where they keys are `u32` and the
values are `String`), then the DictionaryBatch will contain a `StringArray` and the RecordBatch will contain a `UInt32Array`.

Note that since `dict_id` defined in the `Schema` is used as a key to associate dictionary values to their arrays it is required that each
`DictionaryArray` in a `RecordBatch` have a unique `dict_id`.

The current implementation does not support "delta" dictionaries so a new dictionary batch will be sent each time the encoder sees a
dictionary which is not pointer-equal to the previously observed dictionary for a given `dict_id`.

For clients which may not support `DictionaryEncoding`, the `DictionaryHandling::Hydrate` method will bypass the process defined above
and "hydrate" any `DictionaryArray` in the batch to their underlying value type (e.g. `TypedDictionaryArray<'_, UInt32Type, Utf8Type>` will
be sent as a `StringArray`). With this method all data will be sent in ``MessageHeader::RecordBatch` messages and the batch schema
will be adjusted so that all dictionary encoded fields are changed to fields of the dictionary value type.

<a id="op-1274858835cec039b729e6f3"></a>
## Hydrate

`variant` · `arrow_flight::encode::DictionaryHandling::Hydrate` · arrow-flight 59.3.0

```rust
Hydrate
```

Source: `src/encode.rs:477`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Expands to the underlying type (default). This likely sends more data
over the network but requires less memory (dictionaries are not tracked)
and is more compatible with other arrow flight client implementations
that may not support `DictionaryEncoding`

See also:
* <https://github.com/apache/arrow-rs/issues/1206>

<a id="op-4ca86ac204d7df4411001d3a"></a>
## Resend

`variant` · `arrow_flight::encode::DictionaryHandling::Resend` · arrow-flight 59.3.0

```rust
Resend
```

Source: `src/encode.rs:487`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Send dictionary FlightData with every RecordBatch that contains a
[`DictionaryArray`]. See [`Self::Hydrate`](../operations/arrow_flight.encode.DictionaryHandling.md#op-1274858835cec039b729e6f3) for more tradeoffs. No
attempt is made to skip sending the same (logical) dictionary values
twice.

[`DictionaryArray`]: arrow_array::DictionaryArray

This requires identifying the different dictionaries in use and assigning

<a id="op-cbcbe2d1bf34fbff75a15572"></a>
## eq

`function` · `arrow_flight::encode::DictionaryHandling::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &DictionaryHandling) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 17], "end": [468, 26], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/encode.rs:468`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-764d17a263dacbc832f2912b"></a>
## fmt

`function` · `arrow_flight::encode::DictionaryHandling::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::encode::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [468, 10], "end": [468, 15], "filename": "src/encode.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/encode.rs:468`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
