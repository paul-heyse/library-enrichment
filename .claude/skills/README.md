# Development skills

Workflows for agents *building* this repository. A skill is written when a workflow has
actually repeated, not in anticipation of one.

| Skill | Written because |
|---|---|
| [`design-review/`](design-review/SKILL.md) | Design review had already run twice — commits `e74d012` and `0cd7330` — finding a gate C20 repository-boundary breach, a gate passing on an unearned assertion, and a skipped test reported as `failed`. Neither round left a durable artifact and neither produced a `rules/` entry. ADR-0009. |
| [`adr/`](adr/SKILL.md) | Recording a decision is the most repeated workflow here, and the one a deviation is required to use. |
| [`phase-gate/`](phase-gate/SKILL.md) | Run at every phase boundary, and the place truthful reporting is easiest to get wrong. |
| [`verify-upstream/`](verify-upstream/SKILL.md) | Run before every pin; ADR-0005 exists because it contradicted a documented claim. |
| [`acceptance-report/`](acceptance-report/SKILL.md) | Run before any delivery report, and audited by a subagent rather than trusted. |
| [`handoff/`](handoff/SKILL.md) | Run at the end of any session that changed what is true. |

## Capability repositories

A second, different kind of skill lives here: a **pinned, offline index of a library**, built
once and queried with ripgrep, ast-grep and `Read`. These are not workflows. They exist because
an agent writing against a fast-moving library will otherwise write the API it remembers, and be
wrong in the way that is hardest to notice -- code that reads correctly and fails at runtime.

| Repository | Indexes | Built from |
|---|---|---|
| [`datafusion/`](datafusion/SKILL.md) | DataFusion and the Arrow family | hosted rustdoc JSON |
| [`deltalake/`](deltalake/SKILL.md) | delta-rs and the kernel | rustdoc JSON, part locally built |
| [`fastmcp/`](fastmcp/SKILL.md) | `fastmcp`, `mcp`, `mcp-types`, plus 774 files of upstream docs, examples and tests | Griffe + the ty language server + pinned GitHub tarballs |
| [`pyrefly-ruff/`](pyrefly-ruff/SKILL.md) | 42 ruff and pyrefly crates, and every catalog the two tools emit about themselves | rustdoc JSON + the tools' own JSON oracles |
| [`ast-grep-ripgrep/`](ast-grep-ripgrep/SKILL.md) | every flag, node kind, rule field and regex construct of both tools, plus the 23 library crates underneath | the installed binaries' own help, ast-grep's shipped schemas, PCRE2 10.48's manual, rustdoc JSON — and 49 **executed** probes |
| [`rust-code-model/`](rust-code-model/SKILL.md) | the seven layers of Rust program knowledge -- rustdoc JSON, ra_ap_syntax, ra_ap_hir, the project-loading layer, MIR, dataflow and cargo metadata -- across 11 crates | docs.rs rustdoc JSON, pinned rustc and rust-analyzer source, and 33 **executed** probes |
| [`typer-rich/`](typer-rich/SKILL.md) | `typer`, `rich`, and the Click that typer 0.26 vendored, as three subjects; plus 855 files of upstream docs, runnable tutorials, examples and tests | Griffe + ty + pyrefly + pinned GitHub tarballs, and 19 **executed** probes that capture rendered output |

Each carries a `build/` that can reproduce it, a `verify.py` whose checks include a byte-for-byte
determinism rebuild, and a `reference.md` stating what it deliberately does **not** claim. Read
that section before concluding a capability is absent: silence in an index is not evidence.

`rust-code-model` is the one whose subjects *compete*. The other repositories index a single
library; this one indexes seven ways to learn the same thing about Rust code, which overlap
enough to look interchangeable and answer different questions. Its most valuable artifact is
therefore not an API index but `questions.tsv`, whose `rejected` column names the layer an agent
was about to reach for and why it is wrong -- rustdoc JSON contains no function bodies at all,
the syntax tree resolves nothing, MIR has forgotten the variable names. `verify.py` re-executes
every recipe in that table, because a curated router is the one index here that could quietly
stop being true while everything around it still rebuilt cleanly. Building it turned up that
`rustdoc-types` 0.61.0 -- the crate that *defines* format version 61 -- is served by docs.rs at
format 60, and that a rustdoc `Id` shifts when an unrelated item is added before it.

`typer-rich` is the one that disproves the line `ast-grep-ripgrep` opens with. That skill's
`probes.py` says the sibling repositories cannot execute their subjects because "DataFusion and
delta-rs are libraries: the only thing a builder can do with them is read their documented
surface." Typer and Rich are libraries too, and they *render* -- deterministically, into a
`StringIO` -- so this is the first library repository here whose behavioural claims are
observations. That matters because the question an agent gets wrong about Rich is never "does
this method exist", it is "what will this look like, and how wide", which no signature carries.
Its captures pin the answer: a help string containing `[path]` renders without the word, silently,
because `typer.Typer()` defaults `rich_markup_mode` to `"rich"` and an unknown style tag is
dropped rather than raised. Building it turned up something sharper still -- `from rich import
Console` raises `ImportError`, while `ty` and `pyrefly` both accept the line, because rich imports
it under `if TYPE_CHECKING:`. Griffe records that as an ordinary alias, so the index recommended
the failing spelling until a tripwire was written against it. It is also the only repository here
with three subjects and two pins: typer's vendored Click is indexed separately, because
`typer._click.Command` and `click.Command` are different classes with the same name and the one an
agent imports is the wrong one. `nameable: no` was where fastmcp's model stopped; here 201 rows
carry a `reachability` column instead, saying how you meet a class you cannot import.

`ast-grep-ripgrep` is the one whose subjects are *programs* rather than libraries, so it can do
something most of the others cannot: execute them. Its index carries observed behaviour alongside
documented behaviour, every behavioural claim is backed by a probe **and a control that must
fail**, and `verify.py` re-runs all 49 of them. That was not decoration — probing surfaced that
`rg --pcre2-version` understates the linked runtime by three releases, and that the default regex
engine accepts `(?R)` and silently returns a different answer rather than erroring. It pins PCRE2
as an asserted baseline rather than a discovered floor, using PCRE2's own version conditional,
which is evaluated inside the library and so cannot be fooled by a stale build string.

They are development skills, not products. `skills/library-research/` remains the skill this
service ships.

**Everything an agent workflow needs lives here, not in a runtime-specific directory.** A Claude
Code slash-command directory is visible to Claude Code alone; Codex never sees it. The five
workflows that used to live in one — `adr`, `phase-gate`, `verify-upstream`,
`acceptance-report`, `handoff` — are skills for that reason, and both runtimes surface a skill
as `/name`. `scripts/check_agent_config.py` fails if a runtime-specific instruction surface
reappears.

Progressive disclosure is the second reason: `design-review` is one page plus a reference the
reviewer loads only when a lens is needed, and its normative standard lives in
`docs/design_review/design_principles/` where ADRs and reviews cite it too.

`.claude/skills/` and `.claude/agents/` are the single source. `.codex/` and `.agents/` expose
both through symlinks, so Claude Code and Codex discover the same files. `just lint-agents`
checks that they resolve, that every skill's `name` matches its directory, and that every path
and `just` recipe the instructions name actually exists.

**Not to be confused with `skills/library-research/`**, which is the skill this service *ships*.
That one is a product deliverable governed by the blueprint, it stays synchronized with the
implemented tool contract, and it is installed to user scope only by an explicit, operator-run
`just install-skill --apply`.
