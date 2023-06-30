use color_eyre::eyre::{eyre, Result};
use common::{
    data_types::waypoint::Waypoint,
    flight_route::types::{coords_to_vec, from_csv},
};

pub fn get_waypoints(str: &str) -> Result<Vec<Waypoint>> {
    from_csv(str)
        .into_iter()
        .skip(1)
        .map(|row| {
            Ok(Waypoint {
                name: (*row.first().ok_or_else(|| eyre!("No name"))?).into(),
                coords: coords_to_vec(row.get(1).ok_or_else(|| eyre!("No coords"))?)?,
            })
        })
        .collect::<Result<Vec<_>>>()
}
