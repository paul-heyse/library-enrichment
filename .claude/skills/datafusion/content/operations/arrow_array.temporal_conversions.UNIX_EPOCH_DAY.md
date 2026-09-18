# `arrow_array::temporal_conversions::UNIX_EPOCH_DAY`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.temporal_conversions.UNIX_EPOCH_DAY.json).

<a id="op-912fc5d8d7c6d559934a67fd"></a>
## UNIX_EPOCH_DAY

`constant` · `arrow_array::temporal_conversions::UNIX_EPOCH_DAY` · arrow-array 59.3.0

```rust
const UNIX_EPOCH_DAY: i64 = 719_163
```

Source: `src/temporal_conversions.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constant from chrono crate

Number of days between Januari 1, 1970 and December 31, 1 BCE which we define to be day 0.
4 full leap year cycles until December 31, 1600     4 * 146097 = 584388
1 day until January 1, 1601                                           1
369 years until Januari 1, 1970                      369 * 365 = 134685
of which floor(369 / 4) are leap years          floor(369 / 4) =     92
except for 1700, 1800 and 1900                                       -3 +
                                                                 --------
                                                                 719163
