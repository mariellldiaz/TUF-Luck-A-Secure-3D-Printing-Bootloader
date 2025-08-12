#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include "monocypher.h"

#define FIRMWARE_PATH "firmware.bin"

void print_hex(const uint8_t *hash, size_t length) {
    for (size_t i = 0; i < length; ++i) {
        printf("%02x", hash[i]);
    }
    printf("\n");
}

int main() {
    FILE *file = fopen(FIRMWARE_PATH, "rb");
    if (!file) {
        perror("Error opening firmware file");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);

    if (size < 0) {
        perror("ftell failed");
        fclose(file);
        return 1;
    }

    uint8_t *firmware_data = malloc(size);
    if (!firmware_data) {
        fprintf(stderr, "Memory allocation failed\n");
        fclose(file);
        return 1;
    }

    fread(firmware_data, 1, size, file);
    fclose(file);

    uint8_t hash[64];
    crypto_blake2b_ctx ctx;
    crypto_blake2b_init(&ctx, 64);
    crypto_blake2b_update(&ctx, firmware_data, size);
    crypto_blake2b_final(&ctx, hash);

    free(firmware_data);

    printf("Firmware BLAKE2b: ");
    print_hex(hash, 64);

    return 0;
}

