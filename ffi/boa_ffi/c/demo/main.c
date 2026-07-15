#include <stdio.h>
#define static_assert _Static_assert
#include "../generated/free_functions.h"

int32_t boa_ffi_demo_add(int32_t a, int32_t b);

int main(void) {
    int32_t result = boa_ffi_demo_sub(5, 3);

    printf("C program called Rust and got: %d\n", result);
    return 0;
}
