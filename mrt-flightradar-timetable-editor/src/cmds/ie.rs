use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn ie(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let flight = arg!(cmd_str "flight" get_flight, air_facilities)?;
    file.flights.push(flight);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, ie, to_cmd_str, Action};

    #[test]
    fn ie_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;
        let mut cmd_str = to_cmd_str!(r#""Test Aircraft" REG AB1234 PRA 0000"#);
        assert_eq!(
            ie(&mut cmd_str, &mut file, &air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful insert"
        );
        assert_eq!(file.flights[1].aircraft, SmolStr::from("Test Aircraft"));
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (air_facilities, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(ie(&mut cmd_str, &mut file, &air_facilities), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(ie_no_idx, "");
}
