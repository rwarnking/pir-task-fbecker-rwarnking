fn main() {
    for i in 1..5 {
        println!("magic({}) --> {}", i, magic(i));
    }
}

/// rough draft of what the function "might" look like
fn magic(param: u64) -> bool {
    let mut a = param;
    let mut b = 2;
    let mut div = 0;

    loop {
        if b >= a {
            return true
        }
        div = a % b;
        a /= b;
        b += 1;

        if div == 0 {
            break;
        }
    }

    false
}
