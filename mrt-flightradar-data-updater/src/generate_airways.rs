use common::data_types::{airway::Airway, waypoint::Waypoint};

fn nearest_waypoints<'a>(waypoints: &'a [Waypoint], wp: &Waypoint) -> Vec<&'a Waypoint> {
    let mut radius = 0.0;
    let mut nearest = vec![];
    while nearest.len() < 3 {
        radius += 1000.0;
        nearest = waypoints
            .iter()
            .filter(|w| *w != wp)
            .filter(|w| w.coords.distance(wp.coords) < radius)
            .collect()
    }
    nearest
}

pub fn generate_airways(waypoints: &[Waypoint]) -> Vec<Airway> {
    let mut airways = vec![];
    for wp in waypoints {
        for nw in nearest_waypoints(waypoints, wp) {
            let airway = Airway {
                waypoint1: wp.name.to_owned(),
                waypoint2: nw.name.to_owned(),
            };
            if !airways.contains(&airway) {
                airways.push(airway);
            }
        }
    }
    println!("{:#?}", airways);
    airways
}
