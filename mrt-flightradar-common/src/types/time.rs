use std::{
    fmt::{Display, Formatter},
    ops::Add,
    str::FromStr,
};

use anyhow::{anyhow, Error};
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    h: u8,
    m: u8,
}
impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}{:02}", self.h, self.m)
    }
}
impl Add<f32> for Time {
    type Output = Time;
    fn add(mut self, rhs: f32) -> Self::Output {
        let h = rhs.floor() as u8;
        let m = ((rhs - rhs.floor()) * 60.0).round() as u8;
        self.h += h;
        self.m += m;
        self.simplified()
    }
}
impl Time {
    fn simplified(mut self) -> Self {
        while self.m >= 60 {
            self.m -= 60;
            self.h += 1;
        }
        while self.h >= 24 {
            self.h -= 24;
        }
        self
    }
}
impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([0-9]{2})([0-9]{2})$")?
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid format"))?;
        Ok(Self {
            h: re
                .get(1)
                .ok_or_else(|| anyhow!("No group 1"))?
                .as_str()
                .parse()?,
            m: re
                .get(2)
                .ok_or_else(|| anyhow!("No group 2"))?
                .as_str()
                .parse()?,
        }
        .simplified())
    }
}
