mod cmds;

use anyhow::Result;
use bunt::println;
use common::types::{airport::get_air_facilities, timetable::AirlineTimetable};
use itertools::Itertools;
use native_dialog::FileDialog;
use rustyline::{error::ReadlineError, Editor};

use crate::cmds::{c::c, d::d, e::e, h::h, i::i, ie::ie, is::is, m::m, q::q, Action};

macro_rules! cprintln {
    (red $($f:tt)+) => {
        println!("{$red+bold}{}{/$}", format!($($f)+))
    };
    (yellow $($f:tt)+) => {
        println!("{$yellow+bold}{}{/$}", format!($($f)+))
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    cprintln!(yellow "MRT FlightRadar Timetable Editor");
    let (mut file, path) = loop {
        println!("Select file...");
        let dialog = FileDialog::new()
            .add_filter("MRT FlightRadar timetable file", &["fpln"])
            .show_open_single_file()?;
        let file = if let Some(file) = dialog {
            file
        } else {
            cprintln!(yellow "Quitting");
            return Ok(());
        };
        break (
            match AirlineTimetable::from_file(file.to_owned()) {
                Ok(at) => at,
                Err(err) => {
                    cprintln!(red "Error reading file: {err}");
                    continue;
                }
            },
            file.parent().map(|a| a.to_path_buf()).unwrap_or(file),
        );
    };
    let air_facilities = get_air_facilities()?;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Editing {[yellow]}\nEnter {$cyan}h{/$} for help", file.name);
        cprintln!(yellow "#\t(a) Aircraft\t(reg) Registry\t(a1) Airport 1\t(d1) Dep. 1\t(a2) Airport 2\t(d2) Dep. 2");
        println!(
            "{}",
            file.flights
                .iter()
                .enumerate()
                .map(|(i, f)| format!(
                    "{}\t{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}",
                    i,
                    f.aircraft,
                    f.registry,
                    f.airport1,
                    f.depart_time1,
                    f.airport2,
                    f.depart_time2
                ))
                .join("\n")
        );
        match rl.readline("> ") {
            Ok(cmd_str) => {
                let mut cmd_str = cmd_str.split(' ').peekable();

                let action = match cmd_str.next() {
                    Some("q") => q(&mut file, &path),
                    Some("h") => h(),
                    Some("i") => i(&mut cmd_str, &mut file, &air_facilities),
                    Some("is") => is(&mut cmd_str, &mut file, &air_facilities),
                    Some("ie") => ie(&mut cmd_str, &mut file, &air_facilities),
                    Some("c") => c(&mut cmd_str, &mut file),
                    Some("d") => d(&mut cmd_str, &mut file),
                    Some("m") => m(&mut cmd_str, &mut file),
                    Some("e") => e(&mut cmd_str, &air_facilities),
                    Some(a) => Ok(Action::Err(format!("Unknown command `{a}`"))),
                    None => Ok(Action::Refresh),
                }?;
                match action {
                    Action::Refresh => {}
                    Action::Hold => {
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Action::Msg(str) => {
                        cprintln!(yellow "{str}");
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Action::Err(str) => {
                        cprintln!(red "{str}");
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Action::Quit(str) => {
                        cprintln!(yellow "{str}");
                        return Ok(());
                    }
                }

                file.to_file(path.to_owned())?;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                file.to_file(path)?;
                cprintln!(yellow "Quitting");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        }
    }
}
