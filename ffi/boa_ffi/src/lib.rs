/// Diplomat bridge entry points for the demo crate.
#[diplomat::bridge]
pub mod ffi {
    /// Adds two integers and returns the result.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32{
        a - b
    }

    pub fn mult(a: i32, b: i32) -> i32{
        a * b
    }

    pub fn div(a: i32, b: i32) -> i32{
        a / b
    }

}

#[unsafe(no_mangle)]
pub extern "C" fn boa_ffi_demo_add(a: i32, b: i32) -> i32 {
    ffi::add(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn boa_ffi_demo_sub(a: i32, b: i32) -> i32 {
    ffi::sub(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn boa_ffi_demo_mult(a: i32, b: i32) -> i32 {
    ffi::mult(a, b)
}
#[unsafe(no_mangle)]
pub extern "C" fn boa_ffi_demo_div(a: i32, b: i32) -> i32 {
    ffi::div(a, b)
}


#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    fn add_two_integers() {
        assert_eq!(ffi::add(2, 3), 5);
    }
}
