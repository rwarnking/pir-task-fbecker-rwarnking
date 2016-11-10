fn main() {

    let mut x = 27;
    let mut y = 0;

    while x != 1 {

        println!("{} -> {}", y, x);

        if x % 2 == 0 {
            x = x / 2;
        }
        else {
            x = 3 * x + 1;
        }
        y += 1;
    }
    println!("{} -> {}", y, x);
}
