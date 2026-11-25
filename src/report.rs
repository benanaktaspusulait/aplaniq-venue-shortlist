use std::{
    borrow::Cow,
    io::{self, Write},
};

use chrono::{Datelike, Month, NaiveDate, Weekday};
use venue_shortlist::{
    dedup::Deduplication,
    domain::{
        Accessibility, CateringStatus, Criterion, Currency, Eligibility, EventRequest, Evidence,
        MinorUnits, PriceBasis, SeatingLayout, VatStatus, Venue,
    },
    evaluate::{
        BudgetGap, ConfirmationCause, CostEvidence, ListingAssessment,
        RequirementOutcome::{Satisfied, Uncertain, Violated},
        ScenarioAssessment, VenueAssessment,
    },
    ingest::Diagnostic,
    rank::{QualityPreference, Recommendation, Shortlist, TransportPreference},
};

pub(super) fn write_report(
    out: &mut impl Write,
    request: &EventRequest,
    groups: &Deduplication<'_>,
    shortlist: &Shortlist<'_>,
) -> io::Result<()> {
    write_event(out, request)?;
    write_counts(out, request, groups, shortlist)?;
    writeln!(out, "\nRanked eligible shortlist")?;
    if shortlist.eligible.is_empty() {
        writeln!(
            out,
            "  No venues meet all requirements with the available evidence."
        )?;
    }
    for entry in &shortlist.eligible {
        write_ranked(out, entry)?;
    }
    writeln!(out, "\nConditional alternatives")?;
    if shortlist.conditional.is_empty() {
        writeln!(out, "  None.")?;
    }
    for entry in &shortlist.conditional {
        write_other(out, entry, true)?;
    }
    writeln!(out, "\nRejected venues")?;
    if shortlist.rejected.is_empty() {
        writeln!(out, "  None.")?;
    }
    for entry in &shortlist.rejected {
        write_other(out, entry, false)?;
    }

    writeln!(out, "\nNext steps")?;
    writeln!(
        out,
        "  Confirm the headcount, seated layout and simultaneous breakout rooms."
    )?;
    writeln!(
        out,
        "  Ask the venue to confirm event hours, step-free routes, accessible facilities and the final written quote."
    )?;
    writeln!(
        out,
        "  For follow-up venues, resolve the specific evidence gaps above before making a booking."
    )
}

fn write_event(out: &mut impl Write, request: &EventRequest) -> io::Result<()> {
    writeln!(out, "AplanIQ Venue Shortlist\n\nEvent request")?;
    writeln!(
        out,
        "  Primary: {} seated attendees",
        request.seated_attendees
    )?;
    if let Some(expansion) = request.expansion_attendees {
        writeln!(out, "  Expansion: {expansion} seated attendees")?;
    }
    writeln!(
        out,
        "  Working budget: {} including VAT, catering and mandatory charges",
        money(request.budget.minor_units, "EUR")
    )?;
    writeln!(
        out,
        "  Candidate dates: {}",
        request
            .candidate_dates
            .iter()
            .map(|date| date_label(*date))
            .collect::<Vec<_>>()
            .join(", ")
    )?;
    writeln!(
        out,
        "  Thursday preferred. At least {} simultaneous breakout rooms and wheelchair access are required.",
        request.minimum_breakouts
    )?;
    writeln!(
        out,
        "  Yearless March dates use the event year. Booking lead time needs confirmation if no booking reference is supplied."
    )?;
    writeln!(
        out,
        "  Results use unverified listings. Prices are preliminary and dates show day-level evidence."
    )?;

    Ok(())
}

fn write_counts(
    out: &mut impl Write,
    request: &EventRequest,
    groups: &Deduplication<'_>,
    shortlist: &Shortlist<'_>,
) -> io::Result<()> {
    writeln!(out, "\nProcessing summary")?;
    writeln!(out, "  Source records: {}", groups.source_record_count)?;
    writeln!(
        out,
        "  Normalised records: {}",
        groups.normalised_record_count
    )?;
    writeln!(out, "  Invalid records: {}", groups.rejected_records.len())?;
    writeln!(out, "  Distinct venue entities: {}", groups.entities.len())?;
    writeln!(
        out,
        "  Potential duplicate warnings: {} pairs across {} listings",
        groups.potential_duplicates.len(),
        groups.potential_record_count
    )?;
    let mut expansion_counts = [0; 3];
    let entries = shortlist
        .eligible
        .iter()
        .map(|entry| &entry.assessment)
        .chain(&shortlist.conditional)
        .chain(&shortlist.rejected);
    for scenario in entries.filter_map(|entry| entry.expansion.as_ref()) {
        let index = match scenario.eligibility {
            Eligibility::Eligible => 0,
            Eligibility::RequiresConfirmation => 1,
            Eligibility::Rejected => 2,
        };
        if let Some(count) = expansion_counts.get_mut(index) {
            *count += 1;
        }
    }
    writeln!(
        out,
        "  {} attendees: {} eligible, {} require confirmation, {} rejected",
        request.seated_attendees,
        shortlist.eligible.len(),
        shortlist.conditional.len(),
        shortlist.rejected.len()
    )?;
    if let Some(expansion) = request.expansion_attendees {
        let [eligible, conditional, rejected] = expansion_counts;
        writeln!(
            out,
            "  {expansion} attendees: {eligible} eligible, {conditional} require confirmation, {rejected} rejected"
        )?;
    }

    Ok(())
}

fn write_ranked(out: &mut impl Write, entry: &Recommendation<'_>) -> io::Result<()> {
    let assessment = &entry.assessment;
    let Some(venue) = assessment.entity.listings().first() else {
        return Ok(());
    };
    writeln!(out, "  {}. {}", entry.rank, name(venue))?;
    let factors = &entry.factors;
    if let Some(date) = factors.recommended_date {
        writeln!(out, "     Recommended date: {}", date_label(date))?;
    }
    let other_dates: Vec<_> = assessment
        .primary
        .dates
        .iter()
        .filter(|day| {
            day.eligibility == Eligibility::Eligible && Some(day.date) != factors.recommended_date
        })
        .map(|day| date_label(day.date))
        .collect();
    if !other_dates.is_empty() {
        writeln!(
            out,
            "     Other supported dates: {}",
            other_dates.join(", ")
        )?;
    }
    let transport = match factors.transport {
        TransportPreference::ShortCentraalWalk(_) => "short walk from Utrecht Centraal",
        TransportPreference::LongerCentraalWalk(_) => "documented walk from Utrecht Centraal",
        TransportPreference::DocumentedConnection => "supported public transport connection",
        TransportPreference::Unconfirmed => "transport convenience unconfirmed",
    };
    let preferred = if factors.preferred_weekday {
        "preferred weekday, "
    } else {
        ""
    };
    writeln!(
        out,
        "     Ranking evidence: {preferred}{transport}, {} daylight, {} Wi-Fi.",
        quality(factors.daylight),
        quality(factors.wifi)
    )?;
    for listing in &assessment.primary.listings {
        write_source(out, listing, assessment.primary.listings.len() > 1)?;
        write_location(out, listing.listing)?;
        write_capacity(out, listing.listing)?;
        write_cost(out, listing)?;
    }
    if let Some(headroom) = factors.budget_headroom {
        writeln!(out, "     Budget headroom: {}", money(headroom, "EUR"))?;
    }
    write_expansion(out, assessment, true)?;
    write_duplicates(out, assessment)
}

fn write_other(
    out: &mut impl Write,
    entry: &VenueAssessment<'_>,
    conditional: bool,
) -> io::Result<()> {
    let Some(venue) = entry.entity.listings().first() else {
        return Ok(());
    };
    writeln!(out, "  {}", name(venue))?;
    write_findings(out, &entry.primary, conditional, conditional)?;
    if conditional {
        write_expansion(out, entry, false)?;
    }
    write_duplicates(out, entry)
}

fn write_findings(
    out: &mut impl Write,
    scenario: &ScenarioAssessment<'_>,
    conditional: bool,
    show_cost: bool,
) -> io::Result<()> {
    for listing in &scenario.listings {
        write_source(out, listing, scenario.listings.len() > 1)?;
        let budget_failed = listing.budget.requirement.outcome == Violated;
        if show_cost || budget_failed {
            write_cost(out, listing)?;
        }
        for check in listing.requirements.iter().filter(|check| {
            check.outcome == Violated
                || (conditional
                    && check.outcome == Uncertain
                    && check.criterion != Criterion::Budget)
        }) {
            writeln!(out, "     {}: {}", criterion(check.criterion), check.reason)?;
            let venue = listing.listing;
            let source_detail = match check.criterion {
                Criterion::Accessibility => match &venue.accessibility {
                    Evidence::Known(Accessibility::Partial(detail)) => Some(detail.as_str()),
                    _ => None,
                },
                Criterion::Daylight => venue.daylight_qualification.as_deref(),
                Criterion::Wifi => venue.wifi_qualification.as_deref(),
                Criterion::Transport => venue.station_access.public_transport_details.as_deref(),
                _ => None,
            };
            if let Some(detail) = source_detail {
                writeln!(out, "       Listing detail: {}", terminal_text(detail))?;
            }
        }
        if !listing.dates.is_empty()
            && listing
                .dates
                .iter()
                .all(|day| day.availability.outcome == Violated)
        {
            writeln!(
                out,
                "     Availability: none of the candidate dates pass the listing's availability checks."
            )?;
        } else if conditional
            && listing
                .dates
                .iter()
                .all(|day| day.availability.outcome != Satisfied)
        {
            if let Some(day) = listing
                .dates
                .iter()
                .find(|day| day.availability.outcome == Uncertain)
            {
                writeln!(out, "     Availability: {}", day.availability.reason)?;
            }
        }
    }
    if scenario
        .dates
        .iter()
        .any(|day| day.confirmation == Some(ConfirmationCause::SourceDisagreement))
    {
        writeln!(
            out,
            "     Confirmation: Source listings disagree on whether a candidate date meets the requirements."
        )?;
    }
    Ok(())
}

fn write_expansion(
    out: &mut impl Write,
    entry: &VenueAssessment<'_>,
    ranked: bool,
) -> io::Result<()> {
    let Some(expansion) = &entry.expansion else {
        return Ok(());
    };
    let verdict = match expansion.eligibility {
        Eligibility::Eligible => "Preliminarily eligible",
        Eligibility::RequiresConfirmation => "Requires confirmation",
        Eligibility::Rejected => "Not supported",
    };
    writeln!(out, "     Expansion ({}): {verdict}.", expansion.attendees)?;
    if ranked || expansion.eligibility != entry.primary.eligibility {
        write_findings(
            out,
            expansion,
            expansion.eligibility == Eligibility::RequiresConfirmation,
            ranked,
        )?;
    }
    Ok(())
}

fn write_source(
    out: &mut impl Write,
    listing: &ListingAssessment<'_>,
    multiple: bool,
) -> io::Result<()> {
    if multiple {
        if let Some(source) = listing.listing.sources.first() {
            writeln!(
                out,
                "     Source row {}: {}",
                source.record_index + 1,
                name(listing.listing)
            )?;
        }
    }
    Ok(())
}

fn write_duplicates(out: &mut impl Write, entry: &VenueAssessment<'_>) -> io::Result<()> {
    for pair in &entry.potential_duplicates {
        writeln!(
            out,
            "     Possible duplicate: {} and {}. Confirm which space is bookable.",
            terminal_text(pair.left.id.as_str()),
            terminal_text(pair.right.id.as_str())
        )?;
    }
    Ok(())
}

fn write_location(out: &mut impl Write, venue: &Venue) -> io::Result<()> {
    let city = terminal_text(venue.city.as_deref().unwrap_or("City not recorded"));
    if let Evidence::Known(station) = &venue.station_access.station {
        let station = terminal_text(station);
        if let Evidence::Known(walk) = &venue.station_access.walk {
            let approximate = if walk.approximate {
                "approximately "
            } else {
                ""
            };
            writeln!(
                out,
                "     Location: {city}. {approximate}{} min walk from {station}.",
                walk.minutes
            )?;
        } else {
            writeln!(
                out,
                "     Location: {city}. Nearest listed station: {station}."
            )?;
        }
    } else {
        writeln!(out, "     Location: {city}.")?;
    }
    if let Some(detail) = venue.station_access.public_transport_details.as_deref() {
        writeln!(out, "     Travel note: {}", terminal_text(detail))?;
    }
    Ok(())
}

fn write_capacity(out: &mut impl Write, venue: &Venue) -> io::Result<()> {
    if let Evidence::Known(capacity) = &venue.capacity {
        let seats: Vec<_> = capacity
            .seated
            .iter()
            .map(|seats| {
                let layout = match seats.layout {
                    SeatingLayout::Unspecified => "layout unconfirmed",
                    SeatingLayout::Seated => "seated",
                    SeatingLayout::Theatre => "theatre",
                    SeatingLayout::Cabaret => "cabaret",
                };
                let count = seats.bounds();
                format!("{} ({layout})", bounds(count.minimum(), count.maximum()))
            })
            .collect();
        if !seats.is_empty() {
            writeln!(out, "     Listed plenary capacity: {}.", seats.join(", "))?;
        }
    }
    if let Evidence::Known(rooms) = &venue.breakouts {
        writeln!(
            out,
            "     Listed breakout rooms: {}.",
            bounds(rooms.count.minimum(), rooms.count.maximum())
        )?;
        if let Some(detail) = rooms.qualification.as_deref() {
            writeln!(out, "     Breakout note: {}", terminal_text(detail))?;
        }
    }
    Ok(())
}

fn write_cost(out: &mut impl Write, listing: &ListingAssessment<'_>) -> io::Result<()> {
    match listing.budget.cost {
        CostEvidence::Complete(amount) => writeln!(
            out,
            "     Price: preliminary package {}.",
            money(amount, "EUR")
        )?,
        CostEvidence::Minimum(amount) => writeln!(
            out,
            "     Price: at least {}. Final total not confirmed.",
            money(amount, "EUR")
        )?,
        CostEvidence::Incomplete(amount) => writeln!(
            out,
            "     Price: quoted amount {}. Coverage not confirmed.",
            money(amount, "EUR")
        )?,
        CostEvidence::Unknown => {
            let incomparable = match &listing.listing.price {
                Evidence::Known(price) if price.currency == Evidence::Known(Currency::Usd) => {
                    if let Evidence::Known(amount) = price.amount {
                        let basis = match price.basis {
                            Evidence::Known(PriceBasis::PerPerson) => " per attendee",
                            Evidence::Known(PriceBasis::PerDay) => " per day",
                            Evidence::Known(PriceBasis::PerPartDay) => " per part-day",
                            _ => " with an unconfirmed billing basis",
                        };
                        writeln!(
                            out,
                            "     Price: {}{basis}. Comparable EUR total unavailable.",
                            money(amount, "USD")
                        )?;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if !incomparable {
                writeln!(out, "     Price not available as a comparable total.")?;
            }
        }
    }
    let vat = match &listing.listing.price {
        Evidence::Known(price) if price.vat == Evidence::Known(VatStatus::Included) => {
            "VAT included in the listing quote"
        }
        Evidence::Known(price) if price.vat == Evidence::Known(VatStatus::Excluded) => {
            "VAT excluded from the listing quote"
        }
        _ => "VAT not confirmed",
    };
    let catering = match listing.listing.catering {
        Evidence::Known(CateringStatus::Included) => "catering listed as included",
        Evidence::Known(CateringStatus::Excluded) => "catering cost requires confirmation",
        Evidence::Known(CateringStatus::Partial) => "catering partly included",
        _ => "catering not confirmed",
    };
    writeln!(out, "     Inclusions: {vat}, {catering}.")?;
    if !listing.budget.gaps.is_empty() {
        let gaps: Vec<_> = listing
            .budget
            .gaps
            .iter()
            .map(|gap| budget_gap(*gap))
            .collect();
        writeln!(out, "     Budget follow-up: {}.", gaps.join(", "))?;
    }
    Ok(())
}

fn budget_gap(gap: BudgetGap) -> &'static str {
    match gap {
        BudgetGap::Amount => "supported quote amount",
        BudgetGap::Currency => "a comparable EUR quote",
        BudgetGap::Basis => "full event coverage",
        BudgetGap::QuoteKind => "firm quote",
        BudgetGap::Vat => "VAT treatment",
        BudgetGap::Catering => "catering cost",
        BudgetGap::MandatoryFees => "additional mandatory charges or their VAT",
        BudgetGap::MinimumSpend => "minimum spend",
        BudgetGap::Overflow => "quoted amount exceeds the supported range",
    }
}

fn criterion(criterion: Criterion) -> &'static str {
    match criterion {
        Criterion::Identity => "Identity",
        Criterion::Seating => "Seating",
        Criterion::Breakouts => "Breakouts",
        Criterion::Budget => "Budget",
        Criterion::Availability => "Availability",
        Criterion::Accessibility => "Accessibility",
        Criterion::Transport => "Transport",
        Criterion::Daylight => "Daylight",
        Criterion::Wifi => "Wi-Fi",
    }
}

fn quality(quality: QualityPreference) -> &'static str {
    match quality {
        QualityPreference::Strong => "strong",
        QualityPreference::Acceptable => "acceptable",
        QualityPreference::Unconfirmed => "unconfirmed",
    }
}

fn name(venue: &Venue) -> String {
    format!(
        "{} [{}]",
        terminal_text(&venue.name),
        terminal_text(venue.id.as_str())
    )
}

fn bounds(minimum: u32, maximum: u32) -> String {
    if minimum == maximum {
        minimum.to_string()
    } else {
        format!("{minimum}–{maximum}")
    }
}

fn date_label(date: NaiveDate) -> String {
    let weekday = match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    let month = u8::try_from(date.month())
        .ok()
        .and_then(|month| Month::try_from(month).ok())
        .map_or("Unknown month", |month| month.name());
    format!("{weekday} {} {month} {}", date.day(), date.year())
}

fn money(amount: MinorUnits, currency: &str) -> String {
    let whole = (amount.get() / 100).to_string();
    let mut grouped = String::new();
    for (index, digit) in whole.chars().enumerate() {
        if index != 0 && (whole.len() - index) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    format!("{currency} {grouped}.{:02}", amount.get() % 100)
}

pub(super) fn write_diagnostics(
    out: &mut impl Write,
    diagnostics: &[Diagnostic],
) -> io::Result<()> {
    for diagnostic in diagnostics {
        let source = &diagnostic.source;
        write!(out, "Warning: record {}", source.record_index + 1)?;
        if let Some(id) = &source.venue_id {
            write!(out, " [{}]", terminal_text(id.as_str()))?;
        }
        if let Some(field) = &source.field {
            write!(out, ", {}", terminal_text(field))?;
        }
        writeln!(out, ": {}.", diagnostic.message)?;
    }
    Ok(())
}

pub(super) fn terminal_text(value: &str) -> Cow<'_, str> {
    if value.chars().any(char::is_control) {
        value
            .chars()
            .filter(|character| !character.is_control())
            .collect::<String>()
            .into()
    } else {
        value.into()
    }
}
