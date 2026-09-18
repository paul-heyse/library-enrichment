# `arrow_flight::streams::FallibleRequestStream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.streams.FallibleRequestStream.json).

<a id="op-0446d0228eea9234dd6a0289"></a>
## FallibleRequestStream

`struct` · `arrow_flight::streams::FallibleRequestStream` · arrow-flight 59.3.0

```rust
struct FallibleRequestStream<T, E>
```

Source: `src/streams.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Wrapper around a fallible stream (one that returns errors) that makes it infallible.

Any errors encountered in the stream are ignored are sent to the provided
oneshot sender.

This can be used to accept a stream of `Result<_>` from a client API and send
them to the remote server that wants only the successful results.

<a id="op-eb4e31b7fcb60f827130c37a"></a>
## Item

`assoc_type` · `arrow_flight::streams::FallibleRequestStream::Item` · arrow-flight 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_flight::streams::FallibleRequestStream", "path": "FallibleRequestStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [80, 2], "filename": "src/streams.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/streams.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb93f20da42345825cf34354"></a>
## new

`function` · `arrow_flight::streams::FallibleRequestStream::new` · arrow-flight 59.3.0

```rust
fn new(sender: Sender<E>, fallible_stream: Pin<Box<dyn Stream<Item = std::result::Result<T, E>> + Send + 'static>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_flight::streams::FallibleRequestStream", "path": "FallibleRequestStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [53, 2], "filename": "src/streams.rs"}, "trait": null, "trait_path": null}`

Source: `src/streams.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Create a FallibleRequestStream

<a id="op-2ef94f7e4344d421d886cb9a"></a>
## poll_next

`function` · `arrow_flight::streams::FallibleRequestStream::poll_next` · arrow-flight 59.3.0

```rust
fn poll_next(std::pin::Pin<&mut self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_flight::streams::FallibleRequestStream", "path": "FallibleRequestStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [80, 2], "filename": "src/streams.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/streams.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
