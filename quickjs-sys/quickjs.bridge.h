#include "quickjs/quickjs.h"
#include "quickjs/quickjs-libc.h"

#if !defined(EMSCRIPTEN) && !defined(_MSC_VER)
#define CONFIG_ATOMICS
#endif

/* =========================
   JS_ATOM enum
   ========================= */
enum {
    __JS_ATOM_NULL = JS_ATOM_NULL,
#define DEF(name, str) JS_ATOM_##name,
#include "quickjs-atom.h"
#undef DEF
    JS_ATOM_END,
};

/* =========================
   Type checks
   ========================= */
int js_is_exception(JSValueConst v);
int js_is_null(JSValueConst v);
int js_is_undefined(JSValueConst v);
int js_is_bool(JSValueConst v);
int js_is_number(JSValueConst v);
int js_is_string(JSValueConst v);
int js_is_object(JSValueConst v);
int js_is_array(JSValueConst v);
int js_is_error(JSValueConst v);
int js_is_bigint(JSValueConst v);
int js_is_symbol(JSValueConst v);
int js_is_module(JSValueConst v);
int js_is_uninitialized(JSValueConst v);

/* =========================
   Value creation
   ========================= */
JSValue js_new_int32(JSContext *ctx, int32_t val);

/* =========================
   String conversion
   ========================= */
const char *js_to_cstring_len(
    JSContext *ctx,
    size_t *plen,
    JSValueConst val
);

const char *js_to_cstring(
    JSContext *ctx,
    JSValueConst val
);

void js_free_cstring(
    JSContext *ctx,
    const char *ptr
);


/* =========================
   Number conversion
   ========================= */
int js_to_uint32(
    JSContext *ctx,
    uint32_t *pres,
    JSValueConst val
);

/* =========================
   Function creation
   ========================= */
JSValue js_new_cfunction(
    JSContext *ctx,
    JSCFunction *func,
    const char *name,
    int length
);

JSValue js_new_cfunction_magic(
    JSContext *ctx,
    JSCFunctionMagic *func,
    const char *name,
    int length,
    JSCFunctionEnum cproto,
    int magic
);
