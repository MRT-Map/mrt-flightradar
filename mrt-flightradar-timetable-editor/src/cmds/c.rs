use std::{iter::Peekable, str::Split};

use anyhow::{anyhow, Result};
use common::types::timetable::AirlineTimetable;
use itertools::Itertools;

use crate::{arg, Action};

pub fn c(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, lt)?;
    let field = &*arg!(cmd_str "field" get_str)?;
    let value = cmd_str.take_while(|_| true).join(" ");
    if value.is_empty() {
        return Err(anyhow!("Missing argument <value>"));
    }
    if field != "a" && value.contains(' ') {
        return Err(anyhow!("Value cannot contain spaces"));
    }
    if field == "a" && value.contains('"') {
        return Err(anyhow!("Aircraft cannot contain `\"`"));
    }
    match field {
        "a" => file.flights[index].aircraft = value.into(),
        "reg" => file.flights[index].registry = value.into(),
        "d1" => {
            file.flights[index].depart_time1 = if let Ok(time) = value.parse() {
                time
            } else {
                return Err(anyhow!("Invalid time `{value}`"));
            }
        }
        "a1" => file.flights[index].airport1 = value.into(),
        "d2" => {
            file.flights[index].depart_time2 = if let Ok(time) = value.parse() {
                time
            } else {
                return Err(anyhow!("Invalid time `{value}`"));
            }
        }
        "a2" => file.flights[index].airport2 = value.into(),
        field => return Err(anyhow!("Invalid field name `{field}`")),
    }
    return Ok(Action::Refresh);
}
