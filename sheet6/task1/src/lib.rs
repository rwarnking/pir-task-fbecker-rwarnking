extern crate num_traits;

use num_traits::Num;

/// Clamps a numerical value between two other values
pub fn clamp<T>(value: T, min: T, max: T) -> T
    where T: PartialOrd + Copy
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Returns a tuple containing the sum and product of two numbers
pub fn sum_product<T> (first: T, second: T) -> (T, T)
    where T: Num + Copy
{
    (first + second, first * second)
}

/// Makes an option out of another type
pub trait IntoOptionExt<T> {
    fn into_option(self, T) -> Option<T>;
}

impl<T> IntoOptionExt<T> for bool {
    fn into_option(self, other: T) -> Option<T> {
        match self {
            true => Some(other),
            false => None,
        }
    }
}

