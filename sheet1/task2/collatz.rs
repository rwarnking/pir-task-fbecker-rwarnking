fn main() {

    let mut x: i32 = 27;
    let mut y: i32 = 0;
    
    while x != 1
    {
        println!("{} -> {}", y, x);
        if x%2==0
        {
            x = x / 2;
        }
        else
        {
            x = 3 * x + 1;
        }
        y = y + 1;
    }
    println!("{} -> {}", y, x);
}
