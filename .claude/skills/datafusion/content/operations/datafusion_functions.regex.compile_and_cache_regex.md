# `datafusion_functions::regex::compile_and_cache_regex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.regex.compile_and_cache_regex.json).

<a id="op-01e9a64171de372a5a77ce78"></a>
## compile_and_cache_regex

`function` · `datafusion_functions::regex::compile_and_cache_regex` · datafusion-functions 55.1.0

```rust
fn compile_and_cache_regex<'strings, 'cache>(regex: &'strings str, flags: Option<&'strings str>, regex_cache: &'cache mut std::collections::HashMap<(&'strings str, Option<&'strings str>), regex::Regex>) -> Result<&'cache regex::Regex, arrow::error::ArrowError> where 'strings: 'cache
```

Source: `src/regex/mod.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
