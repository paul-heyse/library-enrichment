---
id: ADR-0015
title: Acquire explicit immutable repository revisions
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-08, DM-14, DM-20, DM-32, DM-43]
design: [§3.1, §3.2, §4.3, §5.1]
review: docs/design_review/reviews/design_review_immutable-revision_2026-09-13.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A consumer requires another repository provider or automatic workspace package selection.
verification: revision_fixture; revision_identity; archive_policy; schema-conformance; gate-phase 3
---

# ADR-0015: Acquire explicit immutable repository revisions

## Context

Revision mode needs its own acquisition identity. A package's manifest version at a commit does
not prove that the commit is the published release; an archive wrapper is not a commit identity.

## Decision

Add optional repository, revision and package_subdir fields to the Rust resolve request and
its generated projections. Require revision mode, a full 40-hex commit and a canonical public
HTTPS github.com owner/repository URL; reject branches, credentials, query/fragment/port and
unsafe roots. Version and revision forms are exclusive. The package root defaults to repository
root and must contain a matching Cargo.toml or pyproject.toml project name; never select the
first nested package. Missing/dynamic identity is an explicit unsupported/ambiguous result.

Rust validates GitHub commit metadata's SHA before downloading the same SHA's archive. The
HTTP client applies existing endpoint/redirect/body policy; send the verified API version.
Archive extraction applies member, expansion, path, collision and link/device limits. Global
PAX metadata is bounded and restricted to a corroborating commit comment; local path extensions
remain subject to the extractor's effective-path validation. Strip one verified common wrapper.
All extraction and worker paths are service owned and cleaned on normal success or failure.

The release key contains the commit as version, the downloaded artifact digest, and a registry
identity containing the canonical repository and explicit package root. Package manifest version
is a declaration, never a released-package association. Store a separate acquisition receipt
with commit/tree identity, repository/root, actual archive digest and HTTP source/validator data.
Offline replay selects this exact registry identity; it cannot reuse another repository's commit
or a published package with the same name. Revalidation may create a different artifact identity.

Python source trees reuse the static worker with an explicit source-tree qualification rather
than requiring wheel metadata. Report inferred source layout and missing build/generated/LFS/
submodule/exported contents. Rust static revision snapshots expose stored source and declared
configuration/docs with missing API coverage; API compilation requires the later build profile.
No target execution, Git clone, dependency install, implicit release JSON borrowing or user repo
working directory is introduced. Existing snapshot table/envelope versions remain readable.

## Evidence

| Source | Retrieved | Requirement |
|---|---|---|
| [GitHub commit](https://docs.github.com/en/rest/commits/commits#get-a-commit) | 2026-09-13 | Commit response has resolved sha; files is a diff rather than a revision inventory |
| [Tar endpoint](https://docs.github.com/en/rest/repos/contents#download-a-repository-archive-tar) | 2026-09-13 | Explicit ref, redirect, no subdirectory selector |
| [Archive stability](https://docs.github.com/en/repositories/working-with-files/using-files/downloading-source-code-archives#stability-of-source-code-archives) | 2026-09-13 | Commit contents and compressed archive identity are separate |
| docs/architecture/compatibility-matrix.md, Phase 3 immutable GitHub revision acquisition | 2026-09-13 | Live exact-SHA probe, PAX comment, API 2026-03-10, external-content limitations |

## Consequences

The first provider is public GitHub. Unsupported hosts and dynamic package identities fail
explicitly. Static revision evidence remains useful without claiming installed or compiled API
completeness. Additional providers and automatic workspace resolution require a later decision.

## Verification

Actual HTTP/archive/worker/MCP fixture: pinned commit, monorepo root, branch and wrong-SHA
rejection, source versus release separation, two repository identities, offline replay, PAX
metadata, hostile archive and unchanged canary repository. Existing acceptance gates stay intact.

## Boundaries preserved

Rust identity/acquisition/publication authority, separate static Python worker, immutable evidence,
six epistemic classes, no project mutations and explicit execution profiles remain unchanged.

## Status history

- 2026-09-13 — accepted at Proposed strength following scoped contract review.
