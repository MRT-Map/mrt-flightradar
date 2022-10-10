use glam::Vec2;

use crate::{
    data_types::vec::{FromLoc, Pos},
    flight_route::types::Angle,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Path {
    Straight(FromLoc),
    Curve {
        centre: Pos<Vec2>,
        from: Pos<Vec2>,
        angle: Angle,
    },
}

impl Path {
    fn length(&self) -> f32 {
        match &self {
            Path::Straight(fl) => fl.vec.length(),
            Path::Curve {
                from,
                centre,
                angle,
            } => (*centre - *from).length() * angle.abs(),
        }
    }
}

pub const ACCEL: f32 = 2.5; // m/s^2
pub const MAX_SPEED: f32 = 15.0; // m/s

#[derive(Debug, Clone)]
pub struct FlightPath(pub Vec<Path>);

impl FlightPath {
    pub fn length(&self) -> f32 {
        self.0.iter().map(Path::length).sum()
    }

    fn hits_max_speed(t: f32) -> bool {
        (ACCEL * t / 2.0) > MAX_SPEED
    }

    pub fn time_taken(&self) -> f32 {
        let mut t = (8.0 * self.length() / ACCEL).sqrt();
        if FlightPath::hits_max_speed(t) {
            t = (self.length() / MAX_SPEED) + (MAX_SPEED / ACCEL);
        }
        t
    }

    pub fn pos_at_time(&self, z: f32) -> Option<Pos<Vec2>> {
        let t = self.time_taken();
        let mut s = if FlightPath::hits_max_speed((8.0 * self.length() / ACCEL).sqrt()) {
            match z {
                z if z < 0.0 => None,
                z if z <= MAX_SPEED / ACCEL => Some(ACCEL * z.powi(2) / 2.0),
                z if z <= t - MAX_SPEED / ACCEL => {
                    Some(MAX_SPEED.powi(2) / (2.0 * ACCEL) + MAX_SPEED * (z - MAX_SPEED / ACCEL))
                }
                z if z <= t => Some(self.length() - (t - z).powi(2) / 2.0),
                _ => None,
            }?
        } else {
            match z {
                z if z < 0.0 => None,
                z if z <= t / 2.0 => Some(ACCEL * z.powi(2) / 2.0),
                z if z <= t => Some(self.length() - (ACCEL * (t - z.powi(2))) / 2.0),
                _ => None,
            }?
        };
        for path in self.0.iter() {
            if path.length() < s {
                s -= path.length();
                continue;
            }
            return Some(match path {
                Path::Straight(from_loc) => from_loc.tail + from_loc.vec.normalize() * s,
                Path::Curve {
                    from,
                    centre,
                    angle,
                } => {
                    *centre + (*from - *centre).rotate(Vec2::from_angle(s / path.length() * angle))
                }
            });
        }
        None
    }
}
