# Venue Shortlist requirements

Source: Aplaniq Engineer Assignment, pp. 1–2. The assessment defines the deliverables. The fictional event manager's message supplies the event context and leaves product choices open.

## What the client said

| Topic | Client statement | Proposed interpretation for review |
| --- | --- | --- |
| Attendance and layout | About 80 attendees, possibly 120, with the count known in January. One plenary and three to four breakouts. | Evaluate capacity in the chosen plenary layout and require simultaneous usable breakouts. Compare an 80-person baseline with a visible 120-person contingency. Alternatives are a 120-person hard minimum or two independently ranked scenarios. Confirm whether three rooms suffice, whether four are preferred and whether larger inventories qualify. |
| Date | Probably mid-March. Thursday preferred and Tuesday acceptable. | Use an explicit set of permitted dates, with Thursday as a preference. March 2027 is a proposal based on the October 2026 export and dated listings. The year, exact window and hours need agreement. |
| Transport | Near Utrecht Centraal or easy by train. Attendees travel from Amsterdam and Eindhoven. | Treat convenient rail access as a potential eligibility requirement and proximity to Utrecht Centraal as a preference. Define an acceptable walking limit, transfers and travel time. An alternative is a stated geographic eligibility rule with rail convenience used for ranking. |
| Budget | Around €15,000 all-in, with catering possibly separate. | Compare complete estimates against €15,000 EUR including VAT if approved as a hard cap. A venue-only cap or a soft target are alternatives. Clarify required services, VAT, minimum spend and how incomplete quotes affect inclusion. |
| Wheelchair access | Several attendees use wheelchairs. | Treat access to required event spaces and routes as a hard requirement. Confirm which facilities and room-level evidence are needed and how a venue-level claim should be treated. |
| Daylight and Wi-Fi | Last year's venue was too dark and Wi-Fi was poor. The client wants to avoid a repeat. | Prefer documented daylight and event Wi-Fi, and surface explicitly poor evidence. Decide whether a known problem is disqualifying and what minimum evidence is sufficient. |
| Timing of reply | Send options this week. | This is a planning deadline. The proposed CLI produces a local report. |

## Three questions for the client

1. Which mid-March dates and hours should we hold, must we plan for 120 now, and what plenary setup and number of simultaneous breakouts are essential?
2. Is €15,000 a firm ceiling including VAT, catering and mandatory services? If catering is separate, what must the venue quote include?
3. Which wheelchair routes and facilities must be confirmed, from arrival through the plenary, breakouts and toilets, for a venue to appear in the shortlist?

No client questions were sent during Phase 1.

## Other decisions for human review

- Publish only candidates that meet every hard requirement in the confirmed shortlist. Consider a separately labelled confirmation list for unresolved candidates, or omit them while reporting why no confirmed option is available.
- Decide the report length, scoring priorities, ordering of ties and the minimum CLI inputs. A stable tie-break and a concise explanation per entry are proposed.
- Record the approved handling of uncertain venue identity, conflicting sources, non-EUR prices and evidence gaps. The [data audit](data-analysis.md) documents why the export alone cannot settle these decisions.

## Acceptance criteria for the implementation

- One documented local command reads the supplied JSON and produces a ranked, explainable shortlist for a defined event request. `cargo test` passes.
- Every confirmed recommendation meets every applicable hard requirement under the recorded assumptions. Preferences cannot override a hard violation. Field issues, conflicts and missing evidence remain visible.
- The same data and request produce the same ordering and explanations, including ties. An empty shortlist explains whether candidates failed or need confirmation.
- Malformed input or an unsupported document shape produces an actionable error. Recoverable venue issues identify the source record and field.
- Claude Code drives the implementation. GSD is preferred and other harnesses are allowed. Commit agent planning artifacts and keep genuine history, including dead ends. Record any manual code fixes and their location.
- Keep `NOTES.md` within two pages using the three required sections. The assessment allows at most three hours in one or two sittings within a week. At the limit, stop and record approximate time spent and next work.
- The eventual submission is a private Git repository link shared with Joa. The current request covers local Phase 1 work only.

## Out of scope

UI, API, authentication, deployment, AWS and databases are excluded by the assessment. Online venue verification, booking, message delivery, live FX and runtime LLM use are outside the proposed CLI. Phase 1 covers the scaffold and planning.
