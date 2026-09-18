# `arrow_select::concat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.concat.json).

<a id="op-604d736a48696a128ac87a9c"></a>
## concat

`module` · `arrow_select::concat` · arrow-select 59.3.0

```rust
mod concat
```

Source: `src/concat.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Defines concat kernel for `ArrayRef`

Example:

```
use arrow_array::{ArrayRef, StringArray};
use arrow_select::concat::concat;

let arr = concat(&[
    &StringArray::from(vec!["hello", "world"]),
    &StringArray::from(vec!["!"]),
]).unwrap();
assert_eq!(arr.len(), 3);
```
