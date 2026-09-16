# `pyrefly_util::test_path`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.test_path.json`](../model/pyrefly_util.test_path.json)

## TestPath

`struct` · `pyrefly_util::test_path::TestPath`

```rust
struct TestPath
```

**Methods** (4)

```rust
fn dir(name: &str, children: Vec<TestPath>) -> Self
fn file(name: &str) -> Self
fn file_with_contents(name: &str, contents: &str) -> Self
fn setup_test_directory(root: &Path, paths: Vec<TestPath>)
```

---
