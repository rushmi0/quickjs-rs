#include <stdio.h>
#include <string.h>
#include "quickjs.bridge.h"

int js_is_exception(JSValue v) {
    return JS_IsException(v);
}

int js_is_null(JSValue v) {
    return JS_IsNull(v);
}

int js_is_undefined(JSValue v) {
    return JS_IsUndefined(v);
}

int js_is_bool(JSValue v) {
    return JS_IsBool(v);
}

int js_is_number(JSValue v) {
    return JS_IsNumber(v);
}

int js_is_string(JSValue v) {
    return JS_IsString(v);
}

int js_is_object(JSValue v) {
    return JS_IsObject(v);
}

int js_is_array(JSValue v) {
    return JS_IsArray(v);
}

int js_is_error(JSValue v) {
    return JS_IsError(v);
}

JSValue js_new_int32(JSContext *ctx, int32_t val) {
    return JS_NewInt32(ctx, val);
}



const char *js_to_cstring_len(
    JSContext *ctx,
    size_t *plen,
    JSValueConst val
) {
    return JS_ToCStringLen2(ctx, plen, val, 0);
}

const char *js_to_cstring(
    JSContext *ctx,
    JSValueConst val
) {
    return JS_ToCStringLen2(ctx, NULL, val, 0);
}
