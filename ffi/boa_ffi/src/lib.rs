//! Diplomat FFI bridge exposing Boa's JavaScript engine over a C ABI.

/// Diplomat bridge entry points for the demo crate.
#[diplomat::bridge]
pub mod ffi {
    use boa_engine::{Context, Source};
    use diplomat_runtime::DiplomatWrite;
    use std::fmt::Write as _;

    /// An owned Boa JS engine, exposed across the FFI boundary as an opaque handle.
    #[derive(Debug)]
    #[diplomat::opaque_mut]
    pub struct BoaContext(Context);

    impl BoaContext {
        /// Creates a fresh JS engine instance.
        pub fn new() -> Box<BoaContext> {
            Box::new(BoaContext(Context::default()))
        }

        /// Evaluates a JS source string and writes the result (or error) as text.
        pub fn eval(&mut self, src: &str, write: &mut DiplomatWrite) {
            let result = self.0.eval(Source::from_bytes(src));
            let output = match result {
                Ok(value) => match value.to_string(&mut self.0) {
                    Ok(s) => s.to_std_string_lossy(),
                    Err(e) => format!("Uncaught {e}"),
                },
                Err(e) => format!("Uncaught {e}"),
            };
            // The DiplomatWrite buffer only fails to accept writes when capped;
            // there's nothing more useful to do with that here.
            let _ = write!(write, "{output}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
    use diplomat_runtime::rust_interop::RustWriteVec;

    #[test]
    fn eval_js_addition() {
        let mut ctx = ffi::BoaContext::new();
        let mut write = RustWriteVec::with_capacity(16);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        ctx.eval("2 + 2", unsafe { write.borrow_mut() });
        assert_eq!(write.borrow().as_bytes(), b"4");
    }

}
