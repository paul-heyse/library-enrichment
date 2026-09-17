"""The `ty` language-server supplement, narrowed to the one question Griffe cannot answer.

Griffe reports annotations as written. Where nothing is written it has nothing to say, and for
this library that residue is almost entirely **attributes**: 521 public ones carry no annotation,
against 20 functions with no return type. So this stage asks a type checker for those, and only
those.

Everything broader was measured against Griffe and lost:

* `typeHierarchy/subtypes` is direct-only -- 10 children of `Provider` against a `bases` closure
  of 26, and its list omits `FastMCP` itself, which reaches `Provider` through
  `AggregateProvider`. "A FastMCP server is itself a Provider" is the central fact of 4.0's
  composition model, and the ty table would have dropped it silently.
* `textDocument/references` returned a single hit for a heavily-used class.
* Whole-file `inlayHint` returns mostly parameter-*name* hints and method-body locals, not
  public API.

Two invariants hold whatever ty says. A declared annotation is **never** overwritten by an
inferred one, and every row carries the ty version, so an inference is visibly second-class
against a fact read out of the source. If ty is missing or errors, the stage reports `blocked`
with the prerequisite named rather than quietly producing a thinner table.
"""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from collections.abc import Callable
from pathlib import Path

TIMEOUT_SECONDS = 30
MAX_RESPONSE_WAIT = 200
# ty renders a hover as a fenced `python` block holding the declaration it inferred.
HOVER_TYPE = re.compile(r"```python\n(.*?)\n```", re.DOTALL)


class LspError(RuntimeError):
    """The language server could not be driven to a useful answer."""


class Client:
    """A minimal LSP client. Framed JSON-RPC over the server's stdin and stdout."""

    def __init__(self, command: list[str], root: Path) -> None:
        self._process = subprocess.Popen(
            command,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
        self._next_id = 0
        self._root = root

    def _send(self, message: dict) -> None:
        blob = json.dumps(message).encode()
        assert self._process.stdin is not None
        self._process.stdin.write(b"Content-Length: %d\r\n\r\n" % len(blob) + blob)
        self._process.stdin.flush()

    def _receive(self) -> dict | None:
        assert self._process.stdout is not None
        length = 0
        while True:
            line = self._process.stdout.readline()
            if not line:
                return None
            if line in (b"\r\n", b"\n"):
                break
            if line.lower().startswith(b"content-length"):
                length = int(line.split(b":")[1])
        if not length:
            return None
        return json.loads(self._process.stdout.read(length))

    def request(self, method: str, params: dict) -> object:
        self._next_id += 1
        identifier = self._next_id
        self._send({"jsonrpc": "2.0", "id": identifier, "method": method, "params": params})
        for _ in range(MAX_RESPONSE_WAIT):
            message = self._receive()
            if message is None:
                raise LspError(f"{method}: server closed the connection")
            if message.get("id") == identifier:
                if "error" in message:
                    raise LspError(f"{method}: {message['error']}")
                return message.get("result")
        raise LspError(f"{method}: no response within {MAX_RESPONSE_WAIT} messages")

    def notify(self, method: str, params: dict) -> None:
        self._send({"jsonrpc": "2.0", "method": method, "params": params})

    def initialize(self) -> dict:
        result = self.request(
            "initialize",
            {
                "processId": None,
                "rootUri": self._root.as_uri(),
                "capabilities": {"textDocument": {"hover": {"contentFormat": ["markdown"]}}},
                "workspaceFolders": [{"uri": self._root.as_uri(), "name": self._root.name}],
            },
        )
        self.notify("initialized", {})
        return result or {}

    def open(self, path: Path, text: str) -> str:
        uri = path.as_uri()
        self.notify(
            "textDocument/didOpen",
            {
                "textDocument": {
                    "uri": uri,
                    "languageId": "python",
                    "version": 1,
                    "text": text,
                }
            },
        )
        return uri

    def close(self, uri: str) -> None:
        self.notify("textDocument/didClose", {"textDocument": {"uri": uri}})

    def hover(self, uri: str, line: int, character: int) -> str | None:
        result = self.request(
            "textDocument/hover",
            {"textDocument": {"uri": uri}, "position": {"line": line, "character": character}},
        )
        if not result:
            return None
        contents = result.get("contents")
        text = contents.get("value") if isinstance(contents, dict) else str(contents)
        if not text:
            return None
        match = HOVER_TYPE.search(text)
        return (match.group(1) if match else text).strip() or None

    def shutdown(self) -> None:
        try:
            self.request("shutdown", {})
            self.notify("exit", {})
        except (LspError, OSError):
            pass
        finally:
            self._process.kill()
            self._process.wait(timeout=TIMEOUT_SECONDS)


def _utf16_column(line_text: str, name: str) -> int | None:
    """Column of `name` in the line, in UTF-16 code units, as LSP positions require."""
    index = line_text.find(name)
    if index < 0:
        return None
    return len(line_text[:index].encode("utf-16-le")) // 2


def _targets(documents: dict[str, dict]) -> list[dict]:
    """Public items whose type the source does not state. Nothing else is asked about."""
    rows: list[dict] = []
    for document in documents.values():
        for item in document["items"]:
            if not item.get("underscore_free") or not item.get("file") or not item.get("lineno"):
                continue
            if item["kind"] == "attribute" and item.get("unannotated"):
                rows.append(
                    {
                        "path": item["path"],
                        "name": item["name"],
                        "kind": "attribute",
                        "file": item["file"],
                        "lineno": item["lineno"],
                    }
                )
            elif item["kind"] == "function" and item.get("unannotated_return"):
                rows.append(
                    {
                        "path": item["path"],
                        "name": item["name"],
                        "kind": "return",
                        "file": item["file"],
                        "lineno": item["lineno"],
                    }
                )
    return sorted(rows, key=lambda row: (row["file"], row["lineno"], row["path"]))


def supplement(
    manifest: dict,
    documents: dict[str, dict],
    site_packages: Path,
    *,
    say: Callable[[str], None],
) -> dict:
    """Ask ty for the types the source does not state. Returns a status record."""
    expected = manifest["tools"].get("ty_version", "")
    if shutil.which("ty") is None:
        return {"status": "blocked", "reason": "ty is not on PATH", "rows": []}

    version = subprocess.run(
        ["ty", "--version"], capture_output=True, text=True, check=False
    ).stdout.strip()
    if expected and expected not in version:
        say(f"  ! ty is {version!r}, manifest expects {expected}")

    targets = _targets(documents)
    say(f"  {len(targets)} positions where the source states no type")
    if not targets:
        return {"status": "passed", "version": version, "rows": [], "asked": 0}

    by_file: dict[str, list[dict]] = {}
    for row in targets:
        by_file.setdefault(row["file"], []).append(row)

    client = Client(["ty", "server"], site_packages)
    resolved: list[dict] = []
    unresolved: list[str] = []
    try:
        client.initialize()
        for relative, rows in sorted(by_file.items()):
            path = site_packages / relative
            if not path.is_file():
                unresolved.extend(row["path"] for row in rows)
                continue
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
            uri = client.open(path, "\n".join(lines))
            try:
                for row in rows:
                    index = row["lineno"] - 1
                    if index < 0 or index >= len(lines):
                        unresolved.append(row["path"])
                        continue
                    column = _utf16_column(lines[index], row["name"])
                    if column is None:
                        unresolved.append(row["path"])
                        continue
                    try:
                        inferred = client.hover(uri, index, column)
                    except LspError:
                        inferred = None
                    if inferred:
                        resolved.append(
                            {
                                "path": row["path"],
                                "kind": row["kind"],
                                "inferred": " ".join(inferred.split()),
                                "source": f"ty@{version.split()[-1] if version else 'unknown'}",
                            }
                        )
                    else:
                        unresolved.append(row["path"])
            finally:
                client.close(uri)
    except (LspError, OSError) as error:
        client.shutdown()
        return {
            "status": "blocked",
            "reason": f"{type(error).__name__}: {error}",
            "version": version,
            "rows": sorted(resolved, key=lambda row: row["path"]),
            "asked": len(targets),
        }
    client.shutdown()

    say(f"  ty answered {len(resolved)} of {len(targets)}")
    return {
        "status": "passed",
        "version": version,
        "asked": len(targets),
        "rows": sorted(resolved, key=lambda row: row["path"]),
        "unresolved": sorted(set(unresolved)),
    }


if __name__ == "__main__":
    sys.stderr.write(__doc__ or "")
