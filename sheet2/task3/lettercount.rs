fn main()
{
    let string = "Peter";
    let letter = 'e';

    let letter_count = count(string, letter);

    println!("Word: {}, Letter: {} --> Occurences: {}", string, letter, letter_count);
}

// write `count()` function
fn count(string: &str, letter: char) -> i32
{
    let mut letter_count = 0;

    for x in string.chars() {

        if x==letter {
            letter_count += 1;
        }
    }

    letter_count
}
