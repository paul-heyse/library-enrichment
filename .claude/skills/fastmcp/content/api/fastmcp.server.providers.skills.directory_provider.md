# `fastmcp.server.providers.skills.directory_provider`

Distribution: `fastmcp`

## logger

`fastmcp.server.providers.skills.directory_provider.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SkillsDirectoryProvider

Import as `fastmcp.server.providers.SkillsDirectoryProvider`  ·  defined at `fastmcp.server.providers.skills.directory_provider.SkillsDirectoryProvider`

```python
class SkillsDirectoryProvider(AggregateProvider)
```

**Also exported as** `fastmcp.server.providers.SkillsDirectoryProvider`, `fastmcp.server.providers.skills.SkillsDirectoryProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AggregateProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that scans directories and creates a SkillProvider per skill folder.

This extends AggregateProvider to combine multiple SkillProviders into one.
Each subdirectory containing a main file (default: SKILL.md) becomes a skill.
Can scan multiple root directories - if a skill name appears in multiple roots,
the first one found wins.

Args:
    roots: Root directory(ies) containing skill folders. Can be a single path
        or a sequence of paths.
    reload: If True, re-discover skills on each request. Defaults to False.
    main_file_name: Name of the main skill file. Defaults to "SKILL.md".
    supporting_files: How supporting files are exposed in child SkillProviders:
        - "template": Accessed via ResourceTemplate, hidden from list_resources().
        - "resources": Each file exposed as individual Resource in list_resources().

Example:
    ```python
    from pathlib import Path
    from fastmcp import FastMCP
    from fastmcp.server.providers.skills import SkillsDirectoryProvider

    mcp = FastMCP("Skills")
    # Single directory
    mcp.add_provider(SkillsDirectoryProvider(
        roots=Path.home() / ".claude" / "skills",
        reload=True,  # Re-scan on each request
    ))
    # Multiple directories
    mcp.add_provider(SkillsDirectoryProvider(
        roots=[Path("/etc/skills"), Path.home() / ".local" / "skills"],
    ))
    ```


