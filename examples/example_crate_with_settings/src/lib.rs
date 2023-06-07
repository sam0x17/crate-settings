use crate_settings::*;

/// This add method is configurable via crate settings. By default, it will function like a
/// regular add method, however if `example_crate_with_settings.extra` is present as an
/// integer, it will be added to the final result.
pub fn add(left: usize, right: usize) -> usize {
    left + right + settings!("example_crate_with_settings", "extra", 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
