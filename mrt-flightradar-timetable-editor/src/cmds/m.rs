use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn m(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index1 = arg!(cmd_str "index1" get_index, file, le);
    let index2 = arg!(cmd_str "index2" get_index, file, le);
    if index1 < index2 {
        file.flights[index1..=index2].rotate_left(1);
    } else {
        file.flights[index2..=index1].rotate_right(1);
    }
    return Ok(Action::Refresh);
}
