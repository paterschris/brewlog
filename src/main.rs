mod recipe;
mod units;

use recipe::{Method, Recipe};
use std::env;
use std::process::ExitCode;

const USAGE: &str = "usage: brewlog <method> (--water <grams> | --coffee <grams> | --cups <count>)";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: Vec<String>) -> Result<String, String> {
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

    Ok(format!(
        "{recipe}\n  steep:  {}",
        units::format_duration(method.steep_seconds())
    ))
}
