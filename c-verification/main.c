#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "cJSON/cJSON.h"
#include "monocypher.h"

unsigned char *read_file(const char *filename, size_t *size) {
    FILE *f = fopen(filename, "rb");
    if (!f) {
        perror("fopen");
        return NULL;
    }
    fseek(f, 0, SEEK_END);
    *size = ftell(f);
    fseek(f, 0, SEEK_SET);
    unsigned char *buf = malloc(*size);
    if (!buf) {
        perror("malloc");
        fclose(f);
        return NULL;
    }
    fread(buf, 1, *size, f);
    fclose(f);
    return buf;
}

int hex_decode(const char *hex, uint8_t *out, size_t out_len) {
    size_t hex_len = strlen(hex);
    if (hex_len != out_len * 2) return -1;
    for (size_t i = 0; i < out_len; i++) {
        unsigned int byte;
        if (sscanf(&hex[i * 2], "%2x", &byte) != 1) return -1;
        out[i] = (uint8_t)byte;
    }
    return 0;
}

int main(void) {
    
    size_t json_size;
    unsigned char *json_buf = read_file("targets.json", &json_size);
    if (!json_buf) return 1;

    cJSON *root = cJSON_Parse((const char *)json_buf);
    free(json_buf);

    if (!root) {
        printf("Error parsing JSON metadata\n");
        return 1;
    }

    cJSON *signed_obj = cJSON_GetObjectItem(root, "signed");
    cJSON *targets = cJSON_GetObjectItem(signed_obj, "targets");
    cJSON *firmware = cJSON_GetObjectItem(targets, "firmware.bin");
    cJSON *hashes = cJSON_GetObjectItem(firmware, "hashes");
    cJSON *sha256_item = cJSON_GetObjectItem(hashes, "sha256");

    if (!cJSON_IsString(sha256_item)) {
        printf("SHA-256 not found in JSON\n");
        cJSON_Delete(root);
        return 1;
    }

    const char *expected_sha256_hex = sha256_item->valuestring;
    printf("[Metadata] Expected SHA-256: %s\n", expected_sha256_hex);

    size_t fw_size;
    unsigned char *fw_buf = read_file("firmware.bin", &fw_size);
    if (!fw_buf) {
        cJSON_Delete(root);
        return 1;
    }

    uint8_t fw_hash[32];
    crypto_blake2b_ctx ctx;
    crypto_blake2b_general_init(&ctx, fw_hash, sizeof(fw_hash), NULL, 0); 
    crypto_blake2b_update(&ctx, fw_buf, fw_size);
    crypto_blake2b_final(&ctx);

    uint8_t expected_hash[32];
    if (hex_decode(expected_sha256_hex, expected_hash, sizeof(expected_hash)) != 0) {
        printf("Error decoding expected SHA-256\n");
        free(fw_buf);
        cJSON_Delete(root);
        return 1;
    }

    if (memcmp(fw_hash, expected_hash, 32) == 0) {
        printf("[Hash Check] Firmware hash matches metadata\n");
    } else {
        printf("[Hash Check] Firmware hash DOES NOT match metadata\n");
    }

    cJSON *sig_item = cJSON_GetObjectItem(firmware, "sig");
    cJSON *pubkey_item = cJSON_GetObjectItem(firmware, "pubkey");

    if (!cJSON_IsString(sig_item) || !cJSON_IsString(pubkey_item)) {
        printf("Signature or public key not found in metadata\n");
        free(fw_buf);
        cJSON_Delete(root);
        return 1;
    }

    uint8_t signature[64], pubkey[32];
    if (hex_decode(sig_item->valuestring, signature, sizeof(signature)) != 0 ||
        hex_decode(pubkey_item->valuestring, pubkey, sizeof(pubkey)) != 0) {
        printf("Error decoding signature/public key\n");
        free(fw_buf);
        cJSON_Delete(root);
        return 1;
    }

    if (crypto_sign_check(signature, pubkey, fw_buf, fw_size) == 0) {
        printf("[Signature Check] Signature is VALID\n");
    } else {
        printf("[Signature Check] Signature is INVALID\n");
    }

    free(fw_buf);
    cJSON_Delete(root);
    return 0;
}
