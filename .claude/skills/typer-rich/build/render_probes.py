"""Project the executed probe payload into the shipped content tree.

Offline, standard library only: this replays `PROBES.json`, it never executes anything. That
separation is what lets `verify.py` re-run `build.py` and demand identical bytes.

Two output channels, because normalising one destroys the other.

* **plain** -- `color_system=None, force_terminal=False`, written byte-exact. This is what lands
  in a log or a CI file, and it is greppable, so a `reference.md` recipe can assert against it.
* **ansi** -- written as the `repr()` of the string, not as raw escapes. Raw escapes break `rg`,
  are re-interpreted by any terminal that cats the file, and would defeat the transferability
  scan that reads every shipped file as text.

`behaviors.tsv` carries the shape and the digest; the capture files carry the bytes; the page
carries a generated projection of both. One fact, three views, and the page is never typed by
hand -- `verify.py` re-renders it and diffs.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

INLINE_LINES = 24
TRUNCATE_TO = 10


def _digest(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()[:12]


def _capture_text(row: dict) -> str:
    """The bytes a probe produced, in the form its channel calls for."""
    output = row["output"]
    if row.get("capture") == "ansi":
        return repr(output)
    return output


def _construction(row: dict) -> str:
    """The one line of source that a reader must reproduce to get these bytes back."""
    for line in reversed(row["source"].splitlines()):
        stripped = line.strip()
        if stripped.startswith("emit("):
            return stripped[5:-1] if stripped.endswith(")") else stripped
    return row["source"].splitlines()[-1].strip()


def write_all(payload: dict, content: Path) -> dict[str, int]:
    """Write `content/probes/` and return the counts PROVENANCE records."""
    out = content / "probes"
    captures = out / "captures"
    captures.mkdir(parents=True, exist_ok=True)

    rows: list[str] = []
    pages: list[str] = []
    captured = 0

    for row in sorted(payload["results"], key=lambda r: r["id"]):
        text = _capture_text(row)
        channel = row.get("capture") or "-"
        capture_path = "-"
        if row.get("capture"):
            suffix = ".ansi.txt" if channel == "ansi" else ".txt"
            name = f"{row['id']}{suffix}"
            # The digest covers exactly the bytes written, newline included, so verify.py can
            # hash the file as it stands rather than re-deriving how it was terminated.
            text = text if text.endswith("\n") else text + "\n"
            (captures / name).write_text(text)
            capture_path = f"probes/captures/{name}"
            captured += 1

        rows.append(
            "\t".join(
                (
                    row["id"],
                    row["subject"],
                    row["topic"],
                    row["verdict"],
                    row["question"].replace("\t", " "),
                    _construction(row).replace("\t", " "),
                    row["expect"],
                    row.get("control_expect", "-") if row.get("control") else "-",
                    ",".join(row["normalise"]) or "-",
                    channel,
                    _digest(text),
                    capture_path,
                )
            )
        )
        pages.append(_page_section(row, text, capture_path))

    (out / "00-index.md").write_text(_index_page(payload, pages))
    return {"probes": len(rows), "probe_captures": captured, "behaviors": len(rows)}


def _page_section(row: dict, text: str, capture_path: str) -> str:
    """One probe, rendered inline.

    Inline because a ladder rung that costs a second read to see one table is a rung nobody
    takes, and "what does this look like" is exactly the question that must not need a second
    hop. The block is generated from the capture, never typed.
    """
    lines = text.splitlines()
    shown = lines
    trailer = ""
    if len(lines) > INLINE_LINES:
        shown = lines[:TRUNCATE_TO]
        trailer = f"\n(truncated -- full capture in {capture_path}, {len(lines)} lines)"

    caption = (
        f"{row['id']} · {_construction(row)} · "
        f"{row['verdict']} · {capture_path if capture_path != '-' else 'not captured'}"
    )
    parts = [
        f"## {row['id']} — {row['question']}",
        "",
        f"`{caption}`",
        "",
        "```text",
        *shown,
        "```" + trailer,
    ]
    if row.get("control"):
        parts += [
            "",
            f"**Control** (`{row['control_expect']}`): "
            f"`{_construction({'source': row['control']})}`",
            "",
            f"> {row['control_output'][:200].strip()}"
            if row.get("control_output")
            else "> (no output)",
        ]
    if row["normalise"]:
        parts += ["", f"_Normalised: {', '.join(row['normalise'])}._"]
    if row.get("note"):
        parts += ["", row["note"]]
    return "\n".join(parts)


def _index_page(payload: dict, sections: list[str]) -> str:
    summary = ", ".join(f"{count} {name}" for name, count in payload["summary"].items())
    header = [
        "# Observed behaviour",
        "",
        f"{len(payload['results'])} probes executed against the pinned capsule: {summary}.",
        "",
        "Every capture below is a fact about a **construction**, not about a terminal. Each pins",
        "`width`, `color_system`, `force_terminal` and `legacy_windows`, and runs under a replaced",
        "environment that inherits nothing -- change any one of those and the bytes change. Quote",
        "the construction whenever you quote the output.",
        "",
        "`confirmed` means the probe and its control both came out as expected and differed.",
        "`recorded` is a shape probe with nothing to falsify -- weaker, and labelled as such.",
        "An `inconclusive` probe demonstrates nothing and is never reported as a pass.",
        "",
        "Animated rendering is deliberately absent; see `reference.md` for what this index does",
        "not claim.",
        "",
        "---",
        "",
    ]
    return "\n".join(header) + "\n\n---\n\n".join(sections) + "\n"


def behaviors_rows(payload: dict) -> list[str]:
    """The `behaviors.tsv` rows, recomputed the same way `write_all` writes them."""
    rows = []
    for row in sorted(payload["results"], key=lambda r: r["id"]):
        text = _capture_text(row)
        channel = row.get("capture") or "-"
        suffix = ".ansi.txt" if channel == "ansi" else ".txt"
        capture_path = f"probes/captures/{row['id']}{suffix}" if row.get("capture") else "-"
        if row.get("capture"):
            text = text if text.endswith("\n") else text + "\n"
        rows.append(
            "\t".join(
                (
                    row["id"],
                    row["subject"],
                    row["topic"],
                    row["verdict"],
                    row["question"].replace("\t", " "),
                    _construction(row).replace("\t", " "),
                    row["expect"],
                    row.get("control_expect", "-") if row.get("control") else "-",
                    ",".join(row["normalise"]) or "-",
                    channel,
                    _digest(text),
                    capture_path,
                )
            )
        )
    return rows


def load(acquired: Path, acquisition: dict) -> dict:
    """Read `PROBES.json`, verified against the digest acquisition recorded.

    Locally produced bytes with no published pin behind them, so they are checked rather than
    trusted -- the same reasoning `load_analysis` applies to the checker payload.
    """
    entry = (acquisition.get("files") or {}).get("PROBES.json")
    path = acquired / "PROBES.json"
    if entry is None or not path.is_file():
        raise SystemExit("PROBES.json is absent from the acquisition; re-run acquire.py")
    blob = path.read_bytes()
    actual = hashlib.sha256(blob).hexdigest()
    if actual != entry["sha256"]:
        raise SystemExit(
            f"PROBES.json does not match its recorded digest "
            f"({actual[:12]} vs {entry['sha256'][:12]}); re-run acquire.py"
        )
    return json.loads(blob)
