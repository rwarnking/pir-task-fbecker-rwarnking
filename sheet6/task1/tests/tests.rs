extern crate task1;

use task1::*;

#[test]
fn test_clamp() {
    assert_eq!(4, clamp(3, 4, 5));
    assert_eq!(2.0, clamp(2.0, 0.0, 10.0));
}

#[test]
fn test_sum_product() {
    assert_eq!((4, 4), sum_product(2, 2));
}

#[test]
fn test_option_trait() {
    assert_eq!(None, false.into_option(3));
    assert_eq!(Some(3), true.into_option(3));
}