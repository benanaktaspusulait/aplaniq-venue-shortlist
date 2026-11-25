# Venue data audit

The original `venues.json` was unavailable at bootstrap. The Phase 1 brief estimates approximately 30 records. Record count, observed formats, affected venues and issue counts are unverified. The audit items below describe what to inspect once the original is available.

| Area | Inspection to perform |
| --- | --- |
| Document structure | Validate the JSON and top-level shape. Count records and inspect every one. Record repeated object keys if present before an object parser can discard them. |
| Field types and missing values | Inventory keys and observed types by field. Count absent keys, explicit nulls, empty containers, blank strings and invalid values separately. |
| Capacity | Look for scalar counts, numeric text, ranges and setup-specific figures. Check invalid or contradictory bounds, labels and standing versus seated capacity where applicable. |
| Pricing and currency | Identify currency codes or symbols, exact quotes, estimates and ranges. Establish the basis for each amount, such as total hire, hourly, per person or minimum spend. Record ambiguous currency, timing, taxes and inclusions. |
| Availability | Identify explicit dates, ranges, weekday rules, exceptions, times and booking status. Check for ambiguous formats, invalid dates and overlapping or contradictory statements. |
| Accessibility | Inventory explicit flags, feature lists and free text. Record what each claim covers, such as access routes or facilities, and where evidence is missing or contradictory. |
| Venue identity | Compare IDs, names, addresses and other available identifiers. Record each potential duplicate with the matching evidence and check for distinct rooms at the same venue. |
| Conflicting information | Compare structured and free-text claims and check potential duplicates for conflicting facts. Record both values and whether any source metadata supports a precedence decision. |
| Other quality issues | Check whitespace, casing, inconsistent units, unmapped fields, implausible values and missing provenance relevant to the client brief. |

Record the input checksum, exact record count, observed shapes and counts, and representative record IDs or JSON paths. Where policy or interpretation requires client input, record the question next to the evidence. Work read-only and preserve the supplied file byte-for-byte.
