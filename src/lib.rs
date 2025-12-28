use quickjs_sys::*;

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
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JS_FreeRuntime(self.raw);
        }
    }
}
