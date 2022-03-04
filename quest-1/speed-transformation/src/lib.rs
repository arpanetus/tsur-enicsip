
pub fn km_per_hour_to_meters_per_second(kph: f64) -> f64 {
    kph/3.6
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_per_hour_to_meters_per_second() {
        assert_eq!(km_per_hour_to_meters_per_second(90.0), 25.0);
        assert_eq!(km_per_hour_to_meters_per_second(50.0), 13.88888888888889);
        assert_eq!(km_per_hour_to_meters_per_second(10.0), 2.7777777777777777);
        assert_eq!(km_per_hour_to_meters_per_second(100.0), 27.77777777777778);
    }
}

