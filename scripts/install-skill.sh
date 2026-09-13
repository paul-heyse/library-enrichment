#!/usr/bin/env bash
# Install the companion library-research skill to user scope.
#
# AGENT_HANDOFF.md: do not install the skill into user configuration without explicit setup
# invocation. This therefore DEFAULTS TO A DRY RUN, states its destinations, refuses to
# overwrite an unrelated skill, and records an installation manifest.
#
#   scripts/install-skill.sh              dry run (default)
#   scripts/install-skill.sh --apply      actually install
#   scripts/install-skill.sh --uninstall  remove a previous installation
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="${ROOT}/skills/library-research"
NAME="library-research"
MODE="dry-run"
case "${1:-}" in
  --apply) MODE=apply ;;
  --uninstall) MODE=uninstall ;;
  ""|--dry-run) MODE=dry-run ;;
  *) echo "usage: install-skill.sh [--apply|--uninstall|--dry-run]" >&2; exit 2 ;;
esac

# Claude Code reads ~/.claude/skills; Codex reads ~/.agents/skills. On this workstation
# ~/.claude/skills is a symlink into ~/.codex/skills, so resolve before deciding.
DESTS=()
for d in "$HOME/.claude/skills" "$HOME/.agents/skills"; do
  real="$(readlink -f "$d" 2>/dev/null || echo "$d")"
  dup=0; for seen in "${DESTS[@]:-}"; do [ "$seen" = "$real" ] && dup=1; done
  [ "$dup" -eq 0 ] && DESTS+=("$real")
done

echo "source:       ${SRC}"
echo "destinations:"; for d in "${DESTS[@]}"; do echo "  ${d}/${NAME}"; done
echo "mode:         ${MODE}"
echo

[ -d "$SRC" ] || { echo "install-skill: source skill not found" >&2; exit 1; }

for d in "${DESTS[@]}"; do
  target="${d}/${NAME}"
  if [ -e "$target" ]; then
    if [ -f "${target}/.install-manifest" ] && grep -q "source=${SRC}" "${target}/.install-manifest"; then
      echo "  ${target}: existing installation from this repository (will be replaced)"
    else
      echo "  ${target}: REFUSING -- a different '${NAME}' skill is already installed there." >&2
      echo "              Remove it deliberately, or install under a different name." >&2
      exit 1
    fi
  else
    echo "  ${target}: new installation"
  fi
done

if [ "$MODE" = "dry-run" ]; then
  echo
  echo "Dry run only. Nothing was written. Re-run with --apply to install."
  exit 0
fi

for d in "${DESTS[@]}"; do
  target="${d}/${NAME}"
  if [ "$MODE" = "uninstall" ]; then
    if [ -f "${target}/.install-manifest" ] && grep -q "source=${SRC}" "${target}/.install-manifest"; then
      rm -rf "$target"; echo "removed ${target}"
    else
      echo "skipped ${target} (not installed from this repository)"
    fi
    continue
  fi
  mkdir -p "$d"; rm -rf "$target"
  cp -r "$SRC" "$target"
  {
    echo "source=${SRC}"
    echo "installed_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "commit=$(git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo unknown)"
    echo "skill_sha256=$(sha256sum "${SRC}/SKILL.md" | cut -d' ' -f1)"
  } > "${target}/.install-manifest"
  echo "installed ${target}"
done

echo
echo "The skill coordinates Context7 and the library-enrichment MCP service."
echo "Installing it does NOT register the service. See docs/operations/ for that step."
