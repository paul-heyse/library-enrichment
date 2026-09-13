#!/usr/bin/env python3
"""Validate the agent configuration for every runtime (standard library only).

    python3 scripts/check_agent_config.py

Agent instructions are the one class of document with no compiler and no test. A path that
stops existing, a `just` recipe that gets renamed, or a skill that only one runtime can see
all fail silently -- the agent simply does slightly worse work and nobody is told. This is
the missing compiler.

It checks five things:

1. **Every runtime sees the same instruction surface.** `.codex/` and `.agents/` must expose
   `skills` and `agents` from `.claude/`, so a workflow written once is available in Claude
   Code and Codex alike. A Claude-only `commands/` directory is the failure this catches: it
   is invisible to Codex, which is why the five workflows here are skills.
2. **`CLAUDE.md` defers to `AGENTS.md`**, which is the canonical source both runtimes read.
3. **Every skill and subagent is well-formed** -- front matter present, and a skill's `name`
   equal to its directory, since that is what both runtimes dispatch on.
4. **Every repository path named in the instructions exists.** Exemptions live in
   `.claude/path-allowlist.txt`, each with a comment saying when it will appear.
5. **Every `just` recipe named in the instructions exists.** `just --list` is advertised to
   agents as the command surface, so a recipe named in prose but absent from the justfile is a
   dead end an agent will walk into.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ALLOWLIST = ROOT / ".claude" / "path-allowlist.txt"

#: Runtime directory -> the entries it must expose from `.claude/`.
RUNTIME_LINKS = {
    ".codex": ("skills", "agents"),
    ".agents": ("skills", "agents"),
}

#: A runtime-specific instruction surface is a trap: only one runtime can see it.
RUNTIME_ONLY_SURFACES = (".claude/commands",)

#: Where instructions live. Every path and recipe named in these is checked.
INSTRUCTION_GLOBS = (
    "AGENTS.md",
    "CLAUDE.md",
    ".claude/rules/*.md",
    ".claude/skills/*/*.md",
    ".claude/skills/*.md",
    ".claude/agents/*.md",
    "docs/design/README.md",
    "docs/adr/README.md",
    "docs/plans/README.md",
    "docs/design_review/design_principles/ADDENDUM.md",
)

#: Top-level directories a backticked token must start with to be treated as a repo path.
PATH_ROOTS = {
    ".agents",
    ".claude",
    ".codex",
    "config",
    "contracts",
    "crates",
    "docs",
    "python",
    "rule-tests",
    "rules",
    "schemas",
    "scripts",
    "skills",
    "tests",
}

CODE_SPAN_RE = re.compile(r"`([^`\n]+)`")
# Anchored, and applied only to code spans and fenced blocks: English prose says things like
# "a report you did not just generate", which is not a reference to a recipe named `generate`.
JUST_RE = re.compile(r"^just\s+([a-z][a-z0-9-]*)")
FENCE_RE = re.compile(r"```[a-z]*\n(.*?)```", re.DOTALL)
# A placeholder is a pattern, not a reference: docs/plans/NN-kebab-case.md names no file.
PLACEHOLDER_RE = re.compile(r"(?:^|[/_-])N{2,}(?:[/._-]|$)")
NAME_RE = re.compile(r"^name:\s*(\S+)\s*$", re.MULTILINE)
DESCRIPTION_RE = re.compile(r"^description:\s*\S", re.MULTILINE)


def out(message: str = "") -> None:
    """stdout, without tripping the linter's ban on stray `print`."""
    sys.stdout.write(f"{message}\n")


def err(message: str) -> None:
    sys.stderr.write(f"{message}\n")


def allowlist() -> set[str]:
    if not ALLOWLIST.is_file():
        return set()
    entries: set[str] = set()
    for line in ALLOWLIST.read_text(encoding="utf-8").splitlines():
        stripped = line.split("#", 1)[0].strip()
        if stripped:
            entries.add(stripped)
    return entries


def instruction_files() -> list[Path]:
    files: list[Path] = []
    for pattern in INSTRUCTION_GLOBS:
        if "*" in pattern:
            files.extend(sorted(ROOT.glob(pattern)))
        elif (ROOT / pattern).is_file():
            files.append(ROOT / pattern)
    return files


def just_recipes() -> set[str]:
    proc = subprocess.run(
        ["just", "--summary"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        return set()
    return set(proc.stdout.split())


def candidate_path(token: str) -> str | None:
    """Is this code span a repository path we can check? Conservative by design."""
    token = token.strip().rstrip(".,;:")
    if not token or any(c in token for c in "*<>{}|$ …") or "/" not in token:
        return None
    if PLACEHOLDER_RE.search(token):
        return None
    if token.startswith(("http://", "https://", "//", "~", "/")):
        return None
    root = token.split("/", 1)[0]
    return token if root in PATH_ROOTS else None


def check_runtime_links(errors: list[str]) -> None:
    for runtime, entries in RUNTIME_LINKS.items():
        base = ROOT / runtime
        if not base.is_dir():
            errors.append(f"{runtime}/ is missing; Codex sees no shared instruction surface")
            continue
        for entry in entries:
            link = base / entry
            target = ROOT / ".claude" / entry
            if not link.exists():
                errors.append(
                    f"{runtime}/{entry} is missing; `.claude/{entry}` is not visible to that "
                    f"runtime. Create it with: ln -s ../.claude/{entry} {runtime}/{entry}"
                )
            elif link.resolve() != target.resolve():
                errors.append(
                    f"{runtime}/{entry} resolves to {link.resolve()}, not {target.resolve()}"
                )

    for surface in RUNTIME_ONLY_SURFACES:
        if (ROOT / surface).is_dir():
            errors.append(
                f"{surface}/ exists: a workflow there is visible to one runtime only. "
                "Write it as a skill under .claude/skills/, which every runtime sees."
            )


def check_canonical_source(errors: list[str]) -> None:
    claude = ROOT / "CLAUDE.md"
    if not claude.is_file():
        errors.append("CLAUDE.md is missing")
        return
    first = claude.read_text(encoding="utf-8").lstrip().splitlines()[:1]
    if not first or first[0].strip() != "@AGENTS.md":
        errors.append(
            "CLAUDE.md must start with `@AGENTS.md`: AGENTS.md is the canonical source both "
            "runtimes read, and CLAUDE.md adds only harness specifics"
        )


def check_skills_and_agents(errors: list[str]) -> None:
    skills_dir = ROOT / ".claude" / "skills"
    for entry in sorted(skills_dir.iterdir()) if skills_dir.is_dir() else []:
        if not entry.is_dir():
            continue
        skill = entry / "SKILL.md"
        if not skill.is_file():
            errors.append(f".claude/skills/{entry.name}/ has no SKILL.md")
            continue
        text = skill.read_text(encoding="utf-8")
        match = NAME_RE.search(text)
        if not match:
            errors.append(f"{skill.relative_to(ROOT)}: front matter has no `name:`")
        elif match.group(1) != entry.name:
            errors.append(
                f"{skill.relative_to(ROOT)}: name `{match.group(1)}` does not match its "
                f"directory `{entry.name}`; both runtimes dispatch on the directory"
            )
        if not DESCRIPTION_RE.search(text):
            errors.append(
                f"{skill.relative_to(ROOT)}: front matter has no `description:`; without one "
                "nothing can decide when the skill applies"
            )

    agents_dir = ROOT / ".claude" / "agents"
    for agent in sorted(agents_dir.glob("*.md")) if agents_dir.is_dir() else []:
        text = agent.read_text(encoding="utf-8")
        match = NAME_RE.search(text)
        if not match:
            errors.append(f"{agent.relative_to(ROOT)}: front matter has no `name:`")
        elif match.group(1) != agent.stem:
            errors.append(
                f"{agent.relative_to(ROOT)}: name `{match.group(1)}` does not match its "
                f"filename `{agent.stem}`"
            )


def check_paths_and_recipes(errors: list[str]) -> tuple[int, int]:
    exempt = allowlist()
    recipes = just_recipes()
    if not recipes:
        errors.append("`just --summary` produced nothing; cannot check recipe references")
    checked_paths = 0
    checked_recipes = 0

    for doc in instruction_files():
        rel = doc.relative_to(ROOT)
        text = doc.read_text(encoding="utf-8")

        for span in CODE_SPAN_RE.findall(text):
            token = candidate_path(span)
            if token is None or token in exempt:
                continue
            checked_paths += 1
            if not (ROOT / token).exists():
                errors.append(
                    f"{rel}: names `{token}`, which does not exist. Fix the reference, or add "
                    "it to .claude/path-allowlist.txt with a comment saying when it appears."
                )

        if recipes:
            commands = list(CODE_SPAN_RE.findall(text))
            for block in FENCE_RE.findall(text):
                commands.extend(block.splitlines())
            for command in commands:
                match = JUST_RE.match(command.strip())
                if match is None:
                    continue
                name = match.group(1)
                checked_recipes += 1
                if name not in recipes:
                    errors.append(
                        f"{rel}: names `just {name}`, which is not a recipe. "
                        "`just --list` is advertised to agents as the command surface."
                    )

    return checked_paths, checked_recipes


def main() -> int:
    errors: list[str] = []
    check_runtime_links(errors)
    check_canonical_source(errors)
    check_skills_and_agents(errors)
    paths, recipes = check_paths_and_recipes(errors)

    for problem in errors:
        err(f"error: {problem}")
    if errors:
        err(f"agent config: {len(errors)} problem(s)")
        return 1

    skills = sorted(p.name for p in (ROOT / ".claude" / "skills").iterdir() if p.is_dir())
    agents = sorted(p.stem for p in (ROOT / ".claude" / "agents").glob("*.md"))
    out(
        f"agent config: OK -- {len(skills)} skill(s) and {len(agents)} subagent(s) shared by "
        f"every runtime; {paths} path reference(s) and {recipes} recipe reference(s) resolve"
    )
    out(f"  skills: {', '.join(skills)}")
    out(f"  agents: {', '.join(agents)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
