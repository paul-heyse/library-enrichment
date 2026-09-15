"""A thin client for the daemon's bounded NDJSON-RPC socket.

This adapter owns no state and indexes nothing (blueprint §2.1). It validates inputs, calls the
daemon, and maps structured errors -- that is the whole job.

**The connection is lazy and per-call.** Nothing here runs at import or during MCP
``initialize``: acceptance gate C14 requires the tool list to appear without a package fetch, a
build, or an LSP start, and connecting eagerly would also make the adapter fail to start
whenever the daemon happens to be down -- exactly when ``service_status`` is most worth asking.
"""

from __future__ import annotations

import asyncio
import json
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Literal

__all__ = [
    "ACQUISITION_TIMEOUT_SECONDS",
    "RPC_MESSAGE_BYTES",
    "DaemonClient",
    "DaemonUnavailableError",
    "socket_path",
]

# `limits.rpc_message_bytes` from the frozen config/service.example.toml. The daemon enforces
# this on its side; matching it here means an over-long request fails locally with a clear
# message instead of being truncated on the wire.
RPC_MESSAGE_BYTES = 1_048_576

# Acquisition (registry, tarball, docs.rs JSON, normalization) runs inline in Phase 1 rather
# than behind a job receipt; jobs are ADR 0017 (superseding ADR 0016). The daemon's own configured
# `[network].acquisition_timeout_seconds` (120 by default) is the deadline that produces a
# typed envelope; this larger adapter bound exists only so a wedged daemon is still reported.
ACQUISITION_TIMEOUT_SECONDS = 180.0

JSONRPC_VERSION = "2.0"


class DaemonUnavailableError(RuntimeError):
    """The daemon could not be reached.

    Not a bug and not a crash: a stopped daemon is an ordinary, reportable condition. Callers
    turn this into a truthful ``service_status`` answer rather than a failed tool call.
    """

    def __init__(
        self,
        message: str,
        *,
        cause: Literal[
            "connection",
            "timeout",
            "request_limit",
            "response_limit",
            "incomplete_frame",
            "protocol",
        ] = "connection",
    ) -> None:
        super().__init__(message)
        self.cause = cause


def socket_path() -> Path:
    """Resolve the daemon socket the same way the Rust side does.

    Kept in step with ``crates/enrichment-daemon/src/paths.rs``; ADR 0006 records the order.
    Both sides read the same environment, so a development session and the daemon it started
    cannot disagree about where the socket is.
    """
    explicit = os.environ.get("LIBENR_SOCKET")
    if explicit:
        return Path(explicit)
    home = os.environ.get("LIBENR_HOME")
    if home:
        return Path(home) / "run" / "d.sock"
    runtime = os.environ.get("XDG_RUNTIME_DIR")
    if runtime:
        return Path(runtime) / "library-enrichment" / "d.sock"
    cache = os.environ.get("XDG_CACHE_HOME")
    if cache:
        return Path(cache) / "library-enrichment" / "run" / "d.sock"
    return Path.home() / ".cache" / "library-enrichment" / "run" / "d.sock"


@dataclass(frozen=True, slots=True)
class DaemonClient:
    """One request per connection.

    Phase 0 has no connection pooling on purpose: the daemon is the shared, long-lived process
    and this adapter is the disposable one, so keeping no state here is the point.
    """

    path: Path
    timeout_seconds: float = 2.0

    @classmethod
    def from_env(cls) -> DaemonClient:
        """Build a client for the configured socket. Connects to nothing yet."""
        return cls(path=socket_path())

    async def call(
        self,
        method: str,
        params: dict[str, Any] | None = None,
        *,
        timeout_seconds: float | None = None,
    ) -> dict[str, Any]:
        """Send one JSON-RPC request and return the parsed response.

        ``timeout_seconds`` overrides the client default for one call. Acquisition methods pass
        :data:`ACQUISITION_TIMEOUT_SECONDS`; the daemon enforces its own, shorter, configured
        deadline and answers with a typed envelope, so this bound only matters if the daemon
        itself stops responding.

        Raises:
            DaemonUnavailableError: if the daemon is unreachable, too slow, or answers with
                something that is not a single JSON frame.
        """
        request = {
            "jsonrpc": JSONRPC_VERSION,
            "id": 1,
            "method": method,
            "params": params or {},
        }
        frame = json.dumps(request, separators=(",", ":"))
        if len(frame.encode()) > RPC_MESSAGE_BYTES:
            message = f"request exceeds the {RPC_MESSAGE_BYTES}-byte rpc limit"
            raise DaemonUnavailableError(message, cause="request_limit")

        deadline = self.timeout_seconds if timeout_seconds is None else timeout_seconds
        try:
            return await asyncio.wait_for(self._exchange(frame), deadline)
        except TimeoutError as exc:
            message = f"daemon did not answer within {deadline}s at {self.path}"
            raise DaemonUnavailableError(message, cause="timeout") from exc
        except OSError as exc:
            raise DaemonUnavailableError(f"daemon unreachable at {self.path}: {exc}") from exc

    async def _exchange(self, frame: str) -> dict[str, Any]:
        reader, writer = await asyncio.open_unix_connection(
            str(self.path), limit=RPC_MESSAGE_BYTES + 1
        )
        try:
            writer.write(f"{frame}\n".encode())
            await writer.drain()
            # Bounded on this side too: a daemon that streamed without a newline must not be
            # able to exhaust the adapter, which shares a process with the MCP session.
            try:
                line = await reader.readuntil(b"\n")
            except asyncio.LimitOverrunError as exc:
                raise DaemonUnavailableError(
                    f"daemon response exceeds {RPC_MESSAGE_BYTES} bytes", cause="response_limit"
                ) from exc
            except asyncio.IncompleteReadError as exc:
                raise DaemonUnavailableError(
                    "daemon closed without a complete newline-terminated frame",
                    cause="incomplete_frame",
                ) from exc
        finally:
            writer.close()
            await asyncio.gather(writer.wait_closed(), return_exceptions=True)

        if len(line) - 1 > RPC_MESSAGE_BYTES:
            raise DaemonUnavailableError(
                f"daemon response exceeds {RPC_MESSAGE_BYTES} bytes", cause="response_limit"
            )
        try:
            parsed = json.loads(line)
        except (json.JSONDecodeError, UnicodeDecodeError) as exc:
            raise DaemonUnavailableError(
                "daemon returned an invalid JSON frame", cause="protocol"
            ) from exc
        if (
            not isinstance(parsed, dict)
            or parsed.get("jsonrpc") != JSONRPC_VERSION
            or type(parsed.get("id")) is not int
            or parsed["id"] != 1
            or (("result" in parsed) == ("error" in parsed))
            or set(parsed) - {"jsonrpc", "id", "result", "error"}
        ):
            raise DaemonUnavailableError(
                "daemon response violates the correlated JSON-RPC frame contract", cause="protocol"
            )
        if "error" in parsed:
            error = parsed["error"]
            if (
                not isinstance(error, dict)
                or type(error.get("code")) is not int
                or not isinstance(error.get("message"), str)
            ):
                raise DaemonUnavailableError(
                    "daemon returned a malformed RPC error", cause="protocol"
                )
        return parsed
