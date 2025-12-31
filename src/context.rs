use quickjs_sys::*;
use std::ffi::CString;
use std::ptr;

use crate::{JsError, JsValue};

pub struct Context {
    pub(crate) ctx: *mut JSContext,
}

impl Context {
    /* ---------- std / os ---------- */

    pub fn init_std(&self) {
        unsafe {
            js_std_add_helpers(self.ctx, 0, ptr::null_mut());
            js_init_module_std(self.ctx, b"std\0".as_ptr() as _);
            js_init_module_os(self.ctx, b"os\0".as_ptr() as _);
        }
    }

    /* ---------- eval ---------- */

    pub fn eval(&self, code: &str) -> Result<JsValue, JsError> {
        unsafe {
            let c = CString::new(code)?;

            let v = JS_Eval(
                self.ctx,
                c.as_ptr(),
                code.len(),
                b"<eval>\0".as_ptr() as _,
                JS_EVAL_TYPE_GLOBAL as _,
            );

            if js_is_exception(v) != 0 {
                return Err(JsError::from_ctx(self.ctx));
            }

            Ok(JsValue::new(self.ctx, v))
        }
    }

    /* ---------- module ---------- */

    pub fn eval_module(&self, name: &str, src: &str) -> Result<(), JsError> {
        unsafe {
            let src_c = CString::new(src)?;
            let name_c = CString::new(name)?;

            let v = JS_Eval(
                self.ctx,
                src_c.as_ptr(),
                src.len(),
                name_c.as_ptr(),
                (JS_EVAL_TYPE_MODULE | JS_EVAL_FLAG_COMPILE_ONLY) as _,
            );

            if js_is_exception(v) != 0 {
                return Err(JsError::from_ctx(self.ctx));
            }

            let mut len = 0usize;
            let buf = JS_WriteObject(
                self.ctx,
                &mut len,
                v,
                JS_WRITE_OBJ_BYTECODE as _,
            );
            JS_FreeValue(self.ctx, v);

            let obj = JS_ReadObject(self.ctx, buf, len, JS_READ_OBJ_BYTECODE as _);
            js_free(self.ctx as _, buf as _);

            let ret = JS_EvalFunction(self.ctx, obj);
            if js_is_exception(ret) != 0 {
                return Err(JsError::from_ctx(self.ctx));
            }

            JS_FreeValue(self.ctx, ret);
            Ok(())
        }
    }

    pub fn set_i32(&self, name: &str, value: i32) -> Result<(), JsError> {
        unsafe {
            let global = JS_GetGlobalObject(self.ctx);
            let key = CString::new(name)?;
            let val = js_new_int32(self.ctx, value);

            JS_SetPropertyStr(self.ctx, global, key.as_ptr(), val);
            JS_FreeValue(self.ctx, global);
            Ok(())
        }
    }


}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            JS_FreeContext(self.ctx);
        }
    }
}
