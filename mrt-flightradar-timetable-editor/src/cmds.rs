use std::{iter::Peekable, str::Split};

use anyhow::{anyhow, Result};
use common::types::{airport::AirFacility, time::Time, timetable::Flight};
use glam::Vec2;
use itertools::Itertools;
use smol_str::SmolStr;

use crate::cmds::e::estimate_time;

pub mod c;
pub mod d;
pub mod e;
pub mod h;
pub mod i;
pub mod ie;
pub mod is;
pub mod m;
pub mod n;
pub mod q;

#[derive(Clone, Debug)]
pub enum Action {
    Refresh,
    Hold,
    Msg(String),
    Quit(String),
}

fn get_index(
    cmd_str: &mut Peekable<Split<char>>,
    predicate: impl Fn(usize) -> bool,
    name: &str,
) -> Result<usize> {
    if let Some(index) = cmd_str.next() {
        if let Ok(index) = index.parse::<usize>() {
            if predicate(index) {
                Ok(index)
            } else {
                Err(anyhow!("Invalid index `{index}`"))
            }
        } else {
            Err(anyhow!("Invalid index `{index}`"))
        }
    } else {
        Err(anyhow!("Missing argument <{name}>"))
    }
}

fn get_time(cmd_str: &mut Peekable<Split<char>>, name: &str) -> Result<Time> {
    if let Some(index) = cmd_str.next() {
        if let Ok(time) = index.parse::<Time>() {
            Ok(time)
        } else {
            Err(anyhow!("Invalid time `{index}`"))
        }
    } else {
        Err(anyhow!("Missing argument <{name}>"))
    }
}

fn get_aircraft(cmd_str: &mut Peekable<Split<char>>, _: &str) -> Result<SmolStr> {
    if let Some(next) = cmd_str.peek() {
        if !next.starts_with('"') {
            return Err(anyhow!("Aircraft name does not start with `\"`"));
        }
    } else {
        return Err(anyhow!("Missing argument \"<aircraft>\""));
    }

    let mut aircraft = cmd_str
        .take_while_ref(|a| !a.ends_with('"'))
        .map(|a| a.to_string())
        .join(" ");
    aircraft += " ";
    aircraft += cmd_str.next().unwrap_or("");
    let aircraft = aircraft.trim().trim_matches('"').trim();
    if aircraft.contains('"') {
        return Err(anyhow!("Aircraft cannot contain `\"`"));
    }

    Ok(aircraft.into())
}

fn get_str(cmd_str: &mut Peekable<Split<char>>, name: &str) -> Result<SmolStr> {
    if let Some(next) = cmd_str.next() {
        Ok(next.into())
    } else {
        Err(anyhow!("Missing argument <{name}>"))
    }
}

fn get_flight_segment(
    cmd_str: &mut Peekable<Split<char>>,
    air_facilities: &[AirFacility],
    prev_seg: Option<&FlightSegment>,
) -> Result<FlightSegment> {
    let flight_no = arg!(cmd_str "f" get_str)?;
    let airport = arg!(cmd_str "a" get_str)?;
    let depart_time = if let Some(prev_seg) = prev_seg {
        let f = || {
            Ok(prev_seg.depart_time
                + estimate_time(
                    get_main_coord(&prev_seg.airport, air_facilities)?,
                    get_main_coord(&airport, air_facilities)?,
                ))
        };
        if cmd_str.peek() == Some(&"_") || cmd_str.peek().is_none() {
            f()
        } else {
            arg!(opt cmd_str "d" get_time, f)
        }
    } else {
        arg!(cmd_str "d" get_time)
    }?;

    Ok(FlightSegment {
        flight_no,
        depart_time,
        airport,
    })
}

fn get_flight(
    cmd_str: &mut Peekable<Split<char>>,
    air_facilities: &[AirFacility],
) -> Result<Flight> {
    let aircraft = arg!(cmd_str "aircraft" get_aircraft)?;
    let reg = arg!(cmd_str "reg" get_str)?;
    let mut segments = vec![];
    while cmd_str.peek().is_some() {
        segments.push(get_flight_segment(
            cmd_str,
            air_facilities,
            segments.last(),
        )?);
    }
    Ok(Flight {
        aircraft,
        registry: reg,
        segments,
    })
}

pub fn get_main_coord<'a>(
    airport: &SmolStr,
    air_facilities: &'a [AirFacility],
) -> Result<&'a Pos<Vec2>> {
    if let Some(o) = air_facilities.iter().find(|a| *a.code() == *airport) {
        if let Some(c) = o.main_coord() {
            Ok(c)
        } else {
            Err(anyhow!("Airport `{airport}` has no main coords"))
        }
    } else {
        Err(anyhow!("Invalid airport code `{airport}`"))
    }
}

#[macro_export]
macro_rules! arg {
    ($cmd_str:ident $name:literal get_index, $file:ident, $opr:ident) => {
        $crate::cmds::get_index($cmd_str, |index| index.$opr(&$file.flights.len()), $name)
    };
    ($cmd_str:ident $name:literal get_flight, $air_facilities:expr) => {
        $crate::cmds::get_flight($cmd_str, $air_facilities)
    };
    ($cmd_str:ident $name:literal $ty:ident) => {
        $crate::cmds::$ty($cmd_str, $name)
    };
    (opt $cmd_str:ident $name:literal $ty:ident, $opt:expr) => {
        match $crate::cmds::$ty($cmd_str, $name) {
            Ok(aircraft) => Ok(aircraft),
            Err(err) => {
                if err.to_string().contains("Missing argument") {
                    match $opt() {
                        Ok(res) => Ok(res.into()),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(err)
                }
            }
        }
    };
}

use arg;
use common::types::{timetable::FlightSegment, vec::Pos};
