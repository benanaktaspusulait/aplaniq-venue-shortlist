use serde_json::Value;

use crate::domain::{Currency, Evidence, MinorUnits, Price, PriceBasis, PriceKind, VatStatus};

use super::{Ambiguous, Invalid, Issue, Row, Unknown, Unsupported, text};

pub(super) fn normalize(row: &mut Row<'_, '_>, value: Option<&Value>) -> Evidence<Price> {
    let Some(object) = value.and_then(Value::as_object) else {
        return row.evidence("price", value, part_day_price);
    };
    let basis = row.evidence("price.type", object.get("type"), basis);
    let currency = row.evidence("price.currency", object.get("currency"), currency);
    let is_supported_currency = matches!(currency, Evidence::Known(Currency::Eur | Currency::Usd));
    let mut read_amount = |field, value| {
        row.evidence(field, value, |value| {
            if !is_supported_currency {
                return Err((
                    Unsupported,
                    "cannot assign minor units without a supported currency",
                ));
            }
            amount(value)
        })
    };
    let amount = read_amount("price.amount", object.get("amount"));
    let minimum_spend = read_amount("price.minimum_spend", object.get("minimum_spend"));
    let vat = row.evidence("price.vat", object.get("vat"), vat);
    Evidence::Known(Price {
        amount,
        currency,
        basis,
        kind: PriceKind::Quoted,
        vat,
        minimum_spend,
    })
}

fn basis(value: &Value) -> Result<PriceBasis, Issue> {
    match text(value)?.to_ascii_lowercase().as_str() {
        "per_person" => Ok(PriceBasis::PerPerson),
        "per_day" => Ok(PriceBasis::PerDay),
        "per_part_day" => Ok(PriceBasis::PerPartDay),
        _ => Err((Unsupported, "unrecognised pricing unit")),
    }
}

fn currency(value: &Value) -> Result<Currency, Issue> {
    let code = text(value)?.to_ascii_uppercase();
    match code.as_str() {
        "EUR" => Ok(Currency::Eur),
        "USD" => Ok(Currency::Usd),
        _ if code.len() == 3 && code.bytes().all(|c| c.is_ascii_uppercase()) => {
            Ok(Currency::Other(code))
        }
        _ => Err((Invalid, "expected a three-letter currency code")),
    }
}

fn vat(value: &Value) -> Result<VatStatus, Issue> {
    match text(value)?.to_ascii_lowercase().as_str() {
        "incl" => Ok(VatStatus::Included),
        "excl" => Ok(VatStatus::Excluded),
        "unknown" => Err((Unknown, "VAT treatment is unknown")),
        _ => Err((Unsupported, "unrecognised VAT treatment")),
    }
}

fn amount(value: &Value) -> Result<MinorUnits, Issue> {
    if let Some(number) = value.as_number() {
        return decimal_minor_units(number.as_str());
    }
    let label = text(value)?;
    if let Some((whole, fraction)) = label.split_once('.') {
        if (1..=3).contains(&whole.len())
            && fraction.len() == 3
            && digits(whole)
            && digits(fraction)
        {
            return Err((
                Ambiguous,
                "period could be a decimal or a grouping separator",
            ));
        }
    }
    decimal_minor_units(label)
}

fn decimal_minor_units(text: &str) -> Result<MinorUnits, Issue> {
    let (whole, fraction) = match text.split_once('.') {
        Some((whole, fraction)) if digits(fraction) => (whole, fraction.trim_end_matches('0')),
        Some(_) => return Err((Invalid, "invalid decimal fraction")),
        None => (text, ""),
    };
    if !digits(whole) || fraction.len() > 2 {
        return Err((
            Invalid,
            "expected a nonnegative amount exact to two decimal places",
        ));
    }
    let cents = match fraction.len() {
        0 => 0,
        1 => u64::from(fraction.as_bytes()[0] - b'0') * 10,
        _ => fraction
            .parse()
            .map_err(|_| (Invalid, "invalid minor units"))?,
    };
    let minor_units = whole
        .parse::<u64>()
        .ok()
        .and_then(|whole| whole.checked_mul(100))
        .and_then(|whole| whole.checked_add(cents))
        .ok_or((Invalid, "amount overflows minor units"))?;
    Ok(MinorUnits::new(minor_units))
}

fn digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|c| c.is_ascii_digit())
}

fn part_day_price(value: &Value) -> Result<Price, Issue> {
    let label = text(value)?.to_ascii_lowercase();
    if matches!(label.as_str(), "on request" | "unknown" | "ask") {
        return Err((Unknown, "no price was quoted"));
    }
    // The Dutch rate unit and tax label are explicit locale evidence for this grouping.
    let Some(amount) = label
        .strip_prefix('€')
        .and_then(|s| s.strip_suffix(" per dagdeel ex. btw"))
    else {
        return Err((Unsupported, "unrecognised price text"));
    };
    let (whole, cents) = amount.trim().split_once(',').unwrap_or((amount.trim(), ""));
    let mut groups = whole.split('.');
    let first = groups
        .next()
        .filter(|part| digits(part))
        .ok_or((Invalid, "invalid Dutch price"))?;
    let mut ungrouped = first.to_owned();
    for group in groups {
        if first.len() > 3 || group.len() != 3 || !digits(group) {
            return Err((Invalid, "invalid Dutch grouping in quoted price"));
        }
        ungrouped.push_str(group);
    }
    if amount.contains(',') {
        ungrouped.push('.');
        ungrouped.push_str(cents);
    }
    Ok(Price {
        amount: Evidence::Known(decimal_minor_units(&ungrouped)?),
        currency: Evidence::Known(Currency::Eur),
        basis: Evidence::Known(PriceBasis::PerPartDay),
        kind: PriceKind::Quoted,
        vat: Evidence::Known(VatStatus::Excluded),
        minimum_spend: Evidence::Missing,
    })
}
