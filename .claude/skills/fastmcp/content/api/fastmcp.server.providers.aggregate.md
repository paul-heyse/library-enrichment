# `fastmcp.server.providers.aggregate`

Distribution: `fastmcp`

## ProviderErrorStrategy

Import as `fastmcp.server.providers.proxy.ProviderErrorStrategy`  ·  defined at `fastmcp.server.providers.aggregate.ProviderErrorStrategy`

```python
ProviderErrorStrategy = Literal['warn', 'raise']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["warn", "raise"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## T

`fastmcp.server.providers.aggregate.T`

```python
T = TypeVar('T')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## logger

`fastmcp.server.providers.aggregate.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AggregateProvider

Import as `fastmcp.server.providers.AggregateProvider`  ·  defined at `fastmcp.server.providers.aggregate.AggregateProvider`

```python
class AggregateProvider(Provider)
```

**Also exported as** `fastmcp.server.providers.AggregateProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`

**Declared members (7)**

- `def add_provider(self, provider: Provider, namespace: str = '') -> None`
  Add a provider with optional namespace.
- `async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None`  _async_
  Query all child providers for an app tool.
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Get all task-eligible components from all providers.
- `async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None`  _async_
  Query all child providers for a tool matching a hash.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
  Combine lifespans of all providers.
- `provider_error_strategy = provider_error_strategy`  _instance-attribute_
- `providers: list[Provider] = list(providers or [])`  _instance-attribute_

**Inherited (13)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Utility provider that combines multiple providers into one.

Components are aggregated from all providers. For get_* operations,
providers are queried in parallel and the highest version is returned.

When adding providers with a namespace, wrap_transform() is used to apply
the Namespace transform. This means namespace transformation is handled
by the wrapped provider, not by AggregateProvider.

Errors from individual providers are logged and skipped by default. Set
``provider_error_strategy="raise"`` to fail the aggregate operation when
any provider fails.

Example:
    ```python
    combined = AggregateProvider()
    combined.add_provider(db_provider)
    combined.add_provider(api_provider, namespace="api")
    # db_provider's tools keep original names
    # api_provider's tools become "api_foo", "api_bar", etc.
    ```


