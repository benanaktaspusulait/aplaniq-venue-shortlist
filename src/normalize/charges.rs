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
        charges.unresolved |= seen
            || amount.is_none()
            || !(sentence.contains("vat included") || sentence.contains("including vat"));
        seen = true;
    }
    charges
}

fn required_charge(sentence: &str) -> bool {
    let cost = ["fee", "charge", "surcharge", "catering", "rental", "cost"]
        .iter()
        .any(|term| sentence.contains(term));
    let required = sentence.contains("mandatory")
        || sentence.contains("required")
        || sentence.contains("catering not included")
        || sentence.contains("catering is not included");
    let negated = [
        "no mandatory",
        "no required",
        "not mandatory",
        "not required",
        "optional",
    ]
    .iter()
    .any(|term| sentence.contains(term));
    cost && required && (!negated || sentence.contains(" except "))
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
