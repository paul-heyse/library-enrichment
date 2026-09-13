#!/usr/bin/env bash
# PreToolUse(Edit|Write|NotebookEdit). Pure path classification -- no file parsing.
set -uo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/_common.sh"
hook_read_input
root="$(hook_root)"

while read -r p; do
  [ -n "$p" ] || continue
  case "$p" in /*) abs="$p" ;; *) abs="$root/$p" ;; esac
  rel="${abs#"$root"/}"

  # Outside the repository entirely.
  case "$abs" in
    "$root"/*) ;;
    *) hook_deny "Refusing to edit outside the repository: ${abs}

Blueprint §2.3: a working repository is never an extraction or install destination (gate C20). Development service state belongs in \$LIBENR_HOME (.dev-state/)." ;;
  esac

  case "$rel" in
    # Frozen provenance: the specification as delivered, and the digests proving it.
    docs/blueprint/*|docs/provenance/*)
      hook_deny "\`${rel}\` is frozen provenance -- the governing specification as delivered, verified by \`just provenance-check\`.

If the specification is wrong, that is a decision to deviate, recorded with \`/adr\`. It is not an edit to the specification." ;;

    # Frozen contracts: manifested, so editing breaks provenance-check.
    contracts/*|tests/ACCEPTANCE_PLAN.md)
      hook_deny "\`${rel}\` is a frozen contract listed in MANIFEST.sha256; editing it breaks \`just provenance-check\`.

contracts/research-envelope.schema.json is the Phase-0 acceptance target -- generated schemas must match it behaviourally (\`just schema-conformance\`), not the other way round. Acceptance gate IDs are stable identifiers and are never renumbered.

Changing either requires an ADR (\`/adr\`) that re-freezes the bundle under a new dated directory." ;;

    # Generated artifacts.
    schemas/generated/*|python/enrichment_mcp/_generated/*|crates/*/src/generated/*)
      hook_deny "\`${rel}\` is generated. Rust wire types are authoritative (blueprint §6.3).

Change the Rust type, run \`just schemas-generate\`, and commit both sides together. Hand-editing makes the Rust type, the JSON Schema, and the Pydantic DTO drift silently." ;;

    # Governance layer.
    AGENTS.md|CLAUDE.md|.claude/*|.codex/*|.agents/*|justfile|scripts/*|rules/*|rule-tests/*)
      hook_deny "\`${rel}\` is operator-owned governance. An agent working under these constraints does not relax them mid-task.

If a guardrail is genuinely wrong, say so and stop -- do not route around it or add a narrow exemption to make the current task pass. Propose the change in docs/adr/." ;;

    # Service state must never be committed into the repo.
    blobs/*|snapshots/*|capsules/*|jobs/*|logs/*|contexts/*|bundles/*)
      hook_deny "\`${rel}\` is service state inside the repository (blueprint §2.3).

Evidence snapshots, capsules, job records and logs live outside every working repository. During development they belong in \$LIBENR_HOME (.dev-state/, gitignored)." ;;
  esac
done < <(hook_paths)

exit 0
