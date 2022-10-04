use std::{iter::Peekable, str::Split};

use anyhow::{anyhow, Result};

use crate::{airport_names::get_airport_names, arg, Action};

pub fn n(cmd_str: &mut Peekable<Split<char>>) -> Result<Action> {
    let airport = arg!(cmd_str "airport" get_str)?;
    let airport_names = get_airport_names()?;
    let name = if let Some(name) = airport_names.get(&airport) {
        name
    } else {
        return Err(anyhow!("Airport {airport} is not recorded"));
    };
    Ok(Action::Msg(format!("{airport} is {name}")))
}
