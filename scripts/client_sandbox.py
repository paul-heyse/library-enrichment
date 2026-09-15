"""A throwaway user directory for exercising real MCP clients.

Blueprint §13 Phase 5 says to "install the skill in a controlled test user directory and
exercise both clients", and §12.3 forbids "automatic edits to the user's global client
configuration or skill folders during ordinary test runs". Those two together are this module:
a real `claude mcp add --scope user` and a real `codex mcp add`, run against a `HOME` that is
not the operator's.

The redirection is checked rather than assumed. Before anything is installed, the sandbox
refuses to proceed unless `HOME` actually points inside the service's own state, and it digests
the operator's real client directories so that "nothing was touched" is a measurement at the
end rather than a hope.
"""

from __future__ import annotations

import hashlib
import json
import os
import signal
import stat
import subprocess
from contextlib import suppress
from dataclasses import dataclass, field
from pathlib import Path

#: The operator directories that must be identical before and after. Every client we drive
#: keeps its configuration, credentials or skills in one of them.
REAL_USER_DIRECTORIES = (
    ".claude/settings.json",
    ".claude/settings.local.json",
    ".claude/.credentials.json",
    ".claude/skills",
    ".claude.json",
    ".codex/config.toml",
    ".codex/auth.json",
    ".codex/AGENTS.md",
    ".codex/skills",
    ".agents/skills",
    ".config/claude",
    ".config/codex",
)
# Active operator sessions append their own histories. Protect configuration, credentials and
# skills; unrelated concurrent history writes are neither installation nor configuration edits.


def digest_user_directories(home: Path) -> dict[str, str]:
    """Path, mode and content digest for each real client directory that exists."""
    digests = {}
    for relative in REAL_USER_DIRECTORIES:
        root = home / relative
        if not os.path.lexists(root):
            digests[relative] = "absent"
            continue
        entries = []
        for path in (
            [root, *sorted(root.rglob("*"))] if root.is_dir() and not root.is_symlink() else [root]
        ):
            name = path.relative_to(root).as_posix()
            mode = stat.S_IMODE(path.lstat().st_mode)
            name = f"{name}\t{mode:o}"
            try:
                if path.is_symlink():
                    entries.append(f"{name}\tlink\t{os.readlink(path)}")
                elif path.is_dir():
                    entries.append(f"{name}\tdir")
                else:
                    entries.append(f"{name}\tfile\t{hashlib.sha256(path.read_bytes()).hexdigest()}")
            except OSError as err:
                # An unreadable entry still contributes a stable line, so a permission change
                # is visible rather than silently identical.
                entries.append(f"{name}\tunreadable\t{err.errno}")
        digests[relative] = hashlib.sha256("\n".join(entries).encode()).hexdigest()
    return digests


@dataclass
class Sandbox:
    """A user directory that exists only for one acceptance run."""

    root: Path
    real_home: Path
    client: str | None = None
    before: dict[str, str] = field(default_factory=dict)

    @classmethod
    def create(cls, root: Path) -> Sandbox:
        """Build the directory layout and record the operator's real state."""
        real_home = Path.home()
        root = root.resolve()
        service = os.environ.get("LIBENR_HOME")
        if not service:
            raise ValueError("sandbox requires an explicit LIBENR_HOME")
        owned = Path(service).resolve() / "clients"
        protected = [real_home / name for name in (".codex", ".claude", ".agents", ".config")]
        if (
            not root.is_relative_to(owned)
            or root == owned
            or any(
                root == path or root.is_relative_to(path) or path.is_relative_to(root)
                for path in protected
            )
        ):
            raise ValueError(f"sandbox must be inside isolated service clients state: {root}")
        if root.exists():
            raise ValueError(f"sandbox already exists: {root}")
        root.mkdir(parents=True, mode=0o700)
        for relative in (".claude", ".codex", ".agents", ".config", ".local/share", ".cache"):
            (root / relative).mkdir(parents=True, exist_ok=True)
        return cls(root=root, real_home=real_home, before=digest_user_directories(real_home))

    def adopt_operator_credentials(self, client: str) -> list[str]:
        """Copy the minimum credential material the clients need, and nothing else.

        Both CLIs authenticate from a file under the operator's home rather than an environment
        variable, so a sandboxed `HOME` cannot log in on its own. This copies exactly those two
        files -- not settings, not history, not projects -- into the throwaway directory, which
        is deleted when the run ends.

        Opt-in for a reason. Moving an OAuth token anywhere is a decision an operator makes, not
        one a test harness makes for them, so nothing calls this without both `--apply` and
        `--use-operator-credentials`.
        """
        adopted = []
        if client not in {"claude", "codex"}:
            raise ValueError("unsupported client")
        for relative in (
            (".claude/.credentials.json",) if client == "claude" else (".codex/auth.json",)
        ):
            source = self.real_home.joinpath(relative)
            if not source.is_file():
                continue
            target = self.root.joinpath(relative)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read_bytes())
            target.chmod(0o600)
            adopted.append(relative)
        return adopted

    def env(self, extra: dict[str, str] | None = None) -> dict[str, str]:
        """An environment whose user-scope paths all resolve inside the sandbox.

        Credentials are inherited from the ambient environment when present, and *this method*
        inherits nothing else: a key the operator already exported is theirs to spend, and
        letting it reach a subprocess asks nothing new of them.

        Copying an operator's credential *file* is the stronger act, so it is not done here. It
        has its own method, `adopt_operator_credentials`, reached only through an explicit
        `--use-operator-credentials`. The split is the whole point: what the sandbox does by
        default and what it does when asked must be two different code paths, or the flag is
        decorative and "throwaway `HOME`" is not true.
        """
        env = {
            key: value
            for key, value in os.environ.items()
            if not key.startswith(("XDG_", "LIBENR_"))
            and key not in {"CODEX_HOME", "CODEX_SQLITE_HOME", "CLAUDE_CONFIG_DIR", "CLAUDECODE"}
        }
        env.update(
            {
                "HOME": str(self.root),
                "CODEX_HOME": str(self.root / ".codex"),
                "CODEX_SQLITE_HOME": str(self.root / ".codex/sqlite"),
                "CLAUDE_CONFIG_DIR": str(self.root / ".claude"),
                "XDG_CONFIG_HOME": str(self.root / ".config"),
                "XDG_DATA_HOME": str(self.root / ".local/share"),
                "XDG_CACHE_HOME": str(self.root / ".cache"),
                "XDG_STATE_HOME": str(self.root / ".local/state"),
            }
        )
        if self.client == "codex":
            for key in ("ANTHROPIC_API_KEY", "CLAUDE_CODE_OAUTH_TOKEN"):
                env.pop(key, None)
            if (self.root / ".codex/auth.json").is_file():
                env.pop("CODEX_API_KEY", None)
                env.pop("OPENAI_API_KEY", None)
            elif not env.get("CODEX_API_KEY") and env.get("OPENAI_API_KEY"):
                env["CODEX_API_KEY"] = env["OPENAI_API_KEY"]
        elif self.client == "claude":
            for key in ("OPENAI_API_KEY", "CODEX_API_KEY"):
                env.pop(key, None)
            if (self.root / ".claude/.credentials.json").is_file():
                env.pop("ANTHROPIC_API_KEY", None)
                env.pop("CLAUDE_CODE_OAUTH_TOKEN", None)
        env.update(extra or {})
        return env

    def untouched(self) -> tuple[bool, list[str]]:
        """Whether the operator's real client directories are exactly as they were."""
        after = digest_user_directories(self.real_home)
        changed = [name for name, digest in after.items() if self.before.get(name) != digest]
        return not changed, changed

    def run(
        self, argv: list[str], *, timeout: int = 600, extra_env: dict[str, str] | None = None
    ) -> subprocess.CompletedProcess[str]:
        """Run a client command inside the sandbox, capturing everything it says."""
        process = subprocess.Popen(
            argv,
            env=self.env(extra_env),
            cwd=str(self.root),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            start_new_session=True,
        )
        try:
            stdout, stderr = process.communicate(timeout=timeout)
        except BaseException:
            # A wrapper may have spawned its native client and MCP adapters. Stop the owned
            # process group and reap it before its isolated home can be removed.
            with suppress(ProcessLookupError):
                os.killpg(process.pid, signal.SIGKILL)
            process.communicate()
            raise
        return subprocess.CompletedProcess(argv, process.returncode, stdout, stderr)

    def record(self, out: Path, name: str, result: subprocess.CompletedProcess[str]) -> Path:
        """Write one command's complete trace, including what was asked and what came back."""
        out.mkdir(parents=True, exist_ok=True, mode=0o700)
        out.chmod(0o700)
        target = out / f"{name}.json"
        target.write_text(
            json.dumps(
                {
                    "argv": result.args,
                    "exit_code": result.returncode,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                    "home": str(self.root),
                },
                indent=1,
            )
            + "\n"
        )
        target.chmod(0o600)
        return target
