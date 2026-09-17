# Exit codes

The two tools do not share an exit vocabulary, and in one case the status carries no information at all. Both facts break scripts quietly.

| tool | command | code | means | trap |
|---|---|---|---|---|
| ast-grep | ast-grep outline | 0 | always, including an empty outline AND a missing file | Probe A007: outline exits 0 even for a path that does not exist, reporting the error only on stderr. The status carries no information; branch on stdout. |
| ast-grep | ast-grep run | 0 | at least one match |  |
| ast-grep | ast-grep run | 1 | ran successfully, no match |  |
| ast-grep | ast-grep run | 8 | the pattern or kind could not be parsed | Not 2. ast-grep does not share ripgrep's exit vocabulary. |
| ast-grep | ast-grep test | 0 | tests ran, or --update-all was used | --update-all always exits 0, so it can never gate anything. |
| ast-grep | ast-grep test | 3 | --filter matched no rule | A filter typo looks like a clean run unless you check for 3. |
| rg | rg | 0 | at least one match |  |
| rg | rg | 1 | ran successfully, no match | A clean no-match, not an error. |
| rg | rg | 2 | an error occurred | Distinct from 1. Collapsing them turns a broken invocation into a false negative. |
