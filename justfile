# library-enrichment operational API.
#
# This is the first command surface an agent should inspect. Recipes express intent, not tool
# flags, so implementations can change without invalidating what callers know.
#
# Two rules govern everything below:
#
#   1. Mutating recipes are never dependencies of a validation recipe. Everything in the
#      [mutating] group must be invoked deliberately and its diff inspected.
#   2. Gates are additive and phase-scoped. A gate whose target does not exist yet reports
#      `not_run` with a reason -- never a false pass, and never a hard failure that tempts
#      anyone to weaken it.
#
# direnv puts .venv/bin and target/debug on PATH (see .envrc and scripts/env.sh), so `ty`,
# `ruff`, `pytest` and the built binaries work without a `uv run` prefix.

set positional-arguments
set dotenv-load := false

root := justfile_directory()

# The built daemon, named explicitly rather than relied on from PATH: direnv puts
# target/debug on PATH but `just` must behave the same without it (see the rules above).
daemon_bin := justfile_directory() / "target/debug/library-enrichmentd"

# Put the project environment on PATH for every recipe, so `just` works identically with
# direnv, without direnv, and in CI. Recipes therefore call `ruff`/`pytest`/`ty` directly
# rather than prefixing `uv run`, and a bare tool name can never silently resolve to a
# different copy on the system -- which matters most for `ty`, since this workstation has
# pyrefly and pyright installed and globally enabled.
export PATH := justfile_directory() / ".venv/bin" + ":" + env_var("PATH")

[private]
default:
    @just --list --unsorted

# ---------------------------------------------------------------------------- environment

[doc("Full first-time setup: toolchain, Python environment, direnv, dev state.")]
[group('environment')]
setup:
    @echo "==> rust toolchain (pinned by rust-toolchain.toml)"
    rustc --version
    @echo "==> python environment"
    just sync
    @echo "==> dev state"
    mkdir -p .dev-state/cache .dev-state/data .dev-state/logs
    @command -v direnv >/dev/null && echo "==> run: direnv allow" || echo "==> direnv not installed (optional)"
    @just doctor

[doc("Resolve and install Python dependencies from the lockfile.")]
[group('environment')]
sync:
    uv sync --frozen --all-groups

[doc("Report which required tools and producers are present. Fails on a missing hard requirement.")]
[group('environment')]
doctor:
    @"{{ root }}/scripts/doctor.sh"

[doc("Preview the scoped development evidence reset; --apply removes only service payloads under ownership locks.")]
[group('mutating')]
state-reset *args:
    @cargo run --locked -p enrichment-daemon --bin library-enrichmentd -- reset-development "{{ root }}/.dev-state" {{ args }}

# ---------------------------------------------------------------------------- validation

[doc("Assert the active toolchain is the pinned one, not this workstation's nightly default.")]
[group('gate')]
toolchain-check:
    @"{{ root }}/scripts/toolchain-check.sh"

[doc("Verify every file delivered in the specification bundle still has its delivered bytes.")]
[group('gate')]
provenance-check:
    @"{{ root }}/scripts/provenance-check.sh"

[doc("Detect changes to the enforcement layer. A guard cannot check itself; this is the other half.")]
[group('gate')]
guardrails-check:
    @"{{ root }}/scripts/guardrails-check.sh"

[doc("Re-record the enforcement-layer manifest. Operator action; asserts the change was intended.")]
[group('mutating')]
guardrails-record:
    @"{{ root }}/scripts/guardrails-check.sh" --record

[doc("Run the hook behaviour tests. The hooks are the enforcement layer and need regression cover.")]
[group('gate')]
hooks-test:
    @"{{ root }}/scripts/test-hooks.sh"

[doc("Run the ast-grep rule corpus against its fixtures.")]
[group('gate')]
rules-test:
    ast-grep test

[doc("Scan the working tree with the ast-grep rule corpus.")]
[group('gate')]
rules-scan:
    ast-grep scan

[doc("Check native ownership and retired evidence/query/build entry points.")]
[group('gate')]
architecture-check:
    uv run python scripts/architecture-check.py

[doc("Compile-check the Rust workspace.")]
[group('gate')]
check:
    @if [ -f Cargo.toml ]; then cargo check --workspace --all-targets; \
     else echo "check: no Cargo workspace yet (phase 0)"; fi

# `scripts` is in scope. It was outside it for a long time, which let 44 findings accumulate
# there while the recipe reported a clean tree -- a lint that does not look at a directory is
# not evidence about that directory. `|| true` is gone for the same reason: a tool that is
# present and failing must fail the gate, and a tool that is absent says so.
[doc("Lint everything: clippy, ruff, taplo, typos.")]
[group('gate')]
lint:
    @if [ -f Cargo.toml ]; then cargo clippy --workspace --all-targets --all-features -- -D warnings; fi
    @if [ -d python ]; then ruff check python tests scripts; fi
    @if command -v taplo >/dev/null; then taplo check; else echo "lint: taplo is not installed; TOML is unchecked"; fi
    @if command -v typos >/dev/null; then typos; else echo "lint: typos is not installed; spelling is unchecked"; fi

[doc("Check formatting without modifying anything.")]
[group('gate')]
fmt-check:
    @if [ -f Cargo.toml ]; then cargo fmt --all -- --check; fi
    @if [ -d python ]; then ruff format --check python tests scripts; fi

[doc("Type-check the Python boundary with ty. Not pyrefly, not pyright, not mypy.")]
[group('gate')]
typecheck:
    @if [ -d python ]; then ty check python; else echo "typecheck: no python/ yet (phase 0)"; fi

# Writes docs/reports/logs/nextest.json. That file is the ONLY thing that can promote a gate
# past not_run: scripts/acceptance-report.py joins it against tests/gates.toml, and
# scripts/acceptance-check.py refuses any `passed` gate without a log_path. libtest-json is
# still experimental in nextest 0.9.143, hence the opt-in env var.
[doc("Run the Rust test suite, including doctests (nextest does not run them).")]
[group('gate')]
test-rust:
    # Prepare the exact test graph before binding native binary hashes. A store-only build
    # can produce a worker with a different Cargo feature union than workspace integration tests.
    @cargo nextest run --workspace --no-run
    @mkdir -p "{{ root }}/docs/reports/logs"
    @NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 python3 scripts/evidence_run.py --log "{{ root }}/docs/reports/logs/nextest.json" --stdout -- cargo nextest run --workspace \
        --message-format libtest-json-plus
    @cargo test --doc --workspace

# Writes docs/reports/logs/pytest.json, for the same reason as test-rust above.
[doc("Run the Python test suite.")]
[group('gate')]
test-python:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{ root }}/docs/reports/logs"
    # The contract and e2e tiers drive the real daemon binary. Without this they skip, and
    # acceptance-report then reports their gates `blocked` -- a correct report of a needlessly
    # incomplete run.
    cargo build --quiet -p enrichment-daemon --bin library-enrichmentd -p enrichment-core --bin library-enrichment-executor -p enrichment-store --bin library-enrichment-native-worker
    # And the sandbox tier needs to know which images a qualification receipt covers. Read here
    # rather than in `gate-phase` alone, so that `just test`, `just ci` and `just gate-phase`
    # cannot report different gate results for the same tree.
    libenr_execution_environment=$(bash "{{ root }}/scripts/execution-env.sh")
    eval "$libenr_execution_environment"
    python3 scripts/evidence_run.py --log "{{ root }}/docs/reports/logs/pytest.json" -- pytest -m "not live and not client" --json-report --json-report-file="{{ root }}/docs/reports/logs/pytest.json"

[doc("Run every test tier that does not require the network.")]
[group('gate')]
test:
    #!/usr/bin/env bash
    failed=0
    just test-rust || failed=1
    just test-python || failed=1
    exit "$failed"

[doc("Opt-in live tests against real registries. Records exact versions at run time.")]
[group('gate')]
test-live:
    @cargo build --quiet -p enrichment-daemon --bin library-enrichmentd -p enrichment-core --bin library-enrichment-executor -p enrichment-store --bin library-enrichment-native-worker
    @python3 scripts/evidence_run.py --log "{{ root }}/docs/reports/logs/live.json" -- pytest -m live --json-report --json-report-file="{{ root }}/docs/reports/logs/live.json"

[doc("Client acceptance against real Codex and Claude Code. Needs client binaries and credentials.")]
[group('gate')]
test-client:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --quiet -p enrichment-daemon --bin library-enrichmentd -p enrichment-core --bin library-enrichment-executor -p enrichment-store --bin library-enrichment-native-worker
    libenr_execution_environment=$(bash "{{ root }}/scripts/execution-env.sh")
    eval "$libenr_execution_environment"
    python3 scripts/evidence_run.py --log "{{ root }}/docs/reports/logs/client.json" -- pytest -m client --json-report --json-report-file="{{ root }}/docs/reports/logs/client.json"

[doc("Run the real ignored containment/cleanup tier and record independently joinable evidence.")]
[group('gate')]
test-execution:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --quiet --locked -p enrichment-daemon --bin library-enrichmentd -p enrichment-core --bin library-enrichment-executor -p enrichment-store --bin library-enrichment-native-worker
    libenr_execution_environment=$(bash "{{ root }}/scripts/execution-env.sh")
    eval "$libenr_execution_environment"
    : "${LIBENR_EXECUTION_TEST_ROOT:?qualify and select the execution root first}"
    NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 python3 scripts/evidence_run.py --log "{{ root }}/docs/reports/logs/nextest-execution.json" --stdout -- cargo nextest run --locked -p enrichment-daemon --test execution_boundary --test execution_cleanup --run-ignored only --message-format libtest-json-plus

[doc("Assert dependency policy: banned classes, one Arrow/DataFusion type universe, licenses.")]
[group('gate')]
deps-policy:
    @if [ -f Cargo.toml ]; then cargo deny check bans licenses sources; \
     else echo "deps-policy: no Cargo workspace yet (phase 0)"; fi
    @"{{ root }}/scripts/deps-policy-python.sh"

[doc("Assert generated schemas match the frozen Phase-0 contract behaviourally.")]
[group('gate')]
schema-conformance:
    @"{{ root }}/scripts/schema-conformance.sh"

[doc("Assert the real XDG service paths were untouched by the test suite.")]
[group('gate')]
state-leak-check:
    @"{{ root }}/scripts/state-leak-check.sh"

[doc("Regenerate docs/reports/acceptance.json from real test output.")]
[group('gate')]
acceptance-report:
    @python3 "{{ root }}/scripts/acceptance-report.py" \
        --nextest "{{ root }}/docs/reports/logs/nextest.json" \
        --nextest "{{ root }}/docs/reports/logs/nextest-execution.json" \
        --pytest  "{{ root }}/docs/reports/logs/pytest.json" \
        --live "{{ root }}/docs/reports/logs/live.json" \
        --client "{{ root }}/docs/reports/logs/client.json"

[doc("Validate the acceptance report: no gate passes without a command and a log.")]
[group('gate')]
acceptance-check:
    @python3 "{{ root }}/scripts/acceptance-check.py"

[doc("Agent configuration: every runtime sees the same skills and agents; every path and recipe resolves.")]
[group('gate')]
lint-agents:
    @python3 "{{ root }}/scripts/check_agent_config.py"

[doc("Run the behaviour tests for the decision-record lint and the register checker.")]
[group('gate')]
adr-lint-test:
    @bash "{{ root }}/scripts/test-adr-lint.sh"

[doc("Validate every decision record, the generated index, and the deferred-decision register.")]
[group('gate')]
adr-lint:
    @python3 "{{ root }}/scripts/adr.py" lint
    @python3 "{{ root }}/scripts/adr.py" index --check
    @python3 "{{ root }}/scripts/check_register.py" --lint

[doc("Report the deferred-decision register rows that are due, running their checks.")]
[group('gate')]
register-check:
    @python3 "{{ root }}/scripts/check_register.py" --due

[doc("Run the acceptance gate for one phase (0-6).")]
[group('gate')]
gate-phase n:
    @"{{ root }}/scripts/gate-phase.sh" "$1"

# Kept in the same order as .github/workflows/ci.yml so a local run and a CI run check the
# same things. If you add a step to one, add it to the other.
[doc("Everything that must hold on every change. The default pre-commit surface.")]
[group('gate')]
ci: toolchain-check provenance-check guardrails-check hooks-test rules-test architecture-check lint-agents adr-lint adr-lint-test fmt-check lint check test typecheck schema-conformance deps-policy acceptance-report acceptance-check
    @echo "ci: complete"

# ---------------------------------------------------------------------------- decisions

# These write files. They are in [mutating] because a validation recipe must never depend on
# something that rewrites the tree -- `adr-lint` is the read-only half and lives in [gate].

[doc("Start a decision record from the template: just adr-new my-slug --title \"...\"")]
[group('mutating')]
adr-new slug *args:
    @python3 "{{ root }}/scripts/adr.py" new "$@"

[doc("Regenerate docs/adr/README.md from the records' front matter.")]
[group('mutating')]
adr-index:
    @python3 "{{ root }}/scripts/adr.py" index

[doc("Mark one decision record superseded by another: symmetric links and a status-history line.")]
[group('mutating')]
adr-supersede old new:
    @python3 "{{ root }}/scripts/adr.py" supersede "$1" "$2"

[doc("Start an implementation plan under docs/plans/NN-<slug>.md.")]
[group('mutating')]
plan slug:
    @"{{ root }}/scripts/plan-new.sh" "$1"

# ---------------------------------------------------------------------------- mutating

[doc("Format Rust and Python in place.")]
[group('mutating')]
fmt:
    @if [ -f Cargo.toml ]; then cargo fmt --all; fi
    @if [ -d python ]; then ruff format python tests; ruff check --fix python tests; fi
    @command -v taplo >/dev/null && taplo fmt || true

[doc("Regenerate wire schemas from the authoritative Rust types, then the Python DTOs.")]
[group('mutating')]
schemas-generate:
    @"{{ root }}/scripts/schemas-generate.sh"

[doc("Record the identity of each installed nightly and the rustdoc format_version it emits.")]
[group('mutating')]
rustdoc-format-matrix:
    @"{{ root }}/scripts/rustdoc-format-matrix.sh"

# Rebuilds the fixture crate captures (rustdoc JSON, .crate tarballs, index and API documents)
# with the dated nightly from config/toolchains.toml, and records their provenance. The
# outputs are committed; run this only when the fixture crate or the producer nightly changes,
# and inspect the diff.
[doc("Rebuild the Rust fixture captures under tests/fixtures/ and record their provenance.")]
[group('mutating')]
fixtures-build:
    @"{{ root }}/scripts/fixtures-build.sh"

[doc("Install the companion skill to user scope. Requires --apply; defaults to a dry run.")]
[group('mutating')]
install-skill *args:
    @"{{ root }}/scripts/install-skill.sh" "$@"

[doc("Print absolute launch commands and an installation-specific service configuration; writes nothing.")]
[group('operations')]
launch-config *args:
    @uv run --frozen python "{{ root }}/scripts/launch_configuration.py" {{args}}

[doc("Build the two producer execution images from their pinned inputs. Requires --apply.")]
[group('mutating')]
execution-images *args:
    @python3 "{{ root }}/scripts/execution-images.py" "$@"

[doc("Qualify built execution images with a real containment run and record the receipt. Requires --apply.")]
[group('mutating')]
execution-qualify *args:
    @python3 "{{ root }}/scripts/execution-qualify.py" "$@"

[doc("Reclaim writable capsule storage. Regenerable only; retained evidence is never touched. Requires --apply.")]
[group('mutating')]
capsules-prune *args:
    @cargo run --locked -p enrichment-daemon --bin library-enrichmentd -- cleanup cache {{ args }}

[doc("Preview explicit retained evidence removal under reader and ownership locks; requires --apply.")]
[group('mutating')]
evidence-cleanup *args:
    @cargo run --locked -p enrichment-daemon --bin library-enrichmentd -- cleanup evidence {{ args }}

[doc("Drive real Codex and Claude Code against the service in a throwaway user directory. Requires --apply.")]
[group('mutating')]
client-acceptance *args:
    @python3 "{{ root }}/scripts/client-acceptance.py" "$@"

# ---------------------------------------------------------------------------- provenance

# `export` writes outside the repository by design (ADR-0018): a bundle goes wherever the
# operator names, and the destination must be empty. `verify-bundle` reads and is a [gate].

[doc("Export one context's evidence as a portable, offline-verifiable bundle (ADR-0018).")]
[group('mutating')]
export context out:
    @"{{ daemon_bin }}" export "$1" "$2"

[doc("Check an exported bundle against its own manifest. Needs nothing but the bundle.")]
[group('gate')]
verify-bundle dir:
    @"{{ daemon_bin }}" verify-bundle "$1"
