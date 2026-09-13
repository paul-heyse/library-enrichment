#!/usr/bin/env bash
# Hook behaviour tests. The hooks are the enforcement layer, so they need the same
# regression discipline as the code they guard. Run by `just hooks-test` and by CI.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export CLAUDE_PROJECT_DIR="$ROOT"
pass=0; fail=0

_json() { python3 -c 'import json,sys;print(json.dumps(sys.argv[1]))' "$1"; }

bash_case() {  # desc, command, expect(deny|allow)
  local out got
  out=$(printf '{"hook_event_name":"PreToolUse","tool_name":"Bash","cwd":"%s","tool_input":{"command":%s}}' \
        "$ROOT" "$(_json "$2")" | "$ROOT/scripts/hooks/pre_bash.sh" 2>/dev/null)
  got=allow; printf '%s' "$out" | grep -q '"deny"' && got=deny
  if [ "$got" = "$3" ]; then pass=$((pass+1)); printf '  ok   %-5s %s\n' "$got" "$1"
  else fail=$((fail+1)); printf '  FAIL want=%s got=%s  %s\n         %s\n' "$3" "$got" "$1" "$2"; fi
}

edit_case() {  # desc, path, expect(deny|allow)
  local out got
  out=$(printf '{"hook_event_name":"PreToolUse","tool_name":"Write","cwd":"%s","tool_input":{"file_path":%s}}' \
        "$ROOT" "$(_json "$2")" | "$ROOT/scripts/hooks/pre_edit.sh" 2>/dev/null)
  got=allow; printf '%s' "$out" | grep -q '"deny"' && got=deny
  if [ "$got" = "$3" ]; then pass=$((pass+1)); printf '  ok   %-5s %s\n' "$got" "$1"
  else fail=$((fail+1)); printf '  FAIL want=%s got=%s  %s\n         %s\n' "$3" "$got" "$1" "$2"; fi
}

echo "pre_bash: binding decisions"
bash_case "pyrefly"             'uv run pyrefly check src/'                            deny
bash_case "pyright"             'pyright python/'                                      deny
bash_case "mypy"                'uv run mypy .'                                        deny
bash_case "ty is the engine"    'uv run ty check python/'                              allow

echo "pre_bash: pinning and toolchain"
bash_case "uvx @latest"         'uvx ruff@latest check'                                deny
bash_case "uvx unpinned"        'uvx cowsay hello'                                     deny
bash_case "uvx pinned"          'uvx --from ruff==0.14.0 ruff check'                   allow
bash_case "bare +nightly"       'cargo +nightly rustdoc -- -Z unstable-options'        deny
bash_case "dated nightly"       'cargo +nightly-2026-08-18 rustdoc --lib'              allow

echo "pre_bash: user-scope configuration"
bash_case "claude mcp add"      'claude mcp add --scope user library-enrichment -- /x' deny
bash_case "codex mcp add"       'codex mcp add library-enrichment -- /x'               deny
bash_case "skill install"       'cp -r skills/library-research ~/.claude/skills/'      deny

echo "pre_bash: banned dependency classes"
bash_case "cargo add lancedb"   'cargo add lancedb --features remote'                  deny
bash_case "uv add fastapi"      'uv add fastapi'                                       deny
bash_case "cargo add serde"     'cargo add serde --features derive'                    allow

echo "pre_bash: repository boundary (C20)"
bash_case "rm outside"          'rm -rf /home/paul/smartref/build'                     deny
bash_case "redirect outside"    'echo x > /home/paul/other/file.txt'                   deny
bash_case "read outside"        'cat /home/paul/Code_Fabric/justfile'                  allow
bash_case "grep outside"        'rg pattern /home/paul/smartref'                       allow
bash_case "relative write"      'mkdir -p crates/enrichment-core/src'                  allow
bash_case "dev-state write"     'mkdir -p .dev-state/cache'                            allow
bash_case "plain ls"            'ls -la docs/'                                         allow

echo "pre_bash: frozen paths cannot be written through the shell"
bash_case "heredoc to AGENTS.md"  'cat > AGENTS.md <<EOF\nx\nEOF'                   deny
bash_case "redirect to blueprint" 'echo x > docs/blueprint/IMPLEMENTATION_BLUEPRINT.md' deny
bash_case "sed -i a contract"     'sed -i s/a/b/ contracts/research-envelope.schema.json' deny
bash_case "sed -i a hook"         'sed -i s/a/b/ scripts/hooks/pre_bash.sh'           deny
bash_case "tee a .claude rule"    'echo x | tee .claude/rules/python-boundary.md'     deny
bash_case "shell-write justfile"  'echo x >> justfile'                                allow
bash_case "shell-write a rule"    'cat > rules/new.yml <<EOF\nx\nEOF'                allow
bash_case "write a generated dto" 'echo x > python/enrichment_mcp/_generated/m.py'    deny
bash_case "absolute frozen path"  'echo x > /home/paul/library-enrichment/AGENTS.md'  deny
bash_case "READ the blueprint"    'cat docs/blueprint/IMPLEMENTATION_BLUEPRINT.md'    allow
bash_case "grep the contracts"    'rg envelope contracts/'                           allow
bash_case "run a script"          './scripts/provenance-check.sh'                    allow
bash_case "write to STATUS.md"    'echo x > STATUS.md'                               allow
bash_case "write an ADR"          'cat > docs/adr/0005-x.md <<EOF\nx\nEOF'          allow
bash_case "write core source"     'echo x > crates/enrichment-core/src/policy.rs'     allow

echo "pre_edit: frozen, generated, governance"
edit_case "blueprint"           'docs/blueprint/IMPLEMENTATION_BLUEPRINT.md'           deny
edit_case "provenance manifest" 'docs/provenance/bundle-2026-09-13/MANIFEST.sha256'    deny
edit_case "frozen schema"       'contracts/research-envelope.schema.json'              deny
edit_case "acceptance plan"     'tests/ACCEPTANCE_PLAN.md'                             deny
edit_case "generated schema"    'schemas/generated/envelope.json'                      deny
edit_case "generated dto"       'python/enrichment_mcp/_generated/models.py'           deny
edit_case "AGENTS.md"           'AGENTS.md'                                            deny
edit_case "CLAUDE.md"           'CLAUDE.md'                                            deny
edit_case "settings"            '.claude/settings.json'                                deny
edit_case "a .claude rule"      '.claude/rules/python-boundary.md'                     deny
edit_case "a hook"              'scripts/hooks/pre_bash.sh'                            deny
edit_case "the shared env"      'scripts/env.sh'                                       deny
edit_case "state in repo"       'snapshots/snap_x/manifest.json'                       deny
edit_case "outside repo"        '/home/paul/smartref/src/main.py'                      deny
edit_case "core source"         'crates/enrichment-core/src/lib.rs'                    allow
edit_case "adapter source"      'python/enrichment_mcp/server.py'                      allow
edit_case "a test"              'tests/contract/test_envelope.py'                      allow
edit_case "an ADR"              'docs/adr/0004-something.md'                           allow
edit_case "gates registry"      'tests/gates.toml'                                     allow
edit_case "STATUS.md"           'STATUS.md'                                            allow

echo "pre_edit: the working surface must stay editable"
edit_case "justfile"            'justfile'                                             allow
edit_case "a gate script"       'scripts/gate-phase.sh'                                allow
edit_case "a new ast-grep rule" 'rules/no-unbounded-fetch.yml'                          allow
edit_case "a rule fixture"      'rule-tests/no-unbounded-fetch-test.yml'                allow
edit_case "a subagent"          '.claude/agents/upstream-verifier.md'                   allow
edit_case "a slash command"     '.claude/commands/phase-gate.md'                        allow
edit_case "a dev skill"         '.claude/skills/some-workflow/SKILL.md'                 allow

printf '\nhooks: %d passed, %d failed\n' "$pass" "$fail"
[ "$fail" -eq 0 ]
