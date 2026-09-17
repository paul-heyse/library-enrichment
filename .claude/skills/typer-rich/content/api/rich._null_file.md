# `rich._null_file`

Distribution: `rich`

## NULL_FILE

Import as `rich.console.NULL_FILE`  ·  defined at `rich._null_file.NULL_FILE`

```python
NULL_FILE = NullFile()
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## NullFile

Import as `rich.logging.NullFile`  ·  defined at `rich._null_file.NullFile`

```python
class NullFile(IO[str])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `IO[str]`

**Declared members (15)**

- `def close(self) -> None`
- `def fileno(self) -> int`
- `def flush(self) -> None`
- `def isatty(self) -> bool`
- `def read(self, __n: int = 1) -> str`
- `def readable(self) -> bool`
- `def readline(self, __limit: int = 1) -> str`
- `def readlines(self, __hint: int = 1) -> List[str]`
- `def seek(self, __offset: int, __whence: int = 1) -> int`
- `def seekable(self) -> bool`
- `def tell(self) -> int`
- `def truncate(self, __size: Optional[int] = 1) -> int`
- `def writable(self) -> bool`
- `def write(self, text: str) -> int`
- `def writelines(self, __lines: Iterable[str]) -> None`

