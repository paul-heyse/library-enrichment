# `ruff_linter::packaging`

Crate `ruff_linter` · 2 public items · structured records in [`model/ruff_linter.packaging.json`](../model/ruff_linter.packaging.json)

## detect_package_root

`function` · `ruff_linter::packaging::detect_package_root`

```rust
fn detect_package_root<'a>(path: &'a std::path::Path, namespace_packages: &[std::path::PathBuf]) -> Option<&'a std::path::Path>
```

Return the package root for the given path to a directory with Python file.

---

## is_package

`function` · `ruff_linter::packaging::is_package`

```rust
fn is_package(path: &std::path::Path, namespace_packages: &[std::path::PathBuf]) -> bool
```

Return `true` if the directory at the given `Path` appears to be a Python
package.

---
