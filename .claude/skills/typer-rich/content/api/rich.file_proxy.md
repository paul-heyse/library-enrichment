# `rich.file_proxy`

Distribution: `rich`

## FileProxy

`rich.file_proxy.FileProxy`

```python
class FileProxy(io.TextIOBase)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `io.TextIOBase`

**Declared members (5)**

- `def fileno(self) -> int`
- `def flush(self) -> None`
- `def isatty(self) -> bool`
- `rich_proxied_file: IO[str]`  _property_
  Get proxied file.
- `def write(self, text: str) -> int`

Wraps a file (e.g. sys.stdout) and redirects writes to a console.


