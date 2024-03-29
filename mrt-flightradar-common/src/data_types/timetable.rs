use std::{
    fmt::{Display, Formatter},
    fs,
    path::PathBuf,
};

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use regex::Regex;
use smol_str::SmolStr;

use crate::data_types::time::Time;

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
    pub segments: Vec<FlightSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlightSegment {
    pub flight_no: SmolStr,
    pub depart_time: Time,
    pub airport: AirportCode,
}

impl AirlineTimetable {
    pub fn from_string(file_cont: &str, name: SmolStr) -> Result<Self> {
        let flights = file_cont
            .split('\n')
            .filter(|a| !a.is_empty())
            .map(|row| {
                let row_re = Regex::new(r#"^"([^"]+)",(\w*);(.*)$"#)?
                    .captures(row)
                    .ok_or_else(|| eyre!("Invalid syntax"))?;
                let aircraft = row_re.get(1).unwrap().as_str();
                let registry = row_re.get(2).unwrap().as_str();
                let segments = row_re
                    .get(3)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split(';')
                    .map(|seg| {
                        let seg_re = Regex::new(r"^(\w+),(\w+),(\d+)$")?
                            .captures(seg)
                            .ok_or_else(|| eyre!("Invalid syntax"))?;
                        Ok(FlightSegment {
                            flight_no: seg_re.get(1).unwrap().as_str().into(),
                            airport: seg_re.get(2).unwrap().as_str().to_uppercase().into(),
                            depart_time: seg_re.get(3).unwrap().as_str().parse()?,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                Ok(Flight {
                    aircraft: aircraft.into(),
                    registry: registry.into(),
                    segments,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { name, flights })
    }
    pub fn from_file(file: PathBuf) -> Result<Self> {
        let name = file
            .file_stem()
            .map_or_else(|| SmolStr::from("Unknown"), |a| a.to_string_lossy().into());
        let file_cont = fs::read_to_string(file)?;
        Self::from_string(&file_cont, name)
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
                r#""{}",{};{}"#,
                flight.aircraft,
                flight.registry,
                flight
                    .segments
                    .iter()
                    .map(std::string::ToString::to_string)
                    .join(";")
            )
        });
        write!(f, "{}", flights.join("\n"))
    }
}
impl Display for FlightSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.flight_no, self.airport, self.depart_time
        )
    }
}

#[cfg(test)]
pub mod tests {
    use color_eyre::eyre::Result;

    use crate::data_types::timetable::AirlineTimetable;

    #[test]
    pub fn serde_airline_timetable() -> Result<()> {
        let raw = r#"
"Test",REG;AB123,ABC,0000;CD456,DEF,0100
"Test",REG;AB123,ABC,0000;CD456,DEF,0100
        "#
        .trim()
        .to_owned();
        let deserialised = AirlineTimetable::from_string(&raw, "Test Airline".into())?;
        assert_eq!(deserialised.to_string(), raw);
        Ok(())
    }
}
