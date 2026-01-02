#include <stdio.h>
#include <string.h>
#include "quickjs.bridge.h"

/* =========================
   Type checks
   ========================= */
int js_is_exception(JSValueConst v) {
    return JS_IsException(v);
}

int js_is_null(JSValueConst v) {
    return JS_IsNull(v);
}

int js_is_undefined(JSValueConst v) {
    return JS_IsUndefined(v);
}

int js_is_bool(JSValueConst v) {
    return JS_IsBool(v);
}

int js_is_number(JSValueConst v) {
    return JS_IsNumber(v);
}

int js_is_string(JSValueConst v) {
    return JS_IsString(v);
}

int js_is_object(JSValueConst v) {
    return JS_IsObject(v);
}

int js_is_array(JSValueConst v) {
    return JS_IsArray(v);
}

int js_is_error(JSValueConst v) {
    return JS_IsError(v);
}

int js_is_bigint(JSValueConst v) {
    return JS_IsBigInt(v);
}

int js_is_symbol(JSValueConst v) {
    return JS_IsSymbol(v);
}

int js_is_module(JSValueConst v) {
    return JS_IsModule(v);
}

int js_is_uninitialized(JSValueConst v) {
    return JS_IsUninitialized(v);
}

/* =========================
   Value creation
   ========================= */
JSValue js_new_int32(JSContext *ctx, int32_t val) {
    return JS_NewInt32(ctx, val);
}

/* =========================
   String conversion
   ========================= */
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

void js_free_cstring(
    JSContext *ctx,
    const char *ptr
) {
    JS_FreeCString(ctx, ptr);
}


/* =========================
   Number conversion
   ========================= */
int js_to_uint32(
    JSContext *ctx,
    uint32_t *pres,
    JSValueConst val
) {
    return JS_ToUint32(ctx, pres, val);
}

/* =========================
   Function creation
   ========================= */
JSValue js_new_cfunction(
    JSContext *ctx,
    JSCFunction *func,
    const char *name,
    int length
) {
    return JS_NewCFunction(ctx, func, name, length);
}

JSValue js_new_cfunction_magic(
    JSContext *ctx,
    JSCFunctionMagic *func,
    const char *name,
    int length,
    JSCFunctionEnum cproto,
    int magic
) {
    return JS_NewCFunctionMagic(ctx, func, name, length, cproto, magic);
}
