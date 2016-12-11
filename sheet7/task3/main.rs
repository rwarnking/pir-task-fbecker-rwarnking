fn main() {
    let my_test = [2, 4, 0];

    // product() simulation
    let mut fold = my_test.iter().fold(1, |pro, x| pro * x );
    let mut orig = my_test.iter().product();
    println!("product(): {}, fold(): {}", orig, fold);

    // max() simulation
    fold = my_test.iter().fold(0, |max, &x| if x > max {x} else {max});
    orig = *my_test.iter().max().unwrap();
    println!("max(): {}, fold(): {}", orig, fold);

    // all() simulation
    fold = my_test.iter().fold(1, |max, &x| if x % 2 != 0 {0} else {max});
    let all = my_test.iter().all(|x| x % 2 == 0);
    println!("all(): {}, fold(): {}", all, fold==1);
}

