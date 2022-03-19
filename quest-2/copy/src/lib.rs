pub fn nbr_function(c: u32) -> (u32, f64, f64) {
    return (c, (c as f64).exp(), (c as f64).ln().abs());
}

pub fn str_function(a: String) -> (String, String) {
    return (
        a.clone(),
        a.split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| (s.parse::<u32>().unwrap() as f64).exp().to_string())
            .collect::<Vec<String>>()
            .join(" "),
    );
}

pub fn vec_function(b: Vec<u32>) -> (Vec<u32>, Vec<f64>) {
    return (
        b.clone(),
        b.iter().map(|c| (*c as f64).ln()).collect::<Vec<f64>>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbr_fn() {
        let (c, exp, ln) = nbr_function(0);

        assert_eq!(c, 0);
        assert_eq!(exp, 1.0);
        assert_eq!(ln, f64::INFINITY);
    }

    #[test]
    fn test_str_fn() {
        let (a, b) = str_function("1 2 3 4 5".to_owned());

        assert_eq!(a, "1 2 3 4 5");
        assert_eq!(b, "2.718281828459045 7.38905609893065 20.085536923187668 54.598150033144236 148.4131591025766");
    }

    #[test]
    fn test_vec_fn() {
        let (b, c) = vec_function(vec![1, 2, 3, 4, 5]);

        assert_eq!(b, vec![1, 2, 3, 4, 5]);
        assert_eq!(
            c,
            vec![
                0.0,
                0.6931471805599453,
                1.0986122886681098,
                1.3862943611198906,
                1.6094379124341003
            ]
        );
    }
}
