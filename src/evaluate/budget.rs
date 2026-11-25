use crate::domain::{
    CateringStatus, Criterion, Currency, Evidence, MinorUnits, Money, PriceBasis, PriceKind,
    VatStatus, Venue,
};

use super::{
    RequirementAssessment,
    RequirementOutcome::{Satisfied, Uncertain, Violated},
    check,
};

/// EUR cost evidence scoped to a single configuration and attendance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostEvidence {
    /// Stated comparable all-in package for preliminary assessment.
    Complete(MinorUnits),
    /// Comparable, supported lower bound with an unconfirmed final cost.
    Minimum(MinorUnits),
    /// Reported quote or estimate whose coverage is unresolved.
    Incomplete(MinorUnits),
    /// No usable comparable figure.
    Unknown,
}

/// Reason a quote cannot be treated as complete and comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetGap {
    /// The listing supplied no supported rate.
    Amount,
    /// Rate or spending floor cannot be compared in EUR.
    Currency,
    /// Rate does not cover the full event on its stated basis.
    Basis,
    /// Stated rate is not a firm total.
    QuoteKind,
    /// Tax is additional or unconfirmed.
    Vat,
    /// Required catering is not explicitly covered.
    Catering,
    /// The listing describes an unpriced mandatory charge.
    MandatoryFees,
    /// The source supplied an unreadable or unresolved spending floor.
    MinimumSpend,
    /// Expansion of the stated rate exceeded the monetary representation.
    Overflow,
}

/// Cost evidence, outstanding questions and the budget verdict.
#[derive(Debug, PartialEq, Eq)]
pub struct BudgetAssessment {
    /// Evidence for the attributable price at this attendance.
    pub cost: CostEvidence,
    /// Quote details that must be resolved before treating the price as complete.
    pub gaps: Vec<BudgetGap>,
    /// Hard budget comparison.
    pub requirement: RequirementAssessment,
}

pub(super) fn assess(venue: &Venue, ceiling: &Money, attendees: u32) -> BudgetAssessment {
    let mut gaps = Vec::new();
    if venue.catering != Evidence::Known(CateringStatus::Included) {
        gaps.push(BudgetGap::Catering);
    }
    if venue.notes.as_deref().is_some_and(|notes| {
        let lower = notes.to_ascii_lowercase();
        lower.contains("mandatory fee")
            || lower.contains("mandatory charge")
            || lower.contains("mandatory av fee")
            || lower.contains("mandatory surcharge")
            || lower.contains("catering mandatory")
            || lower.contains("mandatory catering")
            || lower.contains("catering not included")
            || lower.contains("catering is not included")
    }) {
        gaps.push(BudgetGap::MandatoryFees);
    }

    let Evidence::Known(price) = &venue.price else {
        gaps.push(BudgetGap::Amount);
        return unresolved(CostEvidence::Unknown, gaps);
    };
    if ceiling.currency != Currency::Eur || price.currency != Evidence::Known(Currency::Eur) {
        gaps.push(BudgetGap::Currency);
        return unresolved(CostEvidence::Unknown, gaps);
    }
    if price.vat != Evidence::Known(VatStatus::Included) {
        gaps.push(BudgetGap::Vat);
    }
    if price.kind != PriceKind::Quoted {
        gaps.push(BudgetGap::QuoteKind);
    }
    let minimum = match price.minimum_spend {
        Evidence::Known(minimum) => Some(minimum),
        Evidence::Missing => None,
        _ => {
            gaps.push(BudgetGap::MinimumSpend);
            None
        }
    };
    let basis_supported = matches!(
        price.basis,
        Evidence::Known(PriceBasis::PerPerson | PriceBasis::PerDay)
    );
    if !basis_supported {
        gaps.push(BudgetGap::Basis);
    }
    let rate = match (&price.amount, &price.basis) {
        (Evidence::Known(value), Evidence::Known(PriceBasis::PerPerson)) => {
            let Some(rate) = value.checked_mul(attendees) else {
                gaps.push(BudgetGap::Overflow);
                if price.kind != PriceKind::Estimate {
                    return result(
                        CostEvidence::Unknown,
                        gaps,
                        Violated,
                        "The firm rate exceeds the budget and representable monetary range.",
                    );
                }
                return with_floor(minimum, ceiling, gaps, None);
            };
            Some(rate)
        }
        (Evidence::Known(value), _) => Some(*value),
        _ => {
            gaps.push(BudgetGap::Amount);
            None
        }
    };
    let firm_rate = rate.filter(|_| basis_supported && price.kind != PriceKind::Estimate);
    let lower_bound = minimum
        .into_iter()
        .chain(firm_rate)
        .max_by_key(|amount| amount.get());
    if let Some(total) = lower_bound.filter(|_| gaps.is_empty()) {
        let (outcome, reason) = if total.get() > ceiling.minor_units.get() {
            (Violated, "The comparable total exceeds the budget.")
        } else {
            (Satisfied, "The stated all-in package fits the budget.")
        };
        return result(CostEvidence::Complete(total), gaps, outcome, reason);
    }
    with_floor(lower_bound, ceiling, gaps, rate)
}

fn with_floor(
    floor: Option<MinorUnits>,
    ceiling: &Money,
    gaps: Vec<BudgetGap>,
    rate: Option<MinorUnits>,
) -> BudgetAssessment {
    if let Some(floor) = floor {
        if floor.get() > ceiling.minor_units.get() {
            return result(
                CostEvidence::Minimum(floor),
                gaps,
                Violated,
                "The known mandatory cost exceeds the budget.",
            );
        }
        return unresolved(CostEvidence::Minimum(floor), gaps);
    }
    unresolved(
        rate.map_or(CostEvidence::Unknown, CostEvidence::Incomplete),
        gaps,
    )
}

fn unresolved(cost: CostEvidence, gaps: Vec<BudgetGap>) -> BudgetAssessment {
    result(
        cost,
        gaps,
        Uncertain,
        "Confirm the comparable all-in price and outstanding cost components.",
    )
}

fn result(
    cost: CostEvidence,
    gaps: Vec<BudgetGap>,
    outcome: super::RequirementOutcome,
    reason: &'static str,
) -> BudgetAssessment {
    BudgetAssessment {
        cost,
        gaps,
        requirement: check(Criterion::Budget, outcome, reason),
    }
}
