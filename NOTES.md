# Assessment notes

## Spec and assumptions

The client is planning a mid-March partner day for about 80 people, possibly 120. Thursday is preferred and Tuesday is acceptable. The venue needs train access, a plenary plus three to four breakouts and wheelchair access. The client wants good Wi-Fi and a brighter venue. The budget is about €15,000 and catering may be separate.

For review, March 2027 is the likely year based on the export date and listings. The dates and hours, attendance threshold, room setup and cost basis still need a decision. Phase 1 produced a scaffold, requirements and architecture. The assignment excludes UI, API, auth, deployment, AWS and databases.

Questions to ask, pending human review:

1. Which mid-March dates and hours should we hold, must we plan for 120 now, and what plenary setup and number of simultaneous breakouts are essential?
2. Is €15,000 a firm ceiling including VAT, catering and mandatory services? If catering is separate, what must the venue quote include?
3. Which wheelchair routes and facilities must be confirmed, from arrival through the plenary, breakouts and toilets, for a venue to appear in the shortlist?

## Product decisions

| Question | Options to review | Choice and rationale |
| --- | --- | --- |
| What headcount should the shortlist serve? | 80 baseline with a 120-person contingency, 120 hard minimum, or two distinct scenarios. | Pending. |
| How should incomplete costs affect inclusion? | Flag for quote confirmation, exclude until priced, or use an explicitly approved estimate. | Pending. |
| What evidence establishes wheelchair access? | Require route and room-level confirmation or accept a general access claim with a stated caveat. | Pending. |
| How should near-duplicate listings be handled? | Flag for review or consolidate under a documented identity and evidence rule. | Pending. |

## Review

Codex produced the Phase 1 scaffold and planning documents. Claude Code implementation, implementation review and steering are pending. Record concrete findings, how fixes were directed, any manual edits and the strongest diff here when they exist. See the [assessment log](docs/assessment-log.md) for elapsed time and checks.

Proposed next work is to settle the open criteria, record the Claude Code plan, and build the ingest-to-shortlist path with focused tests. Keep the completed notes within two pages.
