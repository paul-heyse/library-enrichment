# `fastmcp.utilities.timeout`

Distribution: `fastmcp`

## normalize_timeout_to_seconds

Import as `fastmcp.client.client.normalize_timeout_to_seconds`  ·  defined at `fastmcp.utilities.timeout.normalize_timeout_to_seconds`

```python
def normalize_timeout_to_seconds(value: int | float | datetime.timedelta | None) -> float | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Normalize a timeout value to seconds (float).

Args:
    value: Timeout value as int/float (seconds), timedelta, or None.
        Zero values are treated as "disabled" and return None.

Returns:
    float seconds if value provided and non-zero, None otherwise


## normalize_timeout_to_timedelta

Import as `fastmcp.client.transports.sse.normalize_timeout_to_timedelta`  ·  defined at `fastmcp.utilities.timeout.normalize_timeout_to_timedelta`

```python
def normalize_timeout_to_timedelta(value: int | float | datetime.timedelta | None) -> datetime.timedelta | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Normalize a timeout value to a timedelta.

Args:
    value: Timeout value as int/float (seconds), timedelta, or None

Returns:
    timedelta if value provided, None otherwise


