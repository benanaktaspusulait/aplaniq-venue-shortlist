# Assessment notes

## Spec and assumptions

The client wants a rail-accessible partner-day venue with a plenary, three to four breakouts, wheelchair access, acceptable daylight and Wi-Fi. They expect about 80 people and may reach 120. Thursday is preferred and Tuesday is accepted. The approximate €15,000 budget might exclude catering.

The candidate accepted 80 seated attendees, at least three simultaneous usable breakouts and 11, 16 or 18 March 2027 as working values. The export is unverified. Shortlist evidence supports comparison, with route, toilet, quote and reservation details to be checked before booking. Ask Joa about dates, headcount and setup, budget inclusions and required access facilities. The three questions are [recorded in English](docs/requirements.md#three-proposed-client-questions) and remain unsent. UI, API, auth, deployment, AWS, databases and booking are outside the CLI.

## Product decisions

These choices were adopted by the candidate for the exercise. Joa did not confirm them. See the [full decision record](docs/requirements.md#working-product-decisions).

- **Attendance.** Rank for 80 and show the 120 verdict. Requiring capacity for 120 would remove venues that fit the expected headcount.
- **Budget.** Use an approximate €15,000 all-in ceiling and assess both counts separately. Compared with a venue-only cap, this keeps mandatory costs visible. Incomplete quotes need confirmation.
- **Access.** Accept a positive listing flag for preliminary eligibility and check detailed facilities before booking. Room-by-room proof at shortlist time would discard useful leads. Keep documented access violations as rejection reasons.
- **Travel and quality.** Prefer a short walk from Utrecht Centraal and consider documented connections. A hard 15-minute cutoff would discard viable alternatives. Reject explicitly unacceptable daylight or Wi-Fi and flag uncertain evidence.
- **Ranking and duplicate listings.** Keep source evidence and apply ordered preferences and a stable tie-break. Compared with merging on name and using a weighted score, this preserves adverse facts and exposes the ordering.

## Review

The candidate revised the access and transport rules to distinguish shortlist eligibility from booking readiness and directed Codex to implement, a deviation from the PDF's Claude Code requirement. R1 preserved independent catering and minimum spend evidence. R2 fixed silent JSON overwrites and lost room qualifiers. Three venues pass at 80 and only Fabriek 1912 passes at 120. R3 caught a mandatory fee omission. The corrected native CLI is verified. See the [assessment log](docs/assessment-log.md).

The assessment was stopped at approximately 22:22 GMT, about 3 hours 22 minutes after the reported start. Logged task intervals account for 1 hour 56 minutes 30 seconds before TASK-008. The full active total could not be reconstructed. Given further time, I would run the clean-checkout major gate and Docker smoke test, review any end-to-end coverage gaps, and complete the final review and handoff. See the [validation status](docs/tasks/TASK-008-validation.md#validation-matrix).
