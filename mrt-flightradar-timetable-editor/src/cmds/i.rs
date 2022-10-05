use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn i(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let flight = arg!(cmd_str "flight" get_flight, air_facilities)?;
    file.flights.insert(index, flight);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, i, to_cmd_str, Action};

    #[test]
    fn i_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;
        let mut cmd_str = to_cmd_str!(r#"0 "Test Aircraft" REG AB1234 PRA 0000"#);
        assert_eq!(
            i(&mut cmd_str, &mut file, &air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful insert"
        );
        assert_eq!(file.flights[0].aircraft, SmolStr::from("Test Aircraft"));
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (air_facilities, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(i(&mut cmd_str, &mut file, &air_facilities), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(i_no_idx, "");
    assert_err!(i_no_flight, "0");
    assert_err!(i_invalid_idx, r#"100 "Test Aircraft" REG AB1234 PRA 0000"#);
}
