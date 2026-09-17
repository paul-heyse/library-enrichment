"""Which of the three subjects a path belongs to.

Three subjects, two pins. `typer._click` needs no second Griffe load and no entry in
`distributions`: Griffe already descends into private submodules, so the vendored copy is
indexed whether or not anyone names it. Naming it is what lets a row say *which library* it is
a fact about -- and that distinction is the whole point here, because `typer._click.Command` and
`click.Command` are different classes with the same name, and the one an agent imports is the
wrong one.

Longest-prefix wins, so `typer._click.core` resolves to `click` rather than `typer` even though
both prefixes match.
"""

from __future__ import annotations

from collections.abc import Callable, Iterable


class SubjectError(RuntimeError):
    """A path belongs to no declared subject."""


def _rules(manifest: dict) -> list[tuple[str, str, tuple[str, ...]]]:
    rules: list[tuple[str, str, tuple[str, ...]]] = []
    for subject in manifest["subjects"]:
        for prefix in subject["module_prefixes"]:
            rules.append((prefix, subject["name"], tuple(subject.get("exclude_prefixes", ()))))
    # Longest prefix first: `typer._click` must be tested before `typer`.
    return sorted(rules, key=lambda row: len(row[0]), reverse=True)


def _matches(path: str, prefix: str) -> bool:
    return path == prefix or path.startswith(f"{prefix}.")


def classifier(manifest: dict) -> Callable[[str], str]:
    """Return `path -> subject name`, built once per build."""
    rules = _rules(manifest)

    def subject_of(path: str) -> str:
        for prefix, name, excludes in rules:
            if not _matches(path, prefix):
                continue
            if any(_matches(path, skip) for skip in excludes):
                continue
            return name
        raise SubjectError(
            f"{path!r} belongs to no declared subject; every indexed path must be attributable, "
            "because a row that cannot say which library it describes is worse than no row"
        )

    return subject_of


def counts(manifest: dict, paths: Iterable[str]) -> dict[str, int]:
    subject_of = classifier(manifest)
    tally = {subject["name"]: 0 for subject in manifest["subjects"]}
    for path in paths:
        tally[subject_of(path)] += 1
    return tally
