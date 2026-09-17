# Deliberately-wrong fixtures

Real Python files carrying the mistakes the `project-*` rules look for, so rung 13 of the ladder
has a runnable target and `verify.py`'s recipe check has something non-empty to assert against.

They are not tests. `rule-tests/` holds the precision fixtures, with both `valid:` and `invalid:`
cases. These exist so that an agent reading SKILL.md can run the scan once, see it fire, and know
what firing looks like before pointing it at a repository that matters.
