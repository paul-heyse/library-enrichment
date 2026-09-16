# `fastmcp.utilities.prefab`

Distribution: `fastmcp`

## _could_be_prefab

`fastmcp.utilities.prefab._could_be_prefab`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _could_be_prefab(value_or_type: Any) -> bool
```

Cheaply reject ordinary values before importing Prefab UI.


## _get_prefab_types

`fastmcp.utilities.prefab._get_prefab_types`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_prefab_types() -> tuple[type[Any], type[Any]] | None
```

Import and return Prefab's public app and component types on demand.


## is_prefab_app

Import as `fastmcp.tools.base.is_prefab_app`  ·  defined at `fastmcp.utilities.prefab.is_prefab_app`

```python
def is_prefab_app(value: Any) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether a value is a Prefab app.


## is_prefab_component

Import as `fastmcp.tools.base.is_prefab_component`  ·  defined at `fastmcp.utilities.prefab.is_prefab_component`

```python
def is_prefab_component(value: Any) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether a value is a Prefab component.


## is_prefab_type

Import as `fastmcp.tools.function_parsing.is_prefab_type`  ·  defined at `fastmcp.utilities.prefab.is_prefab_type`

```python
def is_prefab_type(candidate: Any) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether a type is a Prefab app or component type.


## prefab_app_from_component

Import as `fastmcp.tools.base.prefab_app_from_component`  ·  defined at `fastmcp.utilities.prefab.prefab_app_from_component`

```python
def prefab_app_from_component(component: Any) -> Any
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Wrap a Prefab component in a Prefab app.


## prefab_available

Import as `fastmcp.server.providers.local_provider.decorators.tools.prefab_available`  ·  defined at `fastmcp.utilities.prefab.prefab_available`

```python
def prefab_available() -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return whether Prefab UI is installed without importing it.


