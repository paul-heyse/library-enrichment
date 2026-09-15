# Agent-shell user session configuration

Verified 2026-09-14, workstation user `paul` (UID 1000).

The rootless Podman broker requires the existing user-session bus for systemd cgroup
management. Agent shells lacked `XDG_RUNTIME_DIR` and `DBUS_SESSION_BUS_ADDRESS`, although
the user-owned mode-0700 runtime directory and bus socket existed. The initial cleanup
test run failed before its intended probes. An explicit bus address fixed that run; the
configuration below makes that repair persistent.

## Changes

- `~/.config/shell/user-session-env.sh` fills unset session variables only after validating
  the owner and permissions of `/run/user/<uid>` and ownership of its existing bus socket.
  Explicitly configured values remain unchanged.
- `~/.profile` and `~/.bashrc` source the helper before interactive-shell early returns.
  All prior startup content is preserved.
- `~/.config/environment.d/60-user-session-bus.conf` supplies defaults for this user's
  future systemd-managed processes. Only these two named variables were imported into the
  running user manager, followed by `systemctl --user daemon-reload`.

The original startup files and before/after SHA-256 receipt are retained at
`/home/paul/.local/state/agent-environment/backups/20260914T061056Z/`.
To reverse this change while preserving later edits, remove the marked source block from
each startup file and remove the two created configuration files. The backups provide the
exact prior content if there have been no subsequent edits. No session restart was required.

## Validation

Shell syntax checks passed. Clean login shells, clean interactive shells, and explicitly
sourced noninteractive `.bashrc` restored both values and connected to the user manager.
A separate check confirmed explicit values were preserved.

From `env -u XDG_RUNTIME_DIR -u DBUS_SESSION_BUS_ADDRESS bash -lc`, using the existing
admitted images and private execution root `.dev-state/p4p`, this command passed:

```sh
cargo test -p enrichment-daemon --test execution_cleanup -- --ignored --test-threads=1
```

All three real-container tests passed in 3.97 seconds: task-drop descendant cleanup,
failed-removal quarantine, and cancellation/restart orphan reconciliation. The wrapper set
only the test root/image selectors; it supplied no bus or runtime-directory override.
No new owned records remained, and all 43 pre-existing records remained byte-identical.

Evidence is in `.dev-state/phase456-completion-validation/`:

- `user-session-manager.json`
- `user-session-shell-checks.json`
- `execution-cleanup-persistent-session.json`
- `execution-cleanup-persistent-session.log`

The initial missing-bus failure remains in `execution-lease-tests-missing-bus.log`.
Its one attributable leftover was separately removed and confirmed absent before this run.
This verifies the environment repair and these cleanup scenarios, not phase completion.

Configuration syntax was checked against installed systemd 255 man pages and the
[systemd v255 environment.d source](https://github.com/systemd/systemd/blob/v255/man/environment.d.xml),
following Context7 discovery. Importing only named variables avoids copying unrelated shell
environment into the manager.
