use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn is(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let flight = arg!(cmd_str "flight" get_flight, air_facilities)?;
    file.flights.insert(0, flight);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, is, to_cmd_str, Action};

    #[test]
    fn is_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;
        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000"#);
        assert_eq!(
            is(&mut cmd_str, &mut file, air_facilities).unwrap(),
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
                    is(&mut cmd_str, &mut file, &air_facilities).is_err(),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(is_no_idx, "");
}
