#ifndef BoaContext_H
#define BoaContext_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "JsValue.d.h"

#include "BoaContext.d.h"






BoaContext* BoaContext_new(void);

void BoaContext_eval(BoaContext* self, DiplomatStringView src, DiplomatWrite* write);

JsValue* BoaContext_eval_value(BoaContext* self, DiplomatStringView src);

void BoaContext_set_global(BoaContext* self, DiplomatStringView name, const JsValue* value);

void BoaContext_destroy(BoaContext* self);





#endif // BoaContext_H
