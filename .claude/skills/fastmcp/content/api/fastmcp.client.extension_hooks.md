# `fastmcp.client.extension_hooks`

Distribution: `fastmcp`

## InternalClientExtensionFactory

`fastmcp.client.extension_hooks.InternalClientExtensionFactory`

```python
InternalClientExtensionFactory = Callable[['ElicitationFnT | None'], 'ClientExtension | None']
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(ElicitationFnT | None, /) -> ClientExtension | None'> ````

## _internal_client_extension_factories

`fastmcp.client.extension_hooks._internal_client_extension_factories`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_internal_client_extension_factories: list[InternalClientExtensionFactory] = []
```

## build_internal_client_extensions

Import as `fastmcp.client.client.build_internal_client_extensions`  ·  defined at `fastmcp.client.extension_hooks.build_internal_client_extensions`

```python
def build_internal_client_extensions(elicitation_callback: ElicitationFnT | None) -> list[ClientExtension]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build the internal extensions to fold into a ``Client`` under construction.

Each registered factory is invoked with the client's elicitation callback;
factories that return ``None`` contribute nothing. Empty when no companion
package has registered a factory (plain core, or ``fastmcp_tasks`` unimported).


## register_internal_client_extension_factory

`fastmcp.client.extension_hooks.register_internal_client_extension_factory`

```python
def register_internal_client_extension_factory(factory: InternalClientExtensionFactory) -> None
```

Register a factory whose extension every ``Client`` folds in automatically.

Idempotent: registering the same factory object twice is a no-op, so a
package importing more than once does not double-register.


