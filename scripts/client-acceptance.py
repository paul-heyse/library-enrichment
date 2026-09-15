#!/usr/bin/env python3
"""Drive real Codex and Claude Code against the service, and record what they actually did.

Gates A01-A06. Every one of them is about a real client: `.claude/rules/evidence-truthfulness.md`
is explicit that "a mocked client is never a pass for A01 or A02", and `AGENT_HANDOFF.md` adds
"do not claim success from mocks or static screenshots".

So this installs the product skill and registers the service for real -- `claude mcp add
--scope user`, `codex mcp add` -- inside a throwaway `HOME` that is not the operator's, runs
each scenario non-interactively, and writes the complete transcript of every run. The operator's
own `~/.claude`, `~/.codex` and `~/.agents` are digested before and after, and the run reports a
failure if any of them changed.

Previews by default. A missing client binary, absent credentials, or an unreachable Context7 is
reported as a **blocked** prerequisite naming what is missing -- never as a pass, and never
worked around.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from datetime import UTC, datetime
from pathlib import Path

from cli import say
from client_events import parse
from client_fixture import assess, service, witness
from client_sandbox import Sandbox

ROOT = Path(__file__).resolve().parent.parent
#: The launcher §12.1 requires: absolute path, launchable from an arbitrary directory.
ADAPTER = ROOT / ".venv/bin/library-enrichment-mcp"
DAEMON = ROOT / "target/debug/library-enrichmentd"

#: One scenario per client gate. The prompt is the whole input: no system prompt, no priming,
#: nothing that would tell the agent what the gate wants to see.
SCENARIOS: dict[str, dict[str, str]] = {
    "A01": {
        "client": "codex",
        "intent": "Codex real stdio connection",
        "prompt": (
            "Using the library-enrichment MCP server, call service_status, then resolve_library "
            "for the Rust crate enr-fixture version 0.2.0, then search_evidence on that context "
            "for 'shape', then read_artifact on one artifact from the resolve result. "
            "Report each tool you called and whether it succeeded. Do not edit any file."
        ),
    },
    "A02": {
        "client": "claude",
        "intent": "Claude Code real stdio connection",
        "prompt": (
            "Using the library-enrichment MCP server, call service_status, then resolve_library "
            "for the Rust crate enr-fixture version 0.2.0, then search_evidence on that context "
            "for 'shape', then read_artifact on one artifact from the resolve result. "
            "Report each tool you called and whether it succeeded. Do not edit any file."
        ),
    },
    "A03": {
        "client": "claude",
        "intent": "Skill on unfamiliar library design question",
        "prompt": (
            "I need geometry capabilities in a Rust service and I do not know "
            "the ecosystem. Research the crate enr-fixture 0.2.0 with the library-research "
            "workflow: get a breadth overview first, shortlist what looks relevant, then pull "
            "targeted evidence on the shortlist, and finish with a short deployment brief. "
            "Do not edit any file."
        ),
    },
    "A04": {
        "client": "claude",
        "intent": "Context7 documentation has unknown/mismatched version",
        "prompt": (
            "Use Context7 to retrieve documentation about enabling Serde derive macros. "
            "This project uses the Rust crate serde exactly 1.0.228. Check the retrieved "
            "documentation's version scope, then resolve that exact release with "
            "library-enrichment and retrieve its feature/manifest evidence. Explain which "
            "statements are supported by exact-release evidence and which remain general "
            "documentation or unverified project assumptions. Do not edit any file."
        ),
    },
    "A05": {
        "client": "claude",
        "intent": "Known symbol question",
        "prompt": (
            "What is the exact signature of enr_fixture::Shape in enr-fixture 0.2.0? "
            "I only need that one symbol. Do not edit any file."
        ),
    },
    "A06": {
        "client": "claude",
        "intent": "Skill installed without enrichment service",
        "prompt": (
            "Using the library-research skill, tell me the exact public API of enr-fixture "
            "0.2.0. Do not edit any file."
        ),
    },
}


for selected_client in ("codex", "claude"):
    SCENARIOS[f"upgrade-{selected_client}"] = {
        "client": selected_client,
        "intent": "Two-release upgrade research",
        "prompt": "Use the library-research skill to compare Python evidence-demo 1.0 and 2.0. "
        "Explain observed API changes and compatibility limits with evidence identities. "
        "Do not edit files.",
    }
    SCENARIOS[f"runtime-{selected_client}"] = {
        "client": selected_client,
        "intent": "Qualified runtime and static alternatives",
        "prompt": "Use library-enrichment to resolve Python runtime-demo 1.0 for Python 3.14. "
        "Inspect runtime_demo.choose and its source/stub alternatives, then explicitly execute "
        "runtime object inspection with execute_on_miss intent, runtime profile, "
        "module runtime_demo "
        "and attributes [choose]. Report the actual observed signature, disagreement, and evidence "
        "identities. Do not edit files.",
    }

for scenario in SCENARIOS.values():
    scenario["prompt"] += (
        " Complete pending service jobs using job_control wait and retrieve every page of any "
        "BUDGET_EXCEEDED result artifact before drawing conclusions. Use only observed evidence; "
        "report limitations explicitly."
    )


def out_dir(explicit: str | None) -> Path:
    """Where the client transcripts go.

    Default is under service state, not the repository. A transcript is service output, and
    §2.3 keeps service output out of a working repository -- but the sharper reason is what a
    transcript of a `--use-operator-credentials` run contains: the complete stdout and stderr of
    a real, authenticated client session. That is credential-adjacent material, and the
    repository is the one directory on this machine whose contents are routinely committed.

    `--out` still accepts any path, because an operator asking for a report in a specific place
    is making a deliberate choice about their own files.
    """
    if explicit:
        return Path(explicit).resolve()
    stamp = datetime.now(UTC).strftime("%Y%m%dT%H%M%SZ")
    home = os.environ.get("LIBENR_HOME")
    if not home:
        sys.exit("set LIBENR_HOME; client transcripts live under service state, not in the repo")
    return Path(home).resolve() / "clients" / stamp / "traces"


def sandbox_root() -> Path:
    home = os.environ.get("LIBENR_HOME")
    if not home:
        sys.exit("set LIBENR_HOME; the throwaway client directory lives under service state")
    stamp = datetime.now(UTC).strftime("%Y%m%dT%H%M%SZ")
    return Path(home).resolve() / "clients" / stamp


def available(command: str) -> str | None:
    return shutil.which(command)


def prerequisites(client: str, adopt: bool, service: bool) -> list[str]:
    """Each client has its own authentication and service prerequisites."""
    missing = []
    if not available(client):
        missing.append(f"the {client} CLI is not on PATH")
    keys = (
        ("CODEX_API_KEY", "OPENAI_API_KEY")
        if client == "codex"
        else ("ANTHROPIC_API_KEY", "CLAUDE_CODE_OAUTH_TOKEN")
    )
    credential = Path.home() / (
        ".codex/auth.json" if client == "codex" else ".claude/.credentials.json"
    )
    if not any(os.environ.get(key) for key in keys) and not (adopt and credential.is_file()):
        missing.append(f"{client} requires an exported credential or explicit existing-login reuse")
    if service:
        for path in (ADAPTER, DAEMON, DAEMON.with_name("library-enrichment-native-worker")):
            if not path.is_file():
                missing.append(f"required executable is missing: {path}")
    return missing


def register(sandbox: Sandbox, out: Path, client: str, launch: dict | None) -> None:
    """Use the actual installer and selected client's registration command."""
    installed = sandbox.run([str(ROOT / "scripts/install-skill.sh"), "--apply"])
    sandbox.record(out, "setup-install-skill", installed)
    if installed.returncode:
        raise ValueError("skill installation failed; inspect setup-install-skill trace")
    if client == "codex":
        # `mcp add --url` initiates interactive OAuth on 0.154.0 even for an anonymous
        # server. Direct TOML registration is the documented noninteractive contract.
        config = sandbox.root / ".codex/config.toml"
        config.write_text(
            "[features]\ncode_mode_only=false\n[features.code_mode]\nenabled=false\n"
            '[mcp_servers.context7]\nurl="https://mcp.context7.com/mcp"\n'
            "startup_timeout_sec=60\ntool_timeout_sec=120\n"
        )
        config.chmod(0o600)
        (out / "setup-context7.json").write_text(
            json.dumps(
                {
                    "method": "documented direct TOML",
                    "path": str(config),
                    "url": "https://mcp.context7.com/mcp",
                    "authentication": "anonymous",
                },
                indent=2,
            )
        )
    else:
        added = sandbox.run(
            [
                client,
                "mcp",
                "add",
                "--scope",
                "user",
                "--transport",
                "http",
                "context7",
                "https://mcp.context7.com/mcp",
            ],
            timeout=90,
        )
        sandbox.record(out, "setup-context7", added)
        if added.returncode:
            raise ValueError("separate Context7 registration failed")
    if launch is None:
        return
    adapter = launch["mcpServers"]["library-enrichment"]
    argv = (
        [client, "mcp", "add", "--scope", "user", "--transport", "stdio", "library-enrichment"]
        if client == "claude"
        else [client, "mcp", "add", "library-enrichment"]
    )
    for key, value in adapter["env"].items():
        argv.extend(["--env", f"{key}={value}"])
    argv.extend(["--", adapter["command"], *adapter["args"]])
    added = sandbox.run(argv)
    sandbox.record(out, f"setup-{client}-mcp-add", added)
    if added.returncode:
        raise ValueError(f"{client} registration failed; inspect setup trace")
    if client == "codex":
        # This scenario explicitly authorizes contained inspection. Approve only that MCP
        # tool while preserving Codex's read-only filesystem sandbox and truthful annotations.
        with (sandbox.root / ".codex/config.toml").open("a") as config:
            config.write(
                '\n[mcp_servers."library-enrichment".tools.inspect_symbol]\n'
                'approval_mode="approve"\n'
            )


def drive(
    sandbox: Sandbox, out: Path, gate: str, scenario: dict[str, str], launch: dict | None
) -> dict[str, object]:
    """Run one scenario against its client and record the whole transcript."""
    client = scenario["client"]
    if not available(client):
        return {"gate": gate, "status": "blocked", "detail": f"{client} is not installed"}

    if client == "claude":
        argv = [
            "claude",
            "-p",
            scenario["prompt"],
            "--output-format",
            "stream-json",
            "--verbose",
            "--permission-mode",
            "dontAsk",
            "--permission-prompts",
            "none",
            "--no-session-persistence",
            "--allowedTools",
            "Read",
            "Glob",
            "Grep",
            "Skill",
            "mcp__library-enrichment__*",
            "mcp__context7__*",
        ]
    else:
        argv = [
            "codex",
            "exec",
            "--json",
            "--ephemeral",
            "--sandbox",
            "read-only",
            "--skip-git-repo-check",
            scenario["prompt"],
        ]

    say(f"  {gate} ({client}): {scenario['intent']}")
    try:
        result = sandbox.run(argv, timeout=900)
    except subprocess.TimeoutExpired:
        return {"gate": gate, "status": "failed", "detail": "the client did not finish in 900s"}
    trace = sandbox.record(out, f"{gate}-{client}", result)
    detail = result.stderr.strip()[-400:] if result.returncode else ""
    if result.returncode and client == "claude":
        try:
            authentication = parse(client, result.stdout).prerequisite_error
        except ValueError:
            authentication = None
        if authentication:
            return {
                "gate": gate,
                "client": client,
                "status": "blocked",
                "exit_code": result.returncode,
                "trace": str(trace),
                "detail": authentication,
            }
    if result.returncode == 0:
        try:
            parsed = parse(client, result.stdout)
            if not parsed.successful_turn or not parsed.answer.strip():
                raise ValueError("client did not complete a useful answer")
            for call in parsed.calls:
                if (
                    call.server == "library-enrichment"
                    and call.completed is not None
                    and not call.error
                ):
                    parsed.answer_for(call)
            assess(gate, parsed)
            observed = witness(parsed, launch)
            (out / f"{gate}-witness.json").write_text(json.dumps(observed, indent=2) + "\n")
        except ValueError as error:
            detail = str(error)
    return {
        "gate": gate,
        "client": client,
        "status": "passed" if result.returncode == 0 and not detail else "failed",
        "exit_code": result.returncode,
        "trace": str(trace.relative_to(ROOT)) if trace.is_relative_to(ROOT) else str(trace),
        "detail": detail,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="actually install, register and run")
    parser.add_argument("--out", help="where traces are written")
    parser.add_argument(
        "--gate",
        action="append",
        choices=tuple(SCENARIOS),
        help="run only these gates (repeatable)",
    )
    parser.add_argument(
        "--keep", action="store_true", help="keep the throwaway user directory for inspection"
    )
    parser.add_argument(
        "--use-operator-credentials",
        action="store_true",
        help="copy the operator's client credential files into the throwaway directory",
    )
    args = parser.parse_args()

    gates = list(dict.fromkeys(args.gate or SCENARIOS))
    out = out_dir(args.out)
    results = []
    clean = True
    changed = []
    base = sandbox_root()
    for gate in gates:
        scenario = SCENARIOS[gate]
        client = scenario["client"]
        missing = prerequisites(client, args.use_operator_credentials, gate != "A06")
        if gate.startswith("runtime-") and not all(
            os.environ.get(name)
            for name in ("LIBENR_EXECUTION_TEST_ROOT", "LIBENR_EXECUTION_TEST_PYTHON")
        ):
            missing.append("runtime journey requires the selected qualified Python execution image")
        say(
            f"{gate}: {scenario['intent']}"
            + (f"; blocked: {'; '.join(missing)}" if missing else "")
        )
        if not args.apply:
            continue
        if missing:
            results.append(
                {"gate": gate, "client": client, "status": "blocked", "detail": "; ".join(missing)}
            )
            continue
        sandbox = None
        try:
            sandbox = Sandbox.create(base / gate)
            sandbox.client = client
            if args.use_operator_credentials:
                sandbox.adopt_operator_credentials(client)
            with service(sandbox.root, gate != "A06") as launch:
                register(sandbox, out / gate, client, launch)
                entry = drive(sandbox, out / gate, gate, scenario, launch)
            for name in ("daemon.log", "requests.json"):
                path = sandbox.root / name
                if path.is_file():
                    shutil.copyfile(path, out / gate / name)
        except Exception as error:
            entry = {
                "gate": gate,
                "client": client,
                "status": "failed",
                "detail": f"{type(error).__name__}: {error}",
            }
        finally:
            if sandbox is not None:
                try:
                    unchanged, paths = sandbox.untouched()
                    clean = clean and unchanged
                    changed.extend(paths)
                    if not args.keep:
                        shutil.rmtree(sandbox.root)
                except Exception as error:
                    clean = False
                    entry = {
                        "gate": gate,
                        "client": client,
                        "status": "failed",
                        "detail": (
                            f"sandbox teardown at {sandbox.root}: {type(error).__name__}: {error}"
                        ),
                    }
        results.append(entry)
    if not args.apply:
        say("Preview only; use --apply to install and run the selected clients.")
        return 0
    out.mkdir(parents=True, exist_ok=True)
    (out / "summary.json").write_text(
        json.dumps(
            {
                "recorded_at": datetime.now(UTC).isoformat(timespec="seconds"),
                "real_user_directories_unchanged": clean,
                "changed": sorted(set(changed)),
                "results": results,
            },
            indent=2,
        )
        + "\n"
    )
    for result in results:
        say(f"{result['gate']}: {result['status']} {result.get('detail', '')}")
    return 0 if clean and all(r["status"] == "passed" for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
