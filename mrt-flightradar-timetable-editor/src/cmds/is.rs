use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn is(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let flight = arg!(cmd_str "flight" flight);
    file.flights.insert(0, flight);
    Ok(Action::Refresh)
}
