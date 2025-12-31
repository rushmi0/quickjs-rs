#include "quickjs/quickjs.h"
#include "quickjs/quickjs-libc.h"

int js_is_exception(JSValue v);
int js_is_null(JSValue v);
int js_is_undefined(JSValue v);
int js_is_bool(JSValue v);
int js_is_number(JSValue v);
int js_is_string(JSValue v);
int js_is_object(JSValue v);
int js_is_array(JSValue v);
int js_is_error(JSValue v);

JSValue js_new_int32(JSContext *ctx, int32_t val);


const char *js_to_cstring_len(
    JSContext *ctx,
    size_t *plen,
    JSValueConst val
);

const char *js_to_cstring(
    JSContext *ctx,
    JSValueConst val
);
