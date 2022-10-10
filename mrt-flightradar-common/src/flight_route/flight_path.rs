use glam::Vec2;
use itertools::Itertools;

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

pub fn get_flight_path(
    start: FromLoc,
    end: FromLoc,
    waypoints: Vec<Pos<Vec2>>,
    max_turn_radius: f32,
) -> FlightPath {
    let start_head = start.head();
    let end_head = end.head();
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
    let mut route = wp_rots
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
        .collect::<Vec<_>>();
    route.append(&mut {
        match start_vec.turning_rot(end.tail) {
            None => {
                vec![Path::Straight(FromLoc::new(start_vec.head(), end.tail))]
            }
            Some(rot) => {
                let end_centre = end.vec.perp().normalize()
                    * max_turn_radius
                    * if rot == Rotation::Anticlockwise {
                        -1.0
                    } else {
                        1.0
                    };
                let mut next_paths = get_path_between_waypoints(
                    start_vec,
                    rot,
                    end_centre,
                    if start_vec.intersects(end) {
                        rot
                    } else {
                        rot.opp()
                    },
                    max_turn_radius,
                );
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

    FlightPath(route)
}
