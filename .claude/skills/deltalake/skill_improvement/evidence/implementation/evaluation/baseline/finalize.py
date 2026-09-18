from pathlib import Path
import collections,datetime,hashlib,json,re
OUT=Path(__file__).resolve().parent
responses=json.loads((OUT/'responses.json').read_text())
roots={k:Path(v) for k,v in responses['source_roots'].items()}
commands=[json.loads(line) for line in (OUT/'commands.jsonl').read_text().splitlines()]
checks=[c for c in commands if c['command'][0]=='cargo']
now=datetime.datetime.now(datetime.timezone.utc)
started=datetime.datetime.fromisoformat(checks[0]['started_at'])
source_map=json.loads((OUT/'source-map.json').read_text())
materials={ref['path'] for task in source_map['tasks'] for ref in task['references']}
additional={
'baseline':['content/topics/00-map.md','content/topics/writing.md','content/topics/transactions.md','content/topics/storage.md','content/topics/schema-and-types.md','content/topics/catalogs.md','content/topics/maintenance.md','content/api/deltalake_core.delta_datafusion.md','content/api/deltalake_core.delta_datafusion.session.md','content/index/symbols.tsv','queries/sgconfig.yml'],
'cargo':['crates/core/src/table/mod.rs','crates/core/src/operations/mod.rs','crates/core/src/kernel/snapshot/mod.rs','crates/core/src/delta_datafusion/planner.rs','crates/core/src/logstore/mod.rs'],
'output':['consumer/Cargo.toml','consumer/Cargo.lock','consumer/tests/decisions.rs','build_responses.py','run_tests.py']}
for root,items in additional.items():
 for rel in items:materials.add(str(roots[root]/rel))
materials.add(str(OUT.parent/'tasks.json'))
manifest=[]
for path in sorted(materials):
 p=Path(path)
 assert p.exists(),str(p)
 manifest.append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size})
(OUT/'materials.json').write_text(json.dumps({'scope':'Pages/source files read or searched directly and evaluation inputs/scripts. Reading a source/test is not executing it. Paths under Cargo cache were used only at the supplied exact pin.','files':manifest,'additional_search_scopes':['Exact-pin crates/core/src tree searched for public update_datafusion_session definition','Exact-pin delta_datafusion/table_provider/next searched for column_mapping test names','Frozen baseline content/corpus file inventory and command_merge.rs duplicate test names','Frozen baseline content/index/symbols.tsv capability lookup','Frozen baseline queries/sgconfig.yml and its project rules executed by ast-grep']},indent=2)+'\n')
findings=dict(collections.Counter(re.findall(r'^help\[([^]]+)\]',(OUT/'capability-gap.log').read_text(),re.M)))
scan={'command':['ast-grep','scan','-c',str(roots['baseline']/'queries/sgconfig.yml'),'--filter','^project-','consumer'],'cwd':str(OUT),'started_at':None,'elapsed_seconds':None,'exit_code':0,'log':str(OUT/'capability-gap.log'),'log_sha256':hashlib.sha256((OUT/'capability-gap.log').read_bytes()).hexdigest(),'source_sha256':checks[-1]['source_sha256'],'findings':findings}
if not any(c['command'][0]=='ast-grep' for c in commands):
 with (OUT/'commands.jsonl').open('a') as output:output.write(json.dumps(scan)+'\n')
metrics={
'evaluation':'blind_baseline', 'completed_at':now.isoformat(), 'task_count':38, 'decision_responses':38,
'code_required_count':8, 'consumer_runtime':{'passed':8,'failed':0,'blocked':0,'not_run':0},
'non_code_decisions':{'count':30,'runtime_status':'not_run','support':'source inspection and bounded inferences; not behavioral certification'},
'cargo_attempts':len(checks),'cargo_total_elapsed_seconds':sum(c['elapsed_seconds'] for c in checks),
'first_instrumented_event':checks[0]['started_at'], 'elapsed_seconds_from_first_instrumented_event':(now-started).total_seconds(),
'elapsed_seconds_total':None,'elapsed_time_note':'Initial reference reading was not timestamped; parent confirmed spawn time was not recorded. The measured interval is a lower bound on total evaluation elapsed time, not a substitute estimate.',
'context_tokens':None,'context_tokens_note':'No trustworthy context-token instrumentation was available; none invented.',
'observed_tool_calls':{'functions_exec_wrappers':25,'nested_exec_command':24,'nested_write_stdin':2,'collaboration_send_message':1,'count_scope':'This independent agent through the finalization command, manually tallied from visible tool calls.'},
'no_network':True,'live_cloud_changes':False,'runtime_storage':'Disposable in-memory Delta tables only',
'cache_source_usage':True,'cache_source_note':'Exact delta-rs Cargo checkout 58f07cd and DataFusion 55.1.0 registry source inspected; detailed paths/digests in materials.json.',
'final_source_sha256':checks[-1]['source_sha256'],'final_log_sha256':checks[-1]['log_sha256'],
'failure_history':[
{'attempt':1,'stage':'compile','exit_code':101,'observations':['Private LogStoreExt import rejected','Awaited provider already returns Arc<dyn TableProvider>; extra Arc layer rejected'],'repair':'Removed private trait import and redundant Arc wrappers; runtime assertions unchanged.'},
{'attempt':2,'stage':'runtime','exit_code':101,'passed':6,'failed':2,'observations':['Caller ordinary DataFusion planner cannot lower Delta MetricObserver','CDF commit-version output was not an Int64Array; direct downcast panicked'],'repair':'Kept the ordinary-planner failure as an explicit negative assertion; cloned caller SessionState and installed DeltaPlanner while preserving runtime/config/UDF. Decoded displayed commit-version scalar to i64 and kept exact expected version/value/change-type assertions.'},
{'attempt':3,'stage':'runtime','exit_code':0,'passed':8,'failed':0,'note':'All eight required task tests ran; no test was removed or ignored.'}],
'capability_gap_scan':{'exit_code':0,'findings':findings,'interpretation':'Help-level syntax suggestions on deliberate fixed-schema/non-retried fixtures, including false-positive method chains. No DeltaOps use. These findings are not semantic defects or evidence of replay safety.'},
'baseline_guidance_corrections':[
'Application transaction markers do not automatically suppress sequential replay: verified by runtime test.',
'Baseline DML topic says duplicate source matches never error; exact-pin MergeBuilder source has a duplicate relevant WHEN MATCHED validation failure test. Source normalization is still required.',
'Concrete session preservation must also supply Delta extension planning: plain planner runtime failure retained and compatible planner path executed.'
]}
(OUT/'metrics.json').write_text(json.dumps(metrics,indent=2)+'\n')
readme='''# Blind baseline evaluation — 2026-09-18

Completed 38 decision responses; all eight required consumer runtime tests pass.
This is an evaluation of the frozen baseline reference, not project acceptance certification.

- `responses.json`: candidates, selected conditions, input/output/effect contracts, sources and unknowns for each task.
- `source-map.json`: per-task source paths/digests and consumer test locations.
- `materials.json`: consulted material and supplied-input digests; Cargo-cache source use is explicit.
- `consumer/tests/decisions.rs`: eight executable runtime tests.
- `commands.jsonl`: exact Cargo invocations, durations, exit codes, source/log digests; capability-gap scan invocation.
- `cargo-test-01.log`: initial compile failure (private trait and redundant provider Arc).
- `cargo-test-02.log`: six passes and two runtime failures (Delta planner requirement and CDF metadata representation).
- `cargo-test-03.log`: final eight passes.
- `metrics.json`: truthful counters, failure history and measured timing interval. Total elapsed and context tokens were not instrumented.
- `capability-gap.log`: 36 help-level syntax suggestions retained for review, not treated as acceptance failures.

The final tests prove refreshed-provider behavior, incremental projection, caller-plan execution with retained UDF/store environment, predicate overwrite validation, serialized replay guarding and marker-only duplicate behavior, CDF row changes, restore history, and explicit stale-store replacement. They do not prove concurrent exactly-once delivery, cloud integration, every table feature, every schema conversion, or vacuum safety.

Reproduce from `consumer/`:

```sh
cargo +1.98.1 test --locked --offline --target-dir /home/paul/library-enrichment/.claude/skills/deltalake/skill_improvement/evidence/.build-target -j 4 -- --show-output --test-threads=1
```

All Delta data used by runtime tests was in memory. Only the assigned output directory and the explicitly authorized shared build target were written. The active skill, planning/assessment/design evidence, sibling skills, other evaluation responses, and expected answers were not consulted. No network or live cloud operation was used.
'''
(OUT/'README.md').write_text(readme)
# Validate task coverage and every recorded source digest without rerunning successful tests.
expected=json.loads((OUT.parent/'tasks.json').read_text())['tasks']
assert {t['id'] for t in expected}=={t['id'] for t in responses['responses']}
assert sum(t['code_required'] for t in responses['responses'])==8
assert all(t['verification']['status']=='passed' for t in responses['responses'] if t['code_required'])
assert hashlib.sha256((OUT/'consumer/tests/decisions.rs').read_bytes()).hexdigest()==checks[-1]['source_sha256']
assert 'test result: ok. 8 passed; 0 failed' in (OUT/'cargo-test-03.log').read_text()
for item in manifest:assert hashlib.sha256(Path(item['path']).read_bytes()).hexdigest()==item['sha256']
for task in source_map['tasks']:
 for ref in task['references']:assert hashlib.sha256(Path(ref['path']).read_bytes()).hexdigest()==ref['sha256']
print(json.dumps({'decisions':38,'tests_passed':8,'cargo_attempts':len(checks),'materials':len(manifest),'elapsed_seconds_from_first_instrumented_event':metrics['elapsed_seconds_from_first_instrumented_event'],'elapsed_seconds_total':None,'validated':'coverage, final log, source digests and source map','output':str(OUT)},indent=2))
