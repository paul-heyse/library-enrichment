# `object_store::Error::NotImplemented`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.NotImplemented.json).

<a id="op-ac250888db655b752004c718"></a>
## implementer

`struct_field` · `object_store::Error::NotImplemented::implementer` · object_store 0.13.2

```rust
implementer: String
```

Source: `src/lib.rs:2101`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Which driver this is that hasn't implemented this operation,
to aid debugging in contexts that may be using multiple implementations.

<a id="op-d634ba822870edee5721e58d"></a>
## operation

`struct_field` · `object_store::Error::NotImplemented::operation` · object_store 0.13.2

```rust
operation: String
```

Source: `src/lib.rs:2097`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

What isn't implemented. Should include at least the method
name that was called; could also include other relevant
subcontexts.
