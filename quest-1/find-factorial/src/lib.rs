fn factorial_accumulated(n: u32, acc: u128) -> u128 {
    match n {
        0 => acc,
        _ => factorial_accumulated(n - 1, acc * n as u128)
    }
}

pub fn factorial(num: u32) -> u128 {
    factorial_accumulated(num, 1)
}

mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(120, factorial(5));
        assert_eq!(40320, factorial(8));
        assert_eq!(3628800, factorial(10));
        assert_eq!(87178291200, factorial(14));
        assert_eq!(6402373705728000, factorial(18));
        assert_eq!(121645100408832000, factorial(19));
        assert_eq!(2432902008176640000, factorial(20));
       }
}