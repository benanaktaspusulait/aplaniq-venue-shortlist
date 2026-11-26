//! Native entry point for the venue shortlist.

mod report;

use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

use venue_shortlist::{
    dedup::deduplicate, evaluate::partner_day_request, ingest::load_venues, rank::rank_venues,
};

const HELP: &str = "AplanIQ Venue Shortlist
Compare local venue listings for an 80-person partner day with a 120-person expansion option.

Usage: venue-shortlist --venues <PATH>

Options:
  --venues <PATH>  Read venue listings from this JSON file
  --help           Print this help
  --version        Print the application version
";

enum Command {
    Help,
    Version,
    Run(PathBuf),
}

fn parse_args(mut args: impl Iterator<Item = OsString>) -> Result<Command, &'static str> {
    match args.next().as_deref().and_then(std::ffi::OsStr::to_str) {
        Some("--help") if args.next().is_none() => Ok(Command::Help),
        Some("--version") if args.next().is_none() => Ok(Command::Version),
        Some("--venues") => {
            let path = args.next().ok_or("--venues requires a file path")?;
            if path.is_empty() || path.to_str().is_some_and(|path| path.starts_with("--")) {
                return Err("--venues requires a file path");
            }
            if args.next().is_some() {
                return Err("unexpected argument after the venue file path");
            }
            Ok(Command::Run(PathBuf::from(path)))
        }
        None => Err("a venue file is required"),
        _ => Err("unrecognised arguments"),
    }
}

fn run(
    path: &Path,
    out: &mut impl Write,
    diagnostics: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = partner_day_request();
    let year = request.candidate_dates.first().map(chrono::Datelike::year);
    let input = load_venues(path, year)?;
    let groups = deduplicate(&input);
    let shortlist = rank_venues(&groups, &request);
    report::write_report(out, &request, &groups, &shortlist)?;
    out.flush()?;
    report::write_diagnostics(diagnostics, input.diagnostics())?;
    diagnostics.flush()?;
    Ok(())
}

fn main() -> ExitCode {
    let mut out = io::stdout().lock();
    let mut err = io::stderr().lock();
    let result = match parse_args(env::args_os().skip(1)) {
        Ok(Command::Help) => write!(out, "{HELP}")
            .and_then(|()| out.flush())
            .map_err(Into::into),
        Ok(Command::Version) => writeln!(out, "venue-shortlist {}", env!("CARGO_PKG_VERSION"))
            .and_then(|()| out.flush())
            .map_err(Into::into),
        Ok(Command::Run(path)) => run(&path, &mut out, &mut err),
        Err(message) => {
            // A failing diagnostic stream cannot change the primary usage status.
            let _ = writeln!(
                err,
                "Error: {message}. Run venue-shortlist --help for usage."
            );
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            // Preserve the primary failure status if stderr is also closed.
            let _ = writeln!(err, "Error: {}", report::terminal_text(&error.to_string()));
            ExitCode::FAILURE
        }
    }
}
