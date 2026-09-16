# `fastmcp.server.providers.skills.vendor_providers`

Distribution: `fastmcp`

## CodexSkillsProvider

Import as `fastmcp.server.providers.skills.CodexSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.CodexSkillsProvider`

```python
class CodexSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.CodexSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Codex skills from /etc/codex/skills/ and ~/.codex/skills/.

Scans both system-level and user-level directories. System skills take
precedence if duplicates exist.


## CopilotSkillsProvider

Import as `fastmcp.server.providers.skills.CopilotSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.CopilotSkillsProvider`

```python
class CopilotSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.CopilotSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

GitHub Copilot skills from ~/.copilot/skills/.


## CursorSkillsProvider

Import as `fastmcp.server.providers.skills.CursorSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.CursorSkillsProvider`

```python
class CursorSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.CursorSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Cursor skills from ~/.cursor/skills/.


## GeminiSkillsProvider

Import as `fastmcp.server.providers.skills.GeminiSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.GeminiSkillsProvider`

```python
class GeminiSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.GeminiSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Gemini skills from ~/.gemini/skills/.


## GooseSkillsProvider

Import as `fastmcp.server.providers.skills.GooseSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.GooseSkillsProvider`

```python
class GooseSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.GooseSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Goose skills from ~/.config/agents/skills/.


## OpenCodeSkillsProvider

Import as `fastmcp.server.providers.skills.OpenCodeSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.OpenCodeSkillsProvider`

```python
class OpenCodeSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.OpenCodeSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OpenCode skills from ~/.config/opencode/skills/.


## VSCodeSkillsProvider

Import as `fastmcp.server.providers.skills.VSCodeSkillsProvider`  ·  defined at `fastmcp.server.providers.skills.vendor_providers.VSCodeSkillsProvider`

```python
class VSCodeSkillsProvider(SkillsDirectoryProvider)
```

**Also exported as** `fastmcp.server.providers.skills.VSCodeSkillsProvider`

**Bases** `SkillsDirectoryProvider`

**Inherited (20)**

- from `fastmcp.server.providers.aggregate.AggregateProvider`: `add_provider`, `get_app_tool`, `get_tasks`, `get_tool_by_hash`, `lifespan`, `provider_error_strategy`, `providers`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

VS Code skills from ~/.copilot/skills/.


