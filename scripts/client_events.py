"""Correlate native client invocations and results; model text cannot establish a tool call.

Parser fixtures test this mechanism only. A client acceptance gate still requires an actual
authenticated trace, successful typed results and independent daemon identity checks.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass, field
from typing import Any

from enrichment_mcp.envelope import validate_document, validate_tool_data

SERVICE = "library-enrichment"
SERVERS = {SERVICE, "context7"}


@dataclass
class Invocation:
    call_id: str
    server: str
    tool: str
    arguments: dict[str, Any]
    started: int
    completed: int | None = None
    result: object = None
    error: str | None = None

    def envelope(self) -> dict[str, Any]:
        """Validate the wire and tool payload emitted by the Rust-owned service contract."""
        if self.server != SERVICE or self.completed is None:
            raise ValueError(f"{self.call_id}: no completed service result")
        payload = result_object(self.result)
        valid, reason = validate_document(json.dumps(payload))
        if not valid:
            raise ValueError(f"{self.call_id}: invalid result envelope: {reason}")
        flagged = self.error or (
            isinstance(self.result, dict)
            and (self.result.get("isError") or self.result.get("is_error"))
        )
        terminal = payload.get("data", {}).get("result")
        failed_job = isinstance(terminal, dict) and terminal.get("outcome") == "error"
        if flagged and payload["status"] != "error" and not failed_job:
            raise ValueError(f"{self.call_id}: failed MCP result claims usable evidence")
        if payload["status"] in {"ok", "partial"} and payload["delivery"]["mode"] == "inline":
            valid, reason = validate_tool_data(self.tool, payload["data"])
            if not valid:
                raise ValueError(f"{self.call_id}: invalid {self.tool} payload: {reason}")
        return payload

    def failure_preview(self) -> dict[str, Any] | None:
        """A bounded native recovery projection is evidence of failure, never success."""
        if not self.error or self.completed is None or self.server != SERVICE:
            return None
        content = self.result
        if isinstance(content, dict):
            content = content.get("content")
        if isinstance(content, list):
            texts = [
                b.get("text") for b in content if isinstance(b, dict) and b.get("type") == "text"
            ]
        else:
            texts = [content]
        previews = []
        for text in texts:
            if not isinstance(text, str):
                continue
            try:
                value = json.loads(text)
            except json.JSONDecodeError:
                continue
            if isinstance(value, dict) and value.get("format") == "research-error-preview/1":
                previews.append(value)
        if len(previews) != 1:
            return None
        preview = previews[0]
        # A pre-admission argument error is not a terminal job observation, even when
        # the rejected request contained a job_id. Only terminal previews bind a job.
        if "job_id" not in preview:
            return None
        if preview.get("job_id") != self.arguments.get("job_id") or not preview.get("code"):
            raise ValueError("failure preview does not match its native job invocation")
        return preview


@dataclass
class ArtifactChain:
    previous: ArtifactChain | None
    chunk: bytes
    end: int
    completed: int


@dataclass
class Trace:
    calls: list[Invocation] = field(default_factory=list)
    successful_turn: bool = False
    answer: str = ""
    prerequisite_error: str | None = None

    def successful(self, tool: str, *, server: str = SERVICE) -> list[Invocation]:
        calls = []
        for call in self.calls:
            if call.server != server or call.tool != tool or call.completed is None or call.error:
                continue
            if isinstance(call.result, dict) and (
                call.result.get("isError") or call.result.get("is_error")
            ):
                continue
            if server == SERVICE and self.answer_for(call)["status"] not in {"ok", "partial"}:
                continue
            calls.append(call)
        return calls

    def answer_for(self, call: Invocation) -> dict[str, Any]:
        """Follow only completed native job and artifact results visible to this client."""
        payload = call.envelope()
        if payload["status"] == "pending":
            job_id = payload["job"]["job_id"]
            for poll in self.calls:
                if (
                    poll.server == SERVICE
                    and poll.tool == "job_control"
                    and poll.arguments.get("job_id") == job_id
                    and poll.started > call.started
                    and poll.completed is not None
                    and not poll.error
                ):
                    response = self.expand(poll.envelope())
                    result = response.get("data", {}).get("result")
                    if response["status"] == "ok" and isinstance(result, dict):
                        payload = self.terminal_descriptor(response, result)
                        break
        payload = self.expand(payload)
        valid, reason = validate_document(json.dumps(payload))
        if not valid:
            raise ValueError(f"invalid completed envelope: {reason}")
        if payload["status"] in {"ok", "partial"}:
            valid, reason = validate_tool_data(call.tool, payload["data"])
            if not valid:
                raise ValueError(f"invalid completed {call.tool} payload: {reason}")
        return payload

    def expand(self, payload: dict[str, Any]) -> dict[str, Any]:
        if payload["delivery"]["mode"] != "artifact":
            return payload
        artifact = payload["delivery"]["artifact_id"]
        return self.result_artifact(payload, artifact)[0]

    @staticmethod
    def terminal_descriptor(response: dict[str, Any], result: dict[str, Any]) -> dict[str, Any]:
        """A compact job result points directly to the retained research envelope."""
        if result["delivery"]["mode"] != "artifact":
            raise ValueError("terminal job result has no durable delivery")
        return response | {
            "status": result["outcome"],
            "summary": result["summary"],
            "coverage": result["coverage"],
            "error": result["error"],
            "job": None,
            "data": {},
            "delivery": result["delivery"],
            "context_id": result["context_id"],
            "snapshot_id": result["snapshot_id"],
        }

    def result_artifact(self, payload: dict, artifact: str) -> tuple[dict, int]:
        """Accept a complete native cursor chain, including a restarted smaller-page read."""
        states: dict[tuple[str | None, str | None], ArtifactChain] = {}
        identity = None
        for call in self.calls:
            if (
                call.server != SERVICE
                or call.tool != "read_artifact"
                or call.arguments.get("artifact_id") != artifact
                or call.completed is None
                or call.error
            ):
                continue
            page = call.envelope()
            if page["status"] != "ok":
                continue
            data = page["data"]
            section = data["section"]
            selected = call.arguments.get("section")
            expected = selected.get("name") if isinstance(selected, dict) else None
            if section != expected or section not in {None, "data"}:
                continue
            raw = data["content"].encode()
            current = (data["artifact"]["sha256"], data["total"])
            if (
                data["artifact"]["artifact_id"] != artifact
                or data["encoding"] != "utf8"
                or data["end"] != data["start"] + len(raw)
                or data["content_digest"] != hashlib.sha256(raw).hexdigest()
                or data["total"] > 32 * 1024 * 1024
                or (identity is not None and identity != current)
            ):
                raise ValueError("invalid native result artifact page")
            identity = current
            cursor_in = call.arguments.get("cursor")
            previous = states.get((section, cursor_in))
            if cursor_in is None:
                previous = ArtifactChain(None, b"", data["start"] if section else 0, 0)
            if previous is None or data["start"] != previous.end:
                continue
            chain = ArtifactChain(
                previous, raw, data["end"], max(previous.completed, call.completed)
            )
            cursor = data["page"]["next_cursor"]
            if cursor is not None:
                states[section, cursor] = chain
                continue
            if (section is None and chain.end != data["total"]) or data["remaining"] != 0:
                raise ValueError("incomplete result artifact")
            chunks = []
            node: ArtifactChain | None = chain
            while node is not None:
                chunks.append(node.chunk)
                node = node.previous
            complete = b"".join(reversed(chunks))
            if section == "data":
                # Direct sections intentionally do not read the complete artifact. Each page
                # has a digest and a contiguous, section-bound cursor chain; the live witness
                # independently rereads those exact requests against the immutable daemon.
                value = json.loads(complete)
                if not isinstance(value, dict):
                    raise ValueError("result data section is not an object")
                return payload | {
                    "data": value,
                    "delivery": {
                        "mode": "inline",
                        "limits": {"requested_max_bytes": None, "effective_max_bytes": None},
                    },
                }, chain.completed
            if hashlib.sha256(complete).hexdigest() != data["artifact"]["sha256"]:
                raise ValueError("result artifact digest mismatch")
            document = json.loads(complete)
            if (
                not isinstance(document, dict)
                or document.get("index", {}).get("format") != "research-result/3"
            ):
                raise ValueError("result artifact is not an indexed research result")
            result = document["result"]
            if not isinstance(result, dict):
                raise ValueError("result artifact has no research envelope")
            # The immutable document has a fixed retained request placeholder. This caller
            # owns only the transport identity; research fields remain exactly retained.
            result["request_id"] = payload["request_id"]
            return result, chain.completed
        raise ValueError("client did not read the complete result artifact")

    def usable_at(self, call: Invocation) -> int | None:
        """When this client actually possessed the complete answer, including job/pages."""
        if self.answer_for(call)["status"] not in {"ok", "partial"}:
            return None
        payload = call.envelope()
        at = call.completed
        if at is None:
            return None
        if payload["status"] == "pending":
            for poll in self.calls:
                if (
                    poll.server == SERVICE
                    and poll.tool == "job_control"
                    and poll.arguments.get("job_id") == payload["job"]["job_id"]
                    and poll.started > call.started
                    and poll.completed is not None
                    and not poll.error
                ):
                    response = self.expand(poll.envelope())
                    result = response.get("data", {}).get("result")
                    if response["status"] == "ok" and isinstance(result, dict):
                        finished = self.usable_at(poll)
                        if finished is None:
                            return None
                        at, payload = max(at, finished), self.terminal_descriptor(response, result)
                        break
        if payload["delivery"]["mode"] == "artifact":
            artifact = payload["delivery"]["artifact_id"]
            _, finished = self.result_artifact(payload, artifact)
            at = max(at, finished)
        return at


def result_object(result: object) -> dict[str, Any]:
    """Accept only an explicit structured result or a complete JSON text result."""
    if isinstance(result, dict):
        if "schema_version" in result:
            return result
        for name in ("structured_content", "structuredContent"):
            if isinstance(result.get(name), dict):
                return result[name]
        result = result.get("content")
    if isinstance(result, list):
        # Hosts can render resource links as text beside the structured JSON. Only one
        # complete envelope block is authority; neither a preview nor conflicting JSON is.
        envelopes = []
        for block in result:
            if not isinstance(block, dict):
                raise ValueError("malformed service content block")
            if block.get("type") == "text" and isinstance(block.get("text"), str):
                try:
                    candidate = json.loads(block["text"])
                except json.JSONDecodeError:
                    continue
                if isinstance(candidate, dict) and "schema_version" in candidate:
                    envelopes.append(candidate)
        if len(envelopes) != 1:
            raise ValueError("service result must contain one explicit JSON envelope")
        return envelopes[0]
    if isinstance(result, str):
        value = json.loads(result)
        if isinstance(value, dict) and "schema_version" in value:
            return value
    raise ValueError("service result has no explicit envelope")


def name(value: object) -> tuple[str, str] | None:
    if not isinstance(value, str):
        return None
    for server in SERVERS:
        prefix = f"mcp__{server}__"
        if value.startswith(prefix) and value[len(prefix) :]:
            return server, value[len(prefix) :]
    return None


def parse(client: str, stdout: str) -> Trace:
    if client not in {"codex", "claude"}:
        raise ValueError("unsupported client event contract")
    if len(stdout.encode()) > 16 * 1024 * 1024:
        raise ValueError("client trace exceeds 16 MiB")
    lines = stdout.splitlines()
    if len(lines) > 10_000:
        raise ValueError("client trace event count exceeded")
    trace = Trace()
    calls: dict[str, Invocation] = {}
    for position, line in enumerate(lines):
        if not line.strip():
            continue
        if len(line.encode()) > 1024 * 1024:
            raise ValueError("client event exceeds 1 MiB")
        event = json.loads(line)
        if not isinstance(event, dict):
            raise ValueError("client event is not an object")
        if client == "codex":
            codex(event, position, calls, trace)
        else:
            claude(event, position, calls, trace)
    trace.calls = sorted(calls.values(), key=lambda call: call.started)
    return trace


def begin(
    calls: dict[str, Invocation],
    call_id: object,
    server: object,
    tool: object,
    arguments: object,
    position: int,
) -> Invocation:
    if (
        not isinstance(call_id, str)
        or not call_id
        or not isinstance(server, str)
        or server not in SERVERS
        or not isinstance(tool, str)
        or not tool
        or not isinstance(arguments, dict)
    ):
        raise ValueError("malformed native tool invocation")
    if call_id in calls:
        old = calls[call_id]
        if (old.server, old.tool, old.arguments) != (server, tool, arguments):
            raise ValueError("conflicting native invocation identity")
        return old
    call = Invocation(call_id, server, tool, arguments, position)
    calls[call_id] = call
    return call


def finish(call: Invocation, result: object, error: str | None, position: int) -> None:
    if call.completed is not None:
        raise ValueError("duplicate native completion")
    call.completed, call.result, call.error = position, result, error


def codex(event: dict, position: int, calls: dict[str, Invocation], trace: Trace) -> None:
    kind = event.get("type")
    if kind == "turn.completed":
        trace.successful_turn = True
    elif kind in {"turn.failed", "error"}:
        raise ValueError("Codex reported a failed turn")
    if kind not in {"item.started", "item.updated", "item.completed"}:
        return
    item = event.get("item")
    if not isinstance(item, dict):
        raise ValueError("Codex item event has no item")
    if item.get("type") == "agent_message" and kind == "item.completed":
        trace.answer = str(item.get("text", ""))
    if item.get("type") != "mcp_tool_call" or item.get("server") not in SERVERS:
        return
    call = begin(
        calls, item.get("id"), item.get("server"), item.get("tool"), item.get("arguments"), position
    )
    if kind == "item.completed":
        if item.get("status") not in {"completed", "failed"}:
            raise ValueError("Codex completion has no terminal tool status")
        error = item.get("error")
        finish(
            call,
            item.get("result"),
            str(error or "tool failed") if error or item["status"] == "failed" else None,
            position,
        )


def claude(event: dict, position: int, calls: dict[str, Invocation], trace: Trace) -> None:
    kind = event.get("type")
    if event.get("error") == "authentication_failed" and event.get("is_api_error_message") is True:
        trace.prerequisite_error = (
            "Claude authentication failed; refresh the selected Claude Code login and rerun."
        )

    if kind == "result":
        trace.successful_turn = event.get("subtype") == "success" and event.get("is_error") is False
        trace.answer = str(event.get("result", ""))
        return
    if kind not in {"assistant", "user"}:
        return
    message = event.get("message")
    if not isinstance(message, dict) or not isinstance(message.get("content"), list):
        return
    for block in message["content"]:
        if not isinstance(block, dict):
            raise ValueError("malformed Claude message block")
        if kind == "assistant" and block.get("type") == "tool_use":
            selected = name(block.get("name"))
            if selected is not None:
                begin(calls, block.get("id"), *selected, block.get("input"), position)
        elif kind == "user" and block.get("type") == "tool_result":
            call_id = block.get("tool_use_id")
            if not isinstance(call_id, str):
                raise ValueError("Claude tool result has no invocation identity")
            call = calls.get(call_id)
            if call is not None:
                finish(
                    call,
                    block.get("content"),
                    "tool failed" if block.get("is_error") else None,
                    position,
                )
