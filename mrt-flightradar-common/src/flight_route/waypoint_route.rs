use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use glam::Vec2;
use smol_str::SmolStr;
use tracing::trace;

use crate::data_types::{
    vec::{FromLoc, Pos},
    waypoint::Waypoint,
    RAW_DATA,
};

fn a_star(start: &'static Waypoint, end: &'static Waypoint) -> Option<Vec<&'static Waypoint>> {
    let to_wp = |n: &SmolStr| RAW_DATA.waypoints.iter().find(|w| w.name == *n).unwrap();
    let h = |n: &SmolStr| to_wp(n).coords.distance(end.coords);
    let neighbours = |n: &SmolStr| {
        RAW_DATA
            .airways
            .iter()
            .filter_map(|aw| {
                if aw.waypoint1 == *n {
                    Some(&aw.waypoint2)
                } else if aw.waypoint2 == *n {
                    Some(&aw.waypoint1)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let mut came_from = HashMap::<&SmolStr, &SmolStr>::new();
    let mut g_score = HashMap::from([(&start.name, 0.0)]);
    let mut f_score = HashMap::from([(&start.name, h(&start.name))]);

    while let Some((mut current, _)) = f_score
        .iter()
        .map(|(a, b)| (*a, *b))
        .min_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
    {
        if **current == end.name {
            let mut total_path = vec![to_wp(current)];
            while let Some(new_current) = came_from.get(current) {
                current = new_current;
                total_path.push(to_wp(current));
            }
            total_path.reverse();
            trace!(path = ?total_path.iter().map(|a| &a.name).collect::<Vec<_>>(), "Path found");
            return Some(total_path);
        }
        f_score.remove(current);

        for neighbour in neighbours(current) {
            let tent_g = *g_score.get(current).unwrap_or(&f32::INFINITY)
                + to_wp(current).coords.distance(to_wp(neighbour).coords);
            if tent_g < *g_score.get(neighbour).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tent_g);
                f_score.insert(neighbour, tent_g + h(neighbour));
            }
        }
    }
    trace!("Couldn't find path");
    None
}

#[tracing::instrument(skip_all)]
pub fn get_waypoint_route(
    start: FromLoc,
    end: FromLoc,
) -> Result<(Vec<&'static Waypoint>, Vec<Pos<Vec2>>)> {
    let start_wp = RAW_DATA
        .waypoints
        .iter()
        .min_by_key(|wp| start.head().distance(wp.coords) as u32)
        .ok_or_else(|| eyre!("No waypoints found"))?;
    let end_wp = RAW_DATA
        .waypoints
        .iter()
        .min_by_key(|wp| end.tail.distance(wp.coords) as u32)
        .ok_or_else(|| eyre!("No waypoints found"))?;
    trace!(?start_wp, ?end_wp);

    let waypoints = a_star(start_wp, end_wp).ok_or_else(|| eyre!("No route found"))?;

    Ok((
        waypoints.to_owned(),
        waypoints.iter().map(|wp| wp.coords).collect(),
    ))
}
