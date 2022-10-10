use std::{iter::Peekable, str::Split};

use color_eyre::eyre::Result;
use common::data_types::timetable::AirlineTimetable;

use crate::{arg, Action};

pub fn d(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, lt)?;
    file.flights.remove(index);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{cmds::test_setup, d, to_cmd_str, Action};

    #[test]
    fn d_normal() -> Result<()> {
        let (_, mut file) = test_setup()?;
        let mut cmd_str = to_cmd_str!("0");
        assert_eq!(
            d(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful delete"
        );
        assert_eq!(file.flights.len(), 0, "Faulty delete");
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (_, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(d(&mut cmd_str, &mut file), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(d_no_index, "");
    assert_err!(d_invalid_index, "2");
}
