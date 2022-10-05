use std::{iter::Peekable, str::Split};

use anyhow::{anyhow, Result};
use common::types::timetable::AirlineTimetable;
use itertools::Itertools;
use regex::Regex;

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
        field => {
            if let Some(re) = Regex::new(r"^a(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? + 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.airport = value.into()
                } else {
                    return Err(anyhow!("No index {idx}"));
                }
            } else if let Some(re) = Regex::new(r"^f(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? + 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.flight_no = value.into()
                } else {
                    return Err(anyhow!("No index {idx}"));
                }
            } else if let Some(re) = Regex::new(r"^d(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? + 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.depart_time = if let Ok(value) = value.parse() {
                        value
                    } else {
                        return Err(anyhow!("Invalid time `{value}`"));
                    };
                } else {
                    return Err(anyhow!("No index {idx}"));
                }
            } else {
                return Err(anyhow!("Invalid field name `{field}`"));
            }
        }
    }
    Ok(Action::Refresh)
}
