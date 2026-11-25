# AplanIQ Venue Shortlist implementation backlog

This is the implementation backlog. It is based on both pages of the original assessment, the supplied `venues.json` and the Phase 1 [requirements](../requirements.md), [architecture](../architecture.md) and [data audit](../data-analysis.md). TASK-001 is complete with candidate-approved working assumptions. TASK-002 is complete. TASK-003 is next.

## Backlog and execution order

| Task | Deliverable | Direct dependencies | Estimate |
| --- | --- | --- | --- |
| [TASK-001](TASK-001-requirements.md) | Requirements, product decisions and reference event scenario | None | 12 min original estimate |
| [TASK-002](TASK-002-domain-model.md) | Rust domain, evidence, error and report types | 001 | 8 min |
| [TASK-003](TASK-003-normalisation.md) | JSON ingestion and normalisation of observed shapes | 002 | 30 min |
| [TASK-004](TASK-004-deduplication.md) | Conservative duplicate handling and conflicting evidence | 003 | 10 min |
| [TASK-005](TASK-005-eligibility.md) | Hard requirements and criterion-level results | 003, 004 | 27 min |
| [TASK-006](TASK-006-ranking.md) | Deterministic ranking and recommendation reasons | 005 | 9 min |
| [TASK-007](TASK-007-cli.md) | Local CLI, report and first documented invocation | 006 | 13 min |
| [TASK-008](TASK-008-validation.md) | Integration and original-dataset validation | 007 | 12 min |
| [TASK-009](TASK-009-code-review.md) | Independent evidence review and targeted fixes | 008 | 12 min |
| [TASK-010](TASK-010-submission.md) | Final checks and private-submission handoff | 009 | 8 min |

```mermaid
flowchart LR
    T001["001 Requirements"] --> T002["002 Domain"]
    T002 --> T003["003 Normalisation"]
    T003 --> T004["004 Deduplication"]
    T003 --> T005["005 Eligibility"]
    T004 --> T005
    T005 --> T006["006 Ranking"]
    T006 --> T007["007 CLI"]
    T007 --> T008["008 Validation"]
    T008 --> T009["009 Review"]
    T009 --> T010["010 Submission"]
```

Follow numerical order for completed deliverables. Once TASK-001 defines the scenario and policies, the integration fixture and expected results can be drafted alongside the core work. Once the report contract is agreed, CLI argument and rendering tests can be drafted before the full pipeline is wired. Complete and verify each task after its dependencies.

## Assessment traceability

| Assessment requirement | Owning tasks | Evidence at handoff |
| --- | --- | --- |
| Given an event request, recommend a ranked shortlist meeting the client's hard requirements | 001, 005, 006, 008 | Approved policy and request, tested rejection and confirmation, deterministic ranks with explanations |
| Small Rust CLI or library with tests, using the supplied messy data | 002, 003, 004, 007, 008 | Runnable Rust entry point, unchanged source data, tests for irregular fields, near-duplicates and contradictions |
| Run locally with one documented command and pass `cargo test` | 007, 008, 010 | README invocation and recorded test and smoke results |
| `NOTES.md` is at most two pages, with Spec and assumptions, Product decisions and Review | 001, 009, 010 | Client facts, assumptions, proposed questions and exclusions, at least three decisions with alternatives and rationale, observed findings, steering and strongest diff |
| Let agents write implementation, use Claude Code, prefer GSD or use another harness, and disclose any small hand edits | 002–008, 009, 010 | Actual agent and harness record, diff review and disclosed manual edits if any |
| Commit the actual agent planning artifacts and retain genuine history, including dead ends | 001–010 | Tracked planning artifacts, task evidence, actual attempts and follow-up commits |
| Work at most three hours in one or two sittings within one week. At the limit, stop and log approximate time and next steps | 001–010, finalised in 010 | Dated assessment log and concise notes on unfinished work |
| Questions are optional, with up to three questions to Joa by email. Log assumptions for unanswered points | 001 | At most three selected questions and attributed answers if sent, plus numbered assumptions when facts are unavailable |
| Exclude UI, API, auth, deployment, AWS and real databases | 002, 007, 009, 010 | Local-file scope checked in code and final review |
| Submit a private repository link shared with Joa, containing code, notes and agent artifacts | 010 | Private link and verified recipient access after user authorisation |

## Open decisions and delivery risks

- Use [PD-001–007 and AC-01–15](../requirements.md). The export is unverified. Keep provisional eligibility separate from booking readiness. A controlled fixture and the v019 regression demonstrate the intended shortlist behaviour.
- The PDF requires Claude Code. The candidate directed Codex to implement instead and the deviation is documented in the assessment log. No GSD planning directory existed at the planning handoff. The final push and sharing are a separate external milestone and are outside this planning run.

## Boundaries and time budget

TASK-003 parses evidence, TASK-004 handles identity and conflicts, TASK-005 decides eligibility and TASK-006 ranks confirmed candidates. TASK-007 renders what the core decided. Test each implementation task as it lands. TASK-008 checks the assembled path and the real input. TASK-009 reviews observed code and test evidence. TASK-010 assembles and verifies the final delivery.

The tasks cover the assessment without a separate dependency-selection milestone, duplicate testing project or infrastructure work. The gaps are product policy, the executable feature, genuine Claude Code planning artifacts, verified review and authorised delivery.

The original active-work estimate is **141 minutes**, including **109 minutes** for TASK-002–008 implementation and validation. TASK-003–010 total 121 estimated minutes after TASK-002. The first recorded Phase 1 check through final verification took **3 minutes 59 seconds**. Count Phase 2, TASK-001 and any other assessment work against the same 180-minute cap. See the [assessment log](../assessment-log.md) for measured intervals. Protect the last 20 minutes for review and final verification, already included in the 141-minute estimate. If pressure increases, reduce parser breadth for undocumented formats, cosmetic output work and optional follow-up display. Protect the working command, hard filters, ranked results, focused tests and honest notes. At the limit, stop and record what remains.

## Agent workflow

Before starting a task, read it and its dependencies, inspect the current code, confirm scope and acceptance criteria, and write a focused plan. Record the actual implementing agent and add relevant tests. The candidate selected Codex for this workspace. Run the listed checks, examine the actual diff and acceptance evidence, and correct observed issues. Record the test results, references and task status in the assessment log and active plan. Mark a task complete only when its acceptance evidence exists.

Use these task IDs and links in agent planning. If GSD is chosen, import or reference this backlog when creating the genuine plan. Maintain execution status in that plan once adopted. Until then, maintain status in the individual task files and link to the assessment log for evidence. Record the selected plan location so there is one source for live status. Commit the actual artifacts emitted by the selected workflow.

Commit each meaningful, coherent local change. Review `git status` and the staged diff, run relevant checks, and keep unrelated staged changes out of the commit. Retain failed attempts and their corrections in the log and any existing commits. Verify against the actual diff and test output before recording completion.
