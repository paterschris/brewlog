use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::units::format_duration;

/// Counts down in place on one terminal line. Each tick sleeps until the next
/// whole second measured from the start, so slow redraws don't accumulate
/// drift over a long steep.
pub fn run_countdown(total_seconds: u32) -> io::Result<()> {
    let started = Instant::now();
    let mut stdout = io::stdout();
    for elapsed in 0..=total_seconds {
        let remaining = total_seconds - elapsed;
        write!(stdout, "\r  brewing: {:>8}", format_duration(remaining))?;
        stdout.flush()?;
        let next_tick = started + Duration::from_secs(u64::from(elapsed) + 1);
        if let Some(wait) = next_tick.checked_duration_since(Instant::now()) {
            thread::sleep(wait);
        }
    }
    writeln!(stdout, "\r  done!              ")?;
    Ok(())
}
