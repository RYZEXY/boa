#[diplomat::bridge]
pub mod ffi {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    fn add_two_integers() {
        assert_eq!(ffi::add(2, 3), 5);
    }
}
