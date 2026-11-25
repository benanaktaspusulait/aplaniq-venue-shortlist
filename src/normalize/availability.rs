use chrono::{Datelike, NaiveDate, Weekday};
use serde_json::Value;

use crate::diagnostic::DiagnosticKind;
use crate::domain::{Availability, AvailabilityRule, DateRange, DateStatus, Evidence};

use super::{Ambiguous, Invalid, Issue, Row, Unknown, Unsupported, count, text};

use DateStatus::{Available, Unavailable};
use Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};

const WEEKDAYS: [Weekday; 7] = [Mon, Tue, Wed, Thu, Fri, Sat, Sun];

pub(super) fn normalize(
    row: &mut Row<'_, '_>,
    value: Option<&Value>,
    assumed_year: Option<i32>,
) -> Evidence<Availability> {
    let mut inferred = false;
    let result = row.evidence("availability", value, |value| {
        parse(
            &text(value)?.to_ascii_lowercase(),
            assumed_year,
            &mut inferred,
        )
    });
    if inferred && matches!(result, Evidence::Known(_)) {
        row.flag(
            Some("availability"),
            (
                DiagnosticKind::AssumedYear,
                "yearless March claim uses the supplied event year",
            ),
            value,
        );
    }
    result
}

fn parse(
    label: &str,
    assumed_year: Option<i32>,
    inferred: &mut bool,
) -> Result<Availability, Issue> {
    let mut result = Availability {
        rules: Vec::new(),
        minimum_notice_days: None,
        preferred_weekday: None,
    };
    if matches!(label, "ask" | "unknown" | "on request") {
        return Err((Unknown, "availability is unconfirmed"));
    }
    if let Some(year) = label.strip_prefix("fully booked q1 ") {
        let year = parse_year(year)?;
        result.rules.push(rule(
            date(year, 1, 1)?,
            date(year, 3, 31)?,
            None,
            Unavailable,
        )?);
        return Ok(result);
    }
    if label.starts_with("most ") || !label.contains("march") {
        return Err((
            Ambiguous,
            "no specific supported dates or month and year were supplied",
        ));
    }
    if label.contains(';') || has_numbered_weekday(label) {
        result.rules = parse_dated_weekdays(label, assumed_year, inferred)?;
        return Ok(result);
    }

    let (before, after) = label
        .split_once("march")
        .ok_or((Unsupported, "expected a March claim"))?;
    let (year, remainder) = march_year(after, assumed_year, inferred)?;
    let start = date(year, 3, 1)?;
    let end = date(year, 3, 31)?;

    if before.trim().is_empty() {
        result.rules = parse_march_restriction(remainder, year, start, end)?;
        return Ok(result);
    }

    let pattern = before.trim();
    let pattern = pattern
        .strip_suffix(" available in")
        .or_else(|| pattern.strip_suffix(" in"))
        .unwrap_or(pattern);
    let (pattern, only) = match pattern.strip_suffix(" only") {
        Some(pattern) => (pattern, true),
        None => (pattern, false),
    };
    let days =
        weekday_pattern(pattern).ok_or((Unsupported, "unrecognised March weekday pattern"))?;
    match remainder {
        "" | "open" => {}
        ", tuesdays preferred by venue" => result.preferred_weekday = Some(Tue),
        detail if detail.starts_with(", book ") && detail.ends_with(" weeks ahead") => {
            let weeks = detail
                .trim_start_matches(", book ")
                .trim_end_matches(" weeks ahead");
            result.minimum_notice_days = Some(
                count(weeks)?
                    .checked_mul(7)
                    .ok_or((Invalid, "booking lead time overflows days"))?,
            );
        }
        _ => return Err((Unsupported, "unrecognised booking restriction")),
    }
    result
        .rules
        .push(rule(start, end, Some(days.to_vec()), Available)?);
    if only {
        let other_days = WEEKDAYS
            .into_iter()
            .filter(|day| !days.contains(day))
            .collect();
        result
            .rules
            .push(rule(start, end, Some(other_days), Unavailable)?);
    }
    Ok(result)
}

fn parse_march_restriction(
    remainder: &str,
    year: i32,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<Vec<AvailabilityRule>, Issue> {
    let mut rules = Vec::new();
    let details = remainder
        .strip_prefix(':')
        .map(str::trim)
        .ok_or((Unsupported, "expected a restriction after March"))?;
    if let Some(except) = details.strip_prefix("fully booked except ") {
        for date in except_dates(year, except)? {
            rules.push(exact(date, Available)?);
        }
        rules.push(rule(start, end, None, Unavailable)?);
        return Ok(rules);
    }
    let (pattern, exceptions) = details
        .split_once(", except ")
        .ok_or((Unsupported, "unrecognised March restriction"))?;
    let days = pattern
        .strip_suffix(" only")
        .and_then(weekday_pattern)
        .ok_or((Unsupported, "unrecognised March weekday restriction"))?;
    for date in except_dates(year, exceptions)? {
        rules.push(exact(date, Unavailable)?);
    }
    let other_days = WEEKDAYS
        .into_iter()
        .filter(|day| !days.contains(day))
        .collect();
    rules.push(rule(start, end, Some(days.to_vec()), Available)?);
    rules.push(rule(start, end, Some(other_days), Unavailable)?);
    Ok(rules)
}

fn parse_dated_weekdays(
    label: &str,
    assumed_year: Option<i32>,
    inferred: &mut bool,
) -> Result<Vec<AvailabilityRule>, Issue> {
    let mut rules = Vec::new();
    for block in label.split(';') {
        let (before, after) = block
            .trim()
            .split_once("march")
            .ok_or((Unsupported, "expected March in each dated weekday block"))?;
        let (year, remainder) = march_year(after, assumed_year, inferred)?;
        if !remainder.is_empty() {
            return Err((Unsupported, "unrecognised dated weekday suffix"));
        }
        let (day_label, days) = before
            .trim()
            .split_once(' ')
            .ok_or((Invalid, "expected a weekday and dates"))?;
        let day = weekday(day_label).ok_or((Unsupported, "unrecognised weekday"))?;
        for date in march_dates(year, days)? {
            if date.weekday() != day {
                return Err((Invalid, "listed date does not match its weekday"));
            }
            rules.push(exact(date, Available)?);
        }
    }
    Ok(rules)
}

fn march_year<'a>(
    after: &'a str,
    assumed_year: Option<i32>,
    inferred: &mut bool,
) -> Result<(i32, &'a str), Issue> {
    let after = after.trim_start();
    if after.starts_with(|c: char| c.is_ascii_digit()) {
        let (year, rest) = after.split_at(after.bytes().take_while(u8::is_ascii_digit).count());
        return Ok((parse_year(year)?, rest.trim()));
    }
    let year = assumed_year.ok_or((Ambiguous, "March claim omits its year"))?;
    *inferred = true;
    Ok((year, after.trim()))
}

fn parse_year(year: &str) -> Result<i32, Issue> {
    if year.len() != 4 || !year.bytes().all(|c| c.is_ascii_digit()) {
        return Err((Invalid, "expected a four-digit year"));
    }
    year.parse().map_err(|_| (Invalid, "invalid calendar year"))
}

fn date(year: i32, month: u32, day: u32) -> Result<NaiveDate, Issue> {
    NaiveDate::from_ymd_opt(year, month, day).ok_or((Invalid, "invalid calendar date"))
}

fn except_dates(year: i32, label: &str) -> Result<Vec<NaiveDate>, Issue> {
    let days = label
        .strip_suffix(" march")
        .ok_or((Unsupported, "expected March dates in the exception"))?;
    march_dates(year, days)
}

fn march_dates(year: i32, label: &str) -> Result<Vec<NaiveDate>, Issue> {
    label
        .replace(" and ", ",")
        .split(',')
        .map(|part| date(year, 3, count(part)?))
        .collect()
}

fn has_numbered_weekday(label: &str) -> bool {
    let mut words = label.split_whitespace();
    words.next().and_then(weekday).is_some()
        && words
            .next()
            .is_some_and(|part| part.starts_with(|c: char| c.is_ascii_digit()))
}

fn weekday(label: &str) -> Option<Weekday> {
    match label {
        "mon" => Some(Mon),
        "tue" => Some(Tue),
        "wed" => Some(Wed),
        "thu" => Some(Thu),
        "fri" => Some(Fri),
        "sat" => Some(Sat),
        "sun" => Some(Sun),
        _ => None,
    }
}

fn weekday_pattern(label: &str) -> Option<&'static [Weekday]> {
    match label {
        "tue/thu" | "tue and thu" => Some(&[Tue, Thu]),
        "mon-fri" | "weekdays" => Some(&[Mon, Tue, Wed, Thu, Fri]),
        "thu" | "thursday" | "thursdays" => Some(&[Thu]),
        "tue" | "tuesday" | "tuesdays" => Some(&[Tue]),
        _ => None,
    }
}

fn exact(date: NaiveDate, status: DateStatus) -> Result<AvailabilityRule, Issue> {
    rule(date, date, None, status)
}

fn rule(
    start: NaiveDate,
    end: NaiveDate,
    weekdays: Option<Vec<Weekday>>,
    status: DateStatus,
) -> Result<AvailabilityRule, Issue> {
    let dates = DateRange::new(start, end).ok_or((Invalid, "availability range is reversed"))?;
    Ok(AvailabilityRule {
        dates,
        weekdays,
        status,
    })
}
