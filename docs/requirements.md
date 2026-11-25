# Venue Shortlist requirements

Source: Aplaniq Engineer Assignment, pp. 1–2. The classifications below reflect the fictional client message. PD-001–007 are candidate-approved working assumptions following human review. No answers have been received from Joa. Shortlist eligibility is preliminary and uses the unverified export. Booking readiness requires independent verification of the venue, access, quote and slot.

## Client requirements

| ID | Topic | Classification | What the client said or left open |
| --- | --- | --- | --- |
| REQ-01 | Attendance | Ambiguous requirement | About 80, possibly 120, with the count known in January. The shortlist's planning threshold is open. |
| REQ-02 | Plenary and breakout rooms | Confirmed hard requirement | One plenary and three to four breakout rooms. Seating layout, concurrent use, and whether four are essential need clarification. |
| REQ-03 | Budget, catering and VAT | Ambiguous requirement | Approximately €15,000 all-in, though catering might be separate. A firm ceiling, included services and VAT treatment are unspecified. |
| REQ-04 | Date window and duration | Ambiguous requirement | Probably mid-March. The year, exact dates and hours are unspecified. |
| REQ-05 | Weekday | Soft preference | Thursday is preferred. Tuesday is expressly acceptable. Other weekdays have no affirmative support in the message. |
| REQ-06 | Rail access | Confirmed hard requirement | Near Utrecht Centraal or easy by train for people coming from Amsterdam and Eindhoven. The limits for both alternatives are open. |
| REQ-07 | Wheelchair access | Confirmed hard requirement | Several attendees use wheelchairs. The client has yet to specify the facilities, routes and evidence needed. |
| REQ-08 | Natural light and Wi-Fi | Quality requirement with ambiguous threshold | The client wants to avoid a repeat of last year. The working decision rejects explicitly unacceptable evidence and ranks the strength of acceptable evidence. |
| REQ-09 | Reply this week | Out of scope for the CLI | A service deadline for the planner. The CLI produces a local report. |
| ASM-01 | Reference values and evidence policy | Candidate-approved assumption | The headcounts, dates, thresholds, evidence rules and ranking order appear below. |

The assessment requires a small agent-driven Rust CLI or library with tests that reads the supplied data and returns a ranked shortlist meeting hard requirements. The selected delivery is a local CLI with one documented command and passing `cargo test`. The PDF requires Claude Code and asks for genuine planning artifacts and honest history. GSD is preferred. Earlier business implementation used Codex at the candidate’s direction. This deviated from the Claude Code requirement and must be disclosed. Future business implementation and corrections use Claude Code. Record manual edits if any. Keep `NOTES.md` within two pages, with the required three sections and at least three actual product decisions. The budget is three hours in one or two sittings within a week. At the limit, record the time spent and next steps. The eventual handoff is a private Git repository link shared with Joa under the user's authorisation.

## Working product decisions

All seven decisions are candidate-approved assumptions for the assessment. They are awaiting client confirmation.

### PD-001 — Attendance and room fit

- Assess 80 seated attendees first, then assess 120 independently for expansion. An otherwise suitable 80-person venue keeps its place when expansion fails, with the limitation explained. An 80-only approach hides growth risk and a 120 minimum removes useful options.
- Require at least three usable simultaneous breakout rooms. Prefer explicit seated evidence. An unqualified scalar is a shortlist seating proxy with a caveat unless contradicted. For an ambiguous range, use the lower bound as the supported count, fail above the upper bound, and require confirmation between them. Standing capacity cannot satisfy seating. See AC-03 and AC-15.

### PD-002 — Cost and budget

- Use €15,000 EUR as an approximate all-in planning ceiling. The candidate policy assumes catering, VAT and mandatory fees count until cost scope is clarified. A venue-only cap omits those costs. Separate comparable quotes, known minimum costs, estimates and unknowns.
- Calculate per-head quotes separately for 80 and 120 using exact minor units, and apply a compatible minimum spend as a floor on its covered charges. A comparable complete quote or known minimum above the ceiling fails. Use supported evidence only. An estimate with an unestablished total or a quote with a known mandatory component unpriced requires confirmation. A comparable stated package covering the known requirements and inclusions can pass provisionally. On-request, unknown VAT or incompatible currency cannot become zero. Never invent VAT, exchange rates, catering or other costs. See AC-04–05 and AC-15.

### PD-003 — Wheelchair accessibility

- Accept an explicit positive listing flag provisionally unless limited or contradicted. Accepting only room-by-room verification would exclude usable shortlist leads. A confirmed access violation in required rooms rejects the venue. Partial, conflicting, unknown and missing evidence require confirmation when the source does not already establish a hard failure.
- Label the export as unverified. Check step-free routes and accessible toilets during booking readiness. Test the generic positive flag, v002 stairs-only breakouts, v017 incomplete upper-floor access, unknown and missing cases. See AC-06–07 and AC-15.

### PD-004 — Dates and availability

- March 2027 is supported by the export and dated listings. Assess Thu 11, Tue 16 and Thu 18 March. A bounded set reflects mid-March while keeping Tuesday viable. Prefer Thursday.
- Apply explicit exclusions and booking restrictions. Apply the assumed year to relevant yearless March listings and disclose it. Vague, unsupported or contradictory availability needs confirmation. An omitted date is unresolved unless a rule or exhaustive date list excludes it. A lead-time rule uses an explicit booking reference date when supplied, otherwise it remains unresolved. Positive date evidence is preliminary and the actual slot must be booked separately. See AC-08.

### PD-005 — Utrecht and train access

- Prefer Utrecht Centraal (including Utrecht CS), with 15 minutes on foot as a ranking heuristic. Assess documented rail and connecting public transport, including evidence about access from Amsterdam and Eindhoven where provided. A 15-minute hard cutoff would discard plausible options with a bus or tram connection.
- A documented public transport route can support preliminary eligibility without exact times from both origins. Use explicit poor rail evidence from either origin as a concern. Reject when it definitively rules out practical public transport and no supported alternative exists. Otherwise flag it for confirmation. City or station names alone do not prove convenient access. Never infer journey times. See AC-09.

### PD-006 — Data quality and duplicates

- Preserve missing, invalid and conflicting evidence, source IDs and meaningful negative facts. A source omitting a fact cannot erase adverse evidence from a linked source. Review possible duplicates and consolidate only when evidence confirms shared identity. Similar names alone cannot merge distinct event spaces.
- Keep potential duplicates traceable and flag unresolved material identity or evidence. Avoid cherry-picking the most favourable value. Keep billing basis, currency, VAT and inclusions. Preserve ambiguous values such as v008's `7.900` unless a supported locale rule resolves them. Preserve v027's invalid zero and unverified note. See AC-01–02 and AC-10.

### PD-007 — Eligibility, ranking and explanations

- Reject explicit unacceptable lighting or Wi-Fi evidence, such as a plenary explicitly described as windowless or dark, or clearly poor or unreliable Wi-Fi. Explicit positive listing evidence, including daylight and unqualified fiber without an adverse qualifier, supports preliminary suitability. Unknown, ambiguous or materially conflicting quality needs confirmation. Evidence of a hard violation cannot be offset by preferences.
- `Eligible` means all hard checks pass together on at least one requested date. `Rejected` means every requested date has a hard failure. Other candidates receive `RequiresConfirmation`. A date-independent hard failure applies to all dates. Keep eligible and conditional candidates separate. An empty eligible shortlist is valid.
- Rank eligible candidates by preferred eligible weekday, documented transport convenience, strength of acceptable daylight then Wi-Fi evidence, comparable budget headroom, expansion verdict and stable ID. Use the earliest passing date within a preferred weekday. Define a stable presence order for comparable headroom and leave incomparable values unscored within their bucket. Report evidence, warnings and booking checks. Order conditional candidates by stable ID and label what needs confirmation. See AC-11–15.

## Three proposed client questions

1. **Event fit.** Which mid-March dates and hours work, should we plan for 80 or 120 people, and what seated layout and number of simultaneous breakout rooms do you need?
2. **Budget.** Is €15,000 a firm limit including VAT, catering and mandatory fees? If catering is separate, which costs should count towards the venue limit?
3. **Accessibility.** Which step-free routes, event rooms and accessible facilities should we verify before booking?

These questions can change capacity or date feasibility, invalidate a quote, or exclude an inaccessible venue. Transport thresholds, ranking order and report size have explicit proposed defaults that can be reviewed without asking a fourth question. No questions were sent.

## Implementation acceptance criteria

The outcomes below use the candidate-approved working policy. Isolate the criterion under test when a venue has several independent issues.

| ID | Given / when / then | Requirement or decision | Task |
| --- | --- | --- | --- |
| AC-01 | Given the original JSON, when ingested, all 30 source rows are loaded or individually diagnosed. A fixture with 120 seated places, three usable breakouts, a positive access flag, good daylight and fiber Wi-Fi, a 10-minute Utrecht CS walk, a comparable €10,000 package for both scenarios and explicit availability on 11 March appears as eligible at both headcounts. | Assessment, PD-006 | 003, 005, 008 |
| AC-02 | Given invalid JSON or an unsupported envelope, the command fails with actionable stderr and no partial shortlist. Given a malformed record in a valid array, process remaining records and report its ID or index and field path. Missing, null and invalid evidence remain traceable. | Assessment, PD-006 | 003, 007, 008 |
| AC-03 | Given v019's capacity 85, capacity passes at 80 and fails at 120. Given v002's `80-120` with otherwise usable seating evidence, 80 passes the capacity bound and 120 needs confirmation. Two usable breakouts fail the proposed minimum. | REQ-01–02, PD-001 | 003, 005 |
| AC-04 | Given a complete comparable total of €15,000, budget passes. €15,000.01 or a reliable comparable lower bound of €15,000.01 fails. Apply compatible minimum spend before comparison. | REQ-03, PD-002 | 005 |
| AC-05 | Given a null, on-request or USD price, or an under-cap quote with unpriced required VAT or catering, budget is unresolved. No unknown component is counted as zero. | REQ-03, PD-002 | 003, 005 |
| AC-06 | Given v002's explicitly stairs-only required breakouts, wheelchair access fails. Given explicit suitable access across all required facilities in a fixture, access passes. | REQ-02, REQ-07, PD-003 | 003, 005 |
| AC-07 | Given v017 or a missing access field, accessibility requires confirmation with distinct reasons. A positive flag without contradictory evidence passes the preliminary access check and carries an unverified-source and booking-check note. | REQ-07, PD-003 | 003, 005, 007 |
| AC-08 | Given v002's March rule, 18 March is unavailable and 11 March qualifies at day level under the year assumption. Contradictory unresolved evidence for an otherwise viable date keeps that date unresolved. Explicit closure of all candidate dates fails. | REQ-04–05, PD-004 | 003–005 |
| AC-09 | Given v011's documented bus connection from Utrecht Centraal, the connection can support preliminary location fit without a walking time. Given a comparable but documented longer walk, exceeding 15 minutes alone does not reject. An explicitly taxi-only site with no practical public transport fails. Ambiguous poor access requires confirmation. | REQ-06, PD-005 | 003, 005 |
| AC-10 | Given the three suspected duplicate pairs, preserve both IDs and disagreements, prevent duplicate confirmed entries, and keep a similarly named distinct venue separate. An evidenced hard violation remains visible after grouping. | PD-006 | 004–005, 008 |
| AC-11 | Given eligible Thursday and Tuesday options, prefer Thursday. Given tied venues and shuffled records, the same eligible IDs, order and reasons result. Explicit unacceptable lighting or Wi-Fi stays outside the confirmed ranking regardless of other benefits. Unknown quality is conditional. | REQ-05, REQ-08, PD-007 | 005–006, 008 |
| AC-12 | Given a shortlisted or follow-up entry, explain the applicable verdict, key reasons and warnings, including 120-person impact for confirmed entries and source evidence for unresolved facts. | Assessment, PD-001, PD-007 | 006–008 |
| AC-13 | Given zero eligible venues, the documented command exits successfully with an explicit empty shortlist, rejection and confirmation counts, and concise reasons. Malformed input still uses the error path. | Assessment, PD-007 | 007–008 |
| AC-14 | Given the tracked deliverable, the documented local command runs, `cargo test` passes, and NOTES and genuine planning artifacts meet the assessment requirements. | Assessment | 007–010 |
| AC-15 | Given the unchanged export, De Stadstuin (v019) qualifies and ranks as a strong 80-person candidate based on its documented evidence. Its listed rate yields €6,320 at 80 and €9,480 at 120 before any newly discovered charges. Capacity of 85 fails at 120, with the expansion limit displayed. An equivalent synthetic record produces the same verdicts under a different ID. | REQ-01–08, PD-001–007 | 003, 005–008 |

## Out of scope

UI, API, authentication, deployment, AWS and databases are excluded by the assessment. Live venue verification, booking, sending messages, live FX and runtime LLM use are outside the CLI. Independent access, quote and slot verification belong to booking readiness.
