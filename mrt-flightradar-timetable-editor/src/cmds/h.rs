use anyhow::Result;
use bunt::println;

use crate::Action;

pub fn h() -> Result<Action> {
    let cmds = [
        ("q", "", "Quit the editor"),
        ("h", "", "View this page"),
        (
            "i",
            "<index> \"<aircraft>\" <reg> <a1> <d1> <a2> [d2]",
            "Add flight to buffer (Aircraft must be in quotes)",
        ),
        (
            "is",
            "\"<aircraft>\" <reg> <a1> <d1> <a2> [d2]",
            "Add flight to start of buffer",
        ),
        (
            "ie",
            "\"<aircraft>\" <reg> <a1> <d1> <a2> [d2]",
            "Add flight to end of buffer",
        ),
        (
            "c",
            "<index> <field> <new_value>",
            "Change value of field of flight in buffer",
        ),
        ("d", "<index>", "Remove flight from buffer"),
        (
            "m",
            "<index1> <index2>",
            "Move a flight at index1 to index2",
        ),
        (
            "e",
            "<a1> <d1> <a2>",
            "Estimate an arrival time for a flight",
        ),
    ];
    for (cmd, args, desc) in cmds {
        println!("{[cyan+bold]} {[yellow]}\n{}", cmd, args, desc);
    }
    return Ok(Action::Hold);
}
