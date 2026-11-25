# Assessment notes

## Spec and assumptions

The partner day needs a plenary, three to four breakouts, wheelchair access and a venue near Utrecht Centraal or easy to reach by train. The client expects about 80 with a possible rise to 120, prefers Thursday but accepts Tuesday, and wants to avoid poor Wi-Fi and dark rooms. The approximate €15,000 budget may exclude catering.

The proposed reference event is a seated, one-day meeting on 11, 16 or 18 March 2027, with at least three simultaneous usable breakouts. The date window, seating proxy, cost basis and evidence thresholds require approval. Ask Joa to confirm the date, capacity and room setup, the cost ceiling and its inclusions, and the required accessible routes and facilities. The [requirements](docs/requirements.md) contain the three email-ready questions. No questions were sent. UI, API, auth, deployment, AWS, databases and booking are out of scope.

## Product decisions

These recommendations are pending human approval. The [decision record](docs/requirements.md#proposed-product-decisions) has the full alternatives, reasons and tests.

- **Attendance.** Considered an 80-only shortlist and a 120 minimum. Propose ranking for 80 and showing the 120-person verdict so the growth risk is visible.
- **Budget.** Considered a venue-only limit. Propose a €15,000 all-in cap with follow-up for incomplete quotes, because missing charges can change eligibility.
- **Access.** Considered a general venue-level claim. Propose evidence for the required rooms and routes, with uncertain cases in follow-up, because attendees need to use those spaces.
- **Availability and travel.** Considered all of March and any nearby station. Propose a bounded Tuesday/Thursday window and documented rail access to make both tests specific.
- **Duplicates and ranking.** Considered merging by name and a weighted score. Propose retaining conflicts and applying ordered preferences so the planner can verify the reasons and warnings.

## Review

Codex prepared the scaffold, backlog and review-ready specification. Claude Code implementation and review are pending. The [assessment log](docs/assessment-log.md) records actual work and elapsed time. After approval, record the genuine Claude Code plan and start TASK-002. Add observed review findings, steering, any manual fixes and the strongest diff here when available.
