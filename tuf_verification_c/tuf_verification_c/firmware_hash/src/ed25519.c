#include "monocypher.h"

int crypto_check_ed25519(const uint8_t *signature,
                         const uint8_t *public_key,
                         const uint8_t *message,
                         size_t message_len) {
    return crypto_sign_check(signature, public_key, message, message_len);
}
