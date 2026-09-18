# `arrow_json::reader::value_iter`

Crate `arrow-json` · 1 public items · structured records in [`model/arrow_json.reader.value_iter.json`](../model/arrow_json.reader.value_iter.json)

## ValueIter

`struct` · `arrow_json::reader::value_iter::ValueIter`

Also reachable as `arrow_json::reader::ValueIter`

```rust
struct ValueIter<R: BufRead>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (2)

```rust
fn new(reader: R, max_read_records: Option<usize>) -> Self
fn record_count(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.reader.value_iter.ValueIter.md).


JSON file reader that produces a serde_json::Value iterator from a Read trait

# Example

```
use std::fs::File;
use std::io::BufReader;
use arrow_json::reader::ValueIter;

let mut reader =
    BufReader::new(File::open("test/data/mixed_arrays.json").unwrap());
let mut value_reader = ValueIter::new(&mut reader, None);
for value in value_reader {
    println!("JSON value: {}", value.unwrap());
}
```

---
