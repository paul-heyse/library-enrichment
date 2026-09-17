# `typer.exceptions`

Distribution: `typer`

## Abort

Import as `typer.Abort`  ·  defined at `typer.exceptions.Abort`

```python
class Abort(RuntimeError)
```

**Also exported as** `typer.Abort`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

An internal signalling exception that signals Typer to abort.


## Exit

Import as `typer.Exit`  ·  defined at `typer.exceptions.Exit`

```python
class Exit(RuntimeError)
```

**Also exported as** `typer.Exit`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

**Declared members (1)**

- `exit_code: int = code`  _instance-attribute_

An exception that indicates that the application should exit with some
status code.


## TyperException

Import as `typer.TyperException`  ·  defined at `typer.exceptions.TyperException`

```python
class TyperException(Exception)
```

**Also exported as** `typer.TyperException`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (3)**

- `exit_code = 1`  _class-attribute, instance-attribute_
- `def format_message(self) -> str`
- `message = message`  _instance-attribute_

A Typer-specific exception


