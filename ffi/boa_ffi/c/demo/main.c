#include <stdio.h>
#include <string.h>
#define static_assert _Static_assert
#include "../generated/BoaContext.h"

int main(void) {
    BoaContext* ctx = BoaContext_new();
    char line[4096];

    printf("Enter a JavaScript Expression\n");
    printf("> ");
    fflush(stdout);

    while (fgets(line, sizeof(line), stdin) != NULL) {
        size_t len = strlen(line);
        if (len > 0 && line[len - 1] == '\n') {
            line[--len] = '\0';
        }

        JsValue* value = BoaContext_eval_value(ctx, (DiplomatStringView){line, len});
        double number = JsValue_as_number(value);
        if (number == 0.0 && !JsValue_is_number(value)) {
            printf("<non-numeric value>\n");
        } else {
            printf("%.15g\n", number);
        }
        JsValue_destroy(value);

        printf("> ");
        fflush(stdout);
    }

    printf("\n");
    BoaContext_destroy(ctx);
    return 0;
}
