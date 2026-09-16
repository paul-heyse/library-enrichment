# Command-line surface

565 distinct flags across both tools, parsed from the `--help` output captured at
the pinned versions. Full rows are in `content/index/cli.tsv`:

```bash
rg -P '^ruff\tcheck\t' content/index/cli.tsv | cut -f4,5,6
rg -P '\tConfig Overrides\t' content/index/cli.tsv | cut -f1,4
```

| Tool | Subcommands with flags |
|---|---:|
| `pyrefly` | 14 |
| `ruff` | 11 |

Two shapes worth knowing before reading a flag list:

- ruff groups everything under `Options`, plus a small global group repeated everywhere.
- The checker splits its flags into `Output`, `Behavior` and `Config Overrides`, and
  repeats `Config Overrides` on every subcommand. Every key in its configuration file has
  a matching override there, which is the supported way to answer a one-off question
  without editing a config.
