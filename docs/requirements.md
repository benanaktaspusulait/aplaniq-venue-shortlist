# Venue Shortlist requirements

Source: Aplaniq Engineer Assignment, pp. 1–2. The classifications below reflect the fictional client message. PD-001–007 define a proposed reference policy for review. All seven decisions need human approval. The reference values are engineering assumptions.

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
| REQ-08 | Natural light and Wi-Fi | Soft preference | The client wants to avoid last year's darkness and poor Wi-Fi. Treating either as an automatic veto would need an explicit threshold. |
| REQ-09 | Reply this week | Out of scope for the CLI | A service deadline for the planner. The CLI produces a local report. |
| ASM-01 | Reference values and evidence policy | Assumption requiring approval | The headcounts, dates, thresholds, evidence rules and ranking order appear below. |

The assessment requires a small agent-driven Rust CLI or library with tests that reads the supplied data and returns a ranked shortlist meeting hard requirements. The selected delivery is a local CLI with one documented command and passing `cargo test`. Use Claude Code for implementation, commit its genuine planning artifacts and preserve actual history. GSD is preferred and another harness is acceptable. Record manual edits if any. Keep `NOTES.md` within two pages, with the required three sections and at least three actual product decisions. The budget is three hours in one or two sittings within a week. At the limit, record the time spent and next steps. The eventual handoff is a private Git repository link shared with Joa under the user's authorisation.

## Proposed product decisions

### PD-001 — Attendance and room fit

- **Question and alternatives.** Which headcount should the shortlist serve? An 80-only shortlist maximises choice but hides growth risk. A 120-only minimum protects expansion but discards viable choices at 80. An 80 baseline plus a 120 check gives the planner both views with one report.
- **Recommendation and rationale.** Rank the 80-person baseline. Re-evaluate the hard rules at 120 for displayed candidates and give the verdict and reason. An 80-person candidate may appear with an explicit warning if it cannot fit 120, or a confirmation flag if 120-person fit is unknown. Use a seated plenary and at least three usable simultaneous breakout rooms. More than four available rooms is acceptable.
- **Implementation and tests.** Use seating-specific capacity when present. An unqualified scalar is a proposed plenary seating proxy with a caveat, unless contradicted. For an unqualified range, the lower endpoint is the conservative supported count, above the upper endpoint fails, and a count between them requires confirmation. Apply the same bounds to unqualified breakout ranges. A qualified room counts when evidence shows it is usable and accessible. Test v019 at 80 and 120, v017 seated versus standing, v002's range and lounge, and two versus three rooms. See AC-03.
- **Approval.** Pending, including seating proxy, layout, concurrent rooms and the 120-person contingency.

### PD-002 — Cost and budget

- **Question and alternatives.** Which cost should determine eligibility? A firm €15,000 all-in limit controls overspend but may leave few complete quotes. A venue-only cap yields more comparable quotes but may omit major costs. Conditional results retain useful options while inviting follow-up.
- **Recommendation and rationale.** Use a proposed inclusive €15,000 EUR cap, including VAT, catering and mandatory services. Keep incomplete quotes in follow-up if they otherwise qualify for it. If the client confirms a separate catering budget, revise the cost basis explicitly.
- **Implementation and tests.** Compare a supported complete total using exact minor units. An estimate can pass only if the approved estimate policy accepts its supported all-in upper bound and that bound is at or below the cap. Multiply comparable per-person quotes by scenario attendance, use one day for a daily quote and require a supported billing basis for part-day rates. Treat a minimum spend as a floor only for comparable covered charges on the same tax basis. Honour explicit inclusions and documented amounts. An accepted comparable lower bound above €15,000 fails despite missing charges. An incomplete total at or below the cap, unpriced VAT, an estimate without a reliable upper bound, an unknown cost or USD requires confirmation. Use no invented prices, rates or tax percentages. Test exact cap, one cent above, VAT-exclusive incomplete quote, v009 minimum spend, v017 unpriced, v029 USD and catering-related gaps. See AC-04–05.
- **Approval.** Pending, including firm cap, tax and service inclusions, and how to handle estimates.

### PD-003 — Wheelchair accessibility

- **Question and alternatives.** What evidence establishes wheelchair access? Accepting a venue-level claim increases choice at the risk of inaccessible rooms. Requiring scoped evidence protects attendance but may leave zero confirmed matches. A separate follow-up preserves leads with explicit gaps.
- **Recommendation and rationale.** Require explicit suitable access covering arrival route, plenary, at least three usable breakout rooms and an accessible toilet. Treat a general true claim, incomplete room coverage or contradictory evidence as requiring confirmation. Preserve missing evidence as a distinct reason. Exclude a candidate when the required configuration is explicitly unsuitable.
- **Implementation and tests.** Record which spaces the claim covers. A partial claim fails only where required access is explicitly unavailable and no supported configuration satisfies the need. An undifferentiated false is adverse venue-level evidence. Suitable means the source supports the required scope consistently, with the export caveat visible. Test v002 stairs-only breakouts, v017's unknown second-floor access, v020 unknown, a missing field, a bare true and a scoped positive fixture. See AC-06–07.
- **Approval.** Pending for required routes and facilities and the evidence threshold. The client need for wheelchair access is explicit.

### PD-004 — Dates and availability

- **Question and alternatives.** Which dates and availability evidence should count? One assumed Thursday is simple but unnecessarily excludes Tuesdays. All of March is flexible but weakens mid-March. A bounded date set respects both the wording and the dataset.
- **Recommendation and rationale.** Use 11–20 March 2027 inclusive as a proposed one-day event window, with Thu 11, Tue 16 and Thu 18 as candidates. Prefer Thursday and otherwise use Tuesday. Dates and weekday alignment are reviewable defaults. Year 2027 is supported by the export date and dated listings.
- **Implementation and tests.** A date-specific listing or an explicit applicable March rule can support day-level availability. An omitted date is unresolved unless a rule or an exhaustive date list excludes it. Day-level evidence does not confirm a held slot. Apply the assumed year to unqualified March listings and disclose it. A specific exception controls a broader rule in the same listing. Vague dates, weekday-only text with no month, unverified lead time or unresolved contradictions cannot establish coverage. For lead time, use an explicit reference date if supplied. If later approval makes hours hard, require matching hours or flag confirmation. Test v002 excludes 18 March, v026 supports 11 and 16 under the year assumption, v018 rules out all candidates, and contradictory listings leave the affected dates unresolved. See AC-08.
- **Approval.** Pending for the window, hours, day-level evidence and booking reference date.

### PD-005 — Utrecht and train access

- **Question and alternatives.** What qualifies as practical rail access? An Utrecht-only filter is easy but discards workable alternatives. Treating any nearby station as enough is broad but lacks evidence for both origin groups. Two documented routes preserve the client's alternative.
- **Recommendation and rationale.** Pass the location criterion with an evidenced walk of at most 15 minutes from Utrecht Centraal, including the Utrecht CS alias, or with a walk of at most 15 minutes from another named station and documented rail journey times of at most 60 minutes from both origins to that station. Give the proven Utrecht Centraal route ranking priority. These thresholds are review assumptions.
- **Implementation and tests.** Use only export evidence. A different city's name, a bus or taxi time, approximate walking without a supported upper bound, and vague rail descriptions cannot prove either route. Report a failure when evidence rules out both routes, otherwise flag the missing route evidence. Test the 15-minute boundary, Utrecht CS alias, v011's bus-only claim and v021's qualitative rail position. See AC-09.
- **Approval.** Pending for both thresholds, the proxy and acceptable access evidence.

### PD-006 — Data quality and duplicates

- **Question and alternatives.** How should incomplete and conflicting listings be handled? Taking the newest or highest figure is fast but source timestamps and precedence are weak. Keeping every listing separate preserves provenance but risks duplicate recommendations. Conservative grouping and follow-up retain evidence.
- **Recommendation and rationale.** Keep missing, invalid, conflicting and known evidence distinct. Flag likely duplicate pairs with both IDs. Consolidate only when evidence confirms identity. A suspected group that could otherwise enter the shortlist stays in follow-up while its identity is unresolved. Preserve any definitive hard failure in the affected listing and keep conflicting facts visible.
- **Implementation and tests.** No automatic precedence among the audited pairs. Avoid selecting the cheapest or largest value from mixed records. Preserve the amount, currency, billing basis, VAT and inclusions. Accept bare separators such as v008's `7.900` as unresolved unless an approved locale rule establishes meaning. Treat v027 capacity zero as invalid and retain the website claim as unverified. Test each audited pair and a similar-name distinct venue, unknown/null, invalid zero and conflicting hard facts. See AC-01–02 and AC-10.
- **Approval.** Pending for grouping and evidence-handling policy. See the [data audit](data-analysis.md) for the affected IDs.

### PD-007 — Eligibility, ranking and explanations

- **Question and alternatives.** How should eligible venues be ordered? A weighted score is flexible but harder to explain. Price-first ranking favours cheap quotes and depends on comparability. Ordered preferences give predictable and explainable results with few assumptions.
- **Recommendation and rationale.** For a scenario, a candidate is `Eligible` when every hard criterion passes together on at least one allowed date. If every allowed date has a hard failure, the candidate is `Rejected`. Otherwise it `RequiresConfirmation`. A date-independent hard failure applies to all dates. Rank only eligible venues by best supported weekday (Thu before Tue), proven Utrecht Centraal walk before qualifying alternate rail, daylight evidence (positive, partial or unknown, adverse), Wi-Fi evidence (positive, partial or unknown, adverse), then stable venue or group ID. Use the earliest date within the preferred eligible weekday. List all eligible entries. Show a separate, unranked follow-up ordered by stable ID.
- **Implementation and tests.** Explain the selected date, room fit, access, budget and rail evidence, ordering factors and 120-person verdict. Preserve adverse amenity evidence and other warnings, with source IDs. Treat stated daylight in a usable space as positive. Treat evidence of an event network or documented event Wi-Fi performance as positive. A bare connection type or incomplete room coverage is partial. Distinguish listing evidence from independent venue verification in the report. If no confirmed venues qualify, report the counts and main reasons, even when follow-up is empty. Test hard rejection despite preferences, a date that passes only in isolation, the weekday preference, stable ties and shuffled records. See AC-11–13.
- **Approval.** Pending for ranking order, presentation of follow-up and no result limit.

## Three proposed client questions

1. **Event fit.** Which mid-March dates and hours work, should we plan for 80 or 120 people, and what seated layout and number of simultaneous breakout rooms do you need?
2. **Budget.** Is €15,000 a firm limit including VAT, catering and mandatory fees? If catering is separate, which costs should count towards the venue limit?
3. **Accessibility.** Which step-free routes, event rooms and accessible facilities must be confirmed for every shortlisted venue?

These questions can change capacity or date feasibility, invalidate a quote, or exclude an inaccessible venue. Transport thresholds, ranking order and report size have explicit proposed defaults that can be reviewed without asking a fourth question. No questions were sent.

## Implementation acceptance criteria

The outcomes below use the proposed policy pending approval. Isolate the criterion under test when a venue has several independent issues.

| ID | Given / when / then | Requirement or decision | Task |
| --- | --- | --- | --- |
| AC-01 | Given the original JSON, when ingested, all 30 source rows are loaded or individually diagnosed. A fixture with 120 seated places, three accessible breakouts, step-free arrival and plenary, an accessible toilet, an exact 10-minute Utrecht CS walk, a complete €10,000 quote for both scenarios and explicit availability on 11 March appears as eligible at both headcounts. | Assessment, PD-006 | 003, 005, 008 |
| AC-02 | Given invalid JSON or an unsupported envelope, the command fails with actionable stderr and no partial shortlist. Given a malformed record in a valid array, process remaining records and report its ID or index and field path. Missing, null and invalid evidence remain traceable. | Assessment, PD-006 | 003, 007, 008 |
| AC-03 | Given v019's capacity 85, capacity passes at 80 and fails at 120. Given v002's `80-120` with otherwise usable seating evidence, 80 passes the capacity bound and 120 needs confirmation. Two usable breakouts fail the proposed minimum. | REQ-01–02, PD-001 | 003, 005 |
| AC-04 | Given a complete comparable total of €15,000, budget passes. €15,000.01 or a reliable comparable lower bound of €15,000.01 fails. Apply compatible minimum spend before comparison. | REQ-03, PD-002 | 005 |
| AC-05 | Given a null, on-request or USD price, or an under-cap quote with unpriced required VAT or catering, budget is unresolved. No unknown component is counted as zero. | REQ-03, PD-002 | 003, 005 |
| AC-06 | Given v002's explicitly stairs-only required breakouts, wheelchair access fails. Given explicit suitable access across all required facilities in a fixture, access passes. | REQ-02, REQ-07, PD-003 | 003, 005 |
| AC-07 | Given v017, a venue-level true without scope, or a missing access field, wheelchair access is unresolved and the report identifies its distinct reason. | REQ-07, PD-003 | 003, 005, 007 |
| AC-08 | Given v002's March rule, 18 March is unavailable and 11 March qualifies at day level under the year assumption. Contradictory unresolved evidence for an otherwise viable date keeps that date unresolved. Explicit closure of all candidate dates fails. | REQ-04–05, PD-004 | 003–005 |
| AC-09 | Given an exact 15-minute Utrecht CS walk, the location criterion passes. Given v011's bus-only time or v021's vague rail claim, it remains unresolved without qualifying evidence. | REQ-06, PD-005 | 003, 005 |
| AC-10 | Given the three suspected duplicate pairs, preserve both IDs and disagreements, prevent duplicate confirmed entries, and keep a similarly named distinct venue separate. An evidenced hard violation remains visible after grouping. | PD-006 | 004–005, 008 |
| AC-11 | Given eligible Thursday and Tuesday options, prefer Thursday. Given tied venues and shuffled records, the same eligible IDs, order and reasons result. A hard failure stays outside the confirmed ranking regardless of amenities. | REQ-05, REQ-08, PD-007 | 005–006, 008 |
| AC-12 | Given a shortlisted or follow-up entry, explain the applicable verdict, key reasons and warnings, including 120-person impact for confirmed entries and source evidence for unresolved facts. | Assessment, PD-001, PD-007 | 006–008 |
| AC-13 | Given zero eligible venues, the documented command exits successfully with an explicit empty shortlist, rejection and confirmation counts, and concise reasons. Malformed input still uses the error path. | Assessment, PD-007 | 007–008 |
| AC-14 | Given the tracked deliverable, the documented local command runs, `cargo test` passes, and NOTES and genuine planning artifacts meet the assessment requirements. | Assessment | 007–010 |

## Out of scope

UI, API, authentication, deployment, AWS and databases are excluded by the assessment. Live venue verification, booking, sending messages, live FX and runtime LLM use are outside the CLI. This task ends with the specification ready for human review.
