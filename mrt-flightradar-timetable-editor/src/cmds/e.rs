use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::{airport::AirFacility, vec::Pos};
use glam::Vec2;

use crate::{arg, cmds::get_main_coord, Action};

pub fn e(cmd_str: &mut Peekable<Split<char>>, air_facilities: &[AirFacility]) -> Result<Action> {
    let a1 = arg!(cmd_str "a1" get_str)?;
    let d1 = arg!(cmd_str "d1" get_time)?;
    let a2 = arg!(cmd_str "a2" get_str)?;
    let c1 = get_main_coord(&a1, air_facilities)?;
    let c2 = get_main_coord(&a2, air_facilities)?;
    let time = estimate_time(c1, c2);
    let d2 = d1 + time;
    Ok(Action::Msg(format!(
        "Flight arrives at {d2} after {time:.2} hours"
    )))
}

pub fn estimate_time(c1: &Pos<Vec2>, c2: &Pos<Vec2>) -> f32 {
    (((c2.x - c1.x) + (c2.y - c1.y)) / 5000.0).abs()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{cmds::test_setup, e, to_cmd_str, Action};

    #[test]
    fn e_normal() -> Result<()> {
        let (air_facilities, _) = test_setup()?;
        let mut cmd_str = to_cmd_str!("PRA 0000 KBN");
        assert!(
            matches!(e(&mut cmd_str, &air_facilities).unwrap(), Action::Msg(_)),
            "Unsuccessful estimation"
        );
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (air_facilities, _) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(e(&mut cmd_str, &air_facilities), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(e_no_a1, "");
    assert_err!(e_no_d1, "PRA");
    assert_err!(e_no_a2, "PRA 0000");
    assert_err!(e_invalid_a1, "??? 0000 KBN");
    assert_err!(e_invalid_a2, "PRA 0000 ???");
}
