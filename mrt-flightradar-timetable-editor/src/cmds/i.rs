use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn i(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" index, file, le);
    let flight = arg!(cmd_str "flight" flight);
    file.flights.insert(index, flight);
    Ok(Action::Refresh)
}
