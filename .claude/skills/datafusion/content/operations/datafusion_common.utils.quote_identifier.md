# `datafusion_common::utils::quote_identifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.quote_identifier.json).

<a id="op-64adbb5ba890f93a202b3b1a"></a>
## quote_identifier

`function` · `datafusion_common::utils::quote_identifier` · datafusion-common 55.1.0

```rust
fn quote_identifier(s: &str) -> std::borrow::Cow<'_, str>
```

Source: `src/utils/mod.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wraps identifier string in double quotes, escaping any double quotes in
the identifier by replacing it with two double quotes

e.g. identifier `tab.le"name` becomes `"tab.le""name"`
