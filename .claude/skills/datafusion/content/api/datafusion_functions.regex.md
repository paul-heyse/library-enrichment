# `datafusion_functions::regex`

Crate `datafusion-functions` · 8 public items · structured records in [`model/datafusion_functions.regex.json`](../model/datafusion_functions.regex.json)

## compile_and_cache_regex

`function` · `datafusion_functions::regex::compile_and_cache_regex`

```rust
fn compile_and_cache_regex<'strings, 'cache>(regex: &'strings str, flags: Option<&'strings str>, regex_cache: &'cache mut std::collections::HashMap<(&'strings str, Option<&'strings str>), regex::Regex>) -> Result<&'cache regex::Regex, arrow::error::ArrowError> where 'strings: 'cache
```

---

## compile_regex

`function` · `datafusion_functions::regex::compile_regex`

```rust
fn compile_regex(regex: &str, flags: Option<&str>) -> Result<regex::Regex, arrow::error::ArrowError>
```

---

## functions

`function` · `datafusion_functions::regex::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

Returns all DataFusion functions defined in this package

---

## regexp_count

`function` · `datafusion_functions::regex::regexp_count`

```rust
fn regexp_count() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_count

---

## regexp_instr

`function` · `datafusion_functions::regex::regexp_instr`

```rust
fn regexp_instr() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_instr

---

## regexp_like

`function` · `datafusion_functions::regex::regexp_like`

```rust
fn regexp_like() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_like

---

## regexp_match

`function` · `datafusion_functions::regex::regexp_match`

```rust
fn regexp_match() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_match

---

## regexp_replace

`function` · `datafusion_functions::regex::regexp_replace`

```rust
fn regexp_replace() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_replace

---
