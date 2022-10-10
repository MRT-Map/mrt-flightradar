use std::{iter::Peekable, str::Split};

use color_eyre::eyre::{eyre, Result};

use crate::{airport_names::get_airport_names, arg, Action};

pub fn n(cmd_str: &mut Peekable<Split<char>>) -> Result<Action> {
    let airport = arg!(cmd_str "airport" get_airport)?;
    let airport_names = get_airport_names()?;
    let name = if let Some(name) = airport_names.get(&airport) {
        name
    } else {
        return Err(eyre!("Airport {airport} is not recorded"));
    };
    Ok(Action::Msg(format!("{airport} is {name}")))
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{n, to_cmd_str, Action};

    #[test]
    fn n_normal() -> Result<()> {
        let mut cmd_str = to_cmd_str!("KBN");
        assert!(
            matches!(n(&mut cmd_str).unwrap(), Action::Msg(_)),
            "Unsuccessful name lookup"
        );
        Ok(())
    }

    macro_rules! assert_err {
        ($fn_name:ident, $cmd:literal) => {
            #[test]
            fn $fn_name() -> Result<()> {
                let mut cmd_str = to_cmd_str!($cmd);
                assert!(
                    matches!(n(&mut cmd_str), Err(_)),
                    "`{}` did not error",
                    stringify!($fn_name)
                );
                Ok(())
            }
        };
    }

    assert_err!(n_no_airport, "");
    assert_err!(n_unregisted_airport, "???");
}
