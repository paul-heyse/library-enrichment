# FastMCPComponent

The shared base of tools, resources and prompts -- what every registered thing is.

Import as `fastmcp.tools.base.FastMCPComponent`
Defined at `fastmcp.utilities.components.FastMCPComponent`.

```python
class FastMCPComponent(FastMCPBaseModel)
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
KEY_PREFIX: str = ''
def copy(self) -> Self
description: str | None = Field(default=None, description='The description of the component.')
def disable(self) -> None
def enable(self) -> None
def get_meta(self) -> dict[str, Any]
def get_span_attributes(self) -> dict[str, Any]
icons: list[Icon] | None = Field(default=None, description='Optional list of icons for this component to display in user interfaces.')
key: str
def make_key(cls, identifier: str) -> str
meta: dict[str, Any] | None = Field(default=None, description='Meta information about the component')
name: str = Field(description='The name of the component.')
tags: Annotated[set[str], BeforeValidator(_convert_set_default_none)] = Field(default_factory=set, description='Tags for the component.')
task_config: Annotated[TaskConfig, Field(description="Background task execution configuration (SEP-2663). Only tools support task execution; other component types always carry the default 'forbidden' config.")] = Field(default_factory=lambda: TaskConfig(mode='forbidden'))
title: str | None = Field(default=None, description='The title of the component for display purposes.')
version: Annotated[str | None, BeforeValidator(_coerce_version)] = Field(default=None, description='Optional version identifier for this component. Multiple versions of the same component (same name) can coexist.')
```

## Implementors (28)

Transitive. Read one before writing your own.

- `fastmcp.prompts.base.Prompt`
- `fastmcp.prompts.function_prompt.FunctionPrompt`
- `fastmcp.resources.base.Resource`
- `fastmcp.resources.function_resource.FunctionResource`
- `fastmcp.resources.template.FunctionResourceTemplate`
- `fastmcp.resources.template.ResourceTemplate`
- `fastmcp.resources.types.BinaryResource`
- `fastmcp.resources.types.DirectoryResource`
- `fastmcp.resources.types.FileResource`
- `fastmcp.resources.types.HttpResource`
- `fastmcp.resources.types.TextResource`
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderPrompt`
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderResource`
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderResourceTemplate`
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderTool`
- `fastmcp.server.providers.openapi.components.OpenAPIResource`
- `fastmcp.server.providers.openapi.components.OpenAPIResourceTemplate`
- `fastmcp.server.providers.openapi.components.OpenAPITool`
- `fastmcp.server.providers.proxy.ProxyPrompt`
- `fastmcp.server.providers.proxy.ProxyResource`
- `fastmcp.server.providers.proxy.ProxyTemplate`
- `fastmcp.server.providers.proxy.ProxyTool`
- `fastmcp.server.providers.skills.skill_provider.SkillFileResource`
- `fastmcp.server.providers.skills.skill_provider.SkillFileTemplate`
- `fastmcp.server.providers.skills.skill_provider.SkillResource`
- `fastmcp.tools.base.Tool`
- `fastmcp.tools.function_tool.FunctionTool`
- `fastmcp.tools.tool_transform.TransformedTool`

## Demonstrated by 7 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/examples/providers/sqlite/server.py`](../corpus/examples/providers/sqlite/server.py)
- [`corpus/tests/resources/test_resources.py`](../corpus/tests/resources/test_resources.py)
- [`corpus/tests/server/providers/test_base_provider.py`](../corpus/tests/server/providers/test_base_provider.py)
- [`corpus/tests/server/providers/test_local_provider.py`](../corpus/tests/server/providers/test_local_provider.py)
- [`corpus/tests/server/test_providers.py`](../corpus/tests/server/test_providers.py)
- [`corpus/tests/tasks/server/test_custom_subclass_tasks.py`](../corpus/tests/tasks/server/test_custom_subclass_tasks.py)
- [`corpus/tests/utilities/test_components.py`](../corpus/tests/utilities/test_components.py)

## Documentation

Prose: [`api/fastmcp.utilities.components.md`](../api/fastmcp.utilities.components.md) · records: [`model/fastmcp.utilities.components.json`](../model/fastmcp.utilities.components.json)
