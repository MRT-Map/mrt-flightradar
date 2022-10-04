use std::{iter::Peekable, str::Split};

use common::types::timetable::Flight;
use itertools::Itertools;
use smol_str::SmolStr;

pub mod c;
pub mod d;
pub mod e;
pub mod h;
pub mod i;
pub mod ie;
pub mod is;
pub mod m;
pub mod q;

#[derive(Clone, Debug)]
pub enum Action {
    Refresh,
    Hold,
    Msg(String),
    Err(String),
    Quit(String),
}

fn get_index(
    cmd_str: &mut Peekable<Split<char>>,
    predicate: impl Fn(usize) -> bool,
    name: &str,
) -> Result<usize, String> {
    if let Some(index) = cmd_str.next() {
        if let Ok(index) = index.parse::<usize>() {
            if predicate(index) {
                Ok(index)
            } else {
                Err(format!("Invalid index `{index}`"))
            }
        } else {
            Err(format!("Invalid index `{index}`"))
        }
    } else {
        Err(format!("Missing argument <{name}>"))
    }
}

fn get_time(cmd_str: &mut Peekable<Split<char>>, name: &str) -> Result<Time, String> {
    if let Some(index) = cmd_str.next() {
        if let Ok(time) = index.parse::<Time>() {
            Ok(time)
        } else {
            Err(format!("Invalid time `{index}`"))
        }
    } else {
        Err(format!("Missing argument <{name}>"))
    }
}

fn get_aircraft(cmd_str: &mut Peekable<Split<char>>) -> Result<SmolStr, String> {
    if let Some(next) = cmd_str.peek() {
        if !next.starts_with('"') {
            return Err("Aircraft name does not start with `\"`".into());
        }
    } else {
        return Err("Missing argument \"<aircraft>\"".into());
    }

    let mut aircraft = cmd_str
        .take_while_ref(|a| !a.ends_with('"'))
        .map(|a| a.to_string())
        .join(" ");
    aircraft += " ";
    aircraft += cmd_str.next().unwrap_or("");
    let aircraft = aircraft.trim().trim_matches('"').trim();
    if aircraft.contains('"') {
        return Err("Aircraft cannot contain `\"`".into());
    }

    Ok(aircraft.into())
}

fn get_str(cmd_str: &mut Peekable<Split<char>>, name: &str) -> Result<SmolStr, String> {
    if let Some(next) = cmd_str.next() {
        Ok(next.into())
    } else {
        Err(format!("Missing argument <{name}>"))
    }
}

fn get_flight(cmd_str: &mut Peekable<Split<char>>) -> Result<Flight, String> {
    let aircraft = arg!(fn cmd_str "aircraft" aircraft);
    let reg = arg!(fn cmd_str "reg" str);
    let d1 = arg!(fn cmd_str "d1" time);
    let a1 = arg!(fn cmd_str "a1" str);
    let d2 = arg!(fn cmd_str "d2" time);
    let a2 = arg!(fn cmd_str "a2" str); // or else estimate time TODO
    Ok(Flight {
        aircraft,
        registry: reg,
        depart_time1: d1,
        airport1: a1,
        depart_time2: d2,
        airport2: a2,
    })
}

#[macro_export]
macro_rules! arg {
    ($cmd_str:ident $name:literal index, $file:ident, $opr:ident) => {
        match $crate::cmds::get_index($cmd_str, |index| index.$opr(&$file.flights.len()), $name) {
            Ok(index) => index,
            Err(err) => return Ok(Action::Err(err)),
        }
    };
    ($cmd_str:ident $name:literal aircraft) => {
        match $crate::cmds::get_aircraft($cmd_str) {
            Ok(aircraft) => aircraft,
            Err(err) => return Ok(Action::Err(err)),
        }
    };
    (fn $cmd_str:ident $name:literal aircraft) => {
        match $crate::cmds::get_aircraft($cmd_str) {
            Ok(aircraft) => aircraft,
            Err(err) => return Err(err),
        }
    };
    ($cmd_str:ident $name:literal str) => {
        match $crate::cmds::get_str($cmd_str, $name) {
            Ok(aircraft) => aircraft,
            Err(err) => return Ok(Action::Err(err)),
        }
    };
    (fn $cmd_str:ident $name:literal str) => {
        match $crate::cmds::get_str($cmd_str, $name) {
            Ok(aircraft) => aircraft,
            Err(err) => return Err(err),
        }
    };
    ($cmd_str:ident $name:literal time) => {
        match $crate::cmds::get_time($cmd_str, $name) {
            Ok(aircraft) => aircraft,
            Err(err) => return Ok(Action::Err(err)),
        }
    };
    (fn $cmd_str:ident $name:literal time) => {
        match $crate::cmds::get_time($cmd_str, $name) {
            Ok(aircraft) => aircraft,
            Err(err) => return Err(err),
        }
    };
    ($cmd_str:ident $name:literal flight) => {
        match $crate::cmds::get_flight($cmd_str) {
            Ok(aircraft) => aircraft,
            Err(err) => return Ok(Action::Err(err)),
        }
    };
}
use arg;
use common::types::time::Time;
