mod recipe;
mod timer;
mod units;

use recipe::{Method, Recipe};
use std::env;
use std::process::ExitCode;

const USAGE: &str = "usage: brewlog <method> (--water <grams> | --coffee <grams> | --cups <count>)";

fn main() -> ExitCode {
    let mut arguments: Vec<String> = env::args().skip(1).collect();
    let start_timer = take_flag(&mut arguments, "--timer");
    match run(arguments) {
        Ok((output, steep_seconds)) => {
            println!("{output}");
            if start_timer {
                if let Err(error) = timer::run_countdown(steep_seconds) {
                    eprintln!("error: timer stopped: {error}");
                    return ExitCode::FAILURE;
                }
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn take_flag(arguments: &mut Vec<String>, flag: &str) -> bool {
    let before = arguments.len();
    arguments.retain(|argument| argument != flag);
    arguments.len() != before
}

fn run(arguments: Vec<String>) -> Result<(String, u32), String> {
    let [method, flag, amount] = arguments.as_slice() else {
        return Err("expected exactly three arguments".into());
    };
    let method: Method = method.parse()?;
    let amount: f64 = amount
        .parse()
        .map_err(|_| format!("not a number: {amount}"))?;

    let recipe = match flag.as_str() {
        "--water" => Recipe::for_water(method, amount),
        "--coffee" => Recipe::for_coffee(method, amount),
        "--cups" => Recipe::for_water(method, units::cups_to_grams(amount)),
        other => return Err(format!("unknown flag: {other}")),
    };

    let steep_seconds = method.steep_seconds();
    let output = format!(
        "{recipe}\n  steep:  {}",
        units::format_duration(steep_seconds)
    );
    Ok((output, steep_seconds))
}
