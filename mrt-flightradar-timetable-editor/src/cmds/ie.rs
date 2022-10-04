use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn ie(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let flight = arg!(cmd_str "flight" flight);
    file.flights.push(flight);
    Ok(Action::Refresh)
}
