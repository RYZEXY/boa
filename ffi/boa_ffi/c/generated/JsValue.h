#ifndef JsValue_H
#define JsValue_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "JsObject.d.h"

#include "JsValue.d.h"






JsValue* JsValue_from_number(double n);

bool JsValue_is_number(const JsValue* self);

typedef struct JsValue_as_number_result {union {double ok; }; bool is_ok;} JsValue_as_number_result;
JsValue_as_number_result JsValue_as_number(const JsValue* self);

JsValue* JsValue_from_null(void);

bool JsValue_is_null(const JsValue* self);

JsValue* JsValue_from_undefined(void);

bool JsValue_is_undefined(const JsValue* self);

JsValue* JsValue_from_boolean(bool b);

bool JsValue_is_boolean(const JsValue* self);

typedef struct JsValue_as_boolean_result {union {bool ok; }; bool is_ok;} JsValue_as_boolean_result;
JsValue_as_boolean_result JsValue_as_boolean(const JsValue* self);

JsValue* JsValue_from_string(DiplomatStringView s);

bool JsValue_is_string(const JsValue* self);

void JsValue_as_string(const JsValue* self, DiplomatWrite* write);

bool JsValue_is_object(const JsValue* self);

JsObject* JsValue_as_object(const JsValue* self);

JsValue* JsValue_from_bigint_string(DiplomatStringView s);

bool JsValue_is_bigint(const JsValue* self);

void JsValue_as_bigint_string(const JsValue* self, DiplomatWrite* write);

JsValue* JsValue_from_symbol(DiplomatStringView description);

JsValue* JsValue_from_symbol_no_description(void);

bool JsValue_is_symbol(const JsValue* self);

bool JsValue_symbol_has_description(const JsValue* self);

void JsValue_as_symbol_description(const JsValue* self, DiplomatWrite* write);

void JsValue_destroy(JsValue* self);





#endif // JsValue_H
