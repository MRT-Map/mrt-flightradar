use glam::Vec2;

use crate::{
    data_types::vec::{FromLoc, Pos},
    flight_route::types::{path::Path, Rotation},
};

#[tracing::instrument]
pub fn get_path_between_waypoints(
    start_vec: FromLoc,
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
    let ratio = max_turn_radius * 2.0 / main_path_vec.length();
    let main_path_from_loc = if start_rot == end_rot || ratio.acos().is_nan() {
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
        let angle = ratio.acos()
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
                * 4.0f32
                    .mul_add(-max_turn_radius.powi(2), main_path_vec.length_squared())
                    .sqrt()
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

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::vec2;
    use proptest::{prelude::*, proptest};

    use crate::{
        data_types::vec::FromLoc,
        flight_route::{
            between_waypoints::get_path_between_waypoints,
            types::{path::Path, Rotation},
        },
    };

    proptest! {
        #[test]
        fn get_route_wont_crash(
            (stx, sty) in any::<(f32, f32)>(),
            (svx, svy) in any::<(f32, f32)>(),
            (ex, ey) in any::<(f32, f32)>(),
            max_r in any::<f32>()
        ) {
            get_path_between_waypoints(
                FromLoc {
                    tail: vec2(stx, sty),
                    vec: vec2(svx, svy)
                },
                Rotation::Anticlockwise,
                vec2(ex, ey),
                Rotation::Anticlockwise,
                max_r,
            );
        }
    }

    #[test]
    fn direct_common_tangent_anticlockwise() {
        assert_eq!(
            get_path_between_waypoints(
                FromLoc {
                    tail: vec2(0.0, 1.0),
                    vec: vec2(0.0, -1.0)
                },
                Rotation::Anticlockwise,
                vec2(3.0, 0.0),
                Rotation::Anticlockwise,
                1.0,
            ),
            vec![
                Path::Curve {
                    centre: vec2(1.0, 0.0),
                    from: vec2(0.0, 0.0),
                    angle: PI / 2.0 - 0.000_000_1
                },
                Path::Straight(FromLoc {
                    tail: vec2(1.0, -1.0),
                    vec: vec2(2.0, 0.0)
                })
            ]
        );
    }
    #[test]
    fn direct_transverse_tangent_anticlockwise() {
        assert_eq!(
            get_path_between_waypoints(
                FromLoc {
                    tail: vec2(0.0, 1.0),
                    vec: vec2(0.0, -1.0)
                },
                Rotation::Anticlockwise,
                vec2(3.0, -2.0),
                Rotation::Clockwise,
                1.0,
            ),
            vec![
                Path::Curve {
                    centre: vec2(1.0, 0.0),
                    from: vec2(0.0, 0.0),
                    angle: PI / 2.0 - 0.000_000_1
                },
                Path::Straight(FromLoc {
                    tail: vec2(1.0, -1.0 + 0.000_000_06),
                    vec: vec2(2.0, 0.0)
                })
            ]
        );
    }
}
