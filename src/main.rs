use std::ffi::{c_int, CStr, CString};
use std::ptr;
use quickjs_sys::*;

fn compile(
    ctx: *mut JSContext,
    source: &str,
    id: &str,
) -> Vec<u8> {

    unsafe {
        let src = CString::new(source).unwrap();
        let module_name = CString::new(id).unwrap();

        let flags = JS_EVAL_TYPE_MODULE | JS_EVAL_FLAG_COMPILE_ONLY;

        let val = JS_Eval(
            ctx,
            src.as_ptr(),
            source.len(),
            module_name.as_ptr(),
            flags as c_int,
        );

        if js_is_exception(val) != 0 {
            JS_FreeValue(ctx, val);
            panic!("compile error");
        }

        let mut len: usize = 0;
        let buf = JS_WriteObject(
            ctx,
            &mut len as *mut usize,
            val,
            JS_WRITE_OBJ_BYTECODE as c_int,
        );

        JS_FreeValue(ctx, val);

        if buf.is_null() {
            panic!("JS_WriteObject failed");
        }

        let bytecode = std::slice::from_raw_parts(buf, len).to_vec();
        js_free(ctx as *mut _, buf as *mut _);

        bytecode
    }

}

fn load_bytecode_std(
    ctx: *mut JSContext,
    bytecode: &[u8],
) {
    unsafe {
        js_std_eval_binary(
            ctx,
            bytecode.as_ptr(),
            bytecode.len(),
            0, // flags (ปกติ 0)
        );

        if js_is_exception(JS_GetException(ctx)) != 0 {
            panic!("js_std_eval_binary failed");
        }
    }
}


fn eval_module(ctx: *mut JSContext, source: &str, name: &str) {
    unsafe {
        let src = CString::new(source).unwrap();
        let fname = CString::new(name).unwrap();

        let val = JS_Eval(
            ctx,
            src.as_ptr(),
            source.len(),
            fname.as_ptr(),
            JS_EVAL_TYPE_MODULE as c_int,
        );

        if js_is_exception(val) != 0 {
            let exc = JS_GetException(ctx);
            let msg_ptr = js_to_cstring(ctx, exc);
            let msg = CStr::from_ptr(msg_ptr).to_string_lossy().to_string();

            JS_FreeCString(ctx, msg_ptr);
            JS_FreeValue(ctx, exc);
            JS_FreeValue(ctx, val);

            panic!("module eval failed: {}", msg);
        }


        JS_FreeValue(ctx, val);
    }
}



fn init_stdlib(ctx: *mut JSContext) {
    unsafe {
        js_std_add_helpers(ctx, 0, ptr::null_mut());
        js_init_module_std(ctx, b"std\0".as_ptr() as _);
        js_init_module_os(ctx, b"os\0".as_ptr() as _);
    }

}

fn main() {
    unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);

        init_stdlib(ctx);

        // --- fib module ---
        let source = r#"
        export function fib(n) {
            if (n <= 0) return 0;
            if (n === 1) return 1;
            return fib(n - 1) + fib(n - 2);
        }
        //console.log("fib_module loaded");
        "#;
        let bc = compile(ctx, source, "fib_module");
        load_bytecode_std(ctx, &bc);

        // --- main script ---
        let script_str = r#"
        import { fib } from "fib_module";
        let result = fib(10);
        console.log("fib(10) =", result);
        "#;

        eval_module(ctx, script_str, "<main>");

        JS_FreeContext(ctx);
        JS_FreeRuntime(rt);
    }
}
