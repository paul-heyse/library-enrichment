"""Make each component of the toolchain report on itself.

Every subject in this repository is reached through a program -- `rustc`, `rustdoc`, `cargo`,
`rust-analyzer` -- and each of those will answer a question about its own identity. That makes
the version story evidence rather than assumption, which matters more here than usual because
three of the four report something other than what a reader expects:

    rustdoc          the format version it EMITS is not the one docs.rs SERVES for a crate
    rust-analyzer    reports a rustc release number, not a rust-analyzer version
    cargo metadata   reports a schema version of 1 that has not changed since ~2017, while the
                     payload underneath it grows new keys without a bump

Readings are recorded beside each other and never reconciled. A single "the version is X" would
be a claim no oracle actually supports.

Standard library only.
"""

from __future__ import annotations

import json
import re
import subprocess
import tempfile
from pathlib import Path

# A tiny crate whose only job is to be documented. Keeping it minimal keeps the probe fast and
# keeps the emitted document small enough to parse in memory.
FORMAT_PROBE_SOURCE = "pub struct Probe;\n"


def run(
    *command: str,
    stdin: str | None = None,
    cwd: Path | None = None,
    check: bool = True,
    env: dict[str, str] | None = None,
) -> tuple[int, str, str]:
    """Run a command, returning (exit code, stdout, stderr). Never raises on a non-zero exit."""
    done = subprocess.run(
        command,
        input=stdin,
        cwd=cwd,
        env=env,
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    )
    if check and done.returncode != 0:
        raise OracleError(
            f"{' '.join(command)} exited {done.returncode}: {done.stderr.strip()[:300]}"
        )
    return done.returncode, done.stdout, done.stderr


class OracleError(RuntimeError):
    """A component could not be asked, or answered something the build cannot proceed from."""


def _first_line(text: str) -> str:
    return text.strip().splitlines()[0].strip() if text.strip() else ""


# --------------------------------------------------------------------------- rustc / rustdoc


def rustc_version(toolchain: str) -> str:
    """The pinned compiler's own version string, e.g. `rustc 1.100.0-nightly (809936eac ...)`."""
    code, out, err = run("rustc", f"+{toolchain}", "--version", check=False)
    if code != 0:
        raise OracleError(
            f"toolchain {toolchain} is not installed or not usable: {err.strip()[:200]}\n"
            f"Install it with:\n"
            f"    rustup toolchain install {toolchain}\n"
            f"Every MIR and dataflow fact in this repository is observed through it, so the "
            f"build stops rather than producing an index that describes a different compiler."
        )
    return _first_line(out)


def rustc_asserts(toolchain: str, release: str, commit: str) -> bool:
    """Does the pinned toolchain report exactly the pinned release and commit?"""
    reported = rustc_version(toolchain)
    return release in reported and commit in reported


def rustdoc_format_emitted(toolchain: str) -> int:
    """Produce a rustdoc JSON document with the pinned toolchain and read its format version.

    This is the only reading that describes what the *local* toolchain emits. It is deliberately
    not compared with what docs.rs served: those answer different questions, and collapsing them
    is the error the repository exists to document.
    """
    with tempfile.TemporaryDirectory(prefix="rust-code-model-format-") as workspace:
        root = Path(workspace)
        source = root / "probe.rs"
        source.write_text(FORMAT_PROBE_SOURCE, encoding="utf-8")
        out_dir = root / "doc"
        code, _, err = run(
            "rustdoc",
            f"+{toolchain}",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
            "--out-dir",
            str(out_dir),
            "--crate-name",
            "probe",
            str(source),
            check=False,
        )
        if code != 0:
            raise OracleError(f"rustdoc JSON emission failed on {toolchain}: {err.strip()[:300]}")
        document = json.loads((out_dir / "probe.json").read_text(encoding="utf-8"))
    version = document.get("format_version")
    if not isinstance(version, int):
        raise OracleError("the emitted rustdoc document carries no integer format_version")
    return version


def rustc_unpretty_values(toolchain: str) -> list[str]:
    """Ask rustc which `-Zunpretty` views it supports, by giving it one it does not.

    The error message is the catalogue. Reading it rather than hard-coding the list means the
    view index cannot drift away from the compiler that produced it.
    """
    _, _, err = run(
        "rustc",
        f"+{toolchain}",
        "-Zunpretty=__invalid__",
        "--crate-type=lib",
        "-",
        stdin="",
        check=False,
    )
    marker = "must be one of"
    if marker not in err:
        return []
    listing = err.split(marker, 1)[1].split(";", 1)[0]
    # Splitting on commas would be wrong: several values contain one, such as `hir,typed` and
    # `expanded,identified`. The backticks are the actual delimiters.
    return sorted(set(re.findall(r"`([^`]+)`", listing)))


# --------------------------------------------------------------------------- the other tools


def rust_analyzer_report() -> dict[str, str]:
    """What the installed rust-analyzer says about itself, and what that does and does not mean.

    The rustup-shipped binary is built inside the rust-lang/rust tree, so `--version` reports a
    *rustc* release. There is no in-tree artifact mapping that to an `ra_ap_*` crate version --
    the rust-analyzer repository carries no version number at all, because `xtask publish` mints
    one at publish time. So this reading cannot be compared with the pinned crate version, and
    the build records it without asserting anything about it.
    """
    code, out, _ = run("rust-analyzer", "--version", check=False)
    if code != 0:
        return {
            "binary_version": "",
            "resolution": "absent",
            "answers_for": "-",
            "note": "no rust-analyzer on PATH; the ra_ap_* index is unaffected, but the "
            "behavioural probes that drive the binary are blocked.",
        }
    return {
        "binary_version": _first_line(out),
        "resolution": "binary-self-report",
        "answers_for": "the rustc release this rust-analyzer was built in, not its ra_ap version",
        "note": "Not comparable with the pinned ra_ap_* version by any published mapping.",
    }


def cargo_metadata_schema(toolchain: str | None = None) -> dict[str, object]:
    """Run `cargo metadata` against a throwaway package and record the envelope it emits.

    The `version` field is the documented stability contract, and it has read 1 for years while
    the object beneath it gained keys. Recording the observed key set alongside the version is
    what makes that visible.
    """
    with tempfile.TemporaryDirectory(prefix="rust-code-model-cargo-") as workspace:
        root = Path(workspace)
        (root / "src").mkdir()
        (root / "src" / "lib.rs").write_text("", encoding="utf-8")
        (root / "Cargo.toml").write_text(
            '[package]\nname = "probe"\nversion = "0.0.0"\nedition = "2021"\n',
            encoding="utf-8",
        )
        command = ["cargo"]
        if toolchain:
            command.append(f"+{toolchain}")
        command += ["metadata", "--format-version", "1", "--no-deps"]
        code, out, err = run(*command, cwd=root, check=False)
        if code != 0:
            raise OracleError(f"cargo metadata failed: {err.strip()[:300]}")
        document = json.loads(out)

    cargo_version = _first_line(run("cargo", "--version", check=False)[1])
    return {
        "cargo": cargo_version,
        "schema_version": document.get("version"),
        "envelope_keys": sorted(document.keys()),
        "package_keys": sorted((document.get("packages") or [{}])[0].keys()),
        "resolve_present": document.get("resolve") is not None,
    }


def ast_grep_version() -> str:
    code, out, _ = run("ast-grep", "--version", check=False)
    return _first_line(out) if code == 0 else ""


def toolchain_roster() -> list[str]:
    """Every installed toolchain, so the reader can see what else was available."""
    code, out, _ = run("rustup", "toolchain", "list", check=False)
    if code != 0:
        return []
    return [line.split(" ", 1)[0] for line in out.strip().splitlines() if line.strip()]
