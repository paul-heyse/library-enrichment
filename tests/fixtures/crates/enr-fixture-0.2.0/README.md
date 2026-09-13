# enr-fixture

A deterministic fixture crate for the library-enrichment acceptance gates. It is not useful
software; every item exists to exercise one gate.

## Features

- `std` (default): no effect on the API; present so the default set is non-empty.
- `extra`: enables `extra_only()`.

## Usage

```rust
use enr_fixture::{Shape, Widget, perimeter};
let w = Widget::new(3);
assert_eq!(w.area(), 9);
assert_eq!(perimeter(&w), 12);
```
