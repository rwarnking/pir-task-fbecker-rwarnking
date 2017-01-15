mod mycp;

use mycp::{get_command_input, mycp};

fn main() {
    if let Some((from, to)) = get_command_input() {
        mycp(&from, &to);
        println!("{} has been copied to {}", from, to);
    }
}

