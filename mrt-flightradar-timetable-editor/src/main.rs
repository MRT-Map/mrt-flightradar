use std::{iter::Peekable, path::PathBuf, str::Split};

use anyhow::Result;
use bunt::println;
use common::types::timetable::{AirlineTimetable, Flight};
use itertools::Itertools;
use rustyline::{error::ReadlineError, Editor};
use smol_str::SmolStr;

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
        match rl.readline("Enter path of file to edit: ") {
            Ok(line) => {
                let path = if let Ok(path) = line.parse::<PathBuf>() {
                    path
                } else {
                    cprintln!(red "Invalid path `{line}`");
                    continue;
                };
                break (
                    match AirlineTimetable::from_file(path.to_owned()) {
                        Ok(at) => at,
                        Err(err) => {
                            cprintln!(red "Error reading file: {err}");
                            continue;
                        }
                    },
                    path.parent().map(|a| a.to_path_buf()).unwrap_or(path),
                );
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                cprintln!(yellow "Quitting");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        }
    };
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Editing {[yellow]}\nEnter {$cyan}h{/$} for help", file.name);
        cprintln!(yellow "#\t(a) Aircraft\t(reg) Registry\t(d1) Dep. 1\t(a1) Airport 1\t(d2) Dep. 2\t(a2) Airport 2");
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
                    f.depart_time1,
                    f.airport1,
                    f.depart_time2,
                    f.airport2
                ))
                .join("\n")
        );
        match rl.readline("> ") {
            Ok(cmd_str) => {
                let mut cmd_str = cmd_str.split(' ').peekable();

                macro_rules! view_error {
                    ($expr:expr) => {
                        if let Some(a) = $expr {
                            a
                        } else {
                            let _ = rl.readline("Press enter to continue...");
                            continue
                        }
                    };
                    ($expr:expr, $($f:tt)+) => {
                        if let Some(a) = $expr {
                            a
                        } else {
                            cprintln!(red $($f)+);
                            let _ = rl.readline("Press enter to continue...");
                            continue
                        }
                    }
                }

                match cmd_str.next() {
                    Some("q") => {
                        file.to_file(path)?;
                        cprintln!(yellow "Quitting");
                        return Ok(());
                    }
                    Some("h") => {
                        let cmds = [
                            ("q", "", "Quit the editor"),
                            ("h", "", "View this page"),
                            (
                                "i",
                                "<index> \"<aircraft>\" <reg> <d1> <a1> <d2> <a2>",
                                "Add flight to buffer (Aircraft must be in quotes)",
                            ),
                            (
                                "is",
                                "\"<aircraft>\" <reg> <d1> <a1> <d2> <a2>",
                                "Add flight to start of buffer",
                            ),
                            (
                                "ie",
                                "\"<aircraft>\" <reg> <d1> <a1> <d2> <a2>",
                                "Add flight to end of buffer",
                            ),
                            (
                                "c",
                                "<index> <field> <new_value>",
                                "Change value of field of flight in buffer",
                            ),
                            ("d", "<index>", "Remove flight from buffer"),
                            (
                                "m",
                                "<index1> <index2>",
                                "Move a flight at index1 to index2",
                            ),
                        ];
                        for (cmd, args, desc) in cmds {
                            println!("{[cyan+bold]} {[yellow]}\n{}", cmd, args, desc);
                        }
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Some("i") => {
                        let index = view_error!(get_index(
                            &mut cmd_str,
                            |index| index <= file.flights.len(),
                            "index"
                        ));
                        file.flights
                            .insert(index, view_error!(get_flight(&mut cmd_str)));
                    }
                    Some("is") => {
                        file.flights
                            .insert(0, view_error!(get_flight(&mut cmd_str)));
                    }
                    Some("ie") => {
                        file.flights.push(view_error!(get_flight(&mut cmd_str)));
                    }
                    Some("c") => {
                        let index = view_error!(get_index(
                            &mut cmd_str,
                            |index| index < file.flights.len(),
                            "index"
                        ));
                        let field = view_error!(cmd_str.next(), "Missing argument <field>");
                        let value = cmd_str.take_while(|_| true).join(" ");
                        if value.is_empty() {
                            cprintln!(red "Missing argument <value>");
                            continue;
                        }
                        if field != "a" && value.contains(' ') {
                            cprintln!(red "Value cannot contain spaces");
                            continue;
                        }
                        if field == "a" && value.contains('"') {
                            cprintln!(red "Aircraft cannot contain `\"`");
                            continue;
                        }
                        match field {
                            "a" => file.flights[index].aircraft = value.into(),
                            "reg" => file.flights[index].registry = value.into(),
                            "d1" => file.flights[index].depart_time1 = value.into(),
                            "a1" => file.flights[index].airport1 = value.into(),
                            "d2" => file.flights[index].depart_time2 = value.into(),
                            "a2" => file.flights[index].airport2 = value.into(),
                            field => {
                                cprintln!(red "Invalid field name `{field}`")
                            }
                        }
                    }
                    Some("d") => {
                        let index = view_error!(get_index(
                            &mut cmd_str,
                            |index| index < file.flights.len(),
                            "index"
                        ));
                        file.flights.remove(index);
                    }
                    Some("m") => {
                        let index1 = view_error!(get_index(
                            &mut cmd_str,
                            |index| index < file.flights.len(),
                            "index1"
                        ));
                        let index2 = view_error!(get_index(
                            &mut cmd_str,
                            |index| index < file.flights.len(),
                            "index2"
                        ));
                        if index1 < index2 {
                            file.flights[index1..=index2].rotate_left(1);
                        } else {
                            file.flights[index2..=index1].rotate_right(1);
                        }
                    }
                    Some(a) => {
                        cprintln!(red "Unknown command `{a}`");
                        let _ = rl.readline("Press enter to continue...");
                    }
                    None => {}
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

fn get_index(
    cmd_str: &mut Peekable<Split<char>>,
    predicate: impl Fn(usize) -> bool,
    name: &str,
) -> Option<usize> {
    if let Some(index) = cmd_str.next() {
        if let Ok(index) = index.parse::<usize>() {
            if predicate(index) {
                Some(index)
            } else {
                cprintln!(red "Invalid index `{index}`");
                None
            }
        } else {
            cprintln!(red "Unparseable index `{index}`");
            None
        }
    } else {
        cprintln!(red "Missing argument <{name}>");
        None
    }
}

fn get_flight(cmd_str: &mut Peekable<Split<char>>) -> Option<Flight> {
    if let Some(next) = cmd_str.peek() {
        if !next.starts_with('"') {
            cprintln!(red "Aircraft name does not start with `\"`");
            return None;
        }
    } else {
        cprintln!(red "Missing argument \"<aircraft>\"");
        return None;
    }
    let aircraft = {
        let mut aircraft = cmd_str
            .take_while_ref(|a| !a.ends_with('"'))
            .map(|a| a.to_string())
            .join(" ");
        aircraft += " ";
        aircraft += cmd_str.next().unwrap_or("");
        let aircraft = aircraft.trim().trim_matches('"').trim();
        if aircraft.contains('"') {
            cprintln!(red "Aircraft cannot contain `\"`");
            return None;
        }
        SmolStr::from(aircraft)
    };
    let [reg, d1, a1, d2, a2] = if let Some(arr) = ["reg", "d1", "a1", "d2", "a2"]
        .iter()
        .map(|arg| {
            if let Some(val) = cmd_str.next() {
                Some(SmolStr::from(val))
            } else {
                cprintln!(red "Missing argument <{arg}>");
                None
            }
        })
        .collect::<Option<Vec<_>>>()
    {
        [
            arr[0].to_owned(),
            arr[1].to_owned(),
            arr[2].to_owned(),
            arr[3].to_owned(),
            arr[4].to_owned(),
        ]
    } else {
        return None;
    };
    Some(Flight {
        aircraft,
        registry: reg,
        depart_time1: d1,
        airport1: a1,
        depart_time2: d2,
        airport2: a2,
    })
}
