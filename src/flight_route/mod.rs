use glam::Vec2;

use crate::flight_route::types::{
    BefAftWindowIterator, Direction, FromLoc, Path, Pos, Rotation, LMR,
};

mod types;

fn get_route_between_waypoints(
    start_vec: FromLoc<Vec2>,
    start_rot: Option<Rotation>,
    end_centre: Pos<Vec2>,
    end_rot: Option<Rotation>,
    max_turn_radius: f32,
) -> Vec<Path> {


    unimplemented!()
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
        })
    );
    unimplemented!()
}
