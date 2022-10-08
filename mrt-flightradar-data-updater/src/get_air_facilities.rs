use anyhow::{anyhow, Result};
use common::{
    data_types::airport::{AirFacility, PlaneFacilityType, Runway, RunwayLength},
    flight_route::types::{coords_to_vec, from_csv},
};
use itertools::Itertools;
use smallvec::smallvec;

pub fn get_air_facilities(str: &str) -> Result<Vec<AirFacility>> {
    from_csv(str)
        .into_iter()
        .skip(1)
        .map(|row| {
            let code = row.first().ok_or_else(|| anyhow!("No code"))?;
            match row.get(1) {
                Some(&"Heliport") => {
                    let pad_coord = row.get(3).ok_or_else(|| anyhow!("No pad_coord"))?;
                    Ok(AirFacility::Heliport {
                        code: code.into(),
                        pad_coord: coords_to_vec(*pad_coord)?,
                    })
                }
                Some(&"Airship Terminal") => {
                    let pad_coord = row.get(3).ok_or_else(|| anyhow!("No pad_coord"))?;
                    Ok(AirFacility::AirshipTerminal {
                        code: code.into(),
                        pad_coord: coords_to_vec(*pad_coord)?,
                    })
                }
                Some(ty) => {
                    let ty = match *ty {
                        "Airport" => PlaneFacilityType::Airport,
                        "Airfield" => PlaneFacilityType::Airfield,
                        _ => return Err(anyhow!("Invalid type `{ty}`")),
                    };
                    let mut i = 3;
                    let mut runways = smallvec![];
                    while if let Some(cell) = row.get(i) {
                        !cell.is_empty()
                    } else {
                        false
                    } {
                        let coord1 = row.get(i).ok_or_else(|| anyhow!("No coord1"))?;
                        i += 1;
                        let coord2 = row.get(i).ok_or_else(|| anyhow!("No coord2"))?;
                        i += 1;
                        let (dir1, dir2) = row
                            .get(i)
                            .ok_or_else(|| anyhow!("No direction"))?
                            .split(" - ")
                            .collect_tuple::<(_, _)>()
                            .ok_or_else(|| anyhow!("Invalid direction"))?;
                        i += 1;
                        let length = match row.get(i) {
                            Some(&"Large") => RunwayLength::Large,
                            Some(&"Small") => RunwayLength::Small,
                            Some(ty) => return Err(anyhow!("Invalid runway length `{ty}`")),
                            None => return Err(anyhow!("No runway length")),
                        };
                        i += 1;
                        runways.push(Runway {
                            start: coords_to_vec(*coord1)?,
                            end: coords_to_vec(*coord2)?,
                            direction: (dir1.into(), dir2.into()),
                            length,
                        });
                        runways.push(Runway {
                            start: coords_to_vec(*coord2)?,
                            end: coords_to_vec(*coord1)?,
                            direction: (dir2.into(), dir1.into()),
                            length,
                        });
                    }
                    Ok(AirFacility::Airport {
                        code: code.into(),
                        ty,
                        runways,
                    })
                }
                None => Err(anyhow!("No type")),
            }
        })
        .collect::<Result<Vec<_>>>()
}