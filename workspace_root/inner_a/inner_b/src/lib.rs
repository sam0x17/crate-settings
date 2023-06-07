#[cfg(test)]
mod tests {
    use crate_settings::*;

    #[test]
    fn test_b_access_c_() {
        assert_eq!(settings!("inner_c", "some_inner_int"), 34);
    }

    #[test]
    fn test_b_access_b() {
        assert_eq!(settings!("inner_c", "some_bool"), true);
        assert_eq!(settings!("inner_c", "some_int"), 37);
    }

    #[test]
    fn test_b_access_a() {
        assert_eq!(settings!("inner_c", "something_further_out"), 44);
    }

    #[test]
    fn test_b_access_root() {
        assert_eq!(settings!("inner_c", "only_outer"), true);
    }
}
