# `fastmcp.server.providers.skills.claude_provider`

Distribution: `fastmcp`

## ClaudeSkillsProvider

Import as `fastmcp.server.providers.ClaudeSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.claude_provider.ClaudeSkillsProvider`

```python
class ClaudeSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.ClaudeSkillsProvider`, `fastmcp.server.providers.skills.ClaudeSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider for Claude Code skills from ~/.claude/skills/.

A convenience subclass that sets the default root to Claude's skills location.

Args:
    reload: If True, re-scan on every request. Defaults to False.
    supporting_files: How supporting files are exposed:
        - "template": Accessed via ResourceTemplate, hidden from list_resources().
        - "resources": Each file exposed as individual Resource in list_resources().

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.providers.skills import ClaudeSkillsProvider

    mcp = FastMCP("Claude Skills")
    mcp.add_provider(ClaudeSkillsProvider())  # Uses default location
    ```


