use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::airport::AirFacility;

use crate::{arg, Action};

pub fn e(cmd_str: &mut Peekable<Split<char>>, air_facilities: &Vec<AirFacility>) -> Result<Action> {
    let d1 = arg!(cmd_str "d1" str);
    let a1 = arg!(cmd_str "a1" time);
    let d2 = arg!(cmd_str "d2" str);
    let c1 = if let Some(d1o) = air_facilities.iter().find(|a| *a.code() == d1) {
        if let Some(c1) = d1o.main_coord() {
            c1
        } else {
            return Ok(Action::Err(format!("Airport `{d1}` has no main coords")));
        }
    } else {
        return Ok(Action::Err(format!("Invalid airport code `{d1}`")));
    };
    let c2 = if let Some(d2o) = air_facilities.iter().find(|a| *a.code() == d1) {
        if let Some(c2) = d2o.main_coord() {
            c2
        } else {
            return Ok(Action::Err(format!("Airport `{d2}` has no main coords")));
        }
    } else {
        return Ok(Action::Err(format!("Invalid airport code `{d2}`")));
    };
    let time = (((c2.x - c1.x) + (c2.y - c1.y)) / 5000.0).abs();
    let a2 = a1 + time;
    Ok(Action::Msg(format!(
        "Flight arrives at {a2} after {time:.2} hours"
    )))
}
