use std::{
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use smol_str::SmolStr;

use crate::types::time::Time;

pub type AirportCode = SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AirlineTimetable {
    pub name: SmolStr,
    pub flights: Vec<Flight>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flight {
    pub aircraft: SmolStr,
    pub registry: SmolStr,
    pub depart_time1: Time,
    pub airport1: AirportCode,
    pub depart_time2: Time,
    pub airport2: AirportCode,
}

impl AirlineTimetable {
    pub fn from_string(file_cont: String, name: SmolStr) -> Result<Self> {
        let flights = file_cont
            .split('\n')
            .filter(|a| !a.is_empty())
            .map(|row| {
                let row_re = Regex::new(r#""([^"]+)",(\w*);(\d+),(\w+),(\d+),(\w+)"#)?
                    .captures(row)
                    .ok_or_else(|| anyhow!("Invalid syntax"))?;
                Ok(Flight {
                    aircraft: row_re.get(1).unwrap().as_str().into(),
                    registry: row_re.get(2).unwrap().as_str().into(),
                    depart_time1: row_re.get(3).unwrap().as_str().parse().unwrap(),
                    airport1: row_re.get(4).unwrap().as_str().into(),
                    depart_time2: row_re.get(5).unwrap().as_str().parse().unwrap(),
                    airport2: row_re.get(6).unwrap().as_str().into(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(AirlineTimetable { name, flights })
    }
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let name = file
            .file_stem()
            .map(|a| a.to_string_lossy().into())
            .unwrap_or_else(|| SmolStr::from("Unknown"));
        let file_cont = fs::read_to_string(file)?;
        AirlineTimetable::from_string(file_cont, name)
    }
    pub fn to_file(&self, mut directory: PathBuf) -> Result<()> {
        directory.push(format!("{}.fpln", self.name));

        fs::write(directory, self.to_string())?;
        Ok(())
    }
}
impl Display for AirlineTimetable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut flights = self.flights.iter().map(|flight| {
            format!(
                r#""{}",{};{},{},{},{}"#,
                flight.aircraft,
                flight.registry,
                flight.depart_time1,
                flight.airport1,
                flight.depart_time2,
                flight.airport2
            )
        });
        write!(f, "{}", flights.join("\n"))
    }
}

#[cfg(test)]
pub mod tests {
    use anyhow::Result;

    use crate::types::timetable::AirlineTimetable;

    #[test]
    pub fn serde_airline_timetable() -> Result<()> {
        let raw = r#"
"Plane 1",NG01A;0800,MPI,1600,PCE
"Plane 2",NG02A;0815,SSI,1430,PCE
        "#
        .trim()
        .to_string();
        let deserialised = AirlineTimetable::from_string(raw.to_owned(), "Test Airline".into())?;
        assert_eq!(deserialised.to_string(), raw);
        Ok(())
    }
}
