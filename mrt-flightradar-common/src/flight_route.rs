use glam::Vec2;
use itertools::Itertools;

use crate::types::{
    iter::BefAftWindowIterator,
    path::Path,
    vec::{Direction, FromLoc, Pos},
    Rotation,
};

pub fn get_route_between_waypoints(
    start_vec: FromLoc<Vec2>,
    start_rot: Rotation,
    end_centre: Pos<Vec2>,
    end_rot: Rotation,
    max_turn_radius: f32,
) -> Vec<Path> {
    let start_centre: Pos<Vec2> = start_vec.head()
        + start_vec.vec.perp().normalize()
            * max_turn_radius
            * if start_rot == Rotation::Anticlockwise {
                1.0
            } else {
                -1.0
            };
    let main_path_vec = end_centre - start_centre;
    let main_path_from_loc = if start_rot == end_rot {
        let radius_vec = main_path_vec.perp().normalize()
            * max_turn_radius
            * if start_rot == Rotation::Anticlockwise {
                -1.0
            } else {
                1.0
            };
        FromLoc {
            tail: start_centre + radius_vec,
            vec: main_path_vec,
        }
    } else {
        let angle = (max_turn_radius * 2.0 / main_path_vec.length()).acos()
            * if start_rot == Rotation::Anticlockwise {
                -1.0
            } else {
                1.0
            };
        let radius_vec =
            main_path_vec.normalize().rotate(Vec2::from_angle(angle)) * max_turn_radius;
        FromLoc {
            tail: start_centre + radius_vec,
            vec: radius_vec.perp().normalize()
                * (main_path_vec.length_squared() - 4.0 * max_turn_radius.powi(2)).sqrt()
                * if start_rot == Rotation::Anticlockwise {
                    1.0
                } else {
                    -1.0
                },
        }
    };

    let curve = Path::Curve {
        centre: start_centre,
        from: start_vec.head(),
        angle: start_vec.vec.angle_between(main_path_from_loc.vec),
    };

    vec![curve, Path::Straight(main_path_from_loc)]
}

pub fn get_flight_route(
    start: FromLoc<Vec2>,
    end: FromLoc<Vec2>,
    waypoints: Vec<Pos<Vec2>>,
    max_turn_radius: f32,
) -> Vec<Path> {
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
            let next_paths = get_route_between_waypoints(
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
                let mut next_paths = get_route_between_waypoints(
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

    route
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::vec2;

    use crate::{
        flight_route::get_route_between_waypoints,
        types::{path::Path, vec::FromLoc, Rotation},
    };

    #[test]
    fn direct_common_tangent_anticlockwise() {
        assert_eq!(
            get_route_between_waypoints(
                FromLoc {
                    tail: vec2(0.0, 1.0),
                    vec: vec2(0.0, -1.0)
                },
                Rotation::Anticlockwise,
                vec2(3.0, 0.0),
                Rotation::Anticlockwise,
                1.0
            ),
            vec![
                Path::Curve {
                    centre: vec2(1.0, 0.0),
                    from: vec2(0.0, 0.0),
                    angle: PI / 2.0 - 0.0000001
                },
                Path::Straight(FromLoc {
                    tail: vec2(1.0, -1.0),
                    vec: vec2(2.0, 0.0)
                })
            ]
        )
    }
    #[test]
    fn direct_transverse_tangent_anticlockwise() {
        assert_eq!(
            get_route_between_waypoints(
                FromLoc {
                    tail: vec2(0.0, 1.0),
                    vec: vec2(0.0, -1.0)
                },
                Rotation::Anticlockwise,
                vec2(3.0, -2.0),
                Rotation::Clockwise,
                1.0
            ),
            vec![
                Path::Curve {
                    centre: vec2(1.0, 0.0),
                    from: vec2(0.0, 0.0),
                    angle: PI / 2.0 - 0.0000001
                },
                Path::Straight(FromLoc {
                    tail: vec2(1.0, -1.0 + 0.00000006),
                    vec: vec2(2.0, 0.0)
                })
            ]
        )
    }
}
