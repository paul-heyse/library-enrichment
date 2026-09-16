# Lint rules

All 970 rules ruff 0.16.7 reports, from `ruff rule --all --output-format json`.
970 of them join to a variant of `ruff_linter::codes::Rule`, which carries 970.

Three columns decide whether writing a rule code into a configuration file will do
anything at all, and none of them is visible in the code being linted:

| Status | Rules | What it means for a config file |
|---|---:|---|
| `Stable` | 812 | selectable and enforced |
| `Preview` | 141 | selected only when `preview = true`; otherwise silently inert |
| `Removed` | 17 | selecting it is an error — the rule no longer exists |

Full rows, including each rule's implementing source file and enum variant, are in
`content/index/rules.tsv`. Look a rule up by code rather than by name:

```bash
rg -P '^F401\t' content/index/rules.tsv
rg -P '\tRemoved\t' content/index/rules.tsv | cut -f1,2
```

## Rules by linter

| Prefix | Linter | Rules |
|---|---|---:|
| `PL` | Pylint | 116 |
| `RUF` | Ruff-specific rules | 79 |
| `S` | flake8-bandit | 73 |
| `` | pycodestyle | 67 |
| `PYI` | flake8-pyi | 55 |
| `UP` | pyupgrade | 49 |
| `D` | pydocstyle | 48 |
| `F` | Pyflakes | 43 |
| `B` | flake8-bugbear | 43 |
| `FURB` | refurb | 36 |
| `PTH` | flake8-use-pathlib | 35 |
| `PT` | flake8-pytest-style | 31 |
| `SIM` | flake8-simplify | 30 |
| `C4` | flake8-comprehensions | 19 |
| `ASYNC` | flake8-async | 16 |
| `N` | pep8-naming | 16 |
| `AIR` | Airflow | 13 |
| `PD` | pandas-vet | 13 |
| `ANN` | flake8-annotations | 11 |
| `YTT` | flake8-2020 | 10 |
| `DTZ` | flake8-datetimez | 10 |
| `TRY` | tryceratops | 10 |
| `TC` | flake8-type-checking | 9 |
| `G` | flake8-logging-format | 8 |
| `PIE` | flake8-pie | 8 |
| `RET` | flake8-return | 8 |
| `DJ` | flake8-django | 7 |
| `LOG` | flake8-logging | 7 |
| `TD` | flake8-todos | 7 |
| `DOC` | pydoclint | 7 |
| `PERF` | Perflint | 6 |
| `A` | flake8-builtins | 6 |
| `EXE` | flake8-executable | 5 |
| `Q` | flake8-quotes | 5 |
| `TID` | flake8-tidy-imports | 5 |
| `ARG` | flake8-unused-arguments | 5 |
| `PGH` | pygrep-hooks | 5 |
| `NPY` | NumPy-specific rules | 4 |
| `FIX` | flake8-fixme | 4 |
| `ISC` | flake8-implicit-str-concat | 4 |
| `FAST` | FastAPI | 3 |
| `FBT` | flake8-boolean-trap | 3 |
| `COM` | flake8-commas | 3 |
| `EM` | flake8-errmsg | 3 |
| `INT` | flake8-gettext | 3 |
| `ICN` | flake8-import-conventions | 3 |
| `SLOT` | flake8-slots | 3 |
| `FA` | flake8-future-annotations | 2 |
| `T20` | flake8-print | 2 |
| `I` | isort | 2 |
| `?` | - | 1 |
| `ERA` | eradicate | 1 |
| `BLE` | flake8-blind-except | 1 |
| `CPY` | flake8-copyright | 1 |
| `T10` | flake8-debugger | 1 |
| `INP` | flake8-no-pep420 | 1 |
| `RSE` | flake8-raise | 1 |
| `SLF` | flake8-self | 1 |
| `FLY` | flynt | 1 |
| `C90` | mccabe | 1 |
