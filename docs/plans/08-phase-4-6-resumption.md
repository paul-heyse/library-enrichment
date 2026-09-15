---
title: Resume remaining Phase 4 through Phase 6 implementation
status: done
date: 2026-09-13
adrs: [ADR-0005, ADR-0017]
phase: 4
---

# Resume remaining Phase 4 through Phase 6 implementation

## Stop boundary and authority

The user requested a checkpoint and documentation handoff on 2026-09-13 local. Implementation
stopped after the active execution-boundary correction and focused checks. **No Phase 4 acceptance
or overall completion is claimed.** Resume on a new instruction; do not infer permission to keep
working from this plan. Preserve all uncommitted work over `cd6d949` (original tree was clean).
No commits were created. `git status --short` lists task-owned changes, including new untracked
source files; do not reset or clean them. `.dev-state` contains non-versioned evidence and images.
Final private-root Podman inventory was empty, with no ownership records. The namespace-keeper
PID was matched to this root and stopped; unrelated services were left alone. No task-owned
daemon/test/container process remains. See `.dev-state/phase4-handoff-cleanup.json`.

Read `AGENTS.md`, frozen `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` and `AGENT_HANDOFF.md`,
`STATUS.md`, plan 05, ADR-0017 (adopts unchanged ADR-0016 contracts), and the current execution
review. Existing plans 06/07 remain the Phase 5/6 plans; this document supplies the concrete entry
point and implementation map. Frozen blueprint/config/product-skill/enforcement files stay intact.
Rust owns all core state; the Python adapter is thin. Context7 is a separate client connection.

## Current checkpoint and logs

Source digest `11370f944668fba86525bfb66ef756ac21adf519465858ea59b10d56cdfff090`.
Final receipt-wrapped verification ran 2026-09-14 02:19:45–02:20:07 UTC: **6 passed**.
`just acceptance-report`/`acceptance-check` generated a current report with
0 passed / 0 failed / 0 blocked / 48 not_run. Phase 4 tests are not wired to gates yet, and old
receipt source digests intentionally do not qualify this tree. Retired P08 remains in the count.

| Evidence | Result | Log under `.dev-state/` |
|---|---|---|
| Actual MCP → daemon → container verification | 6 passed | `phase4-handoff-verification.log` |
| Daemon unit tests | 47 passed | `phase4-handoff-daemon-tests.log` |
| Current requirement marker/source parser | 3 passed | `phase4-handoff-requirements.log` |
| Daemon/core Clippy, all targets | passed | `phase4-handoff-clippy-command.log` |
| Ruff `python tests`, ty | passed | `phase4-handoff-ruff.log`, `phase4-handoff-ty.log` |
| Regenerated schema comparison | passed; unsigned-format generator warnings retained | `phase4-handoff-schema.log` |
| Frozen provenance | 15/15 passed | `phase4-handoff-provenance.log` |
| ast-grep fixture corpus | 6 passed | `phase4-handoff-rules.log` |
| Tool inventory | passed | `doctor-phase4-handoff.log` |
| Actual isolation/output/deadline/cancellation | 2 passed | `phase4-handoff-boundary.log` |

Receipt-wrapped commands have adjacent `.execution.json` with exact argv, executable SHA, source
before/after, commit, versions, time, exit and log SHA. Clippy build diagnostics are in the
`-command.log` companion (Cargo writes them to stderr); its receipt log captures stdout.
These are focused checks, not whole-suite/installed-client acceptance. The older
`phase4-verification.log` deliberately retains a 5-pass/1-fail run: the test incorrectly expected
an unfrozen JOB_CANCELLED error; correction checks VERIFICATION_FAILED plus journal `cancelled`.
The final six-test log supersedes that assertion failure. Earlier logs are not silently removed.
An independent auditor checked the current source/report identity, all 48 registry IDs and
acceptance-check, and validated the final verification/daemon-test receipts. There were no passed
gate commands to rerun. The implementation review remains Revise for F3, not phase acceptance.

Last complete phase: independently accepted Phase 3 digest
`e282153379fb21df723c8887749ba6e15edb5bb3d76ebdf20eba36524a382962`, 231 Rust / 90 Python /
2 live tests, 24 passed / 24 not_run. `.dev-state/phase3-final-audit/` holds the receipts.
An earlier C02 audit caught MCP envelope/framing overflow and duplicate long text; final raw
1024/2048/4096 budgets measured envelope830/1285/1285 and frame986/1470/1470 bytes. Preserve that
oracle. Historical Phase 2 digest `567557fb0293821a2ce02dfc81dea60b12f1e91d5de76e85b430cb6407fde0ee`
and Phase 1 digest `46d23252c43eba2d94cd47f515ef7a1fcf931d4014484e0aed5024286a2c025a` are not current.

## Current implementation map

| Surface | Code and behavior |
|---|---|
| Requests/results | `enrichment-core/src/execution.rs`: VerifyRequest, ProbeMode, job control, ProcessObservation, VerificationData. Core-generated schemas/DTOs; frozen envelope/error vocabulary unchanged. |
| Durable jobs | `enrichment-daemon/src/jobs.rs`: journal atomic replace plus fsync, seven states, per-interest cancellation, in-flight dedup, semaphore/queue cap, terminal replay; restart fails interrupted work without replaying runtime. JobRecord currently stores VerifyRequest only. |
| Writer ownership | `service.rs`: data `.daemon.lock` and cache `.execution-owner.lock`, acquired before recovery. Different data roots sharing cache cannot reconcile each other's live work. |
| Container boundary | `execution/mod.rs`: private rootless Podman roots; durable owner record before create, then start/attach; offline target, non-root65532, read-only root, capabilities dropped, cgroups/time/output bounds, explicit environment. Reap/remove/absence check; startup reconciliation precedes journal recovery. |
| Capsules | `execution/capsule.rs`: owned scratch, source SHA validation, actual producer version checks, sanitized Rust consumer and Cargo.lock, Python wheelhouse/offline install, resolved environment identity. |
| Python closure | `execution/python_closure.rs` + core `producer/python/requirements.rs`: parse all metadata before following edges, reject URL/VCS/path dependencies, typed markers, bounded registry fetch/hash/metadata checks, deterministic lock. No networked uv resolution. |
| Verification | `ops/verify.rs`: pin snapshot before queue, store agent snippet/test intent, prepare capsule, store lock and derived context, execute exact Cargo/ty/runtime command, retain outcome/evidence class/ProducerRun. |
| MCP | `python/enrichment_mcp/server.py`: verify_usage, job_control and job resource use ordinary envelopes; pending carries caller-interest token. No native MCP tasks required. |
| Status | `status.rs`: job counts implemented; execution readiness not accepted, LSP not implemented. |
| Regression tests | `tests/e2e/test_verification_fixture.py`: valid/invalid Rust+Python probes, runtime failure, declared→derived identity, job restart, hostile transitive source, blocked download cancellation, daemon crash descendant heartbeat and dual writer exclusion. `execution_boundary.rs` has two ignored-by-default real isolation tests. |

Runtime Python uses `-I -S` with a fixed dependency-path bootstrap; it does not process `.pth`.
Python admission supports pure universal wheels, Python3.14.7/Linux x86_64, bounded greedy closure
(256 packages,1024 expansions,512MiB, aggregate deadline), no backtracking. Conflicts/unsupported
markers/native/source builds remain explicit unresolved environments. Root and transitive source
instructions are rejected before following them, including inactive requirements. Cargo rejects
all path/Git/alternate-registry dependencies, including target/build/dev tables, strips `.cargo`
and toolchain files, fetches before offline checks. No project lock/private configuration import.
Derived contexts are retained but currently have no static snapshot pointer.

## Start here: finish execution failure ownership

The independent implementation review remains **Revise**, with G5/G7 unresolved. F1 Python
admission, F2 nested Cargo sources and F4 cancellation during fetching are corrected in code.
F3 creation/start/removal ordering, restart recovery and cache/data exclusion are corrected;
**task-drop and cleanup-error supervision remain open**.

1. Keep ownership and concurrency authority until container absence is confirmed. Either retain
   an in-process cleanup supervisor/permit or quarantine new execution admission while cleanup
   is unresolved. A failed removal currently returns terminal failure and releases the permit
   while target descendants may live until timeout; repeated failures can exceed concurrency.
2. Exercise actual-container task abort and injected removal failure. Observe a descendant
   heartbeat and independent container absence; prove admission/quarantine and restart recovery.
   Include cancellation during creation, which cannot start target code but can leave uncertain
   helper state. Do not confuse killing the Podman client with killing the container.
3. Review every Runner I/O error mapping: missing isolation must be POLICY_DENIED, not a generic
   environment gap. Preserve failed preparation process evidence. Inspect shutdown bounds and
   task-drop behavior; no early terminal cancellation while cleanup is uncertain.
4. Get the scoped review rechecked. Full C20 and execution acceptance remain separate.

## Remaining Phase 4 in dependency order

1. **Producer setup/readiness:** add preview-first, operator-invoked image build/admission and
   qualification recipes. The advertised `just execution-qualify` does not exist yet. Persist
   actual immutable image/tool identity and make status distinguish configured from qualified.
   Extend hostile resource tests and evaluate aggregate writable-capsule disk bounds.
2. **Complete environment scope:** retain exact locks/artifacts/provenance and declared versus
   actual identity. Make unsupported native/project inputs explicit; review runtime API inspection
   and any `.pth`/extension limits. A successful consumer snippet alone is not a whole runtime API
   scan. Add the exact R09 stable-fails/nightly-passes consumer and environment-mismatch oracles.
3. **Warm LSP sessions:** Rust-owned reusable bounded manager keyed by image/server/capsule identity;
   framed stdin/stdout, initialize/capability/encoding negotiation, versions/didOpen/didChange,
   diagnostics, cancellation, shutdown and eviction. Refactor Runner lifecycle without weakening
   supervision. Integrate concrete semantic jobs (current jobs only persist VerifyRequest),
   InspectRequest/InspectData and adapter, regenerate schemas. Service computes anchor coordinates
   from symbol/snippet; reject ambiguity rather than guessing. Signature-only reads never start LSP.
   Keep unsupported/unresolved/incomplete/empty distinct, retain raw bounded observations and
   mapped locations with provenance. Probe navigation/diagnostics in both languages.
4. **Dated rustdoc fallback:** hosted docs.rs JSON first; explicit build policy only for fallback.
   Use `cargo +nightly-2026-09-13 rustdoc` inside an owned capsule. Verify recorded rustc identity
   and emitted format before normalization; preserve `locally_built_rustdoc` producer/environment
   distinction and publish a new immutable snapshot. Never compile project code with bare nightly.
5. **Gate wiring and acceptance:** align real assertions with IDs below, include missing-tool/image
   prerequisites as blocked, run complete current-tree gates and independent acceptance. Do not
   register a nearby test whose assertions do not prove the frozen scenario.

| Phase 4 IDs | Required evidence / current gap |
|---|---|
| R09, R10 | stable/nightly compatibility distinction; build denied without isolation |
| P08a, P08b | real ty nominal mapped subclass locations; Protocol structural incompleteness (P08 retired) |
| P09, P10 | same consumer's typecheck versus runtime outcome; selected interpreter/dependency identity |
| C10–C12 | hostile source text never becomes instruction, safe archive rejection, artifact handle-only reads |
| C16 | new resolved context preserves original ID semantics; existing fixture is a starting point |
| C17 | actual LSP startup counter proves signature inspection stays static |
| C18 | actual pending result completed via ordinary job control; existing fixture is a starting point |
| C20 | content-and-path canary tree digest across **every enabled profile**; current single-marker tests insufficient |

## Exact local execution prerequisites and restart commands

All image probes are local candidates, not blanket qualification. Private Podman root:
`/home/paul/library-enrichment/.dev-state/p4p`. Podman4.9.3/rootless overlay/cgroup-v2/systemd.
Broker needs the local user DBus address; containers never inherit it. Keep-id uid/gid65532 writes
host-UID1000 owned scratch without `:U`/chown. No `--dns=none` with `--network=none`.

- Python image `sha256:44afc73e29e9a456265b868623b52b76d48e22e7ea3b9a0ff24853f75d5e11de`:
  Python3.14.7, `/opt/producers/bin/ty`0.0.80 and `/opt/producers/bin/uv`0.12.13.
- Rust image `sha256:28fc46e4b2dab9405af3b813727fc05f9e1d935b8724e77364016114f2e5588d`:
  `/usr/local/cargo/bin/{cargo,rustc,rust-analyzer}`, stable1.98.1, rust-src and dated nightly.
- Exact Containerfiles, input/base/output digests and command environment:
  `.dev-state/p4p/producer-builds-20260913/{image-candidates,podman-command-context}.json`.
  These are retained development assets, not a shipped setup workflow.
- Dated nightly rustc1.100.0-nightly (`809936eac`), observed rustdoc format61; normalizer supports
  57/59/60/61. `config/toolchains.toml` is the authority, reverify before changing pins.

```sh
cd /home/paul/library-enrichment
just --list
just doctor
cargo build -p enrichment-daemon
export LIBENR_EXECUTION_TEST_ROOT=/home/paul/library-enrichment/.dev-state/p4p
export LIBENR_EXECUTION_TEST_PYTHON=sha256:44afc73e29e9a456265b868623b52b76d48e22e7ea3b9a0ff24853f75d5e11de
export LIBENR_EXECUTION_TEST_RUST=sha256:28fc46e4b2dab9405af3b813727fc05f9e1d935b8724e77364016114f2e5588d
uv run pytest -q tests/e2e/test_verification_fixture.py
cargo test -p enrichment-daemon --test execution_boundary -- --ignored
```

For an acceptance claim, use receipt-wrapped recipes and record this image configuration too.
`just gate-phase 4` is the later terminal gate; do not run it merely to obtain green historical
IDs. `cargo test --workspace` does not execute ignored isolation tests. Ordinary pytest includes
sandbox tests but skips them without image variables; live/client tiers are separate.
Use `just schemas-generate` for wire changes, then `just schema-conformance`; never edit DTOs.

## Verified upstream details for LSP implementation

Evidence: compatibility matrix Phase4 rows, `.dev-state/p4p/lsp-settings-20260913/`, and
`.dev-state/p4p/pep508-20260913/`. Context7 is discovery, exact-version primary source/probes decide.

**ty0.0.80:** `ty server` takes no CLI configuration flags. Initialization options are flattened:
`untrustedWorkspace:true`, `experimental:{useUv:"off"}`, `configurationFile:
"/capsule/probe-config/ty.toml"`, `diagnosticMode:"openFilesOnly"`, and
`configuration:{environment:{python:"/usr/local/bin/python3",extra-paths:["/capsule/python"],
python-version:"3.14",python-platform:"linux"}}`. Set `workspace.configuration=false`, or
implement its section responses correctly. UTF8 is negotiated when offered with UTF16; default
UTF16. It advertises definition/references/implementation and document/workspace diagnostics.
Nominal implementation returns base-self plus child; Protocol returns protocol-self plus explicit
nominal subclass, omitting structural implementors. P08b's structural-only fixture must record
incomplete coverage, not absence. Retain raw locations/self versus candidates.

CLI `ty check` has **no `--no-config`**. `--config-file` still reads XDG user config, so use the
empty trusted capsule file and `XDG_CONFIG_HOME=/opt/libenr-empty-config` in the read-only image.
The implementation already uses these settings. Offline uv uses `--only-binary=:all:`,
`--require-hashes`, `--no-deps`, `--no-index`, `--offline`; no `uv pip download` subcommand and
`--no-build` conflicts with `--only-binary` in this pin.

**rust-analyzer1.98.1:** stdio executable, no server subcommand. Verified settings include
`cargo.extraArgs:["--offline","--locked"]`, `cargo.extraEnv:{CARGO_NET_OFFLINE:"true"}`,
features/noDefaultFeatures/target/targetDir, `cargo.buildScripts.enable:false`,
`procMacro.enable:false`, `checkOnSave:false`. No `cargo.offline` setting. Avoid `cargo.noDeps:true`
when dependency navigation is required. Navigation capabilities advertised; document diagnostics
supported, workspace diagnostics not advertised. Source/schema checked; real target LSP session
integration is still unimplemented.

**Requirement parser:** pep508_rs0.9.2 was not pinned: it adds thiserror1/thiserror-impl1 beside2
and implements older marker semantics. Current Jan2026 PyPA rules classify version versus string
fields, with string inequalities constrained; `extra ==/!=` is membership/nonmembership over the
full selected-extras set. Do not revert to per-extra evaluation. `extras`/`dependency_groups`
apply only in pylock marker context. Unsupported expressions fail explicitly. No dependency-policy
skip or checker substitution was added.

## Remaining Phase 5

Use plan06. Implement/qualify absolute-path installation and daemon startup locking in a controlled
test user directory; exercise install/update/uninstall manifests and avoid overwriting unrelated
skills or duplicate symlink destinations. Product skill provenance is frozen; ordinary tests never
install into real user config. Capture actual Codex and Claude stdio + separate Context7 traces.
A01/A02 connection, A03 unfamiliar capability shortlist/deployment brief, A04 docs/API version
mismatch, A05 known symbol selective read, A06 unavailable service. Also exercise upgrade comparison
and runtime-dependent Python questions. No mocked client can pass these gates. Missing credentials,
client binaries or explicit user-global setup permission are concrete blocked prerequisites.

## Remaining Phase 6

Use plan07. Implement shared extraction single-flight (verification dedup does not prove C04),
two-adapter interest cancellation (C05), publication crash injection/recovery (C06), and old pinned
snapshot readability after enrichment (C07). Add historical schema/snapshot compatibility and
normalizer/dependency-update regression coverage. Build portable provenance export and preview-first
pruning that distinguishes immutable retained evidence from regenerable caches/capsules; retained
crash capsules currently have no pruning workflow. Add useful structured logs/metrics and complete
operator setup/recovery documentation. Fix metadata-only XDG leak hashing and masked spelling
failures. Broad script Ruff currently finds 45 violations outside ordinary lint scope; no ignores.

Investigate first-artifact-provenance dedup (identical bytes retain only first acquisition metadata),
revision revalidation timestamps in snapshot inputs, and Rust workspace-inherited version support.
These are explicit follow-up concerns, not claims of reproduced acceptance failures. Finish with
current-tree full quality/fixture/live/client evidence, phase gates and independent acceptance;
update STATUS from actual results rather than carrying forward historical green counts.

## Outcome (checkpoint only)

### What was built

See implementation map and current focused evidence. No LSP/fallback/Phase5/6 code was started
at the stop boundary. The cache-owner lock and blocked-transfer regression were completed.

### A mistake made and corrected

Networked uv could follow transitive source instructions before Rust admission; ADR-0017 replaces
it. Recovery initially locked only data roots despite cache-scoped ownership; both are now locked.
The cancellation fixture initially expected an error outside the frozen vocabulary; it now checks
the frozen error and separate terminal job state.

### Deviations from the plan, deliberate

The user explicitly requested this stop/handoff. Remaining work is preserved here. Scope was not
silently reduced and no incomplete phase was declared complete.
