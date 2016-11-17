//! Task 3.1: Rule 90
fn main() {
    let mut cycle = read_input();

    // go through 20 cycles and print each cell
    // two 'fullblock' for a 1 (true) and two 'light shade' otherwise
    for _ in 0..20 {
        for index in 0..cycle.len() {
            if cycle[index] {
                print!("\u{2588}\u{2588}");
            } else {
                print!("\u{2591}\u{2591}");
            }
        }
        print!("\n");
        cycle = next_step(&cycle);
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }
        buffer.trim().to_string()
    };

    // construct bool vector from string
    let mut output = Vec::new();
    for letter in input.chars() {
        if letter == '0' {
            output.push(false);
        } else if letter == '1' {
            output.push(true);
        }
    }
    output
}

// returns a vector containing the next step for each cell of 'old' according to the rules
fn next_step(old: &[bool]) -> Vec<bool> {
    let mut changed = Vec::new();
    let old_owned = old;

    for index in 0..old_owned.len() {
        // if we are at the first element, set previous element as the last
        let prev = if index == 0 { old.len() - 1 } else { index - 1 };
        // if we are at the last element, set next element as the first
        let next = if index == old.len() - 1 { 0 } else { index + 1 };
        // implements rule 1_1 -> 101, (1_0 || 0_1) -> _1_, 0_0 -> 000
        let new_value = match old[prev] && old[next] {
            true => false,
            false => if old[prev] || old[next] { true } else { false },
        };
        changed.push(new_value);
    }
    changed
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
