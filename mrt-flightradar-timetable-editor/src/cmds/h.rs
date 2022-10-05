use anyhow::Result;
use bunt::println;

use crate::Action;

pub fn h() -> Result<Action> {
    println!("{$yellow+bold}Help menu{/$}");
    println!("{$yellow}Note:{/$} <segment> = <flight_no> <airport> <depart_time>");
    println!("{$yellow}Note:{/$} <segments> = <flight_no1> <airport1> <depart_time1> [<flight_no2> <airport2> <depart_time2> [<3> [<4> [<etc>...]]]]");
    println!("{$yellow}Note:{/$} <depart_time{$cyan}n{/$}> can be `_` for automatic estimation if {$cyan}n{/$} > 1");
    println!("{$yellow}Note:{/$} <depart_time{$cyan}n{/$}> can be omitted for automatic estimation if {$cyan}n{/$} is last");
    println!();
    let cmds = [
        ("q", "", "Quit the editor"),
        ("h", "", "View this page"),
        (
            "i",
            "<index> \"<aircraft>\" <reg> <segment>",
            "Insert flight into buffer (Aircraft must be in quotes)",
        ),
        (
            "is",
            "\"<aircraft>\" <reg> <segments>",
            "Add flight to start of buffer",
        ),
        (
            "ie",
            "\"<aircraft>\" <reg> <segments>",
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
        ("n", "<airport>", "Get the airport name, given the code"),
        (
            "sa",
            "<index> <segment_index> <segment>",
            "Insert flight segment into flight segment list. <depart_time> may be omitted if <segment_index> != 0",
        ),
        (
            "sae",
            "<index> <segment>",
            "Add flight segment to end of flight segment list. <depart_time> may be omitted if length of segment list != 0",
        ),
        (
            "sas",
            "<index> <segment>",
            "Add flight segment to start of flight segment list",
        ),
        (
            "sd",
            "<index> <segment_index>",
            "Remove flight segment from flight segment list",
        ),
    ];
    for (cmd, args, desc) in cmds {
        println!("{[cyan+bold]} {[yellow]}\n{}", cmd, args, desc);
    }
    Ok(Action::Hold)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{h, Action};

    #[test]
    fn h_normal() -> Result<()> {
        assert!(
            matches!(h().unwrap(), Action::Hold),
            "Unsuccessful help page"
        );
        Ok(())
    }
}
