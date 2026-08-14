#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#define static_assert _Static_assert
#include "../generated/BoaContext.h"
#include "../generated/JsObject.h"
#include "../generated/JsValue.h"

static DiplomatStringView sv(const char* s) {
    DiplomatStringView view = {s, strlen(s)};
    return view;
}

int main(void) {

    BoaContext* ctx = BoaContext_new();

    //Create real typed FFI values on the native side without JS source text
    JsValue* player_name = JsValue_from_string(sv("Alex"));
    JsValue* player_health = JsValue_from_number(100);
    JsValue* is_alive = JsValue_from_boolean(true);

    //and bind them into the engine's global scope
    BoaContext_set_global(ctx, sv("playerName"), player_name);
    BoaContext_set_global(ctx, sv("playerHealth"), player_health);
    BoaContext_set_global(ctx, sv("isAlive"), is_alive);

    printf("Host created typed JS values:\n");
    printf("  playerName   = \"Alex\"\n");
    printf("  playerHealth = 100\n");
    printf("  isAlive      = true\n\n");

    //Construct Javascript object using bound values
    JsValue* player = BoaContext_eval_value(
        ctx, sv("({ name: playerName, health: playerHealth - 25, alive: isAlive })"));

    //Prints Object if JsValue is actually an Object
    printf("Host received: JsValue(%s)\n\n", JsValue_is_object(player) ? "Object" : "?");

    //JsValue -> JsObject -> JsValue, to prove identity survives
    JsObject* player_obj = JsValue_as_object(player);
    JsValue* round_tripped = JsObject_as_value(player_obj);

    //Object identity:
    //const a = { x: 1 };
    //const b = { x: 1 };
    //a === b is FALSE
    //below test demonstrates that
    //const a = { x: 1 };
    //const b = a;
    //a === b is TRUE, proving we have object identity

    BoaContext_set_global(ctx, sv("original"), player);
    BoaContext_set_global(ctx, sv("roundTripped"), round_tripped);
    JsValue* identity = BoaContext_eval_value(ctx, sv("original === roundTripped"));
    //identity should be true
    JsValue_as_boolean_result identity_result = JsValue_as_boolean(identity);

    printf("Object identity preserved:\n");
    printf("  original === roundTripped: %s\n\n", identity_result.ok ? "true" : "false");

    /* One BigInt demonstration: arbitrary-precision values cross the
     * boundary without narrowing to i64/u64/double. */
    const char* huge = "123456789012345678901234567890";
    JsValue* big_value = JsValue_from_bigint_string(sv(huge));
    BoaContext_set_global(ctx, sv("bigValue"), big_value);
    JsValue* big_result = BoaContext_eval_value(ctx, sv("bigValue + 1n"));
    DiplomatWrite* big_write = diplomat_buffer_write_create(0);
    JsValue_as_bigint_string(big_result, big_write);

    printf("BigInt round trip:\n");
    printf("  %s\n", huge);
    printf("  + 1n\n");
    printf("  =\n");
    printf("  %s\n\n", diplomat_buffer_write_get_bytes(big_write));
    diplomat_buffer_write_destroy(big_write);

    bool success = identity_result.is_ok && identity_result.ok;
    printf("%s: Native code is holding real Boa JavaScript values and objects.\n",
           success ? "SUCCESS" : "FAILURE");

    /* Every handle obtained above gets destroyed explicitly. C has no
     * automatic memory management, unlike the generated JS/C++ bindings. */
    JsValue_destroy(player_name);
    JsValue_destroy(player_health);
    JsValue_destroy(is_alive);
    JsValue_destroy(player);
    JsObject_destroy(player_obj);
    JsValue_destroy(round_tripped);
    JsValue_destroy(identity);
    JsValue_destroy(big_value);
    JsValue_destroy(big_result);
    BoaContext_destroy(ctx);

    return success ? 0 : 1;
}
