#!/usr/bin/env bash
# PreToolUse(Bash). Denies the mistakes that are visible in the command string alone.
# Anything requiring a source file to be parsed belongs in rules/ instead (see AGENTS.md).
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
hook_read_input
cmd="${HOOK_COMMAND:-}"
[ -n "$cmd" ] || exit 0
root="$(hook_root)"

# 1. Wrong Python semantic engine. pyrefly and pyright are installed on this workstation and
#    globally plugin-enabled, so this is the highest-probability defect in the repo.
if printf '%s' "$cmd" | grep -Eq '(^|[^[:alnum:]_./-])(pyrefly|pyright|basedpyright|mypy)([^[:alnum:]_-]|$)'; then
  hook_deny "This repository uses Astral **ty** for Python semantics (blueprint §1.1, §5.4). pyrefly/pyright/mypy are installed and globally plugin-enabled on this workstation, but are NOT this project's engine. Use \`uv run ty check\` or \`uv run ty server\`. If ty is unavailable, the affected gates are \`blocked\` and it warrants an ADR -- never substitute another type checker."
fi

# 2. Unpinned tool execution (blueprint §12.1).
if printf '%s' "$cmd" | grep -q '@latest'; then
  hook_deny "Unpinned tool execution: \`@latest\` (blueprint §12.1 forbids unpinned \`uvx ...@latest\` as an operational command). Add the dependency to [dependency-groups] dev and use \`uv run <tool>\`, or pin explicitly."
fi
if printf '%s' "$cmd" | grep -Eq '(^|[^[:alnum:]_-])(uvx|npx)([^[:alnum:]_-]|$)|uv tool run' \
   && ! printf '%s' "$cmd" | grep -q '=='; then
  hook_deny "Unpinned tool execution (blueprint §12.1). Resolve dependencies at setup, not per invocation. Add it to [dependency-groups] dev and use \`uv run <tool>\`, or pin \`uvx --from pkg==X.Y.Z\`."
fi

# 3. Bare +nightly. This workstation's rustup DEFAULT is nightly, so a floating +nightly is
#    both unpinned and indistinguishable from the default -- see config/toolchains.toml.
if printf '%s' "$cmd" | grep -Eq 'cargo[[:space:]]+\+nightly([[:space:]]|$)'; then
  hook_deny "Bare \`+nightly\` floats, and this workstation's rustup default IS nightly. Producer invocations must use the dated pin recorded in config/toolchains.toml (e.g. \`cargo +nightly-2026-08-18 rustdoc\`) with CARGO_TARGET_DIR set to the capsule, so the emitted rustdoc format_version is reproducible."
fi

# 4. Mutating user-scope client or skill configuration (blueprint §12.2, §12.3).
if printf '%s' "$cmd" | grep -Eq '(claude|codex)[[:space:]]+mcp[[:space:]]+add|claude[[:space:]]+plugin[[:space:]]+(install|add)|\$HOME/\.(claude|codex|agents)/skills|~/\.(claude|codex|agents)/skills'; then
  [ "${LIBENR_ALLOW_USER_INSTALL:-0}" = "1" ] || hook_deny "Registering the service or installing the skill mutates user-scope configuration. Blueprint §12.2/§12.3: these are setup templates requiring explicit operator invocation, and AGENT_HANDOFF.md forbids installing the skill without explicit setup invocation. Use \`just install-skill --dry-run\`, or re-run with LIBENR_ALLOW_USER_INSTALL=1 after the operator authorizes it."
fi

# 5. Banned dependency classes (blueprint §1.1). deny.toml is the canonical oracle; this just
#    stops the command before it edits a manifest.
if printf '%s' "$cmd" | grep -Eq '(cargo[[:space:]]+add|uv[[:space:]]+add)[^|;&]*[[:space:]](lancedb|tonic|redis|neo4rs|qdrant-client|fastapi|grpcio|chromadb|sentence-transformers|torch|weaviate-client|pinecone-client)([[:space:]=<>~!]|$)'; then
  hook_deny "Banned dependency class (blueprint §1.1: no graph database, embeddings, vector database, gRPC, Redis, or FastAPI in the initial implementation). deny.toml and \`just deps-policy\` are the canonical oracles and will fail the gate."
fi

# 6. Writing to a frozen or generated path through the shell.
#    pre_edit.sh blocks these for Edit/Write; without this, `cat > AGENTS.md` would bypass it.
#    Narrow on purpose: only redirections and in-place editors, only these path prefixes.
if printf '%s' "$cmd" | grep -Eq '([[:space:]]>|[[:space:]]>>|tee[[:space:]]|sed[[:space:]]+-i|dd[[:space:]]+of=)'; then
  for frozen in docs/blueprint docs/provenance contracts tests/ACCEPTANCE_PLAN.md \
                schemas/generated python/enrichment_mcp/_generated \
                AGENTS.md CLAUDE.md .claude/rules .claude/settings.json \
                scripts/hooks scripts/env.sh; do
    if printf '%s' "$cmd" | grep -Eq "(^|[[:space:]>]|${root}/)${frozen}([[:space:]/]|\"|'|$)" \
       && printf '%s' "$cmd" | grep -Eq "([[:space:]]>|[[:space:]]>>|tee[[:space:]]+[^|]*|sed[[:space:]]+-i[^|]*|dd[[:space:]]+of=)[^|;&]*${frozen}"; then
      hook_deny "Refusing to write to \`${frozen}\` through the shell.

This path is frozen, generated, or operator-owned governance, and pre_edit.sh blocks the same write for Edit/Write. Routing around a guardrail through a shell redirect is not a workaround -- it is the thing the guardrail exists to stop.

  frozen provenance      docs/blueprint, docs/provenance -- the specification as delivered
  frozen contracts       contracts/, tests/ACCEPTANCE_PLAN.md -- verified by just provenance-check
  generated artifacts    fix the Rust wire type and run just schemas-generate (§6.3)
  governance             AGENTS.md, .claude/, justfile, scripts/, rules/ -- propose via docs/adr/

If a guardrail is genuinely wrong, say so and stop."
    fi
  done
fi

# 6. Writes outside the repository boundary (blueprint §2.3; gate C20). Only inspect commands
#    that actually mutate, so read-only exploration stays unimpeded.
if printf '%s' "$cmd" | grep -Eq '(^|[|;&[:space:]])(rm|mv|cp|tee|install|mkdir|touch|truncate|chmod|chown|ln)([[:space:]])|[[:space:]]>[^>|&]|[[:space:]]>>|sed[[:space:]]+-i'; then
  while read -r p; do
    [ -n "$p" ] || continue
    case "$p" in
      "$root"|"$root"/*) ;;
      "${LIBENR_HOME:-/nonexistent}"/*) ;;
      /tmp/claude-*|/tmp/claude-*/*) ;;
      /dev/null|/dev/stdout|/dev/stderr) ;;
      *) hook_deny "Write outside the repository boundary: ${p}

Blueprint §2.3: service state, environments, caches and outputs live outside working repositories, and a repository under study is never a subprocess working directory or an extraction destination. Gate C20 proves this with a filesystem digest.

Development service state belongs in \$LIBENR_HOME (.dev-state/, gitignored). Temporary files belong in the session scratch directory." ;;
    esac
  done < <(printf '%s' "$cmd" | grep -oE '(^|[[:space:]>])(/|~/)[^[:space:]|;&)"'"'"']*' | sed 's/^[[:space:]>]*//' | sed "s|^~|$HOME|")
fi

exit 0
