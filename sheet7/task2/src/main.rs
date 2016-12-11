use std::collections::HashSet;

fn main() {
    let test_pal = "ben";
    let test_count = &["d aldka", "odlw"];
    let test_sum = &[2, -4, 5, 1, 0];

    println!("Factorial for 3: {}" ,factorial(3));
    println!("Is {} a palindrome? : {}", test_pal, is_palindrome(test_pal));
    println!("Letter count in {:?} : {}", test_count, used_chars_count(test_count));
    println!("Highst subsum in {:?} is: {:?}", test_sum, greatest_subsequencial_sum(test_sum));
    println!("Rot13 encoding of peter is: {:?}", rot13("peter"));
    println!("Rot26 \"encoding\" of peter is: {:?}", rot26("peter"));
}

/// Computes factorial of a given number
fn factorial(num: u64) -> u64 {
    (1..num+1).fold(1, | fac, x | fac * x)
}

/// Checks whether a word is a palindrome
fn is_palindrome(word: &str) -> bool {
    word.chars()
        .take(word.len()/2)
        .zip(word.chars().rev().take(word.len()/2))
        .all( |c| c.0 == c.1 )
}

/// Counts number of differing character in a slice of string slices
fn used_chars_count(words: &[&str]) -> usize {
    let count: HashSet<char> = words.iter()
                                    .flat_map(|&x|
                                        x.chars()
                                        .filter(|a| !a.is_whitespace()))
                                    .collect();

    count.len()
}


/// Looks for greatest subsequencial sum in a slice
fn greatest_subsequencial_sum(numbers: &[i64]) -> &[i64] {
    let subslices = numbers.split(|&num| num <= 0);
    subslices.max_by_key::<i64, _>(|num| num.iter().sum()).unwrap()
}

/// Encodes string with very useful and
/// secure rot26 "encryption"
fn rot26(item: &str) -> String {
    item.trim()
        .chars()
        .map(|c| ascii_add(c, 26))
        .collect::<String>()
}

/// Encodes string with rot13 encryption
/// Sadly I did not find a way to return a string slice
fn rot13(item: &str) -> String {
    item.trim()
        .chars()
        .map(|c| ascii_add(c, 13))
        .collect::<String>()
}

/// Rotates a character a specified number of characters
/// further in the alphabet (upper and lower case)
fn ascii_add(item: char, add: u8) -> char {
    if item.is_lowercase() {
        char::from(in_range(item as u8 + add, 97, 122))
    } else {
        char::from(in_range(item as u8 + add, 65, 90))
    }
}

/// Makes sure character stays in range of possible values
fn in_range(x: u8, min: u8, max: u8) -> u8 {
    if x > max {
        in_range(min + (x - max - 1), min, max)
    }
    else {
        x
    }
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(15), 1_307_674_368_000);
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("bob"));
    assert!(is_palindrome("anna"));
    assert!(is_palindrome("lagerregal"));

    assert!(!is_palindrome("peter"));
}

#[test]
fn test_greatest_subsequencial_sum() {
    let a = [1, 2, 39, 34, 20, -20, -16, 35, 0];
    assert_eq!(greatest_subsequencial_sum(&a), &a[0..5]);

    let b = [-3, -9, -8, -34];
    assert_eq!(greatest_subsequencial_sum(&b), &[]);
}

#[test]
fn test_rot13() {
    assert_eq!(&rot13("hello"), "uryyb");
    assert_eq!(&rot13("uryyb"), "hello");

    assert_eq!(
        &rot13("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"),
        "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm"
    );

    assert_eq!(&rot13("peter"), "crgre");
}

#[test]
fn test_used_letters() {
    assert_eq!(used_chars_count(&["hi", "ih gitt"]), 4);
    assert_eq!(used_chars_count(&["peter"]), 4);
    assert_eq!(used_chars_count(&["p e t e r", "barbara"]), 6);
}
