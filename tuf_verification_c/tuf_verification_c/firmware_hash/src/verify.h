#ifndef VERIFY_H
#define VERIFY_H

#include <stdint.h>
#include <stddef.h>

int verify_signature(const uint8_t *signature,
                     const uint8_t *public_key,
                     const uint8_t *message,
                     size_t message_len);
int verify_sha256(const char *file_path, const uint8_t *expected_hash);

#endif
