extern crate num_traits;

use num_traits::Num;
use std::ops::{Add, Mul};

/// A vector with two coordinates
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
    where T: Num
{
    pub fn origin() -> Vector2<T> {
        Vector2 { x: <T>::zero(), y: <T>::zero() }
    }

    pub fn unit_x() -> Vector2<T> {
        Vector2 { x: <T>::one(), y: <T>::zero() }
    }

    pub fn unit_y() -> Vector2<T> {
        Vector2 { x: <T>::zero(), y: <T>::one() }
    }
}

/// Overwrites '+' operator for the type 'Vector2', vector multiplication
impl<T> Add for Vector2<T>
    where T: Num + Copy
{
    type Output = Vector2<T>;

    // Returns (a1+b1, a2+b2)
    fn add(self, other: Vector2<T>) -> Self::Output {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

/// Overwrites '*' operator for the type 'Vector2', scalar multiplication
impl<T> Mul<T> for Vector2<T>
    where T: Num + Copy
{
    type Output = Vector2<T>;
    // Returns (a1*b, a2*b)
    fn mul(self, other: T) -> Self::Output {
        Vector2 { x : self.x * other, y: self.y * other }
    }
}

