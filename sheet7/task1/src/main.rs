extern crate num_traits;

use num_traits::Unsigned;
use std::iter::Iterator;
use std::fmt;

fn main() {
    Fibonacci::<u32>::show_fib(20);
}

struct Fibonacci<T> {
    current: T,
    last: T,
}

/// Implements Fibonacci sequence
impl<T> Fibonacci<T>
    where T: Unsigned + Copy + fmt::Display
{
    fn new_fib() -> Self {
        Fibonacci{ current: T::zero() , last: T::zero() }
    }

    pub fn show_fib(end: T) {
        let mut me = Fibonacci::<T>::new_fib();
        let mut index : T = T::zero();

        while index != end{
            println!("{}", me.next().unwrap());
            index = index + T::one();
        }
    }
}

/// Implements next() for Fibonacci Iterator
impl<T> Iterator for Fibonacci<T>
    where T: Copy + Unsigned + fmt::Display
{
    type Item = T;

    // On call returns next Fibonacci number
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_zero() {
            self.current = T::one();
            self.last = T::zero();
        } else {
            let tmp = self.current;
            self.current = self.current + self.last;
            self.last = tmp;
        }
        Some(self.current)
    }
}

