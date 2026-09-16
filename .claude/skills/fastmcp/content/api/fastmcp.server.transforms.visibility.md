# `fastmcp.server.transforms.visibility`

Distribution: `fastmcp`

## ComponentT

`fastmcp.server.transforms.visibility.ComponentT`

```python
ComponentT = TypeVar('ComponentT', bound='FastMCPComponent')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## T

`fastmcp.server.transforms.visibility.T`

```python
T = TypeVar('T', bound='FastMCPComponent')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _FASTMCP_KEY

`fastmcp.server.transforms.visibility._FASTMCP_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FASTMCP_KEY = 'fastmcp'
```

## _INTERNAL_KEY

`fastmcp.server.transforms.visibility._INTERNAL_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_INTERNAL_KEY = '_internal'
```

## Visibility

Import as `fastmcp.server.transforms.Visibility`  ·  defined at `fastmcp.server.transforms.visibility.Visibility`

```python
class Visibility(Transform)
```

**Also exported as** `fastmcp.server.transforms.Visibility`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Transform`

**Declared members (14)**

- `components = components`  _instance-attribute_
- `async def get_prompt(self, name: str, call_next: GetPromptNext, version: VersionSpec | None = None) -> Prompt | None`  _async_
  Mark prompt if found.
- `async def get_resource(self, uri: str, call_next: GetResourceNext, version: VersionSpec | None = None) -> Resource | None`  _async_
  Mark resource if found.
- `async def get_resource_template(self, uri: str, call_next: GetResourceTemplateNext, version: VersionSpec | None = None) -> ResourceTemplate | None`  _async_
  Mark resource template if found.
- `async def get_tool(self, name: str, call_next: GetToolNext, version: VersionSpec | None = None) -> Tool | None`  _async_
  Mark tool if found.
- `keys = keys`  _instance-attribute_
- `async def list_prompts(self, prompts: Sequence[Prompt]) -> Sequence[Prompt]`  _async_
  Mark prompts by visibility state.
- `async def list_resource_templates(self, templates: Sequence[ResourceTemplate]) -> Sequence[ResourceTemplate]`  _async_
  Mark resource templates by visibility state.
- `async def list_resources(self, resources: Sequence[Resource]) -> Sequence[Resource]`  _async_
  Mark resources by visibility state.
- `async def list_tools(self, tools: Sequence[Tool]) -> Sequence[Tool]`  _async_
  Mark tools by visibility state.
- `match_all = match_all`  _instance-attribute_
- `names = names`  _instance-attribute_
- `tags = tags`  _instance-attribute_
- `version = version`  _instance-attribute_

Sets visibility state on matching components.

Does NOT filter inline - just marks components with visibility state.
Later transforms in the chain can override earlier marks.
Final filtering happens at the Provider level after all transforms run.

Example:
    ```python
    # Disable components tagged "internal"
    Visibility(False, tags={"internal"})

    # Re-enable specific tool (override earlier disable)
    Visibility(True, names={"safe_tool"})

    # Allowlist via composition:
    Visibility(False, match_all=True)  # disable everything
    Visibility(True, tags={"public"})  # enable public
    ```


## apply_session_transforms

Import as `fastmcp.server.server.apply_session_transforms`  ·  defined at `fastmcp.server.transforms.visibility.apply_session_transforms`

```python
async def apply_session_transforms(components: Sequence[ComponentT]) -> Sequence[ComponentT]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Apply session-specific visibility transforms to components.

This helper applies session-level enable/disable rules by marking
components with their visibility state. Session transforms override
global transforms due to mark-based semantics (later marks win).

Args:
    components: The components to apply session transforms to.

Returns:
    The components with session transforms applied.


## create_visibility_transforms

`fastmcp.server.transforms.visibility.create_visibility_transforms`

```python
def create_visibility_transforms(rules: list[dict[str, Any]]) -> list[Visibility]
```

Convert rule dicts to Visibility transforms.


## disable_components

`fastmcp.server.transforms.visibility.disable_components`

```python
async def disable_components(context: Context, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, match_all: bool = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Disable components matching criteria for this session only.

Session rules override global transforms. Rules accumulate - each call
adds a new rule to the session. Later marks override earlier ones
(Visibility transform semantics).

Sends notifications to this session only: ToolListChangedNotification,
ResourceListChangedNotification, and PromptListChangedNotification.

Args:
    context: The context for this session.
    names: Component names or URIs to match.
    keys: Component keys to match (e.g., {"tool:my_tool@v1"}).
    version: Component version spec to match.
    tags: Tags to match (component must have at least one).
    components: Component types to match (e.g., {"tool", "prompt"}).
    match_all: If True, matches all components regardless of other criteria.


## enable_components

`fastmcp.server.transforms.visibility.enable_components`

```python
async def enable_components(context: Context, names: set[str] | None = None, keys: set[str] | None = None, version: VersionSpec | None = None, tags: set[str] | None = None, components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None, match_all: bool = False) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Enable components matching criteria for this session only.

Session rules override global transforms. Rules accumulate - each call
adds a new rule to the session. Later marks override earlier ones
(Visibility transform semantics).

Sends notifications to this session only: ToolListChangedNotification,
ResourceListChangedNotification, and PromptListChangedNotification.

Args:
    context: The context for this session.
    names: Component names or URIs to match.
    keys: Component keys to match (e.g., {"tool:my_tool@v1"}).
    version: Component version spec to match.
    tags: Tags to match (component must have at least one).
    components: Component types to match (e.g., {"tool", "prompt"}).
    match_all: If True, matches all components regardless of other criteria.


## get_session_transforms

`fastmcp.server.transforms.visibility.get_session_transforms`

```python
async def get_session_transforms(context: Context) -> list[Visibility]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get session-specific Visibility transforms from state store.


## get_visibility_rules

`fastmcp.server.transforms.visibility.get_visibility_rules`

```python
async def get_visibility_rules(context: Context) -> list[dict[str, Any]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Load visibility rule dicts from session state.


## is_enabled

Import as `fastmcp.server.transforms.is_enabled`  ·  defined at `fastmcp.server.transforms.visibility.is_enabled`

```python
def is_enabled(component: FastMCPComponent) -> bool
```

**Also exported as** `fastmcp.server.transforms.is_enabled`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if component is enabled.

Returns True if:
- No visibility mark exists (default is enabled)
- Visibility mark is True

Returns False if visibility mark is False.

Args:
    component: Component to check.

Returns:
    True if component should be enabled/visible to clients.


## reset_visibility

`fastmcp.server.transforms.visibility.reset_visibility`

```python
async def reset_visibility(context: Context) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Clear all session visibility rules.

Use this to reset session visibility back to global defaults.

Sends notifications to this session only: ToolListChangedNotification,
ResourceListChangedNotification, and PromptListChangedNotification.

Args:
    context: The context for this session.


## save_visibility_rules

`fastmcp.server.transforms.visibility.save_visibility_rules`

```python
async def save_visibility_rules(context: Context, rules: list[dict[str, Any]], components: set[Literal['tool', 'resource', 'template', 'prompt']] | None = None) -> None
```

Save visibility rule dicts to session state and send notifications.

Args:
    context: The context to save rules for.
    rules: The visibility rules to save.
    components: Optional hint about which component types are affected.
        If None, sends notifications for all types (safe default).
        If provided, only sends notifications for specified types.


