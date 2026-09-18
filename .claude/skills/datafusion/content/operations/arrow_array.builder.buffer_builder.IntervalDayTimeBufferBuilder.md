# `arrow_array::builder::buffer_builder::IntervalDayTimeBufferBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.buffer_builder.IntervalDayTimeBufferBuilder.json).

<a id="op-be400896985ba78513987f86"></a>
## IntervalDayTimeBufferBuilder

`type_alias` · `arrow_array::builder::buffer_builder::IntervalDayTimeBufferBuilder` · arrow-array 59.3.0

```rust
type IntervalDayTimeBufferBuilder = BufferBuilder<<IntervalDayTimeType as ArrowPrimitiveType>::Native>
```

Source: `src/builder/buffer_builder.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Buffer builder for “calendar” interval in days and milliseconds.
