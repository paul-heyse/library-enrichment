# Native delivery provenance seal — 2026-09-17

Authority: ADR-0054 and the authorized Plan 18 hard pivot. This records the product guide's
wire 5.0 budget and exact-number contract; it does not qualify or activate the application.

The original 2026-09-13 and research-v2 2026-09-15 manifests/maps remain unchanged. The older
research-v2 product guide was already modified at Plan 18's baseline. Its exact sealed bytes
were recovered from commit a78c741 and checked against the original SHA-256 before retention
under predecessor/. RESEARCH_V2_PATHMAP.tsv redirects only that source to those exact bytes.

The gate verifies all three seals. This manifest freezes the current guide, recovered predecessor,
new path map and explanation. There is no runtime reader for an older wire or data format.
