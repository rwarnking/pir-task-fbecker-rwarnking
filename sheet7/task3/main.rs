fn main() {
    let my_test = [2, 4, 0];

    // product() simulation
    let fold_product = my_test.iter().fold(1, |pro, x| pro * x );
    let orig_product: i32 = my_test.iter().product();
    println!("product(): {}, fold(): {}", orig_product, fold_product);

    // max() simulation
    let fold_max = my_test.iter().fold(0, |max, &x| if x > max {x} else {max});
    let orig_max = *my_test.iter().max().unwrap();
    println!("max(): {}, fold(): {}", orig_max, fold_max);

    // all() simulation
    let fold_all = my_test.iter().fold(true, |test, &x| test && x % 2 == 0 );
    let orig_all = my_test.iter().all(|x| x % 2 == 0);
    println!("all(): {}, fold(): {}", orig_all, fold_all);
}

