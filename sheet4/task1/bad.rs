/// Print if they are both
fn main() -> () {

    for i in 1..21 {
        if is_happy_prime(i) {
            println!("{} is a happy prime!", i);
        }
    }
}

/// Is it both?
fn is_happy_prime(n: i32) -> bool {

    is_happy(n) && is_prime(n)
}

/// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn is_happy(number: i32) -> bool {
    match number {
        1 => true,
        4 => false,
        _ => {
            let mut n = number;
            let mut tmp = 0;
            while n > 0 {
                tmp += (n%10) * (n%10);
                n = n/ 10;
            }
            is_happy(tmp)
        },
    }
}

/// Is it prime?
fn is_prime(n: i32) -> bool {
    match n {
        1 => false,
        2 => true,
        _ => {
            for teiler in 2..n {
                if n % teiler == 0 { return false; }
            }
            true
        },
    }
}
