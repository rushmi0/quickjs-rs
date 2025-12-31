use quickjs_sys::*;
use std::ffi::{c_int, CString};
use std::ptr;
use quickjs_rs::Runtime;

fn compile(
    ctx: *mut JSContext,
    source: &str,
    filename: &str,
) -> Vec<u8> {

    unsafe {
        let src = CString::new(source).unwrap();
        let fname = CString::new(filename).unwrap();

        let flags = JS_EVAL_TYPE_MODULE | JS_EVAL_FLAG_COMPILE_ONLY;

        let val = JS_Eval(
            ctx,
            src.as_ptr(),
            source.len(),
            fname.as_ptr(),
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

fn load_bytecode(
    ctx: *mut JSContext,
    bytecode: &[u8],
) {
    unsafe {
        let obj = JS_ReadObject(
            ctx,
            bytecode.as_ptr(),
            bytecode.len(),
            JS_READ_OBJ_BYTECODE as c_int,
        );

        if js_is_exception(obj) != 0 {
            panic!("JS_ReadObject failed");
        }

        let ret = JS_EvalFunction(ctx, obj);

        if js_is_exception(ret) != 0 {
            JS_FreeValue(ctx, ret);
            panic!("execution error");
        }

        JS_FreeValue(ctx, ret);
    }
}

fn call_fib(ctx: *mut JSContext, n: i32) -> i32 {
    unsafe {
        let code = format!(
            r#"
            import {{ fib }} from "fib_module";
            globalThis.__result = fib({});
            "#,
            n
        );
        println!("call_fib result: {}", &code);

        let code_len = code.len();
        let src = CString::new(code).unwrap();
        let fname = CString::new("<call>").unwrap();

        let val = JS_Eval(
            ctx,
            src.as_ptr(),
            code_len,
            fname.as_ptr(),
            JS_EVAL_TYPE_MODULE as c_int,
        );

        if js_is_exception(val) != 0 {
            JS_FreeValue(ctx, val);
            panic!("call fib failed");
        }
        JS_FreeValue(ctx, val);

        let global = JS_GetGlobalObject(ctx);
        let key = CString::new("__result").unwrap();
        let res = JS_GetPropertyStr(ctx, global, key.as_ptr());

        let mut out: i32 = 0;
        JS_ToInt32(ctx, &mut out, res);

        JS_FreeValue(ctx, res);
        JS_FreeValue(ctx, global);

        out
    }
}



fn init_stdlib(ctx: *mut JSContext) {
    unsafe {
        // print(), console.log(), scriptArgs, argv0
        js_std_add_helpers(ctx, 0, ptr::null_mut());

        // import * as std from "std"
        js_init_module_std(ctx, b"std\0".as_ptr() as _);

        // import * as os from "os"
        js_init_module_os(ctx, b"os\0".as_ptr() as _);

        js_init_module_bjson(ctx, b"bjson\0".as_ptr() as _);
    }

}



fn main() {
    let rt = Runtime::new();
    let ctx = rt.context();

    ctx.init_std();
    ctx.set_i32("x", 41).expect("TODO: panic message");

    let v = ctx.eval("x + 1").unwrap().as_i32().unwrap();
    println!("value: {}", v);


    /*unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);
        init_stdlib(ctx);

        let source = r#"
            export function fib(n) {
                print("fib called with", n);
                if (n <= 0) return 0;
                if (n === 1) return 1;
                return fib(n - 1) + fib(n - 2);
            }
            console.log("module loaded");

        "#;

        let bc = compile(ctx, source, "fib_module");
        load_bytecode(ctx, &bc);

        let result = call_fib(ctx, 10);
        println!("fib(10) = {}", result);

        JS_FreeContext(ctx);
        JS_FreeRuntime(rt);
    }*/
}

