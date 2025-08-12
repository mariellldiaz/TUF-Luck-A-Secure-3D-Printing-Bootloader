#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cJSON.h"

#define TARGETS_JSON "targets.json"

int main(void) {
    FILE *file = fopen(TARGETS_JSON, "rb");
    if (!file) {
        perror("Error opening targets.json");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    long length = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *data = malloc(length + 1);
    if (!data) {
        perror("Memory allocation failed");
        fclose(file);
        return 1;
    }

    fread(data, 1, length, file);
    data[length] = '\0';
    fclose(file);

    cJSON *root = cJSON_Parse(data);
    if (!root) {
        printf("Error before: [%s]\n", cJSON_GetErrorPtr());
        free(data);
        return 1;
    }

    cJSON *targets = cJSON_GetObjectItemCaseSensitive(root, "signed");
    targets = cJSON_GetObjectItemCaseSensitive(targets, "targets");
    if (!targets) {
        printf("No 'targets' object found\n");
        cJSON_Delete(root);
        free(data);
        return 1;
    }

    cJSON *target = NULL;
    cJSON_ArrayForEach(target, targets) {
        const char *filename = target->string;
        cJSON *target_obj = cJSON_GetObjectItemCaseSensitive(target, "hashes");
        cJSON *sha256_obj = cJSON_GetObjectItemCaseSensitive(target_obj, "sha256");
        cJSON *length_obj = cJSON_GetObjectItemCaseSensitive(target, "length");

        if (filename && cJSON_IsString(sha256_obj) && cJSON_IsNumber(length_obj)) {
            printf("Target: %s\n", filename);
            printf("  SHA-256: %s\n", sha256_obj->valuestring);
            printf("  Length: %d bytes\n\n", length_obj->valueint);
        }
    }

    cJSON_Delete(root);
    free(data);

    return 0;
}
