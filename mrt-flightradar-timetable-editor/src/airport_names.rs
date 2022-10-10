use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use common::data_types::timetable::AirportCode;
use itertools::Itertools;
use regex::Regex;
use smol_str::SmolStr;

pub fn get_airport_names() -> Result<HashMap<AirportCode, SmolStr>> {
    include_str!("../../data/airport_names.txt")
        .trim()
        .split('\n')
        .map(|row| {
            let re = Regex::new(r"^([^\t\n]*)(?:\t([^t\n]*?)|)$")?
                .captures(row)
                .ok_or_else(|| eyre!("Invalid row"))?;
            if let Some(code) = re.get(2) {
                Ok(Some((
                    code.as_str().into(),
                    re.get(1)
                        .ok_or_else(|| eyre!("No airport name"))?
                        .as_str()
                        .into(),
                )))
            } else {
                Ok(None)
            }
        })
        .filter_map_ok(|a| a)
        .collect::<Result<HashMap<_, _>, _>>()
}

#[cfg(test)]
mod test {
    use color_eyre::eyre::Result;

    use crate::airport_names::get_airport_names;

    #[test]
    fn airport_names_file_is_valid() -> Result<()> {
        get_airport_names()?;
        Ok(())
    }
}
