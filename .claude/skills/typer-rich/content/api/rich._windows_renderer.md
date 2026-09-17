# `rich._windows_renderer`

Distribution: `rich`

## legacy_windows_render

`rich._windows_renderer.legacy_windows_render`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def legacy_windows_render(buffer: Iterable[Segment], term: LegacyWindowsTerm) -> None
```

Makes appropriate Windows Console API calls based on the segments in the buffer.

Args:
    buffer (Iterable[Segment]): Iterable of Segments to convert to Win32 API calls.
    term (LegacyWindowsTerm): Used to call the Windows Console API.


