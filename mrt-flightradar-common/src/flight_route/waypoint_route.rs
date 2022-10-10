use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use anyhow::{anyhow, Result};
use glam::Vec2;
use itertools::Itertools;
use smol_str::SmolStr;

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

    let mut open_set = BinaryHeap::from([Reverse(&start.name)]);
    let mut came_from = HashMap::<_, &_>::new();
    let mut g_score = HashMap::from([(&start.name, 0.0)]);
    let mut f_score = HashMap::from([(&start.name, h(&start.name))]);

    while let Some(Reverse(current)) = open_set.pop() {
        if **current == end.name {
            let mut total_path = vec![to_wp(current)];
            while let Some(current) = came_from.get(current) {
                total_path.push(to_wp(current));
            }
            total_path.reverse();
            return Some(total_path);
        }

        for neighbour in neighbours(current) {
            let tent_g = *g_score.get(current).unwrap_or(&f32::INFINITY)
                + to_wp(current).coords.distance(to_wp(neighbour).coords);
            if tent_g < *g_score.get(neighbour).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tent_g);
                f_score.insert(neighbour, tent_g + h(neighbour));
                if !open_set.iter().contains(&Reverse(neighbour)) {
                    open_set.push(Reverse(neighbour))
                }
            }
        }
    }
    None
}

pub fn get_waypoint_route(start: FromLoc, end: FromLoc) -> Result<Vec<Pos<Vec2>>> {
    let start_wp = RAW_DATA
        .waypoints
        .iter()
        .min_by_key(|wp| start.head().distance(wp.coords) as u32)
        .ok_or_else(|| anyhow!("No waypoints found"))?;
    let end_wp = RAW_DATA
        .waypoints
        .iter()
        .min_by_key(|wp| end.tail.distance(wp.coords) as u32)
        .ok_or_else(|| anyhow!("No waypoints found"))?;

    a_star(start_wp, end_wp)
        .map(|wps| wps.iter().map(|wp| wp.coords).collect())
        .ok_or_else(|| anyhow!("No route found"))
}
