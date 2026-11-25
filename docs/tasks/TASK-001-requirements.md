# TASK-001 — Requirements and Product Decisions

## Status

Ready for human review. PD-001–007 are proposed. Approval is pending and TASK-002 has not started.

## Objective

Provide a testable partner-day request and concrete rule proposals for the human to approve or revise.

## Business justification

The difference between 80 and 120 attendees, catering treatment and the scope of accessible routes can change the shortlist materially.

## Dependencies

None. Read the assessment PDF and the [requirements](../requirements.md) and [data audit](../data-analysis.md).

## Requirements and scope

Confirmed inputs are the client's stated needs and preferences and the assessment's shortlist deliverable. The [requirements](../requirements.md) hold the reference scenario, decision record and acceptance criteria. Resolve the pending policy choices there before releasing TASK-002.

## Explicitly out of scope

Rust implementation, venue verification, booking and contacting the client during the current planning phase.

## Expected files and modules

`docs/requirements.md`, `NOTES.md` and `docs/assessment-log.md`.

## Implementation guidance

Review the three draft questions and PD-001–007. Track client answers separately from human-approved engineering assumptions. Freeze the reference request and minimum CLI inputs after approval.

## Acceptance criteria

- Given the original client message, each stated need or preference maps to an explicit requirement, preference or documented open question.
- The review draft records at least three proposals as question, alternatives, recommended rule and rationale. Before implementation, record the actual choices and actual sources of any client answers.
- A baseline scenario and expected outcome for representative venue cases are concrete enough to derive tests without inferring a threshold from prose.

## Required tests and edge cases

Review AC-01–14 in the requirements. Verify the fixture assumptions and expected outcomes before sending the task to Claude Code.

## Validation commands

`git diff --check`, then inspect `git diff -- docs/requirements.md NOTES.md` and cross-check the policy against the PDF and sample cases.

## Definition of done

The draft is ready for review with a single baseline and testable proposals. Mark the task complete and unblock TASK-002 only after the human approves or revises the policy and the final decisions are recorded.

## Risks and unresolved assumptions

Headcount, dates and hours, interpretation of the cap, travel threshold and required accessible facilities are open. Strict interpretation of the available evidence may yield an empty confirmed shortlist.

## Estimated effort

12 minutes of active work. Waiting for external replies is excluded.

## Suggested Git commit messages

`docs: resolve shortlist policy and acceptance criteria`
