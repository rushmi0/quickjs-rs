use quickjs_sys::*;

pub struct JsValue {
    ctx: *mut JSContext,
    val: JSValue,
}

impl JsValue {
    pub(crate) fn new(ctx: *mut JSContext, val: JSValue) -> Self {
        Self { ctx, val }
    }

    pub fn as_i32(&self) -> Result<i32, ()> {
        unsafe {
            let mut out = 0;
            if JS_ToInt32(self.ctx, &mut out, self.val) < 0 {
                Err(())
            } else {
                Ok(out)
            }
        }
    }
}

impl Drop for JsValue {
    fn drop(&mut self) {
        unsafe {
            JS_FreeValue(self.ctx, self.val);
        }
    }
}
