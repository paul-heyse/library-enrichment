# Provider

Where components come from. A server is itself a Provider, and `mount()` and `create_proxy()` are thin wrappers over one.

Import as `fastmcp.server.providers.Provider`
Defined at `fastmcp.server.providers.base.Provider`.

```python
class Provider
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
def add_transform(self, transform: Transform) -> None
def disable(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None) -> Self
def enable(self, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, only: bool = False) -> Self
async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None
async def get_prompt(self, name: str, version: VersionSpec | None = None) -> Prompt | None
async def get_resource(self, uri: str, version: VersionSpec | None = None) -> Resource | None
async def get_resource_template(self, uri: str, version: VersionSpec | None = None) -> ResourceTemplate | None
async def get_tasks(self) -> Sequence[FastMCPComponent]
async def get_tool(self, name: str, version: VersionSpec | None = None) -> Tool | None
async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None
async def lifespan(self) -> AsyncIterator[None]
async def list_prompts(self) -> Sequence[Prompt]
async def list_resource_templates(self) -> Sequence[ResourceTemplate]
async def list_resources(self) -> Sequence[Resource]
async def list_tools(self) -> Sequence[Tool]
transforms: list[Transform]
def wrap_transform(self, transform: Transform) -> Provider
```

## Implementors (26)

Transitive. Read one before writing your own.

- `fastmcp.apps.app.FastMCPApp`
- `fastmcp.apps.approval.Approval`
- `fastmcp.apps.choice.Choice`
- `fastmcp.apps.file_upload.FileUpload`
- `fastmcp.apps.form.FormInput`
- `fastmcp.apps.generative.GenerativeUI`
- `fastmcp.server.providers.aggregate.AggregateProvider`
- `fastmcp.server.providers.fastmcp_provider.FastMCPProvider`
- `fastmcp.server.providers.filesystem.FileSystemProvider`
- `fastmcp.server.providers.local_provider.local_provider.LocalProvider`
- `fastmcp.server.providers.openapi.provider.OpenAPIProvider`
- `fastmcp.server.providers.proxy.FastMCPProxy`
- `fastmcp.server.providers.proxy.ProxyProvider`
- `fastmcp.server.providers.skills.claude_provider.ClaudeSkillsProvider`
- `fastmcp.server.providers.skills.directory_provider.SkillsDirectoryProvider`
- `fastmcp.server.providers.skills.skill_provider.SkillProvider`
- `fastmcp.server.providers.skills.vendor_providers.CodexSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.CopilotSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.CursorSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.GeminiSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.GooseSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.OpenCodeSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.VSCodeSkillsProvider`
- `fastmcp.server.providers.wrapped_provider._WrappedProvider`
- `fastmcp.server.server.FastMCP`
- `fastmcp.server.sessions.SessionProvider`

## Demonstrated by 5 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/examples/providers/sqlite/server.py`](../corpus/examples/providers/sqlite/server.py)
- [`corpus/tests/apps/test_file_upload.py`](../corpus/tests/apps/test_file_upload.py)
- [`corpus/tests/server/providers/test_base_provider.py`](../corpus/tests/server/providers/test_base_provider.py)
- [`corpus/tests/server/test_providers.py`](../corpus/tests/server/test_providers.py)
- [`corpus/tests/server/test_server_lifespan.py`](../corpus/tests/server/test_server_lifespan.py)

## Documentation

Prose: [`api/fastmcp.server.providers.base.md`](../api/fastmcp.server.providers.base.md) · records: [`model/fastmcp.server.providers.base.json`](../model/fastmcp.server.providers.base.json)
