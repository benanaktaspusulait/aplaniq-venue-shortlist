# TASK-001 — Requirements and Product Decisions

## Status

To do. Phase 1 requirements and data audit exist. Product decisions are pending.

## Objective

Define a testable partner-day request and agree on the rules the implementation will use.

## Business justification

The difference between 80 and 120 attendees, catering treatment and the scope of accessible routes can change the shortlist materially.

## Dependencies

None. Read the assessment PDF and the [requirements](../requirements.md) and [data audit](../data-analysis.md).

## Requirements and scope

Confirmed inputs are the client's stated needs and preferences and the assessment's shortlist deliverable. Record the approved headcount scenario, plenary layout, minimum breakouts, candidate dates and duration, rail criterion, budget components, accessibility evidence, daylight and Wi-Fi policy, and publication of unresolved candidates. Record the shortlist limit and rank priorities, or explicitly leave them uncapped or unweighted.

## Explicitly out of scope

Rust implementation, venue verification, booking and contacting the client during the current planning phase.

## Expected files and modules

`docs/requirements.md`, `NOTES.md` and `docs/assessment-log.md`.

## Implementation guidance

Use the three questions already drafted. Record unanswered points as assumptions selected by the human, with alternatives and reasons. March 2027 has supporting clues in the dataset and is still an assumption. Define how any incomplete price, vague date or general access claim affects the shortlist. Agree the minimum CLI inputs and which rules remain fixed for this case.

## Acceptance criteria

- Given the original client message, each stated need or preference maps to an explicit requirement, preference or documented open question.
- Before implementation, `NOTES.md` records at least three decisions as question, options, selected rule and rationale. Reported client answers have an actual source. Record any remaining assumptions that can change eligibility and how to handle them.
- A baseline scenario and expected outcome for representative venue cases are concrete enough to derive tests without inferring a threshold from prose.

## Required tests and edge cases

Tabletop review of at least v019 at 80 vs 120, v017 for uncertain price and room access, and a quote below €15,000 before excluded costs. Review first eligible day versus a venue that has only an explicitly excluded Tuesday or Thursday.

## Validation commands

`git diff --check`, then inspect `git diff -- docs/requirements.md NOTES.md` and cross-check the policy against the PDF and sample cases.

## Definition of done

The human can hand Claude Code a single baseline, an explicit policy for ambiguous evidence and a concise decision record. Commit the reviewed change locally.

## Risks and unresolved assumptions

Headcount, dates and hours, interpretation of the cap, travel threshold and required accessible facilities are open. Strict interpretation of the available evidence may yield an empty confirmed shortlist.

## Estimated effort

12 minutes of active work. Waiting for external replies is excluded.

## Suggested Git commit messages

`docs: resolve shortlist policy and acceptance criteria`
