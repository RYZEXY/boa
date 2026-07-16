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

        DiplomatWrite* write = diplomat_buffer_write_create(0);
        BoaContext_eval(ctx, (DiplomatStringView){line, len}, write);
        printf("%s\n", diplomat_buffer_write_get_bytes(write));
        diplomat_buffer_write_destroy(write);

        printf("> ");
        fflush(stdout);
    }

    printf("\n");
    BoaContext_destroy(ctx);
    return 0;
}
