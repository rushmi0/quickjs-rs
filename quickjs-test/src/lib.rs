use quickjs_sys::*;
use std::ffi::{c_int, CString};

pub struct Runtime {
    raw: *mut JSRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            let rt = JS_NewRuntime();
            assert!(!rt.is_null());
            Self { raw: rt }
        }
    }

    pub fn context(&self) -> Context {
        unsafe {
            let ctx = JS_NewContext(self.raw);
            assert!(!ctx.is_null());
            Context { raw: ctx }
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JS_FreeRuntime(self.raw);
        }
    }
}

pub struct Context {
    raw: *mut JSContext,
}

impl Context {
    pub fn eval_i32(&self, code: &str) -> i32 {
        unsafe {
            let code = CString::new(code).unwrap();
            let filename = CString::new("<eval>").unwrap();

            let val = JS_Eval(
                self.raw,
                code.as_ptr(),
                code.as_bytes().len(),
                filename.as_ptr(),
                JS_EVAL_TYPE_GLOBAL as c_int,
            );

            if JS_HasException(self.raw) {
                let exc = JS_GetException(self.raw);
                JS_FreeValue(self.raw, exc);
                panic!("JavaScript exception occurred");
            }

            let mut result: i32 = 0;
            JS_ToInt32(self.raw, &mut result, val);
            JS_FreeValue(self.raw, val);
            result
        }
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            JS_FreeContext(self.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quickjs_eval_basic_math() {
        let rt = Runtime::new();
        let ctx = rt.context();

        let result = ctx.eval_i32("1 + 2 * 3");
        assert_eq!(result, 7);
    }

    #[test]
    fn quickjs_eval_variable() {
        let rt = Runtime::new();
        let ctx = rt.context();

        let result = ctx.eval_i32("let a = 40; a + 2;");
        assert_eq!(result, 42);
    }

    #[test]
    fn multiple_contexts_same_runtime() {
        let rt = Runtime::new();

        let ctx1 = rt.context();
        let ctx2 = rt.context();

        assert_eq!(ctx1.eval_i32("10 + 1"), 11);
        assert_eq!(ctx2.eval_i32("20 + 2"), 22);
    }
}
