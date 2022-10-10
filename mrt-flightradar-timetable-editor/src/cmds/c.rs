use std::{iter::Peekable, str::Split};

use color_eyre::eyre::{eyre, Result};
use common::data_types::timetable::AirlineTimetable;
use itertools::Itertools;
use regex::Regex;

use crate::{arg, Action};

pub fn c(cmd_str: &mut Peekable<Split<char>>, file: &mut AirlineTimetable) -> Result<Action> {
    let index = arg!(cmd_str "index" get_index, file, lt)?;
    let field = &*arg!(cmd_str "field" get_str)?;
    let value = cmd_str.take_while(|_| true).join(" ");
    if value.is_empty() {
        return Err(eyre!("Missing argument <value>"));
    }
    if field != "a" && value.contains(' ') {
        return Err(eyre!("Value cannot contain spaces"));
    }
    if field == "a" && value.contains('"') {
        return Err(eyre!("Aircraft cannot contain `\"`"));
    }
    match field {
        "a" => file.flights[index].aircraft = value.into(),
        "reg" => file.flights[index].registry = value.into(),
        field => {
            if let Some(re) = Regex::new(r"^a(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? - 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.airport = value.into()
                } else {
                    return Err(eyre!("No index {idx}"));
                }
            } else if let Some(re) = Regex::new(r"^f(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? - 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.flight_no = value.into()
                } else {
                    return Err(eyre!("No index {idx}"));
                }
            } else if let Some(re) = Regex::new(r"^d(\d+)$")?.captures(field) {
                let idx = re.get(1).unwrap().as_str().parse::<usize>()? - 1;
                if let Some(seg) = file.flights[index].segments.get_mut(idx) {
                    seg.depart_time = value.parse()?;
                } else {
                    return Err(eyre!("No index {idx}"));
                }
            } else {
                return Err(eyre!("Invalid field name `{field}`"));
            }
        }
    }
    Ok(Action::Refresh)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use smol_str::SmolStr;

    use crate::{c, cmds::test_setup, to_cmd_str, Action};

    #[test]
    fn c_normal() -> Result<()> {
        let (_, mut file) = test_setup()?;

        let mut cmd_str = to_cmd_str!("0 a New Aircraft");
        assert_eq!(
            c(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful set for `a`"
        );
        assert_eq!(
            file.flights[0].aircraft,
            SmolStr::from("New Aircraft"),
            "Faulty set for `a`"
        );

        let mut cmd_str = to_cmd_str!("0 reg NEW_REG");
        assert_eq!(
            c(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful set for `reg`"
        );
        assert_eq!(
            file.flights[0].registry,
            SmolStr::from("NEW_REG"),
            "Faulty set for `reg`"
        );

        let mut cmd_str = to_cmd_str!("0 a2 PQR");
        assert_eq!(
            c(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful set for `a2`"
        );
        assert_eq!(
            file.flights[0].segments[1].airport,
            SmolStr::from("PQR"),
            "Faulty set for `a2`"
        );

        let mut cmd_str = to_cmd_str!("0 d2 1234");
        assert_eq!(
            c(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful set for `d2`"
        );
        assert_eq!(
            file.flights[0].segments[1].depart_time,
            "1234".parse().unwrap(),
            "Faulty set for `d2`"
        );

        let mut cmd_str = to_cmd_str!("0 f2 AB1234");
        assert_eq!(
            c(&mut cmd_str, &mut file).unwrap(),
            Action::Refresh,
            "Unsuccessful set for `f2`"
        );
        assert_eq!(
            file.flights[0].segments[1].flight_no,
            SmolStr::from("AB1234"),
            "Faulty set for `f2`"
        );
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let (_, mut file) = test_setup()?;
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(c(&mut cmd_str, &mut file), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(c_no_index, "");
    assert_err!(c_no_field, "0");
    assert_err!(c_no_value, "0 a");
    assert_err!(c_spaces, "0 reg A B");
    assert_err!(c_aircraft_quote_marks, "0 a Aircraft\"");
    assert_err!(c_out_of_index_1, "0 a4 ABC");
    assert_err!(c_out_of_index_2, "0 d4 0000");
    assert_err!(c_out_of_index_3, "0 f4 AB1234");
    assert_err!(c_field_nonexistent, "0 foo bar");
}
