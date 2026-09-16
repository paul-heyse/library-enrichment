# `fastmcp.utilities.tasks`

Distribution: `fastmcp`

## DEFAULT_POLL_INTERVAL

`fastmcp.utilities.tasks.DEFAULT_POLL_INTERVAL`

```python
DEFAULT_POLL_INTERVAL = timedelta(seconds=5)
```

**Inferred type** (`ty`, not declared in the source): `timedelta`

## DEFAULT_POLL_INTERVAL_MS

`fastmcp.utilities.tasks.DEFAULT_POLL_INTERVAL_MS`

```python
DEFAULT_POLL_INTERVAL_MS = int(DEFAULT_POLL_INTERVAL.total_seconds() * 1000)
```

**Inferred type** (`ty`, not declared in the source): `int`

## DEFAULT_TTL_MS

`fastmcp.utilities.tasks.DEFAULT_TTL_MS`

```python
DEFAULT_TTL_MS = 60000
```

**Inferred type** (`ty`, not declared in the source): `Literal[60000]`

## TASKS_EXTENSION_ID

`fastmcp.utilities.tasks.TASKS_EXTENSION_ID`

```python
TASKS_EXTENSION_ID = 'io.modelcontextprotocol/tasks'
```

**Inferred type** (`ty`, not declared in the source): `Literal["io.modelcontextprotocol/tasks"]`

## TaskMode

`fastmcp.utilities.tasks.TaskMode`

```python
TaskMode = Literal['forbidden', 'optional', 'required']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["forbidden", "optional", "required"]'> ````

## TaskConfig

Import as `fastmcp.decorators.TaskConfig`  ·  defined at `fastmcp.utilities.tasks.TaskConfig`

```python
class TaskConfig
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `def from_bool(cls, value: bool) -> TaskConfig`  _classmethod_
  Convert a boolean task flag to a TaskConfig.
- `mode: TaskMode = 'optional'`  _class-attribute, instance-attribute_
- `poll_interval: timedelta = DEFAULT_POLL_INTERVAL`  _class-attribute, instance-attribute_
- `def supports_tasks(self) -> bool`
  Check if this component supports task execution.
- `def validate_function(self, fn: Callable[..., Any], name: str) -> None`
  Validate that a function is compatible with this task config.

Configuration for MCP background task execution.

Controls how a component handles task-augmented requests:

- ``forbidden``: Component does not support task execution.
- ``optional``: Component supports both synchronous and task execution.
- ``required``: Component requires task execution.


## TaskMeta

`fastmcp.utilities.tasks.TaskMeta`

```python
class TaskMeta
```

**Declared members (2)**

- `fn_key: str | None = None`  _class-attribute, instance-attribute_
- `ttl: int | None = None`  _class-attribute, instance-attribute_

Metadata for task-augmented execution requests.

Attributes:
    ttl: Client-requested TTL in milliseconds. If None, uses server default.
    fn_key: Docket routing key. Auto-derived from component name if None.


