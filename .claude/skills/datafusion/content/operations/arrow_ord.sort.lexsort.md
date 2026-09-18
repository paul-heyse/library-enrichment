# `arrow_ord::sort::lexsort`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.sort.lexsort.json).

<a id="op-2bba670a3e4590e6f14d3edf"></a>
## lexsort

`function` · `arrow_ord::sort::lexsort` · arrow-ord 59.3.0

```rust
fn lexsort(columns: &[SortColumn], limit: Option<usize>) -> Result<Vec<ArrayRef>, arrow_schema::ArrowError>
```

Source: `src/sort.rs:927`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Sort a list of `ArrayRef` using `SortOptions` provided for each array.

Performs an unstable lexicographical sort on values and indices.

Returns an `ArrowError::ComputeError(String)` if any of the array type is either unsupported by
`lexsort_to_indices` or `take`.

# Example:

```
# use std::convert::From;
# use std::sync::Arc;
# use arrow_array::{ArrayRef, StringArray, PrimitiveArray};
# use arrow_array::types::Int64Type;
# use arrow_array::cast::AsArray;
# use arrow_ord::sort::{SortColumn, SortOptions, lexsort};
let sorted_columns = lexsort(&vec![
    SortColumn {
        values: Arc::new(PrimitiveArray::<Int64Type>::from(vec![
            None,
            Some(-2),
            Some(89),
            Some(-64),
            Some(101),
        ])) as ArrayRef,
        options: None,
    },
    SortColumn {
        values: Arc::new(StringArray::from(vec![
            Some("hello"),
            Some("world"),
            Some(","),
            Some("foobar"),
            Some("!"),
        ])) as ArrayRef,
        options: Some(SortOptions {
            descending: true,
            nulls_first: false,
        }),
    },
], None).unwrap();

assert_eq!(sorted_columns[0].as_primitive::<Int64Type>().value(1), -64);
assert!(sorted_columns[0].is_null(0));
```

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/)
may be significantly faster

