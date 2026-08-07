#ifndef BoaContext_H
#define BoaContext_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "BoaContext.d.h"






BoaContext* BoaContext_new(void);

void BoaContext_eval(BoaContext* self, DiplomatStringView src, DiplomatWrite* write);

JsValue* BoaContext_eval_value(BoaContext* self, DiplomatStringView src);

bool JsValue_is_number(const JsValue* self);
double JsValue_as_number(const JsValue* self);
void JsValue_destroy(JsValue* self);

void BoaContext_destroy(BoaContext* self);





#endif // BoaContext_H
