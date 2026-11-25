# Requirements and open decisions

The assessment PDF is the authoritative source. It was unavailable during bootstrap, so this document records the supplied Phase 1 brief and proposals for review. No event-specific client requirements have been verified.

## Explicit scope from the Phase 1 brief

- Target a small, deterministic Rust CLI for a supplied `venues.json`, with a testable core independent of terminal I/O.
- Plan for inconsistent data and explain recommendations and warnings. Keep hard requirements separate from preferences. Preserve the original dataset and represent uncertainty explicitly.
- Use Claude Code for implementation, record actual work in the assessment notes, and make an atomic local Git commit after each meaningful change.
- Stop after bootstrap and architecture. The brief cites an overall three-hour assessment limit, whose start and accounting rules still need to be checked against the PDF.

## Candidate event requirements

| Dimension | Potential hard requirement | Potential ranking preference | Open decision |
| --- | --- | --- | --- |
| Capacity | Fit the guest count in the required setup | Prefer additional capacity if useful | Guest count, setup and any headroom rule |
| Availability | Cover the required local date and booking period | Prefer alternate dates if acceptable | Required period, flexibility and acceptable evidence |
| Accessibility | Meet each specified access need | Prefer other documented amenities | Which facilities and routes are essential |
| Budget | Fit a confirmed cap under an agreed cost model | Prefer lower comparable cost | Currency, inclusions, price basis and acceptable estimates |
| Location and other features | Apply any explicit geographic or feature constraints | Score only agreed features or areas | Which criteria matter and their relative priority |

These are candidates. Confirm the event context and decide each criterion's role before encoding thresholds or weights.

## Three questions for the client

1. What guest count, room setup, event date and booking hours are fixed, and which dates or locations can change?
2. What is the cost cap and currency, and which taxes, fees, minimum spends or mandatory services must count towards it?
3. Which accessibility features are essential, and should venues with unverified hard requirements appear in a clearly labelled follow-up list or be omitted from the shortlist?

## Decisions for human review

- Use `Eligible` for candidates with evidence for every hard requirement. Propose a separate follow-up list for `RequiresConfirmation`, with omission as the alternative. A verified violation produces `Rejected`, even if other evidence is unresolved.
- Confirm the shortlist length, ranking criteria and their importance, and the expected command interface and output format. Prefer a documented stable tie-break alongside any agreed ranking rules.
- Resolve conflicting facts only under an approved source precedence rule. In its absence, carry the conflict forward. Manually reviewing suspected duplicates is an alternative to an approved deterministic merge rule.
- Confirm the treatment of unknown costs, alternate currencies and partial availability. Proposed baseline is to surface uncertainty and compare amounts only on a compatible currency and price basis.

## Proposed acceptance criteria

- A valid input and explicit event criteria produce the same ordered, explained result on repeat runs.
- Invalid input syntax or an unsupported document shape produces an actionable error. Recoverable record and field issues produce traceable diagnostics.
- Every confirmed recommendation meets every applicable hard requirement. Missing, invalid or conflicting evidence is visible, and preferences cannot override a hard violation.
- Each shortlist entry explains its eligibility, relevant cost or availability caveats, and ranking factors. An empty shortlist explains whether candidates were rejected or require confirmation.
- Confirm the exact interface, outputs, thresholds and examples against the PDF before implementing these criteria.

## Data limitations and out of scope

Dataset coverage, field semantics, duplication and quality are unverified. Refer to [Data analysis](data-analysis.md) for the pending audit.

Phase 1 contains no business implementation. The proposed CLI scope excludes online venue verification, booking, external communication, live currency conversion, runtime LLMs and hosted services.
