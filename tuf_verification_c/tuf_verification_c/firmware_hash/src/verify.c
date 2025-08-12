#include "verify.h"
#include "monocypher.h"

int verify_signature(const uint8_t *signature,
                     const uint8_t *public_key,
                     const uint8_t *message,
                     size_t message_len) {
    return crypto_eddsa_check(signature, public_key, message, message_len);
}

int verify_sha256(const char *file_path, const uint8_t *expected_hash) {
    FILE *fp = fopen(file_path, "rb");
    if (!fp) {
        perror("fopen");
        return 0;
    }

    crypto_sha256_ctx ctx;
    crypto_sha256_init(&ctx);

    uint8_t buffer[1024];
    size_t read;
    while ((read = fread(buffer, 1, sizeof(buffer), fp)) > 0) {
        crypto_sha256_update(&ctx, buffer, read);
    }

    fclose(fp);

    uint8_t actual_hash[32];
    crypto_sha256_final(&ctx, actual_hash);

    return memcmp(actual_hash, expected_hash, 32) == 0;
}

