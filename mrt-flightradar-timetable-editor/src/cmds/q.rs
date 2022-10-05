use anyhow::Result;

use crate::cmds::Action;

pub fn q() -> Result<Action> {
    Ok(Action::Quit("Quitting".into()))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{q, Action};

    #[test]
    fn q_normal() -> Result<()> {
        assert!(matches!(q().unwrap(), Action::Quit(_)), "Unsuccessful quit");
        Ok(())
    }
}
