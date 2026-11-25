# TASK-001 — Requirements and Product Decisions

## Status

Complete. The candidate reviewed and approved PD-001–007 as working assumptions. Client confirmation remains open.

## Objective

Provide a testable partner-day request and concrete rule proposals for the human to approve or revise.

## Business justification

The difference between 80 and 120 attendees, catering treatment and the scope of accessible routes can change the shortlist materially.

## Dependencies

None. Read the assessment PDF and the [requirements](../requirements.md) and [data audit](../data-analysis.md).

## Requirements and scope

Confirmed inputs are the client's stated needs and preferences and the assessment's shortlist deliverable. The [requirements](../requirements.md) hold the reference scenario, decision record and acceptance criteria. The revised policies distinguish preliminary shortlist eligibility from booking readiness.

## Explicitly out of scope

Rust implementation, venue verification, booking and contacting the client during the current planning phase.

## Expected files and modules

`docs/requirements.md`, `NOTES.md` and `docs/assessment-log.md`.

## Implementation guidance

Use the revised PD-001–007 for implementation. Attribute any later client answers separately. The three draft questions remain unsent.

## Acceptance criteria

- Given the original client message, each stated need or preference maps to an explicit requirement, preference or documented open question.
- The review draft records at least three proposals as question, alternatives, recommended rule and rationale. Before implementation, record the actual choices and actual sources of any client answers.
- A baseline scenario and expected outcome for representative venue cases are concrete enough to derive tests without inferring a threshold from prose.

## Required tests and edge cases

Use AC-01–15 in the requirements, including the v019 regression at 80 and 120.

## Validation commands

`git diff --check`, then inspect `git diff -- docs/requirements.md NOTES.md` and cross-check the policy against the PDF and sample cases.

## Definition of done

The candidate reviewed the baseline and adopted the revised working policies. The requirements record the assumptions and acceptance cases, enabling TASK-002.

## Risks and unresolved assumptions

The decisions are working assumptions. Final headcount, dates and hours, cost scope and booking-level access still require verification.

## Estimated effort

12 minutes of active work. Waiting for external replies is excluded.

## Suggested Git commit messages

`docs: resolve shortlist policy and acceptance criteria`
