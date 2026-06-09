use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + PartialEq + Rem<Output = T>,
    u8: Into<T>,
{
    let zero = 0_u8.into();
    while b != zero {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + Div<Output = T> + Mul<Output = T> + Rem<Output = T>,
    u8: Into<T>,
{
    let zero = 0_u8.into();
    if a == zero || b == zero {
        return zero;
    }

    (a / gcd(a, b)) * b
}
