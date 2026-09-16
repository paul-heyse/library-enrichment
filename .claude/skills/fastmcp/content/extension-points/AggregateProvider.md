# AggregateProvider

Composing several Providers into one. `FastMCP` reaches `Provider` through this.

Import as `fastmcp.server.providers.AggregateProvider`
Defined at `fastmcp.server.providers.aggregate.AggregateProvider`.

```python
class AggregateProvider(Provider)
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
def add_provider(self, provider: Provider, namespace: str = '') -> None
async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None
async def get_tasks(self) -> Sequence[FastMCPComponent]
async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None
async def lifespan(self) -> AsyncIterator[None]
provider_error_strategy = provider_error_strategy
providers: list[Provider] = list(providers or [])
```

## Implementors (11)

Transitive. Read one before writing your own.

- `fastmcp.server.providers.proxy.FastMCPProxy`
- `fastmcp.server.providers.skills.claude_provider.ClaudeSkillsProvider`
- `fastmcp.server.providers.skills.directory_provider.SkillsDirectoryProvider`
- `fastmcp.server.providers.skills.vendor_providers.CodexSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.CopilotSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.CursorSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.GeminiSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.GooseSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.OpenCodeSkillsProvider`
- `fastmcp.server.providers.skills.vendor_providers.VSCodeSkillsProvider`
- `fastmcp.server.server.FastMCP`

## Documentation

Prose: [`api/fastmcp.server.providers.aggregate.md`](../api/fastmcp.server.providers.aggregate.md) · records: [`model/fastmcp.server.providers.aggregate.json`](../model/fastmcp.server.providers.aggregate.json)
