// print if they are both
fn main() -> () {

    for iterationnumber in 1..21 {
        if happy_prime(iterationnumber) {
            println!("{} is a happy prime!", iterationnumber);
        }
    }
}

// Is it both?
fn happy_prime(n: i32) -> bool {

    match check_if_number_is_happy(n) {
        false => false,
        _ => check_if_number_is_prime(n),
    }
}

// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn check_if_number_is_happy(number: i32) -> bool {
    match number {
        1 => true,
        4 => false,
        // noch nicht richtig
        _ => {
            let mut n = number;
            let mut tmp = 0;
            while n > 0 {
                tmp += (n%10) * (n%10);
                n = n/ 10;
            }
            check_if_number_is_happy(tmp)
        },
    }
}

// is it priem?
fn check_if_number_is_prime(n: i32) -> bool {
    match n {
        1 | 2 => false,
        _ => {
            for teiler in 2..n {
                if n % teiler == 0 { return false; }
            }
            true
        },
    }
}
