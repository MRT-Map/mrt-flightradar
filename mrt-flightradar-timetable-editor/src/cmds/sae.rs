use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn sae(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let flight_segment = arg!(cmd_str "flight_segment" get_flight_segment, air_facilities, file.flights[index].segments.last())?;
    file.flights[index].segments.push(flight_segment);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, sae, to_cmd_str, Action};

    #[test]
    fn sae_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#"0 AB1234 PRA 0000"#);
        assert_eq!(
            sae(&mut cmd_str, &mut file, air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful segment insert"
        );
        assert_eq!(
            file.flights[0].segments[2].flight_no,
            SmolStr::from("AB1234"),
            "Faulty segment insert"
        );

        let mut cmd_str = to_cmd_str!(r#"0 AB1234 PRA"#);
        assert_eq!(
            sae(&mut cmd_str, &mut file, air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful segment insert"
        );

        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (air_facilities, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    sae(&mut cmd_str, &mut file, &air_facilities).is_err(),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(sae_no_idx, "");
    assert_err!(sae_no_flight, "0");
}
