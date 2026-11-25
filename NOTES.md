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

The candidate clarified access and transport policy and directed Codex to implement after Claude Code was unavailable as an authenticated local session. This deviates from the PDF's agent requirement. The strongest review diff is `1bac90c`. The R2 fix prevents duplicate JSON fields from silently overwriting adverse evidence and preserves valid neighbouring rows. R3 corrected a mandatory-fee omission. Three venues pass at 80 and only Fabriek 1912 passes at 120.

The assessment stopped at about 22:22 GMT, roughly 3 hours 22 minutes after the reported start. Logged intervals account for 1 hour 56 minutes 30 seconds. Full active time could not be reconstructed. At the stop, clean-checkout, Docker and final review checks remained open. This branch contains a separately requested follow-up that addresses those checks and fixes a fee omission when an optional item shares the same sentence. No manual code edits were recorded in the follow-up. The [assessment log](docs/assessment-log.md) separates the original snapshot and follow-up.
