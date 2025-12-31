use quickjs_sys::*;
use crate::context::Context;

pub struct Runtime {
    rt: *mut JSRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        unsafe {
            let rt = JS_NewRuntime();
            assert!(!rt.is_null());
            Self { rt }
        }
    }

    pub fn context(&self) -> Context {
        unsafe {
            let ctx = JS_NewContext(self.rt);
            assert!(!ctx.is_null());
            Context { ctx }
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JS_FreeRuntime(self.rt);
        }
    }
}
