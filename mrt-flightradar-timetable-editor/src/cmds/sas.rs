use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, Action};

pub fn sas(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let flight_segment = arg!(cmd_str "flight_segment" get_flight_segment, air_facilities, None)?;
    file.flights[index].segments.insert(0, flight_segment);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, sas, to_cmd_str, Action};

    #[test]
    fn sas_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#"0 AB1234 PRA 0000"#);
        assert_eq!(
            sas(&mut cmd_str, &mut file, &air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful segment insert"
        );
        assert_eq!(
            file.flights[0].segments[0].flight_no,
            SmolStr::from("AB1234"),
            "Faulty segment insert"
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
                    matches!(sas(&mut cmd_str, &mut file, &air_facilities), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(sas_no_idx, "");
    assert_err!(sas_no_flight, "0");
    assert_err!(sas_estimation, r#"0 AB1234 PRA"#);
}
