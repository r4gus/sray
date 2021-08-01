/// Compare two f64 floating point numbers for equality.
pub fn equal(lhs: f64, rhs: f64) -> bool {
    const EPSILON: f64 = 1e-10;
    (lhs - rhs).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::equal;

    #[test]
    fn compare_floating_point_number() {
        let x = 3.5_f64;
        let y = -3.5_f64;

        assert!(equal(x, -y));
        assert!(equal(-x, y));
        assert!(!equal(x, y));
        assert!(equal(x, y.powi(2).sqrt()));
    }
}
