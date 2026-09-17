# `rich._fileno`

Distribution: `rich`

## get_fileno

Import as `rich.console.get_fileno`  ·  defined at `rich._fileno.get_fileno`

```python
def get_fileno(file_like: IO[str]) -> int | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get fileno() from a file, accounting for poorly implemented file-like objects.

Args:
    file_like (IO): A file-like object.

Returns:
    int | None: The result of fileno if available, or None if operation failed.


