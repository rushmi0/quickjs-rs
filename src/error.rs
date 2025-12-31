use quickjs_sys::*;
use std::ffi::{CStr, NulError};

#[derive(Debug)]
pub struct JsError {
    pub message: String,
}

impl JsError {
    pub fn from_ctx(ctx: *mut JSContext) -> Self {
        unsafe {
            let exc = JS_GetException(ctx);
            let msg = js_to_cstring(ctx, exc);

            let message = if msg.is_null() {
                "unknown JS error".to_string()
            } else {
                let s = CStr::from_ptr(msg).to_string_lossy().to_string();
                JS_FreeCString(ctx, msg);
                s
            };

            JS_FreeValue(ctx, exc);
            Self { message }
        }
    }
}

/* ===== FIX #1: allow `?` with CString::new ===== */
impl From<NulError> for JsError {
    fn from(_: NulError) -> Self {
        JsError {
            message: "string contains null byte".into(),
        }
    }
}
