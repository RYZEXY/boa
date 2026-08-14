#ifndef JsObject_H
#define JsObject_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "JsValue.d.h"

#include "JsObject.d.h"






JsValue* JsObject_as_value(const JsObject* self);

void JsObject_destroy(JsObject* self);





#endif // JsObject_H
