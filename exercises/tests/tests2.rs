// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(42, 42);
        assert_eq!("42", "42");
        assert_eq!(String::from("42"), String::from("42"));
    }
}
