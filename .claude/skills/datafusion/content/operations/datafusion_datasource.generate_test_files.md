# `datafusion_datasource::generate_test_files`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.generate_test_files.json).

<a id="op-8d5910b8929c6dad4cb0d576"></a>
## generate_test_files

`function` · `datafusion_datasource::generate_test_files` · datafusion-datasource 55.1.0

```rust
fn generate_test_files(num_files: usize, overlap_factor: f64) -> Vec<file_groups::FileGroup>
```

Source: `src/mod.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Generates test files with min-max statistics in different overlap patterns.

Used by tests and benchmarks.

# Overlap Factors

The `overlap_factor` parameter controls how much the value ranges in generated test files overlap:
- `0.0`: No overlap between files (completely disjoint ranges)
- `0.2`: Low overlap (20% of the range size overlaps with adjacent files)
- `0.5`: Medium overlap (50% of ranges overlap)
- `0.8`: High overlap (80% of ranges overlap between files)

# Examples

With 5 files and different overlap factors showing `[min, max]` ranges:

overlap_factor = 0.0 (no overlap):

File 0: [0, 20]
File 1: [20, 40]
File 2: [40, 60]
File 3: [60, 80]
File 4: [80, 100]

overlap_factor = 0.5 (50% overlap):

File 0: [0, 40]
File 1: [20, 60]
File 2: [40, 80]
File 3: [60, 100]
File 4: [80, 120]

overlap_factor = 0.8 (80% overlap):

File 0: [0, 100]
File 1: [20, 120]
File 2: [40, 140]
File 3: [60, 160]
File 4: [80, 180]
