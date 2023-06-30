use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn m(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index1 = arg!(cmd_str "index1" get_index, file, lt)?;
    let index2 = arg!(cmd_str "index2" get_index, file, le)?;
    if index1 < index2 {
        file.flights[index1..=index2].rotate_left(1);
    } else {
        file.flights[index2..=index1].rotate_right(1);
    }
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{cmds::test_setup, i, m, to_cmd_str, Action};

    #[test]
    fn m_normal() -> Result<()> {
        let (air_facilities, mut file) = test_setup()?;
        let mut cmd_str = to_cmd_str!(r#"0 "Test Aircraft" REG AB1234 PRA 0000"#);
        i(&mut cmd_str, &mut file, air_facilities).unwrap();
        let mut cmd_str = to_cmd_str!("0 1");
        assert_eq!(
            m(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful move"
        );
        assert_eq!(
            file.flights[1].aircraft,
            SmolStr::from("Test Aircraft"),
            "Faulty move"
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
                    i(&mut cmd_str, &mut file, &air_facilities).is_err(),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(m_no_idx1, "");
    assert_err!(m_no_idx2, "0");
    assert_err!(m_invalid_idx1, "2");
    assert_err!(m_invalid_idx2, "3");
}
