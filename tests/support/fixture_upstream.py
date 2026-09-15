"""A stand-in upstream for crates.io and docs.rs, served from canned files over loopback HTTP.

This is not a mock of any service component. It is an HTTP server the daemon's real fetcher
talks to through its real client, redirect policy, size bounds and zstd path, so the fixture
tier exercises the boundary the live tier does -- only against bytes committed under
``tests/fixtures/upstream/`` instead of the internet (blueprint §14.1: "Core CI should not
require a live internet response to remain stable").

Routes mirror the measured upstream shapes (docs/architecture/compatibility-matrix.md,
2026-09-13). Each is served from a file under the fixture root:

- ``/index/<prefix>/<name>`` -> ``index/<name>.ndjson``
- ``/api/v1/crates/<name>/<version>`` -> ``api/<name>-<version>.json``
- ``/api/v1/crates/<name>/<version>/download`` -> 302 to ``/static/<name>-<version>.crate``
- ``/static/<file>.crate`` -> ``static/<file>.crate`` as ``application/gzip``
- ``/crate/<name>/<version>/json`` -> ``docsrs/<name>-<version>.json.zst`` as
  ``application/zstd``, or 404 when the file is absent
- ``/crate/<name>/<version>/<target>/json`` -> ``docsrs/<name>-<version>-<target>.json.zst``
- ``/crate/<name>/<version>/status.json`` -> ``{"doc_status": true, "version": ...}``, as
  docs.rs answers even for a release with no JSON

Three deliberate misbehaviours for policy tests, all under ``/_test/``:

- ``/_test/redirect-loop`` redirects to itself.
- ``/_test/redirect-private`` redirects to ``http://169.254.169.254/latest/meta-data``.
- ``/_test/oversized/<bytes>`` streams that many zero bytes.

Run standalone: ``python tests/support/fixture_upstream.py --root tests/fixtures/upstream``.
It binds an ephemeral loopback port and prints ``listening on http://127.0.0.1:<port>`` on
stdout once ready, which is what the Rust integration tests wait for. Import ``serve`` for the
pytest tier.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
import threading
from collections.abc import Iterator
from contextlib import contextmanager
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

NOT_FOUND_BODY = b"<!DOCTYPE html><title>404</title>"


class FixtureUpstream(ThreadingHTTPServer):
    """A threading HTTP server rooted at a fixture directory."""

    daemon_threads = True
    allow_reuse_address = True

    def __init__(self, root: Path, host: str = "127.0.0.1", port: int = 0) -> None:
        self.root = root
        self.request_log: list[str] = []
        self.response_log: list[tuple[str, int, str | None]] = []
        self.held_paths: dict[str, tuple[threading.Event, threading.Event]] = {}
        super().__init__((host, port), FixtureHandler)

    @property
    def base_url(self) -> str:
        host, port = self.server_address[:2]
        return f"http://{host}:{port}"


class FixtureHandler(BaseHTTPRequestHandler):
    """Route one request to a fixture file."""

    server: FixtureUpstream
    protocol_version = "HTTP/1.1"

    def log_message(self, format: str, *args: object) -> None:
        # Keep stdout clean (the port line is a protocol) and stderr quiet in tests. The
        # request log is what a test inspects instead.
        del format, args
        self.server.request_log.append(self.path)

    def do_GET(self) -> None:  # the stdlib dispatches on this exact name
        self._route()

    def do_HEAD(self) -> None:  # the stdlib dispatches on this exact name
        self._route()

    def _route(self) -> None:
        hold = self.server.held_paths.get(self.path)
        if hold:
            entered, release = hold
            entered.set()
            if not release.wait(timeout=30):
                return
        parts = [p for p in self.path.split("?")[0].split("/") if p]
        root = self.server.root

        if len(parts) == 5 and parts[0] == "repos" and parts[3] in {"commits", "tarball"}:
            if self.headers.get("X-GitHub-Api-Version") != "2026-03-10":
                self._bytes(HTTPStatus.BAD_REQUEST, b"Missing API version", "text/plain")
            else:
                self._file(
                    root.joinpath(*parts),
                    "application/json" if parts[3] == "commits" else "application/gzip",
                )
        elif len(parts) == 3 and parts[0] == "pypi" and parts[2] == "json":
            path = root / "pypi" / parts[1] / "registry.json"
            if path.is_file():
                body = path.read_bytes().replace(b"{{BASE_URL}}", self.server.base_url.encode())
                self._bytes(HTTPStatus.OK, body, "application/json")
            else:
                self._bytes(HTTPStatus.NOT_FOUND, NOT_FOUND_BODY, "text/html")
        elif len(parts) == 4 and parts[0] == "pypi" and parts[3] == "json":
            path = root / "pypi" / parts[1] / f"{parts[2]}.json"
            if path.is_file():
                body = path.read_bytes().replace(b"{{BASE_URL}}", self.server.base_url.encode())
                self._bytes(HTTPStatus.OK, body, "application/json")
            else:
                self._bytes(HTTPStatus.NOT_FOUND, NOT_FOUND_BODY, "text/html")
        elif len(parts) == 2 and parts[0] == "simple":
            self._file(
                root / "pypi" / parts[1] / "index.json", "application/vnd.pypi.simple.v1+json"
            )
        elif parts and parts[0] == "docs":
            self._file(root.joinpath(*parts), "application/octet-stream")
        elif len(parts) >= 3 and parts[0] == "index":
            self._file(root / "index" / f"{parts[-1]}.ndjson", "text/plain")
        elif len(parts) == 5 and parts[:3] == ["api", "v1", "crates"]:
            name, version = parts[3], parts[4]
            self._file(root / "api" / f"{name}-{version}.json", "application/json")
        elif len(parts) == 6 and parts[:3] == ["api", "v1", "crates"] and parts[5] == "download":
            name, version = parts[3], parts[4]
            self._redirect(f"{self.server.base_url}/static/{name}-{version}.crate")
        elif len(parts) == 2 and parts[0] == "static":
            self._file(root / "static" / parts[1], "application/gzip")
        elif len(parts) == 4 and parts[0] == "crate" and parts[3] == "json":
            name, version = parts[1], parts[2]
            self._file(root / "docsrs" / f"{name}-{version}.json.zst", "application/zstd")
        elif len(parts) == 4 and parts[0] == "crate" and parts[3] == "status.json":
            body = json.dumps({"doc_status": True, "version": parts[2]}).encode()
            self._bytes(HTTPStatus.OK, body, "application/json")
        elif len(parts) == 5 and parts[0] == "crate" and parts[4] == "json":
            name, version, target = parts[1], parts[2], parts[3]
            path = root / "docsrs" / f"{name}-{version}-{target}.json.zst"
            self._file(path, "application/zstd")
        elif parts[:2] == ["_test", "redirect-loop"]:
            self._redirect(f"{self.server.base_url}/_test/redirect-loop")
        elif parts[:2] == ["_test", "redirect-private"]:
            self._redirect("http://169.254.169.254/latest/meta-data")
        elif len(parts) == 3 and parts[:2] == ["_test", "oversized"]:
            self._bytes(HTTPStatus.OK, b"\0" * int(parts[2]), "application/octet-stream")
        else:
            self._bytes(HTTPStatus.NOT_FOUND, NOT_FOUND_BODY, "text/html")

    def _file(self, path: Path, content_type: str) -> None:
        if not path.is_file():
            self._bytes(HTTPStatus.NOT_FOUND, NOT_FOUND_BODY, "text/html")
            return
        body = path.read_bytes()
        self._bytes(HTTPStatus.OK, body, content_type, etag=f'"{hashlib.sha256(body).hexdigest()}"')

    def _redirect(self, location: str) -> None:
        self.send_response(HTTPStatus.FOUND)
        self.send_header("Location", location)
        self.send_header("Content-Length", "0")
        self.end_headers()

    def _bytes(
        self,
        status: HTTPStatus,
        body: bytes,
        content_type: str,
        etag: str | None = None,
    ) -> None:
        if status == HTTPStatus.OK and etag and self.headers.get("If-None-Match") == etag:
            status, body = HTTPStatus.NOT_MODIFIED, b""
        self.server.response_log.append((self.path, int(status), self.headers.get("If-None-Match")))
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        if etag:
            self.send_header("ETag", etag)
        self.end_headers()
        if self.command != "HEAD":
            try:
                self.wfile.write(body)
            except (BrokenPipeError, ConnectionResetError):
                # Cancellation tests deliberately close the fetch while this server is held.
                return


@contextmanager
def serve(root: Path) -> Iterator[FixtureUpstream]:
    """Run a fixture upstream on an ephemeral loopback port for the duration of the block."""
    server = FixtureUpstream(root)
    thread = threading.Thread(target=server.serve_forever, name="fixture-upstream", daemon=True)
    thread.start()
    try:
        yield server
    finally:
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)


def main(argv: list[str] | None = None) -> int:
    """Serve a fixture root until interrupted, announcing the bound port on stdout."""
    parser = argparse.ArgumentParser(description="Loopback stand-in for crates.io and docs.rs.")
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--port", type=int, default=0)
    args = parser.parse_args(argv)
    if not args.root.is_dir():
        sys.stderr.write(f"fixture_upstream: {args.root} is not a directory\n")
        return 2
    server = FixtureUpstream(args.root.resolve(), port=args.port)
    sys.stdout.write(f"listening on {server.base_url}\n")
    sys.stdout.flush()
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
