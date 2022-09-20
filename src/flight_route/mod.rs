use std::f32::consts::PI;

use glam::Vec2;

use crate::flight_route::types::{
    BefAftWindowIterator, Direction, FromLoc, LMR, Path, Pos, Rotation,
};

mod types;

fn get_route_between_waypoints(
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
        -1.0
    } else {
        1.0
    };
    let main_path_vec = end_centre - start_centre;
    if start_rot == end_rot {
        let radius_vec = main_path_vec.perp().normalize()
            * max_turn_radius
            * if start_rot == Rotation::Anticlockwise {
            -1.0
        } else {
            1.0
        };
        let main_path_from_vec = Path::Straight(FromLoc {
            tail: start_centre + radius_vec,
            vec: main_path_vec,
        });
        let curve = Path::Curve {
            centre: start_centre,
            from: start_vec.head(),
            angle: main_path_vec.angle_between(start_vec.vec)
                - if start_rot == Rotation::Clockwise {
                PI
            } else {
                0.0
            },
        };
        vec![curve, main_path_from_vec]
    } else {
        unimplemented!()
    }
}

fn get_flight_route(
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
    );
    unimplemented!()
}
