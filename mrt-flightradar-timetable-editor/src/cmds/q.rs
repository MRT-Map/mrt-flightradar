use std::path::PathBuf;

use anyhow::Result;
use common::types::timetable::AirlineTimetable;

use crate::cmds::Action;

pub fn q(file: &mut AirlineTimetable, path: &PathBuf) -> Result<Action> {
    file.to_file(path.to_owned())?;
    Ok(Action::Quit("Quitting".into()))
}
