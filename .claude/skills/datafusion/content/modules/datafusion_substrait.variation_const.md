# `datafusion_substrait::variation_const`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.variation_const.json).

<a id="op-cafe8d0ca414b6989cdfa368"></a>
## variation_const

`module` · `datafusion_substrait::variation_const` · datafusion-substrait 55.1.0

```rust
mod variation_const
```

Source: `src/variation_const.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Type variation constants

To add support for types not in the [core specification](https://substrait.io/types/type_classes/),
we make use of the [simple extensions] of substrait type. This module contains the constants used
to identify the type variation.

The rules of type variations here are:
- Default type reference is 0. It is used when the actual type is the same with the original type.
- Extended variant type references start from 1, and usually increase by 1.

TODO: Definitions here are not the final form. All the non-system-preferred variations will be defined
using [simple extensions] as per the [spec of type_variations](https://substrait.io/types/type_variations/)
<https://github.com/apache/datafusion/issues/11545>

[simple extensions]: (https://substrait.io/extensions/#simple-extensions)
