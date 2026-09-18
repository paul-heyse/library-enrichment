from pathlib import Path
import datetime, hashlib, json, subprocess, time
root=Path(__file__).resolve().parent
command=['cargo','+1.98.1','test','--locked','--offline','--target-dir','/home/paul/library-enrichment/.claude/skills/deltalake/skill_improvement/evidence/.build-target','-j','4','--','--show-output','--test-threads=1']
logs=sorted(root.glob('cargo-test-*.log'))
log=root/f'cargo-test-{len(logs)+1:02}.log'
started=datetime.datetime.now(datetime.timezone.utc).isoformat(); timer=time.monotonic()
with log.open('w') as output:
 result=subprocess.run(command,cwd=root/'consumer',stdout=output,stderr=subprocess.STDOUT)
entry={'command':command,'cwd':str(root/'consumer'),'started_at':started,'elapsed_seconds':time.monotonic()-timer,'exit_code':result.returncode,'log':str(log),'log_sha256':hashlib.sha256(log.read_bytes()).hexdigest(),'source_sha256':hashlib.sha256((root/'consumer/tests/decisions.rs').read_bytes()).hexdigest()}
with (root/'commands.jsonl').open('a') as output: output.write(json.dumps(entry)+'\n')
print(json.dumps(entry));print(log.read_text()[-12000:])
