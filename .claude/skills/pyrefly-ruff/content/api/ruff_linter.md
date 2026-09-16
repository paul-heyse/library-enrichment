# `ruff_linter`

Crate `ruff_linter` · 8 public items · structured records in [`model/ruff_linter.json`](../model/ruff_linter.json)

## RUFF_PKG_VERSION

`constant` · `ruff_linter::RUFF_PKG_VERSION`

```rust
const RUFF_PKG_VERSION: &str = "0.16.7"
```

---

## VERSION

`constant` · `ruff_linter::VERSION`

```rust
const VERSION: &str = "0.16.7"
```

---

## display_settings

`macro` · `ruff_linter::display_settings`

```rust
macro_rules! display_settings
```

`display_settings!` is a macro that can display and format struct fields in a readable,
namespaced format. It's particularly useful at generating `Display` implementations
for types used in settings.

# Example
```
use std::fmt;
use ruff_linter::display_settings;
#[derive(Default)]
struct Settings {
    option_a: bool,
    sub_settings: SubSettings,
    option_b: String,
}

struct SubSettings {
    name: String
}

impl Default for SubSettings {
    fn default() -> Self {
        Self { name: "Default Name".into() }
    }

}

impl fmt::Display for SubSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_settings! {
            formatter = f,
            namespace = "sub_settings",
            fields = [
                self.name | quoted
            ]
        }
        Ok(())
    }

}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_settings! {
            formatter = f,
            fields = [
                self.option_a,
                self.sub_settings | nested,
                self.option_b | quoted,
            ]
        }
        Ok(())
    }

}

const EXPECTED_OUTPUT: &str = r#"option_a = false
sub_settings.name = "Default Name"
option_b = ""
"#;

fn main() {
    let settings = Settings::default();
    assert_eq!(format!("{settings}"), EXPECTED_OUTPUT);
}
```

---

## notify_user

`macro` · `ruff_linter::notify_user`

```rust
macro_rules! notify_user
```

---

## warn_user

`macro` · `ruff_linter::warn_user`

```rust
macro_rules! warn_user
```

---

## warn_user_once

`macro` · `ruff_linter::warn_user_once`

```rust
macro_rules! warn_user_once
```

Warn a user once, with uniqueness determined by the calling location itself.

---

## warn_user_once_by_id

`macro` · `ruff_linter::warn_user_once_by_id`

```rust
macro_rules! warn_user_once_by_id
```

Warn a user once, with uniqueness determined by the given ID.

---

## warn_user_once_by_message

`macro` · `ruff_linter::warn_user_once_by_message`

```rust
macro_rules! warn_user_once_by_message
```

Warn a user once, if warnings are enabled, with uniqueness determined by the content of the
message.

---
