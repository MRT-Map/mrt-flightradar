use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use regex::Regex;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AirlineTimetable {
    pub name: SmolStr,
    pub fleet: Vec<AircraftId>,
    pub flights: Vec<Flight>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AircraftId(pub SmolStr);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flight {
    aircraft: AircraftId,
    registry: SmolStr,
    depart_time1: SmolStr,
    airport1: SmolStr,
    depart_time2: SmolStr,
    airport2: SmolStr,
}

impl AirlineTimetable {
    pub fn from_string(file_cont: String, name: SmolStr) -> Result<Self> {
        let file_re = Regex::new(r"(?s)(.*)\n\n(.*)")?
            .captures(&*file_cont)
            .ok_or_else(|| anyhow!("Invalid syntax"))?;
        let fleet = file_re.get(1).unwrap().as_str().split('\n')
            .map(|row| AircraftId(row.into()))
            .collect::<Vec<_>>();
        let flights = file_re.get(2).unwrap().as_str().split('\n')
            .map(|row| {
                let row_re = Regex::new(r"(\d+),(\w*);(\d+),(\w+),(\d+),(\w+)")?
                    .captures(row)
                    .ok_or_else(|| anyhow!("Invalid syntax"))?;
                let aircraft_index = row_re.get(1).unwrap().as_str().parse::<usize>()? - 1;
                Ok(Flight {
                    aircraft: fleet.get(aircraft_index).ok_or_else(|| anyhow!("Invalid index {}", aircraft_index))?.to_owned(),
                    registry: row_re.get(2).unwrap().as_str().into(),
                    depart_time1: row_re.get(3).unwrap().as_str().into(),
                    airport1: row_re.get(4).unwrap().as_str().into(),
                    depart_time2: row_re.get(5).unwrap().as_str().into(),
                    airport2: row_re.get(6).unwrap().as_str().into(),
                })
            }).collect::<Result<Vec<_>>>()?;

        Ok(AirlineTimetable {
            name,
            fleet,
            flights
        })

    }
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let name = file.file_stem().map(|a| a.to_string_lossy().into())
            .unwrap_or_else(|| SmolStr::from("Unknown"));
        let file_cont = fs::read_to_string(file)?;
        AirlineTimetable::from_string(file_cont, name)
    }
    pub fn to_file(&self) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::types::timetable::AirlineTimetable;
    use anyhow::Result;

    #[test]
    pub fn deserialise_airline_timetable() -> Result<()> {
        AirlineTimetable::from_string(r#"
Stratus ST-200

1,NG01A;0800,MPI,1600,PCE
1,NG02A;0815,SSI,1430,PCE
        "#.trim().into(), "Test Airline".into())?;
        Ok(())
    }
}