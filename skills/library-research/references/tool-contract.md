# Tool routing and contracts

These are contracts for the library-enrichment service specified in this bundle. Inspect the installed service's actual schemas; do not assume a skill installs these tools.

| Need | Tool | Avoid |
|---|---|---|
| Exact release/environment | `resolve_library` | Guessing latest or upgrading a pinned project |
| Unfamiliar features | `library_overview` | Dumping all public symbols |
| Find candidates/examples | `search_evidence` | Treating no match as no capability |
| Characterize a candidate | `inspect_symbol` | Starting LSP for every signature |
| Investigate an upgrade | `compare_releases` | Looking only for breaking changes |
| Test a proposed usage | `verify_usage` | Equating type correctness with runtime behavior |
| Read large supporting output | `read_artifact` | Requesting unrestricted filesystem paths |
| Complete an expensive request | `job_control` | Treating a pending handle as evidence of success |
| Check service/tool availability | `service_status` | Repeating status before every cached read |

Every research result identifies its actual context and snapshot. `ok` is success within the stated coverage. `partial` contains usable evidence and gaps. `pending` contains a job handle. `error` includes a stable error code and next action.

Use source locators and evidence IDs from results; do not invent citations. A useful citation names the library/version, artifact or source URI, and the relevant symbol/section/line range. A research brief may include both public upstream references and service evidence handles.

For pagination, pass the returned cursor with the same snapshot, query, filters, and sort. If you change the question, start a new search. Use `read_artifact` to retrieve only the sections supporting the current decision.

For long jobs, poll the supplied job ID with bounded waits. Cancel only your request interest; the service may share the job with another agent. Report persistent errors without silently switching versions or disabling isolation.

Tools may create service-owned caches or jobs, but none should modify the working repository. `verify_usage` and analysis involving build scripts/imports are execution, even if their purpose is inspection.
