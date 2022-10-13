use std::f32::consts::PI;

use glam::Vec2;
use itertools::Itertools;
use tracing::debug;

use crate::{
    data_types::vec::{Direction, FromLoc, Pos},
    flight_route::{
        between_waypoints::get_path_between_waypoints,
        types::{
            iter::BefAftWindowIterator,
            path::{FlightPath, Path},
            Rotation,
        },
    },
};

#[tracing::instrument(skip_all)]
pub fn get_flight_path(
    start: FromLoc,
    end: FromLoc,
    waypoints: Vec<Pos<Vec2>>,
    max_turn_radius: f32,
) -> FlightPath {
    let start_head = start.head();
    let end_head = end.head();
    debug!("Calculating rotations");
    let wp_rots = [(
        &start_head,
        start.turning_rot(*waypoints.first().unwrap_or(&end.tail)),
    )]
    .into_iter()
    .chain(
        BefAftWindowIterator::new(&waypoints).map(|(bef, this, aft)| {
            let bef = bef.unwrap_or(&start_head);
            let aft = aft.unwrap_or(&end_head);
            (this, FromLoc::new(*bef, *this).turning_rot(*aft))
        }),
    )
    .filter_map(|(wp, rot)| Some((wp, rot?)))
    .collect::<Vec<_>>();

    let mut start_vec = start;
    let mut route = vec![Path::Straight(start)];

    debug!("Calculating flight route");
    route.append(
        &mut wp_rots
            .iter()
            .tuple_windows::<(_, _)>()
            .flat_map(|((_, this_rot), (next_wp, next_rot))| {
                let next_paths = get_path_between_waypoints(
                    start_vec,
                    *this_rot,
                    **next_wp,
                    *next_rot,
                    max_turn_radius,
                );
                start_vec = if let Some(Path::Straight(fl)) = next_paths.last() {
                    *fl
                } else {
                    unreachable!()
                };
                next_paths
            })
            .collect(),
    );
    debug!("Calculating last leg of flight route");
    route.append(&mut {
        match start_vec.turning_rot(end.tail) {
            None => {
                vec![Path::Straight(FromLoc::new(start_vec.head(), end.tail))]
            }
            Some(rot) => {
                let end_centre = end.tail
                    + end.vec.perp().normalize()
                        * max_turn_radius
                        * if rot == Rotation::Anticlockwise {
                            -1.0
                        } else {
                            1.0
                        };
                let mut next_paths =
                    get_path_between_waypoints(start_vec, rot, end_centre, rot, max_turn_radius);
                next_paths.push(if let Some(Path::Straight(fl)) = next_paths.last() {
                    Path::Curve {
                        centre: end_centre,
                        from: fl.head(),
                        angle: fl.vec.angle_between(end.vec),
                    }
                } else {
                    unreachable!()
                });
                next_paths
            }
        }
    });
    route.push(Path::Straight(end));

    debug!("Fixing curve directions");
    for (i, (bef, _, after)) in BefAftWindowIterator::new(&route.to_owned()).enumerate() {
        let fl1 = if let Some(Path::Straight(fl)) = bef {
            fl
        } else {
            continue;
        };
        let fl2 = if let Some(Path::Straight(fl)) = after {
            fl
        } else {
            continue;
        };
        match fl1.turning_rot(fl2.tail) {
            Some(Rotation::Anticlockwise) => {
                if let Some(Path::Curve { angle, .. }) = route.get_mut(i) {
                    while *angle < 0.0 {
                        *angle += 2.0 * PI
                    }
                } else {
                    unreachable!()
                }
            }
            Some(Rotation::Clockwise) => {
                if let Some(Path::Curve { angle, .. }) = route.get_mut(i) {
                    while *angle > 0.0 {
                        *angle -= 2.0 * PI
                    }
                } else {
                    unreachable!()
                }
            }
            None => {}
        }
    }

    FlightPath(route)
}
