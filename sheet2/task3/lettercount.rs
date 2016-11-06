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
    //let count = string.chars().count();
    let mut letter_count = 0;

    //let mut chars = string.chars();

    for x in string.chars()
    {
        if x==letter
        {
            letter_count = letter_count + 1;
        }
    }

    return letter_count;
}
