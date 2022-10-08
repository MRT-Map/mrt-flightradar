use std::{iter::Peekable, str::Split};

use anyhow::{anyhow, Result};
#[cfg(test)]
use common::data_types::timetable::AirlineTimetable;
#[cfg(test)]
use common::data_types::RAW_DATA;
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
pub mod sa;
pub mod sae;
pub mod sas;
pub mod sd;

#[derive(Clone, Debug, PartialEq, Eq)]
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

fn get_airport(cmd_str: &mut Peekable<Split<char>>, name: &str) -> Result<AirportCode> {
    if let Some(next) = cmd_str.next() {
        Ok(next.to_uppercase().into())
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
    let airport = arg!(cmd_str "a" get_airport)?;
    let depart_time = if let Some(prev_seg) = prev_seg {
        let f = || {
            Ok(prev_seg.depart_time
                + estimate_time(
                    get_main_coord(&prev_seg.airport, air_facilities)?,
                    get_main_coord(&airport, air_facilities)?,
                ))
        };
        if let Some(a) = cmd_str.peek() {
            if a.trim() == "_" {
                cmd_str.next();
                f()
            } else {
                arg!(opt cmd_str "d" get_time, f)
            }
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
    ($cmd_str:ident $name:literal get_flight_segment, $air_facilities:expr, $prev_seg:expr) => {
        $crate::cmds::get_flight_segment($cmd_str, $air_facilities, $prev_seg)
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

#[cfg(test)]
#[macro_export]
macro_rules! to_cmd_str {
    ($cmd:literal) => {
        $cmd.split(' ').peekable()
    };
}

#[cfg(test)]
fn test_setup() -> Result<(&'static Vec<AirFacility>, AirlineTimetable)> {
    let air_facilities = &RAW_DATA.air_facilities;
    let file = AirlineTimetable::from_string(
        include_str!("../../data/test-timetable.fpln").into(),
        "Test".into(),
    )?;
    Ok((air_facilities, file))
}

use arg;
use common::data_types::{
    airport::AirFacility,
    time::Time,
    timetable::{AirportCode, Flight, FlightSegment},
    vec::Pos,
};

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::cmds::{get_aircraft, get_airport, get_flight, test_setup};

    #[test]
    pub fn get_aircraft_normal() {
        let mut cmd_str = to_cmd_str!(r#""Test Aircraft""#);
        assert_eq!(
            get_aircraft(&mut cmd_str, "").unwrap(),
            "Test Aircraft",
            "Faulty aircraft name parsing"
        )
    }

    #[test]
    pub fn get_aircraft_no_quote() {
        let mut cmd_str = to_cmd_str!("Test Aircraft");
        assert!(matches!(get_aircraft(&mut cmd_str, ""), Err(_)))
    }

    #[test]
    pub fn get_aircraft_contains_quote() {
        let mut cmd_str = to_cmd_str!("Test \"Aircraft");
        assert!(matches!(get_aircraft(&mut cmd_str, ""), Err(_)))
    }

    #[test]
    pub fn get_flight_normal_1seg() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000"#);
        assert!(
            matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)),
            "Unsuccessful flight parsing"
        );
        Ok(())
    }

    #[test]
    pub fn get_flight_normal_2seg_estimation() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000 AB123 KBN"#);
        assert!(
            matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)),
            "Unsuccessful flight parsing"
        );
        Ok(())
    }

    #[test]
    pub fn get_flight_normal_2seg() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000 AB123 KBN 0000"#);
        assert!(
            matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)),
            "Unsuccessful flight parsing"
        );
        Ok(())
    }

    #[test]
    pub fn get_flight_normal_3seg_estimation() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str =
            to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000 AB123 KBN 0000 AB1234 MLH"#);
        assert!(
            matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)),
            "Unsuccessful flight parsing"
        );
        Ok(())
    }

    #[test]
    pub fn get_flight_normal_3seg_underscore_estimation() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str =
            to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000 AB123 KBN _ AB1234 MLH _"#);
        assert!(
            matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)),
            "Unsuccessful flight parsing"
        );
        Ok(())
    }

    #[test]
    pub fn get_flight_normal_0seg() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG"#);
        assert!(matches!(get_flight(&mut cmd_str, air_facilities), Ok(_)));
        Ok(())
    }

    #[test]
    pub fn get_flight_1seg_estimation() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA"#);
        assert!(matches!(get_flight(&mut cmd_str, air_facilities), Err(_)));
        Ok(())
    }

    #[test]
    pub fn get_flight_no_airport() -> Result<()> {
        let (air_facilities, _) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234"#);
        assert!(matches!(get_flight(&mut cmd_str, air_facilities), Err(_)));
        Ok(())
    }

    #[test]
    pub fn get_lowercase_airport_code() -> Result<()> {
        let mut cmd_str = to_cmd_str!("abc");
        assert_eq!(
            get_airport(&mut cmd_str, "").unwrap(),
            "ABC",
            "Faulty airport code parsing"
        );
        Ok(())
    }
}
