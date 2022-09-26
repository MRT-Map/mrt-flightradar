use std::path::PathBuf;

use anyhow::Result;
use smol_str::SmolStr;

pub struct AirlineTimetable {
    pub name: SmolStr,
    pub fleet: Vec<Plane>,
    pub flights: Vec<Flight>,
}

pub struct Plane(pub String);

pub struct Flight {
    number: SmolStr,
    depart_time: SmolStr,
    airport1: SmolStr,
    airport2: SmolStr,
    plane: Plane,
}

impl AirlineTimetable {
    pub fn from_file(file: PathBuf) -> Self {
        unimplemented!()
    }
    pub fn to_file(&self) -> Result<()> {
        unimplemented!()
    }
}
