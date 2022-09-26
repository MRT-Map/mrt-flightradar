use anyhow::{anyhow, Result};
use cached::proc_macro::cached;
use glam::Vec2;
use smol_str::SmolStr;

use crate::types::{coords_to_vec, from_csv, vec::Pos};

#[derive(Clone, PartialEq, Debug)]
pub struct Waypoint {
    pub name: SmolStr,
    pub coords: Pos<Vec2>,
}

#[cached(result = true)]
pub fn get_waypoints() -> Result<Vec<Waypoint>> {
    from_csv(include_str!("../../data/waypoints.csv"))
        .into_iter()
        .skip(1)
        .map(|row| {
            Ok(Waypoint {
                name: row.first().ok_or_else(|| anyhow!("No name"))?.into(),
                coords: coords_to_vec(row.get(1).ok_or_else(|| anyhow!("No coords"))?)?,
            })
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::types::waypoint::get_waypoints;

    #[test]
    fn waypoints_file_is_valid() -> Result<()> {
        get_waypoints()?;
        Ok(())
    }
}
