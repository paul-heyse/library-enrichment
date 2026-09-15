# Research/2.0 provenance seal — 2026-09-15

Authority: ADR-0036/0037 and the scoped research-v2 contract review. This is a contract seal,
not a claim of completed Plan 13 implementation or deployment.

The original bundle-2026-09-13 manifest and path map remain byte-for-byte unchanged.
PREDECESSOR_PATHMAP.tsv changes only the locations of its two superseded product-skill files.
Their bytes were recovered from git commit 2b217cf5e1ac2d8aeb7d832010d5a43da901296e
and matched against the original manifest before writing. All other predecessor mappings are
unchanged. The provenance gate verifies that original manifest through this retained map and
also verifies the new manifest through PATHMAP.tsv. Both bundles are mandatory.

The new manifest covers current research-v2 contract files, the active product skill/policy/
workflow guide, and the retained predecessor material. Generated schemas remain derived from
Rust; architecture/schema checks reject candidate drift. There is no runtime format adoption.
