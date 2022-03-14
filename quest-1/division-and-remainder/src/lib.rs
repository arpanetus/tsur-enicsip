pub fn divide(a: i32, b: i32) -> (i32, i32) {
    let d = a / b;
    let r = a % b;
    return (d, r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn divide_by_0() {
        divide(40, 0);
    }

    #[test]
    fn test_divide() {
        let (div, rem) = divide(40, 3);

        assert_eq!(div, 13);
        assert_eq!(rem, 1);

        let (div, rem) = divide(389, 39);

        assert_eq!(div, 9);
        assert_eq!(rem, 38);

        let (div, rem) = divide(29, 332);

        assert_eq!(div, 0);
        assert_eq!(rem, 29);
    }
}
