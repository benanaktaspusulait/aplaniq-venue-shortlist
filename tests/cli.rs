//! Process-level tests for the native shortlist command.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

const BINARY: &str = env!("CARGO_BIN_EXE_venue-shortlist");
static FIXTURE_NUMBER: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let count = FIXTURE_NUMBER.fetch_add(1, Ordering::Relaxed);
        let directory =
            std::env::temp_dir().join(format!("venue shortlist {} {count}", std::process::id()));
        fs::create_dir(&directory).unwrap();
        Self(directory)
    }

    fn file(&self, filename: &str, contents: &str) -> PathBuf {
        let file = self.0.join(filename);
        fs::write(&file, contents).unwrap();
        file
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(path: &Path) -> Output {
    Command::new(BINARY)
        .arg("--venues")
        .arg(path)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> &str {
    std::str::from_utf8(&output.stdout).unwrap()
}

fn stderr(output: &Output) -> &str {
    std::str::from_utf8(&output.stderr).unwrap()
}

#[test]
fn original_export_reports_supported_shortlist_and_separate_verdicts() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("venues.json");
    let output = run(&path);
    assert_eq!(output.status.code(), Some(0));
    let report = stdout(&output);
    assert!(report.contains("Source records: 30"));
    assert!(report.contains("Normalised records: 30"));
    assert!(report.contains("Invalid records: 0"));
    assert!(report.contains("Distinct venue entities: 30"));
    assert!(report.contains("Potential duplicate warnings: 3 pairs across 6 listings"));
    assert!(report.contains("80 attendees: 3 eligible, 19 require confirmation, 8 rejected"));
    assert!(report.contains("120 attendees: 1 eligible, 16 require confirmation, 13 rejected"));
    let (_, entries) = report.split_once("Ranked eligible shortlist").unwrap();
    let (ranked, follow_up) = entries.split_once("Conditional alternatives").unwrap();
    let (conditional, rejected) = follow_up.split_once("Rejected venues").unwrap();
    let (_, ranked) = ranked.split_once("1. De Stadstuin [v019]").unwrap();
    let (stadstuin, remaining) = ranked.split_once("2. Zaal Overvecht [v028]").unwrap();
    let (overvecht, fabriek) = remaining.split_once("3. Fabriek 1912 [v021]").unwrap();
    for (entry, primary, expanded, expansion_verdict) in [
        (stadstuin, "6,320.00", "9,480.00", "Not supported."),
        (overvecht, "3,040.00", "4,560.00", "Not supported."),
        (fabriek, "5,440.00", "8,160.00", "Preliminarily eligible."),
    ] {
        let (primary_details, expansion_details) = entry.split_once("Expansion (120): ").unwrap();
        assert!(primary_details.contains("Recommended date: Thursday 11 March 2027"));
        assert!(
            primary_details
                .contains("Other supported dates: Tuesday 16 March 2027, Thursday 18 March 2027")
        );
        assert!(primary_details.contains(&format!("Price: preliminary package EUR {primary}.")));
        assert!(expansion_details.starts_with(expansion_verdict));
        assert!(expansion_details.contains(&format!("Price: preliminary package EUR {expanded}.")));
    }
    assert!(stadstuin.contains("Budget headroom: EUR 8,680.00"));
    assert!(conditional.contains("Het Spoorhuis (Utrecht CS) [v005]"));
    assert!(conditional.contains("mixed — casemates have no windows"));
    assert!(conditional.contains("USD 110.00 per attendee"));
    assert!(conditional.contains("Possible duplicate: v004 and v005"));
    assert!(conditional.contains("Price not available as a comparable total."));
    assert!(rejected.contains("Spoorhuis Utrecht [v004]"));
    assert!(rejected.contains("Wi-Fi: The listing explicitly reports unacceptable quality."));
    assert!(
        [stadstuin, overvecht, fabriek]
            .iter()
            .all(|entry| !entry.contains("v004") && !entry.contains("v005"))
    );
    assert!(!report.contains("EUR 0.00"));
    assert!(stderr(&output).contains("Warning: record"));
    assert_eq!(run(&path).stdout, output.stdout);
    assert_eq!(run(&path).stderr, output.stderr);
}

#[test]
fn mixed_fixture_reports_independent_verdicts_and_reasons() {
    use serde_json::json;

    let fixture = Fixture::new();
    let compact = json!({
        "id":"synthetic-stadstuin", "name":"Greenhouse", "city":"Utrecht",
        "nearest_station":"Utrecht Centraal", "walk_minutes":12,
        "capacity":85, "breakout_rooms":4,
        "price":{"type":"per_person","amount":79,"currency":"EUR","vat":"incl"},
        "catering_included":true, "wheelchair_accessible":true,
        "daylight":"yes, greenhouse", "wifi":"fiber", "availability":"Tue and Thu in March",
        "notes":"Great fit at 80 people; cannot grow to 120."
    });
    let mut spacious = compact.clone();
    spacious["id"] = json!("spacious");
    spacious["name"] = json!("Large Hall");
    spacious["capacity"] = json!(130);
    spacious["price"]["amount"] = json!(92);
    spacious["availability"] = json!("Tue and Thu in March 2027");
    spacious["notes"] = json!("");

    let mut partial_access = spacious.clone();
    partial_access["id"] = json!("partial-access");
    partial_access["name"] = json!("Mixed Access");
    partial_access["wheelchair_accessible"] =
        json!("yes, except some breakout rooms are stairs-only");

    let mut over_budget = spacious.clone();
    over_budget["id"] = json!("over-budget");
    over_budget["name"] = json!("Costly Hall");
    over_budget["price"]["type"] = json!("per_day");
    over_budget["price"]["amount"] = json!(14900);
    over_budget["notes"] =
        json!("The screen is optional but a required additional AV fee costs €300 including VAT.");

    let path = fixture.file(
        "mixed.json",
        &json!({"venues":[spacious,partial_access,compact,over_budget]}).to_string(),
    );
    let output = run(&path);
    assert_eq!(output.status.code(), Some(0));
    let report = stdout(&output);
    assert!(report.contains("80 attendees: 2 eligible, 1 require confirmation, 1 rejected"));
    assert!(report.contains("120 attendees: 1 eligible, 1 require confirmation, 2 rejected"));
    let (_, entries) = report.split_once("Ranked eligible shortlist").unwrap();
    let (ranked, follow_up) = entries.split_once("Conditional alternatives").unwrap();
    let (conditional, rejected) = follow_up.split_once("Rejected venues").unwrap();
    let (_, ranked) = ranked
        .split_once("1. Greenhouse [synthetic-stadstuin]")
        .unwrap();
    let (compact, spacious) = ranked.split_once("2. Large Hall [spacious]").unwrap();
    assert!(compact.contains("Recommended date: Thursday 11 March 2027"));
    assert!(compact.contains("Price: preliminary package EUR 6,320.00."));
    assert!(compact.contains("Expansion (120): Not supported."));
    assert!(compact.contains("No listed seated layout fits the headcount."));
    assert!(spacious.contains("Expansion (120): Preliminarily eligible."));
    assert!(conditional.contains("Mixed Access [partial-access]"));
    assert!(conditional.contains("some breakout rooms are stairs-only"));
    assert!(rejected.contains("Costly Hall [over-budget]"));
    assert!(rejected.contains("EUR 15,200.00"));
    assert!(rejected.contains("The comparable total exceeds the budget."));
    assert!(stderr(&output).contains("record 3 [synthetic-stadstuin], availability"));
}

#[test]
fn help_version_and_usage_have_distinct_channels_and_status() {
    let help = Command::new(BINARY).arg("--help").output().unwrap();
    assert_eq!(help.status.code(), Some(0));
    assert!(stdout(&help).contains("Usage: venue-shortlist --venues <PATH>"));
    assert!(stdout(&help).contains("--version"));
    assert_eq!(stderr(&help), "");

    let version = Command::new(BINARY).arg("--version").output().unwrap();
    assert_eq!(version.status.code(), Some(0));
    assert_eq!(
        stdout(&version).trim(),
        concat!("venue-shortlist ", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(stderr(&version), "");

    for (args, expected) in [
        (&[][..], "a venue file is required"),
        (&["--venues"][..], "requires a file path"),
        (&["--venues", "--help"][..], "requires a file path"),
        (&["--unknown"][..], "unrecognised arguments"),
        (&["--venues", "file", "--extra"][..], "unexpected argument"),
    ] {
        let output = Command::new(BINARY).args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert_eq!(stdout(&output), "");
        assert!(stderr(&output).contains(expected));
        assert!(stderr(&output).contains("--help"));
    }
}

#[test]
fn fatal_source_errors_keep_path_and_parser_context_off_stdout() {
    let fixture = Fixture::new();
    let missing = fixture.0.join("missing venues.json");
    let malformed = fixture.file("bad venues.json", "{\n  \"venues\": [}");
    let bad_shape = fixture.file("wrong shape.json", r#"{"venues":{}}"#);
    for (path, reason) in [
        (missing, "cannot read"),
        (malformed, "line 2 column 14"),
        (bad_shape, "expected a top-level object with a venues array"),
    ] {
        let output = run(&path);
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(stdout(&output), "");
        assert!(stderr(&output).contains(path.to_str().unwrap()));
        assert!(stderr(&output).contains(reason), "{}", stderr(&output));
        assert!(!stderr(&output).contains("panicked"));
    }
}

#[test]
fn empty_export_is_successful() {
    let fixture = Fixture::new();
    let file = fixture.file("empty venues.json", r#"{"venues":[]}"#);
    let output = run(&file);
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout(&output).contains("Source records: 0"));
    assert!(
        stdout(&output).contains("80 attendees: 0 eligible, 0 require confirmation, 0 rejected")
    );
    assert!(stdout(&output).contains("No venues meet all requirements"));
    assert_eq!(stderr(&output), "");
}

#[test]
fn bad_rows_are_diagnosed_and_priced_neighbours_keep_exact_pennies() {
    let fixture = Fixture::new();
    let file = fixture.file("venues with spaces.json", r#"{"venues":[
        {"id":"duplicate-key","name":"Broken","wifi":"poor","wifi":"good"},
        {"id":"missing-name"},
        {"id":"good","name":"Safe \u001b[31mRoom","city":"Utrecht","capacity":120,"breakout_rooms":3,
         "price":{"type":"per_day","amount":1001.05,"currency":"EUR","vat":"incl"},
         "catering_included":true,"wheelchair_accessible":true,"availability":"Thu 11 March 2027",
         "nearest_station":"Utrecht Centraal","walk_minutes":7,"daylight":"yes","wifi":"fiber"}
    ]}"#);
    let output = run(&file);
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout(&output).contains("Source records: 3"));
    assert!(stdout(&output).contains("Normalised records: 1"));
    assert!(stdout(&output).contains("Invalid records: 2"));
    assert!(stdout(&output).contains("EUR 1,001.05"));
    assert!(!stdout(&output).contains('\u{001b}'));
    assert!(stderr(&output).contains("record 1 [duplicate-key], wifi"));
    assert!(stderr(&output).contains("repeated JSON property"));
    assert!(stderr(&output).contains("record 2 [missing-name], name"));
}

#[test]
fn unresolved_group_and_all_rejected_inputs_are_valid_reports() {
    use serde_json::json;

    let fixture = Fixture::new();
    let eligible = json!({
        "id":"one", "name":"Same hall", "city":"Utrecht", "capacity":120, "breakout_rooms":3,
        "price":{"type":"per_day","amount":10000,"currency":"EUR","vat":"incl"},
        "catering_included":true,"wheelchair_accessible":true,"availability":"Thu 11 March 2027",
        "nearest_station":"Utrecht Centraal","walk_minutes":7,"daylight":"yes","wifi":"fiber"
    });
    let mut other = eligible.clone();
    other["availability"] = json!("Tue 16 March 2027");
    let unresolved = fixture.file(
        "only conditional.json",
        &json!({"venues":[eligible.clone(),other]}).to_string(),
    );
    let output = run(&unresolved);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        stdout(&output).contains("80 attendees: 0 eligible, 1 require confirmation, 0 rejected")
    );
    assert!(stdout(&output).contains("Source listings disagree"));
    assert!(stdout(&output).contains("Normalised records: 2"));
    assert!(stdout(&output).contains("Distinct venue entities: 1"));

    let mut excluded = eligible;
    excluded["daylight"] = json!("dark");
    let rejected = fixture.file(
        "only rejected.json",
        &json!({"venues":[excluded]}).to_string(),
    );
    let output = run(&rejected);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        stdout(&output).contains("80 attendees: 0 eligible, 0 require confirmation, 1 rejected")
    );
    assert!(
        stdout(&output).contains("Daylight: The listing explicitly reports unacceptable quality.")
    );
}
