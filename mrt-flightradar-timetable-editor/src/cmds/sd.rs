use std::{iter::Peekable, str::Split};

use anyhow::Result;
use common::data_types::timetable::AirlineTimetable;

use crate::{arg, cmds::get_index, Action};

pub fn sd(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, le)?;
    let segment_index = get_index(
        cmd_str,
        |i| i < file.flights[index].segments.len(),
        "seg_index",
    )?;
    file.flights[index].segments.remove(segment_index);
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{cmds::test_setup, sd, to_cmd_str, Action};

    #[test]
    fn sd_normal() -> Result<()> {
        let (_, mut file) = test_setup()?;

        let mut cmd_str = to_cmd_str!(r#"0 1"#);
        assert_eq!(
            sd(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful segment remove"
        );
        assert_eq!(file.flights[0].segments.len(), 1, "Faulty segment remove");
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (_, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(sd(&mut cmd_str, &mut file), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(sd_no_idx, "");
    assert_err!(sd_no_seg_idx, "0");
    assert_err!(sd_invalid_seg_idx, "0 2");
}
