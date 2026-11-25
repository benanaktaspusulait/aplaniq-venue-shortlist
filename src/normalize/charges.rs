use crate::domain::{AdditionalCharges, MinorUnits};

use super::price::decimal_minor_units;

pub(super) fn from_notes(notes: Option<&str>) -> AdditionalCharges {
    let Some(notes) = notes else {
        return AdditionalCharges::default();
    };
    let mut charges = AdditionalCharges::default();
    let mut seen = false;
    for sentence in notes
        .to_ascii_lowercase()
        .split([';', '\n'])
        .flat_map(|line| line.split(". "))
        .flat_map(charge_clauses)
    {
        if !required_charge(sentence) {
            continue;
        }
        let amount = fixed_eur_amount(sentence);
        // Multiple notes can repeat the same charge. Use the largest supported
        // individual minimum until the scope and distinct charges are established.
        charges.minimum_eur = charges
            .minimum_eur
            .into_iter()
            .chain(amount)
            .max_by_key(|amount| amount.get());
        charges.unresolved |= seen || amount.is_none() || !includes_vat(sentence);
        seen = true;
    }
    charges
}

fn includes_vat(clause: &str) -> bool {
    let inclusion = ["vat included", "including vat", "vat is included"];
    let exclusions_or_qualifiers = [
        "not including vat",
        "not vat included",
        "vat not included",
        "vat is not included",
        "excluding vat",
        "vat excluded",
        "vat is excluded",
        "without vat",
        "plus vat",
        "vat treatment is unknown",
    ];
    inclusion.iter().any(|term| clause.contains(term))
        && !exclusions_or_qualifiers
            .iter()
            .any(|term| clause.contains(term))
}

fn charge_clauses(sentence: &str) -> impl Iterator<Item = &str> {
    // Separate a waived or optional item from a compulsory cost on either side.
    // Keep dependent wording such as "is required and costs €300" intact.
    let split = [" but ", " and "]
        .into_iter()
        .flat_map(|join| sentence.match_indices(join))
        .filter_map(|(start, join)| {
            let left = sentence.get(..start)?;
            let right = sentence.get(start + join.len()..)?;
            let independent = join == " but "
                || ["a ", "an ", "the ", "catering "]
                    .iter()
                    .any(|prefix| right.starts_with(prefix));
            independent.then_some((left, right))
        })
        .find(
            |(left, right)| match (negated_charge(left), negated_charge(right)) {
                (true, false) => required_charge(right),
                (false, true) => required_charge(left),
                _ => false,
            },
        );
    let (first, second) = split.map_or((sentence, None), |(left, right)| (left, Some(right)));
    std::iter::once(first).chain(second)
}

fn required_charge(sentence: &str) -> bool {
    let cost = ["fee", "charge", "surcharge", "catering", "rental", "cost"]
        .iter()
        .any(|term| sentence.contains(term));
    let required = sentence.contains("mandatory")
        || sentence.contains("required")
        || sentence.contains("catering not included")
        || sentence.contains("catering is not included");
    cost && required && (!negated_charge(sentence) || sentence.contains(" except "))
}

fn negated_charge(sentence: &str) -> bool {
    [
        "no mandatory",
        "no required",
        "not mandatory",
        "not required",
        "optional",
    ]
    .iter()
    .any(|term| sentence.contains(term))
}

fn fixed_eur_amount(sentence: &str) -> Option<MinorUnits> {
    let extra = ["additional", "extra", "not included"]
        .iter()
        .any(|term| sentence.contains(term));
    let ambiguous = [
        "~",
        "about ",
        "approx",
        "up to",
        " from ",
        " per ",
        " pp",
        " p.p.",
        "estimate",
        " or ",
        " to €",
        " to eur",
        "–",
        "—",
        "$",
        "if ",
        "subject to",
        "unless",
        "between ",
        "discount",
    ]
    .iter()
    .any(|term| sentence.contains(term));
    if !extra || ambiguous {
        return None;
    }
    let mut amounts = sentence
        .split(['€'])
        .skip(1)
        .chain(sentence.split("eur ").skip(1));
    let value = amounts
        .next()?
        .split_whitespace()
        .next()?
        .trim_end_matches(['.', ',', ')']);
    if amounts.next().is_some() {
        return None;
    }
    decimal_minor_units(value).ok()
}
