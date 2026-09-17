# Format-61 renderer patch

ADR-0048 retains the upstream pure renderer at public-api 0.52.2, commit
56b483933ceb741978a9f59228e48cae2013e74d. PROVENANCE.json records the unmodified
source hashes. The patch uses rustdoc-types =0.61.0 and accepts its added associated-type
default_unstable field. The native fact worker preserves all three stability categories;
rendered signature text does not stand in for those facts.

Two public pure functions expose the existing type and generic rendering kernels for structured
callable fields. They use the same format model and path table; no signature is parsed back into
parameters. The private renderer options derive Default for this construction.

Upstream examples required by rustdoc include_str are retained. Snapshot and test manifests are
not runtime dependencies. No old
parser or format conversion is retained. The service boundary fixtures qualify the actual
worker and renderer together.
