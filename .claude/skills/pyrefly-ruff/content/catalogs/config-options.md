# Configuration

All 182 ruff configuration keys, from `ruff config --output-format json`.
Full rows with defaults are in `content/index/config.tsv`.

| Section | Keys |
|---|---:|
| `lint` | 142 |
| `(top level)` | 24 |
| `format` | 9 |
| `analyze` | 7 |

```bash
rg -P '^ruff\tlint\.' content/index/config.tsv | cut -f2,3
```

pyrefly's configuration is a JSON Schema in its own source tree (`schemas/pyrefly.json`)
and every key is also a CLI override under `Config Overrides` in `cli-surface.md`.
