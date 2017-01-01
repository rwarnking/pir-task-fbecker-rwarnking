extern crate num_traits;

use num_traits::int::PrimInt;

/// Uses trait 'Times' for different numeric types
fn main() {
    let i: i64 = 5;
    println!("Mit i64 (5):");
    i.times(|n| println!("\t--> {}", n));

    let j: u8 = 1;
    println!("Mit u8 (1):");
    j.times(|n| println!("\t--> {}", n));

    let k: isize = -1;
    println!("Mit isize (-1):");
    k.times(|n| println!("\t--> {}!", n));
}

/// Allows a closure to be called the amount of times self dictates
trait Times {
    type Input;

    fn times<F>(self, mut f: F) where F: FnMut(Self::Input);
}

/// Implementation of the trait 'Times' for all integers (signed and unsigned)
impl<T> Times for T
    where T: PrimInt
{
    type Input = T;

    fn times<F>(self, mut f: F)
        where F: FnMut(Self::Input)
    {
        if self > T::zero() {
            let mut index = T::zero();
            while index < self {
                f(index);
                index = index + T::one();
            }
        }
    }
}
