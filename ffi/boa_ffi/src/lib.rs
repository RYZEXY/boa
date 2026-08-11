//! Diplomat FFI bridge exposing Boa's JavaScript engine over a C ABI.

/// Diplomat bridge entry points
#[diplomat::bridge]
pub mod ffi {
    use boa_engine::{
        Context, JsBigInt as BoaJsBigInt, JsObject as BoaJsObject, JsSymbol as BoaJsSymbol,
        JsValue as BoaJsValue, Source, js_string,
    };
    use boa_engine::property::Attribute;
    use diplomat_runtime::{DiplomatOption, DiplomatWrite};
    use std::fmt::Write as _;

    /// An owned Boa JS engine, exposed across the FFI boundary as an opaque handle.
    #[derive(Debug)]
    #[diplomat::opaque_mut]
    pub struct BoaContext(Context);

    /// A simple owned JavaScript value exposed across the FFI boundary.
    #[derive(Debug, Clone)]
    #[diplomat::opaque]
    pub struct JsValue(BoaJsValue);

    impl JsValue {
        /// Constructs a new numeric JS value directly from a native number with no JS source text involved.
        #[must_use]
        pub fn from_number(n: f64) -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::from(n)))
        }

        /// Returns true when this value is a number.
        pub fn is_number(&self) -> bool {
            self.0.is_number()
        }

        /// Returns the numeric payload when this value is a number.
        pub fn as_number(&self) -> DiplomatOption<f64> {
            self.0.as_number().into()
        }

        /// Constructs the JS `null` value.
        #[must_use]
        pub fn from_null() -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::null()))
        }

        /// Returns true when this value is `null`.
        pub fn is_null(&self) -> bool {
            self.0.is_null()
        }

        /// Constructs the JS `undefined` value.
        #[must_use]
        pub fn from_undefined() -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::undefined()))
        }

        /// Returns true when this value is `undefined`.
        pub fn is_undefined(&self) -> bool {
            self.0.is_undefined()
        }

        /// Constructs a new boolean JS value directly from a native bool,
        /// with no JS source text involved.
        #[must_use]
        pub fn from_boolean(b: bool) -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::from(b)))
        }

        /// Returns true when this value is a boolean.
        pub fn is_boolean(&self) -> bool {
            self.0.is_boolean()
        }

        /// Returns the boolean payload when this value is a boolean.
        pub fn as_boolean(&self) -> DiplomatOption<bool> {
            self.0.as_boolean().into()
        }

        /// Constructs a new string JS value directly from native text, using
        /// Boa's string APIs rather than evaluating JS source text.
        #[must_use]
        pub fn from_string(s: &str) -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::from(js_string!(s))))
        }

        /// Returns true when this value is a string.
        pub fn is_string(&self) -> bool {
            self.0.is_string()
        }

        /// Writes the string payload when this value is a string.
        /// Writes nothing if it is not — call `is_string` first to distinguish.
        pub fn as_string(&self, write: &mut DiplomatWrite) {
            if let Some(s) = self.0.as_string() {
                let _ = write!(write, "{}", s.to_std_string_lossy());
            }
        }

        /// Returns true when this value is an object (including functions and arrays).
        pub fn is_object(&self) -> bool {
            self.0.is_object()
        }

        /// Converts this value into a `JsObject` handle if it is an object.
        /// Returns `None` (a null pointer in C) for non-object values. This
        /// is the same underlying object, not a copy — see `JsObject::as_value`.
        pub fn as_object(&self) -> Option<Box<JsObject>> {
            self.0.as_object().map(|o| Box::new(JsObject(o)))
        }

        /// Constructs a BigInt JS value from its decimal string representation.
        /// Returns `None` if the string is not a valid BigInt literal
        /// (arbitrary precision is preserved — this never narrows to i64/u64/f64).
        pub fn from_bigint_string(s: &str) -> Option<Box<JsValue>> {
            BoaJsBigInt::from_string(s).map(|b| Box::new(JsValue(BoaJsValue::from(b))))
        }

        /// Returns true when this value is a `BigInt`.
        pub fn is_bigint(&self) -> bool {
            self.0.is_bigint()
        }

        /// Writes the full decimal representation when this value is a BigInt.
        /// Writes nothing otherwise — call `is_bigint` first to distinguish.
        /// Uses arbitrary-precision string conversion, never narrowed to a
        /// fixed-width integer or float.
        pub fn as_bigint_string(&self, write: &mut DiplomatWrite) {
            if let Some(b) = self.0.as_bigint() {
                let _ = write!(write, "{}", b.to_string_radix(10));
            }
        }

        /// Constructs a Symbol JS value with the given description.
        ///
        /// Note: the description does not determine the symbol's identity —
        /// two symbols constructed with the same description are still
        /// distinct values, matching `Symbol("x") !== Symbol("x")` in JS.
        #[must_use]
        pub fn from_symbol(description: &str) -> Box<JsValue> {
            let symbol = BoaJsSymbol::new(Some(js_string!(description)))
                .expect("practically unreachable: symbol id space (u64::MAX) exhausted");
            Box::new(JsValue(BoaJsValue::from(symbol)))
        }

        /// Constructs a Symbol JS value with no description.
        #[must_use]
        pub fn from_symbol_no_description() -> Box<JsValue> {
            let symbol = BoaJsSymbol::new(None)
                .expect("practically unreachable: symbol id space (u64::MAX) exhausted");
            Box::new(JsValue(BoaJsValue::from(symbol)))
        }

        /// Returns true when this value is a `Symbol`.
        pub fn is_symbol(&self) -> bool {
            self.0.is_symbol()
        }

        /// Returns true when this value is a `Symbol` that has a description.
        pub fn symbol_has_description(&self) -> bool {
            self.0
                .as_symbol()
                .is_some_and(|s| s.description().is_some())
        }

        /// Writes the description when this value is a `Symbol` with one.
        /// Writes nothing otherwise — call `is_symbol`/`symbol_has_description`
        /// first to distinguish "not a symbol", "no description", and
        /// "has an empty-string description".
        pub fn as_symbol_description(&self, write: &mut DiplomatWrite) {
            if let Some(Some(desc)) = self.0.as_symbol().map(|s| s.description()) {
                let _ = write!(write, "{}", desc.to_std_string_lossy());
            }
        }
    }

    /// An owned JS object, exposed across the FFI boundary as an opaque handle.
    ///
    /// This is a foundational wrapper only: it establishes the `JsValue` <->
    /// `JsObject` conversion and identity preservation. Property access,
    /// arrays, functions, and error handling are deliberately out of scope
    /// here and land in later, separate work.
    #[derive(Debug, Clone)]
    #[diplomat::opaque]
    pub struct JsObject(BoaJsObject);

    impl JsObject {
        /// Converts this object back into a `JsValue` containing the exact
        /// same underlying JS object (a cheap reference clone of Boa's
        /// internal `Gc` pointer, not a serialization or deep copy) — so JS
        /// object identity (`===`) is preserved across the conversion.
        #[must_use]
        pub fn as_value(&self) -> Box<JsValue> {
            Box::new(JsValue(BoaJsValue::from(self.0.clone())))
        }
    }

    impl BoaContext {
        /// Creates a fresh JS engine instance.
        #[must_use]
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

        /// Evaluates a JS source string and returns the result as an owned JS value.
        pub fn eval_value(&mut self, src: &str) -> Box<JsValue> {
            let result = self.0.eval(Source::from_bytes(src));
            match result {
                Ok(value) => Box::new(JsValue(value)),
                Err(_) => Box::new(JsValue(BoaJsValue::undefined())),
            }
        }

        /// Binds a JS value under a name in the engine's global scope, so
        /// subsequent `eval`/`eval_value` calls can reference it directly.
        pub fn set_global(&mut self, name: &str, value: &JsValue) {
            drop(self.0.register_global_property(
                js_string!(name),
                value.0.clone(),
                Attribute::all(),
            ));
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

    #[test]
    fn eval_js_value_number() {
        let mut ctx = ffi::BoaContext::new();
        let value = ctx.eval_value("2 + 3");
        assert_eq!(value.as_number().into_option(), Some(5.0));
    }

    #[test]
    fn eval_js_value_non_number_has_no_number() {
        let mut ctx = ffi::BoaContext::new();
        let value = ctx.eval_value("'hello'");
        assert!(!value.is_number());
        assert_eq!(value.as_number().into_option(), None);
        assert!(value.is_string());

        let mut write = RustWriteVec::with_capacity(16);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        unsafe { value.as_string(write.borrow_mut()) };
        assert_eq!(write.borrow().as_bytes(), b"hello");
    }

    #[test]
    fn set_global_round_trip() {
        let mut ctx = ffi::BoaContext::new();
        let input = ffi::JsValue::from_number(21.5);
        ctx.set_global("sensorReading", &input);

        let result = ctx.eval_value("sensorReading * 2 + 1");
        assert_eq!(result.as_number().into_option(), Some(44.0));
    }

    #[test]
    fn from_boolean_round_trip() {
        let mut ctx = ffi::BoaContext::new();
        let value = ffi::JsValue::from_boolean(true);
        assert_eq!(value.as_boolean().into_option(), Some(true));

        ctx.set_global("flag", &value);
        let result = ctx.eval_value("!flag");
        assert_eq!(result.as_boolean().into_option(), Some(false));
    }

    #[test]
    fn from_string_round_trip() {
        let mut ctx = ffi::BoaContext::new();
        let value = ffi::JsValue::from_string("world");
        assert!(value.is_string());

        ctx.set_global("name", &value);
        let result = ctx.eval_value("'hello ' + name");
        let mut write = RustWriteVec::with_capacity(16);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        unsafe { result.as_string(write.borrow_mut()) };
        assert_eq!(write.borrow().as_bytes(), b"hello world");
    }

    #[test]
    fn from_null_round_trip() {
        let mut ctx = ffi::BoaContext::new();
        let value = ffi::JsValue::from_null();
        assert!(value.is_null());

        ctx.set_global("nothing", &value);
        let result = ctx.eval_value("nothing === null");
        assert_eq!(result.as_boolean().into_option(), Some(true));
    }

    #[test]
    fn from_undefined_round_trip() {
        let mut ctx = ffi::BoaContext::new();
        let value = ffi::JsValue::from_undefined();
        assert!(value.is_undefined());

        ctx.set_global("nothing", &value);
        let result = ctx.eval_value("nothing === undefined");
        assert_eq!(result.as_boolean().into_option(), Some(true));
    }

    #[test]
    fn from_bigint_string_invalid_is_none() {
        let result = ffi::JsValue::from_bigint_string("not a number");
        assert!(result.is_none());
    }

    #[test]
    fn bigint_round_trip_beyond_64_bits() {
        let mut ctx = ffi::BoaContext::new();
        // 30 digits — far beyond u64::MAX (20 digits) and f64's exact
        // integer range (2^53, 16 digits). Proves no narrowing occurs.
        let huge = "123456789012345678901234567890";
        let value = ffi::JsValue::from_bigint_string(huge).expect("valid bigint literal");
        assert!(value.is_bigint());

        let mut write = RustWriteVec::with_capacity(64);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        unsafe { value.as_bigint_string(write.borrow_mut()) };
        assert_eq!(write.borrow().as_bytes(), huge.as_bytes());

        // Round trip through real JS BigInt arithmetic and back.
        ctx.set_global("bigValue", &value);
        let result = ctx.eval_value("bigValue + 1n");
        assert!(result.is_bigint());
        let mut write2 = RustWriteVec::with_capacity(64);
        // Safety: `write2` is the only `DiplomatWrite` instance in scope.
        unsafe { result.as_bigint_string(write2.borrow_mut()) };
        assert_eq!(
            write2.borrow().as_bytes(),
            b"123456789012345678901234567891"
        );
    }

    #[test]
    fn from_symbol_description_round_trip() {
        let value = ffi::JsValue::from_symbol("mySymbol");
        assert!(value.is_symbol());
        assert!(value.symbol_has_description());

        let mut write = RustWriteVec::with_capacity(32);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        unsafe { value.as_symbol_description(write.borrow_mut()) };
        assert_eq!(write.borrow().as_bytes(), b"mySymbol");
    }

    #[test]
    fn from_symbol_no_description() {
        let value = ffi::JsValue::from_symbol_no_description();
        assert!(value.is_symbol());
        assert!(!value.symbol_has_description());
    }

    #[test]
    fn symbol_set_global_type_check() {
        let mut ctx = ffi::BoaContext::new();
        let value = ffi::JsValue::from_symbol("tag");
        ctx.set_global("sym", &value);
        let result = ctx.eval_value("typeof sym");
        let mut write = RustWriteVec::with_capacity(16);
        // Safety: `write` is the only `DiplomatWrite` instance in scope.
        unsafe { result.as_string(write.borrow_mut()) };
        assert_eq!(write.borrow().as_bytes(), b"symbol");
    }

    #[test]
    fn symbol_identity_not_description() {
        let mut ctx = ffi::BoaContext::new();
        // Same description, but must be distinct symbols — matches
        // `Symbol("x") !== Symbol("x")` in real JS.
        let sym_a = ffi::JsValue::from_symbol("x");
        let sym_b = ffi::JsValue::from_symbol("x");
        ctx.set_global("symA", &sym_a);
        ctx.set_global("symB", &sym_b);

        let distinct = ctx.eval_value("symA === symB");
        assert_eq!(distinct.as_boolean().into_option(), Some(false));

        // A symbol is still equal to itself.
        let same = ctx.eval_value("symA === symA");
        assert_eq!(same.as_boolean().into_option(), Some(true));
    }

    #[test]
    fn eval_object_converts_to_js_object() {
        let mut ctx = ffi::BoaContext::new();
        let value = ctx.eval_value("({x: 1})");
        assert!(value.is_object());
        assert!(value.as_object().is_some());
    }

    #[test]
    fn non_object_value_has_no_js_object() {
        let mut ctx = ffi::BoaContext::new();
        let value = ctx.eval_value("42");
        assert!(!value.is_object());
        assert!(value.as_object().is_none());
    }

    #[test]
    fn js_object_converts_back_to_js_value() {
        let mut ctx = ffi::BoaContext::new();
        let value = ctx.eval_value("({y: 2})");
        let obj = value.as_object().expect("value is an object");
        let round_tripped = obj.as_value();
        assert!(round_tripped.is_object());
    }

    #[test]
    fn js_value_to_js_object_to_js_value_preserves_identity() {
        let mut ctx = ffi::BoaContext::new();
        // A real, distinguishable object created inside the engine.
        let original = ctx.eval_value("({tag: 'unique'})");
        assert!(original.is_object());

        // Round-trip: JsValue -> JsObject -> JsValue.
        let obj = original.as_object().expect("value is an object");
        let round_tripped = obj.as_value();

        // Bind both the original and the round-tripped value into the JS
        // global scope under different names, then let JS itself judge
        // identity with `===` — the authoritative check, not Rust-level
        // wrapper equality.
        ctx.set_global("original", &original);
        ctx.set_global("roundTripped", &round_tripped);
        let same = ctx.eval_value("original === roundTripped");
        assert_eq!(same.as_boolean().into_option(), Some(true));

        // Prove this isn't "any object with the same shape looks equal" —
        // a genuinely different object must NOT be ===, even with
        // identical contents.
        let other = ctx.eval_value("({tag: 'unique'})");
        ctx.set_global("other", &other);
        let different = ctx.eval_value("original === other");
        assert_eq!(different.as_boolean().into_option(), Some(false));
    }
}
