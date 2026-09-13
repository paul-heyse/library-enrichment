#!/usr/bin/env bash
# Single definition of this repository's working environment.
#
# Sourced by BOTH .envrc (interactive shells, via direnv) and scripts/hooks/session_env.sh
# (agent sessions, via CLAUDE_ENV_FILE). Defining it once is the point: if the two drifted,
# an agent and a human would be running different commands against different state.
#
# Requires LIBENR_ROOT to be set by the caller.

: "${LIBENR_ROOT:?scripts/env.sh requires LIBENR_ROOT}"

# --- Service state -----------------------------------------------------------------------
# Production resolves to XDG paths (~/.cache/library-enrichment, ~/.local/share/...).
# Development is redirected into a gitignored in-repo sandbox so no session mutates real
# user state, and so `just state-reset` is a one-line, obviously-safe operation.
export LIBENR_HOME="${LIBENR_ROOT}/.dev-state"
export LIBENR_CACHE_HOME="${LIBENR_HOME}/cache"
export LIBENR_DATA_HOME="${LIBENR_HOME}/data"
export LIBENR_CONFIG="${LIBENR_ROOT}/config/service.dev.toml"

# --- Rust --------------------------------------------------------------------------------
export CARGO_TARGET_DIR="${LIBENR_ROOT}/target"
export RUST_BACKTRACE=1
export CARGO_TERM_COLOR=never
# rust-toolchain.toml is the single toolchain authority. An inherited RUSTUP_TOOLCHAIN would
# override it, and this workstation's rustup default is nightly.
unset RUSTUP_TOOLCHAIN
command -v sccache >/dev/null 2>&1 && export RUSTC_WRAPPER=sccache

# --- Python ------------------------------------------------------------------------------
export UV_PROJECT_ENVIRONMENT="${LIBENR_ROOT}/.venv"
# Do not re-resolve on every `uv run`; `just sync` is the explicit resolution step.
export UV_NO_SYNC=1
export PYTHONDONTWRITEBYTECODE=1

# --- PATH --------------------------------------------------------------------------------
# So `ty`, `ruff`, `pytest`, `python` and the built binaries work WITHOUT `uv run` or a
# ./target/debug/ prefix. Fewer modifiers means fewer ways to run the wrong thing.
libenr_path_add() { case ":${PATH}:" in *":$1:"*) ;; *) PATH="$1:${PATH}" ;; esac; }
libenr_path_add "${LIBENR_ROOT}/.venv/bin"
libenr_path_add "${LIBENR_ROOT}/target/debug"
libenr_path_add "${LIBENR_ROOT}/scripts"
export PATH
unset -f libenr_path_add
