use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use smol_str::SmolStr;

pub fn get_airport_names() -> Result<HashMap<SmolStr, SmolStr>> {
    include_str!("../../data/airport_names.txt")
        .trim()
        .split('\n')
        .map(|row| {
            let re = Regex::new(r"^([^\t\n]*)(?:\t([^t\n]*?)|)$")?
                .captures(row)
                .ok_or_else(|| anyhow!("Invalid row"))?;
            if let Some(code) = re.get(2) {
                Ok(Some((
                    code.as_str().into(),
                    re.get(1)
                        .ok_or_else(|| anyhow!("No airport name"))?
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
