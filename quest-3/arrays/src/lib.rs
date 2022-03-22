pub fn sum(a: &[i32]) -> i32 {
    a.iter().fold(0, |acc, el| acc + el)
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[test]
fn test_thirtytwo_tens() {
	assert_eq!(thirtytwo_tens(), [10; 32]);
}

#[test]
fn test_sum() {
	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	assert_eq!(sum(&a), a.iter().sum());
}