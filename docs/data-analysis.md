# Venue data audit

The original `venues.json` has 30 objects in `venues`, 30 unique IDs and the same 14 top-level venue keys in every row. The JSON parses successfully. A duplicate-key check found no repeated keys in any object. The export is dated 28 October 2026 and describes its inputs as unverified partner spreadsheet and scraped listings. There are no venue addresses or booking slots with times. The checked-in file is byte-for-byte identical to the ZIP entry.

SHA-256 `7a943c847d7e18dd4ee00ff4b93a4335b04749e9b3c1af18db20c037fc4e3a52`.

## Observed field shapes and exceptions

| Area | Evidence | Implication for design |
| --- | --- | --- |
| Capacity | 27 integers, including the invalid zero on v027. v002 is `80-120` with ambiguous range meaning. v017 gives 90 seated and 130 standing. v004 gives 140 theatre and 90 cabaret. | Preserve layout and range semantics. A maximum for a different setup cannot establish plenary fit. |
| Breakouts | 28 integers. v002 lists three with the lounge included. v005 gives `4-6`. v030 mentions weather-dependent outdoor breakout space. | Retain qualifications. The source does not establish the usable or accessible room count for every venue. |
| Pricing | 27 objects, two strings and one null (v005). Objects include 16 per-person and 11 per-day quotes, 26 EUR and one USD (v029), and 16 VAT-exclusive and 11 VAT-inclusive quotes. Amounts are 25 integers, one decimal (v002, 72.5) and one string (v008, `7.900`). | Preserve currency, basis and VAT treatment. Confirm interpretation of the grouped amount before treating it as an exact price. The USD quote needs an agreed conversion policy before a EUR budget comparison. |
| Cost completeness | v003 quotes `€4.200 per dagdeel ex. BTW` for a part-day, and v017 has an on-request price. v009 alone has a structured minimum spend (€10,000). Catering is included in 17 rows, excluded in 11, partial in v003, and null in v005. v018 describes mandatory catering at about €35/person in notes, v023 gives a partner estimate of about €40/person, and v007 mentions a possible extra-cost shuttle. | A day, part-day or per-person quote may cover different things. Missing price components and estimated surcharges affect whether the all-in cap can be confirmed. |
| Availability | All 30 values are text. Examples include explicit March 2027 weekdays (v001/v004), weekday-only rules (v008/v020/v025), a March 18 exception (v002), a six-week lead time (v006), explicit dates (v009/v026), vague coverage (v012), Q1 closure (v018), and three `ask` entries (v003/v010/v024) plus v005's `unknown`. Several March entries leave out the year. | Evaluate only supported patterns, with explicit treatment of inferred year, exceptions, lead time and partial evidence. No record gives a bookable time slot. |
| Wheelchair access | 24 true, two false (v003/v022) and four strings. v002 confirms ground-floor access but describes stairs-only breakouts. v007 excludes casemates. v017 mentions lift access to the first floor and breakouts on the second, leaving second-floor access unestablished. v020 is unknown. | Preserve coverage by relevant space and route. A venue-level true provides a general claim with no room-by-room or accessible toilet details. |
| Station access | 27 integer walk times, one approximate string (`~10 min`, v002) and two nulls (v011/v015). Utrecht Centraal and Utrecht CS appear as aliases. v011 lists a 15-minute bus trip, v015 a 20-minute taxi trip with no public transit, and v020 a tram stop three minutes away. Notes for several venues give rough train journey information. | Store the transport mode and confidence. A missing walk time and an alternate transport time have different meanings. |
| Daylight and Wi-Fi | Each is text in 29 rows and null in v005. Examples include v004's windowless plenary and reported Wi-Fi complaints, v007's room-specific limitations and v025's weak Wi-Fi. Many rows say only `fiber`. | Keep explicit adverse evidence, partial coverage and vague positive claims distinct. Connection type alone gives no room coverage or event throughput. |
| Conflicting or stale evidence | v027's export says capacity 0 while notes claim up to 150 from a website without a room setup. v010 cites a 2025 rate, and v030 was last updated in 2023. | Carry provenance and unresolved conflicts into diagnostics. A repair claim or newer extract date does not independently confirm a venue fact. |

## Potential duplicate venues

| Records | Evidence for a possible match | Unresolved differences | Confidence and disposition |
| --- | --- | --- | --- |
| v004 and v005 | Spoorhuis names, Utrecht Centraal and short walks | 3 versus 2 minutes, 140 theatre / 90 cabaret versus 180 unspecified, four breakouts versus 4–6, and explicit March weekday availability versus unknown. Only v004 supplies price (€12,500 excluding VAT), daylight (windowless plenary) and Wi-Fi (reported complaints). | Medium for the venue name. Keep both listings until the bookable space is verified. |
| v009 and v010 | Same Hotel Domzicht base name, city, station alias and breakout count | 120 versus 110 capacity, €95 with €10,000 minimum versus €89 per person from 2025, 9 versus 10 minutes and specific dates versus ask. | Medium for the hotel identity. Keep both until the event space and current quote are verified. |
| v023 and v024 | Moreelse name, same city, station, walk time, capacity and breakout count | €8,900 versus €9,400 per day, weekday availability versus ask, and a partner catering estimate on v023. The source explicitly describes v024 as separately listed on the booking platform. | Low for the same bookable space. The Event Floor label may denote distinct inventory. Keep both. |

All three pairs have different source IDs. The export has no address, postal code, contact detail or authoritative event-space identifier to confirm any pair. The TASK-004 integration test reports three potential pairs involving six listings. All 30 source rows remain separate, with zero confirmed groups and zero field conflicts counted inside confirmed groups. Differences within potential pairs remain in their source listings.

## TASK-003 normalisation result

The integration test read the original file and accounted for **30 source rows, 30 normalised venues, zero rejected records and 36 diagnostics**. Diagnostics include year assumptions and unresolved evidence. They do not count as venue eligibility verdicts. Required ID and name failures reject individual synthetic test rows with index-based diagnostics.

The Dutch part-day price has explicit locale context and is represented as 420,000 EUR minor units. The bare structured amount in v008 remains ambiguous while its rate basis, currency and VAT are retained. The v029 USD amount is preserved without conversion. Invalid zero seating in v027 retains the raw value and the separate note.

Availability supports the observed March weekday rules, exact dates, exceptions, Q1 closure and lead time. Yearless March claims use a caller-supplied year and produce a diagnostic. Vague and monthless phrases remain unresolved. Travel and mandatory extra-cost notes are retained as text. The normaliser neither estimates these amounts nor verifies transport or bookings. The R2 correction reports repeated JSON members and rejects the affected row or ambiguous envelope. The original export has none.

## Eligibility and ranking validation

The unchanged original export produced 30 source rows, 30 normalised rows, 30 entities, no rejected input rows and 36 ingestion diagnostics. No pair was confirmed identical. Six listings form three potential pairs. `v004/v005`, `v009/v010` and `v023/v024` keep their source evidence and identity warnings.

| Scenario | Eligible | Requires confirmation | Rejected |
| --- | ---: | ---: | ---: |
| 80 seated | 3 | 19 | 8 |
| 120 seated | 1 | 16 | 13 |

### Ranked for 80

All three support 11, 16 and 18 March 2027 at day level. The suggested date is Thursday 11 March. Before booking, verify the slot, routes, accessible facilities and quote.

| Rank | Venue | 80-person package and headroom | Order and 120-person result |
| ---: | --- | --- | --- |
| 1 | v019 De Stadstuin | €6,320, €8,680 headroom | Documented 12-minute Utrecht Centraal walk and strong greenhouse daylight. At 120, the €9,480 rate remains within budget, but 85 seats are insufficient. |
| 2 | v028 Zaal Overvecht | €3,040, €11,960 headroom | Supported Utrecht Overvecht connection. Under the same transport and amenity preference buckets as v021, more headroom ranks it higher. At 120, the €4,560 rate remains within budget, but 100 seats are insufficient. |
| 3 | v021 Fabriek 1912 | €5,440, €9,560 headroom | The listing describes rail access relative to Amsterdam and Eindhoven without precise journey times. Eligible at 120 with a stated €8,160 package and 130 listed seats. |

### Conditional at 80

| Venue | What to confirm | 120-person result if different |
| --- | --- | --- |
| v001 De Seinwachter | All-in quote with VAT and catering. | |
| v005 Het Spoorhuis (Utrecht CS) | Date, quote, daylight, Wi-Fi and whether it is a distinct reservation space from v004. | |
| v006 Kantoortuin Leidsche Rijn | VAT and a booking reference to verify six weeks' notice. | |
| v007 Fort aan de Vecht | Accessible rooms excluding casemates, daylight and Wi-Fi outside casemates, and a practical route or shuttle. | |
| v008 De Werkplaats 030 | Confirm event month and dates, the ambiguous `7.900` amount, VAT and catering. | Rejected. 100 seats. |
| v009 Hotel Domzicht | Current all-in quote and whether v010 is the same event space. | |
| v010 Hotel Domzicht Utrecht | Current quote, dates and event-space identity with v009. | Rejected. 110 seats. |
| v011 Glashuis Papendorp | All-in quote with VAT and catering. | |
| v012 Remise Amersfoort | Confirm which Thursdays are available. | |
| v013 Loods Zeeburg | All-in quote and a practical rail plan for Eindhoven attendees. | |
| v014 Strijp Kantine | Verify rail convenience for Amsterdam attendees. | |
| v017 Het Atelier | Quote and step-free access to second-floor breakouts. | Rejected. 90 seated places. |
| v020 Boothuis Kanaleneiland | Access, dates, all-in catering and price, and Wi-Fi suitability. | |
| v023 Hub Moreelse | Quote with VAT and catering and whether v024 is a distinct space. | |
| v024 HUB Moreelse — Event Floor | Dates, all-in quote and whether the event floor is separately bookable. | |
| v026 Studio Wilhelmina | All-in pricing including catering. | |
| v027 De Graanbeurs | Supported seated capacity and all-in price. The export's zero capacity is invalid. | |
| v029 Lichthal Utrecht | Comparable EUR quote or an approved exchange basis for its USD quote. | |
| v030 Paviljoen Griftpark | At least three simultaneous usable breakout rooms regardless of weather. | |

### Rejected at both headcounts

| Venue | Confirmed reasons |
| --- | --- |
| v002 Pakhuis Catharijne | The listing describes stairs-only required breakouts. |
| v003 Villa Oudegracht | Insufficient seated capacity and no wheelchair access. |
| v004 Spoorhuis Utrecht | Windowless plenary and Wi-Fi complaints. Linked to v005 for identity follow-up. |
| v015 Landgoed Heidezicht | Taxi transfer and explicit absence of public transport. |
| v016 Muziekzaal Tivoli-Oost | Two breakouts, €18,000 minimum before extras, and unacceptable daylight. |
| v018 Congrescentrum Lunetten | Fully booked in Q1 2027. |
| v022 Kasteel Nijenrode Bos | No wheelchair access. |
| v025 Theehuis Rhijnauwen | Insufficient seats and breakouts, and weak Wi-Fi. |

In addition to the three 80-person conditional entries that fail seating at 120, v019 and v028 also fail seating at that headcount.

### Reason distributions

Counts are per entity within the named verdict. An entity can contribute more than one reason. Availability is counted as a hard failure when every requested date is excluded.

| Criterion | 80 rejected | 80 conditional | 120 rejected | 120 conditional |
| --- | ---: | ---: | ---: | ---: |
| Seating | 2 | 1 | 7 | 1 |
| Breakouts | 2 | 1 | 2 | 1 |
| Budget | 1 | 15 | 1 | 12 |
| Accessibility | 3 | 3 | 3 | 2 |
| Availability | 1 | 7 | 1 | 5 |
| Transport | 1 | 3 | 1 | 3 |
| Daylight | 2 | 2 | 2 | 2 |
| Wi-Fi | 2 | 3 | 2 | 3 |
| Identity | 0 | 5 | 0 | 4 |

Reproduce the per-entity verdicts and observed ranking with:

```sh
cargo test --locked --test eligibility original_export_and_synthetic_stadstuin -- --nocapture
cargo test --locked --test ranking original_export_produces_the_verified_shortlist -- --nocapture
```
