# Capsules

Service-owned synthetic consumer workspaces (blueprint §9.1): exact dependency resolution
inputs, a selected toolchain or interpreter, a small consumer file, and analysis settings.

This directory is gitignored apart from this file. Capsules are regenerable build state, not
source. During development they are created under `$LIBENR_HOME` (`.dev-state/`); in production
they live under the XDG cache.

A capsule's manifest records what was reproduced from the working project and what was not. A
library API scan and a project-compatible consumer check are different operations, and a
capsule must never be described as the latter when it is only the former.
