use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn d(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, lt)?;
    file.flights.remove(index);
    return Ok(Action::Refresh);
}
