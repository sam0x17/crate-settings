pub use settings_macros::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        assert_eq!(settings!("example-crate", "some-key"), "hey");
    }

    #[test]
    fn test_integers() {
        assert_eq!(settings!("another-crate", "number"), 33);
    }

    #[test]
    fn test_floats() {
        assert_eq!(settings!("another-crate", "float"), 55.6);
    }

    #[test]
    fn test_date_times() {
        assert_eq!(
            settings!("cool_crate", "some_date_time"),
            "1979-05-27T07:32:00Z"
        )
    }

    #[test]
    fn test_bools() {
        assert_eq!(settings!("example-crate", "something_else"), false);
    }

    #[test]
    fn test_string_arrays() {
        assert_eq!(settings!("another-crate", "arr"), ["a", "b", "c"]);
    }

    #[test]
    fn test_mixed_arrays() {
        // mixed arrays get automatically loaded as string arrays
        assert_eq!(settings!("weird_crate", "mixed_arr"), ["hey", "1", "false"]);
    }

    #[test]
    fn test_tables() {
        // tables get automatically converted to strings
        assert_eq!(
            settings!("weird_crate", "table"),
            "{ key1 = \"hey\", key2 = 3 }"
        )
    }
}
