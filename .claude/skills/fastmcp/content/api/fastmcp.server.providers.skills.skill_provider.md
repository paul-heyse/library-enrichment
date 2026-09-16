# `fastmcp.server.providers.skills.skill_provider`

Distribution: `fastmcp`

## logger

`fastmcp.server.providers.skills.skill_provider.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SkillFileResource

`fastmcp.server.providers.skills.skill_provider.SkillFileResource`

```python
class SkillFileResource(Resource)
```

**Bases** `Resource`

**Declared members (4)**

- `file_path: str`  _instance-attribute_
- `def get_meta(self) -> dict[str, Any]`
- `async def read(self) -> str | bytes | ResourceResult`  _async_
  Read the file content.
- `skill_info: SkillInfo`  _instance-attribute_

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource representing a specific file within a skill.


## SkillFileTemplate

`fastmcp.server.providers.skills.skill_provider.SkillFileTemplate`

```python
class SkillFileTemplate(ResourceTemplate)
```

**Bases** `ResourceTemplate`

**Declared members (3)**

- `async def create_resource(self, uri: str, params: dict[str, Any]) -> Resource`  _async_
  Create a resource for the given URI and parameters.
- `async def read(self, arguments: dict[str, Any]) -> str | bytes | ResourceResult`  _async_
  Read a file from the skill directory.
- `skill_info: SkillInfo`  _instance-attribute_

**Inherited (28)**

- from `fastmcp.resources.template.ResourceTemplate`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `from_mcp_template`, `get_span_attributes`, `key`, `matches`, `mime_type`, `parameters`, `resolve_security`, `security`, `set_default_mime_type`, `to_mcp_template`, `uri_template`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A template for accessing files within a skill.


## SkillProvider

Import as `fastmcp.server.providers.SkillProvider`  ·  defined at `fastmcp.server.providers.skills.skill_provider.SkillProvider`

```python
class SkillProvider(Provider)
```

**Also exported as** `fastmcp.server.providers.SkillProvider`, `fastmcp.server.providers.skills.SkillProvider`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`

**Declared members (1)**

- `skill_info: SkillInfo`  _property_
  Get the loaded skill info.

**Inherited (17)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `lifespan`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that exposes a single skill folder as MCP resources.

Each skill folder must contain a main file (default: SKILL.md) and may
contain additional supporting files.

Exposes:
- A Resource for the main file (skill://{name}/SKILL.md)
- A Resource for the synthetic manifest (skill://{name}/_manifest)
- Supporting files via ResourceTemplate or Resources (configurable)

Args:
    skill_path: Path to the skill directory.
    main_file_name: Name of the main skill file. Defaults to "SKILL.md".
    supporting_files: How supporting files (everything except main file and
        manifest) are exposed to clients:
        - "template": Accessed via ResourceTemplate, hidden from list_resources().
          Clients discover files by reading the manifest first.
        - "resources": Each file exposed as individual Resource in list_resources().
          Full enumeration upfront.

Example:
    ```python
    from pathlib import Path
    from fastmcp import FastMCP
    from fastmcp.server.providers.skills import SkillProvider

    mcp = FastMCP("My Skill")
    mcp.add_provider(SkillProvider(
        Path.home() / ".claude/skills/pdf-processing"
    ))
    ```


## SkillResource

`fastmcp.server.providers.skills.skill_provider.SkillResource`

```python
class SkillResource(Resource)
```

**Bases** `Resource`

**Declared members (4)**

- `def get_meta(self) -> dict[str, Any]`
- `is_manifest: bool = False`  _class-attribute, instance-attribute_
- `async def read(self) -> str | bytes | ResourceResult`  _async_
  Read the resource content.
- `skill_info: SkillInfo`  _instance-attribute_

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource representing a skill's main file or manifest.


