use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::types::{airport::AirFacility, timetable::AirlineTimetable};

use crate::{arg, cmds::get_index, Action};

pub fn sa(
    cmd_str: &mut Peekable<Split<char>>,
    file: &mut AirlineTimetable,
    air_facilities: &[AirFacility],
) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let segment_index = get_index(
        cmd_str,
        |i| i <= file.flights[index].segments.len(),
        "seg_index",
    )?;
    let flight_segment = arg!(cmd_str "flight_segment" get_flight_segment, air_facilities, if segment_index >= 1 {file.flights[index].segments.get(segment_index - 1)} else {None})?;
    file.flights[index]
        .segments
        .insert(segment_index, flight_segment);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, sa, to_cmd_str, Action};

    #[test]
    fn sa_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#"0 0 AB1234 PRA 0000"#);
        assert_eq!(
            sa(&mut cmd_str, &mut file, &air_facilities).unwrap(),
            Action::Refresh,
            "Unsuccessful segment insert"
        );
        assert_eq!(
            file.flights[0].segments[0].flight_no,
            SmolStr::from("AB1234"),
            "Faulty segment insert"
        );

        let mut cmd_str = to_cmd_str!(r#"0 1 AB1234 PRA"#);
        assert_eq!(
            sa(&mut cmd_str, &mut file, &air_facilities).unwrap(),
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
                    matches!(sa(&mut cmd_str, &mut file, &air_facilities), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(sa_no_idx, "");
    assert_err!(sa_no_seg_idx, "0");
    assert_err!(sa_no_flight, "0 0");
    assert_err!(sa_estimation_at_idx_0, "0 0 AB1234 PRA");
}
