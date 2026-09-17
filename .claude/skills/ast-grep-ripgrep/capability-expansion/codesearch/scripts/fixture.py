"""Materialise a modified copy of the skill, so `compare` has two snapshots to compare.

`compare` is the only projection that crosses snapshots, and over identical inputs all three of
its paths report "no change" -- which is the one case that proves nothing. There is no git history
for the skill's shipped indexes, so a genuine earlier snapshot is not available and one has to be
constructed.

The copy lives OUTSIDE every repository, per the boundary rule, and it has to be a copy for a
second reason the boundary rule does not cover: the skill's own `build/verify.py` checks every
shipped file against a recorded digest, so editing an index in place would break the host skill's
gate rather than this tool's.

Three mutations, chosen so that all three of `compare`'s kinds are exercised:

    added     one flag row appended
    removed   one flag row deleted
    changed   one behaviour verdict upgraded from `recorded` to `confirmed`

The flag pair is deliberately one-in-one-out. `PROVENANCE.json` records a row count per index and
the build refuses a file that disagrees with it, so a net change of zero keeps the fixture valid
without rewriting the provenance the copy inherited.

The removed flag is named by none of the four seeds. A seed naming a mechanism the catalog does
not hold is a build error, so a careless choice here would not produce a subtle wrong answer -- it
would simply fail to build -- but choosing deliberately is cheaper than rediscovering that.
"""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

#: The flag to delete. No seed names it, and `scripts/bench.py` measures `--pcre2`, not this.
REMOVE_FLAG = "block-buffered"

#: The flag to add. Named so that it cannot be mistaken for something ripgrep actually ships.
ADD_FLAG = "fixture-only-flag"

#: The probe whose verdict is upgraded.
UPGRADE_PROBE = "A003"


def mutate_flags(path: Path) -> tuple[str, str]:
    """Remove one flag row and append another. Returns the two `long` names, for reporting."""
    rows = [line for line in path.read_text().splitlines() if line]
    kept = [r for r in rows if r.split("\t")[:3] != ["rg", "rg", REMOVE_FLAG]]
    if len(kept) != len(rows) - 1:
        gone = len(rows) - len(kept)
        raise SystemExit(f"expected one `rg` row for --{REMOVE_FLAG}, removed {gone}")

    # Eight columns: tool, command, long, short, category, arg, values, summary. The `arg` column
    # is empty on purpose -- a flag that takes an argument would also produce a `catalog.parameter`
    # row, and the smaller the expected diff, the sharper the assertion over it.
    summary = "A flag that exists only in this fixture."
    kept.append(f"rg\trg\t{ADD_FLAG}\t\tSEARCH OPTIONS\t\t\t{summary}")
    path.write_text("\n".join(kept) + "\n")
    return REMOVE_FLAG, ADD_FLAG


def mutate_behaviours(path: Path) -> str:
    """Upgrade one probe's verdict from `recorded` to `confirmed`."""
    rows = [line for line in path.read_text().splitlines() if line]
    out: list[str] = []
    upgraded = 0
    for row in rows:
        cells = row.split("\t")
        if cells[0] == UPGRADE_PROBE and cells[3] == "recorded":
            cells[3] = "confirmed"
            upgraded += 1
        out.append("\t".join(cells))
    if upgraded != 1:
        raise SystemExit(f"expected one `recorded` {UPGRADE_PROBE} row, upgraded {upgraded}")
    path.write_text("\n".join(out) + "\n")
    return UPGRADE_PROBE


def main() -> int:
    write = sys.stdout.write
    if len(sys.argv) != 3:
        write("usage: fixture.py <skill-root> <destination>\n")
        return 2
    skill_root = Path(sys.argv[1]).resolve()
    destination = Path(sys.argv[2]).resolve()

    source = skill_root / "content"
    if not (source / "PROVENANCE.json").is_file():
        write(f"blocked: {source} does not look like a skill's content directory\n")
        return 1
    if destination.is_relative_to(skill_root):
        write("blocked: the fixture must not be written inside the skill it copies\n")
        return 1

    if destination.exists():
        shutil.rmtree(destination)
    shutil.copytree(source, destination / "content")

    index = destination / "content" / "index"
    removed, added = mutate_flags(index / "flags.tsv")
    upgraded = mutate_behaviours(index / "behaviors.tsv")

    write(f"fixture   {destination}\n")
    write(f"removed   flags.tsv  rg --{removed}\n")
    write(f"added     flags.tsv  rg --{added}\n")
    write(f"upgraded  behaviors.tsv  {upgraded}  recorded -> confirmed\n")
    write("\nBuild it with a second `codesearch-build --skill-root` against this path. The\n")
    write("snapshot id is derived from the index bytes, so it differs on its own.\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
