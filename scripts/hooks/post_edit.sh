#!/usr/bin/env bash
# PostToolUse(Edit|Write). Format, lint and rule-scan the SINGLE file just edited.
# Scoping to one file is deliberate: a repo-wide scan would fire on pre-existing content
# and get disabled. Exit 2 feeds the message back to the agent.
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
hook_read_input
root="$(hook_root)"
problems=""

note() { problems="${problems}$1"$'\n'; }

while read -r p; do
  [ -n "$p" ] || continue
  case "$p" in /*) f="$p" ;; *) f="$root/$p" ;; esac
  [ -f "$f" ] || continue
  rel="${f#"$root"/}"

  case "$f" in
    *.rs)
      out=$(rustfmt --edition 2024 --check "$f" 2>&1) || note "rustfmt: ${rel} is not formatted. Run \`cargo fmt\`.
${out}" ;;
    *.py)
      if [ -x "$root/.venv/bin/ruff" ]; then RUFF="$root/.venv/bin/ruff"; else RUFF="$(command -v ruff || true)"; fi
      if [ -n "$RUFF" ]; then
        out=$("$RUFF" check "$f" 2>&1) || note "ruff: ${rel}
${out}"
        out=$("$RUFF" format --check "$f" 2>&1) || note "ruff format: ${rel} is not formatted. Run \`uv run ruff format\`."
      fi ;;
    *.json)
      out=$(python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$f" 2>&1) || note "invalid JSON: ${rel}
${out}" ;;
    *.toml)
      out=$(python3 -c 'import tomllib,sys; tomllib.load(open(sys.argv[1],"rb"))' "$f" 2>&1) || note "invalid TOML: ${rel}
${out}" ;;
    *.sh)
      out=$(bash -n "$f" 2>&1) || note "shell syntax: ${rel}
${out}" ;;
  esac

  # The rules/ corpus, scoped to this one file. This is how code-shape invariants get
  # enforced in the edit loop without putting a parser inside a hook (see AGENTS.md).
  case "$f" in
    *.rs|*.py)
      if [ -f "$root/sgconfig.yml" ] && command -v ast-grep >/dev/null 2>&1; then
        out=$(cd "$root" && ast-grep scan --config sgconfig.yml "$f" 2>&1)
        printf '%s' "$out" | grep -qiE '\b(error|warning)\b' && note "rules: ${rel}
${out}"
      fi ;;
  esac

  # Zero-suppression house rule, scoped to the edited file.
  case "$f" in
    *.py|*.rs)
      out=$(grep -nE '(#[[:space:]]*noqa|#[[:space:]]*type:[[:space:]]*ignore|#!?\[allow\()' "$f" 2>/dev/null || true)
      [ -n "$out" ] && note "suppression in ${rel}: this repository does not use \`# noqa\`, \`# type: ignore\`, or \`#[allow(...)]\`. Fix the cause, or record an ADR if the lint is genuinely wrong.
${out}" ;;
  esac
done < <(hook_paths)

if [ -n "$problems" ]; then
  printf '%s' "$problems" >&2
  exit 2
fi
exit 0
