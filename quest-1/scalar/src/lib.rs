extern crate num;

use num::One;
use num::Zero;

pub fn sum_range<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Clone + One + Zero + Copy,
{
    let mut acc = T::zero();

    let mut a = a.clone();
    let mut b = b.clone();

    if a > b {
        let tmp = a;
        a = b;
        b = tmp;
    }

    while a <= b {
        acc = acc + a;
        a = a + T::one();
    }

    acc
}

pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_overflow_sum() {
        sum(1, 255);
    }

    #[test]
    #[should_panic]
    fn test_overflow_diff() {
        diff(-32768, 32766);
    }

    #[test]
    #[should_panic]
    fn test_overflow_pro() {
        pro(-128, 2);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(234, 2), 236);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
    }

    #[test]
    fn test_pro() {
        assert_eq!(pro(23, 2), 46);
    }

    #[test]
    fn test_quo() {
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(quo(-128.23, 2.0), -64.115);
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(22.0, 2.0), 0.0);
        assert_eq!(rem(-128.23, 2.0), -0.22999573);
    }
}   


