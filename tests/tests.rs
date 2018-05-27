extern crate is_travis;

#[cfg(test)]
mod tests {
    use std::env;
    use is_travis::is_travis;

    #[test]
    fn not_travis() {
        env::remove_var("TRAVIS");
        env::remove_var("CI");
        assert!(!is_travis());
    }

    #[test]
    fn travis() {
        env::set_var("TRAVIS", "1".to_string());
        env::set_var("CI", "1".to_string());
        assert!(is_travis());
    }
}
