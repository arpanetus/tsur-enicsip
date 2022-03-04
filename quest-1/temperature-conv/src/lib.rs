pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_fahrenheit_to_celsius() {
    	assert_eq!(fahrenheit_to_celsius(20.0), -6.666666666666667);
        assert_eq!(fahrenheit_to_celsius(83.0), 28.333333333333332);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(27.0), 80.6);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

}

