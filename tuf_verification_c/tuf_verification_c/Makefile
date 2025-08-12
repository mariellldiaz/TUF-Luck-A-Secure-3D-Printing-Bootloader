CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -I./src -I./monocypher

SRC_DIR = src
BUILD_DIR = build

OBJS = $(BUILD_DIR)/main.o $(BUILD_DIR)/verify.o $(BUILD_DIR)/monocypher.o

all: firmware-verify

firmware-verify: $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o firmware-verify

$(BUILD_DIR)/main.o: $(SRC_DIR)/main.c
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/verify.o: $(SRC_DIR)/verify.c
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/monocypher.o: monocypher/monocypher.c
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR) firmware-verify
build/parson.o: parson/parson.c
	$(CC) $(CFLAGS) -c $< -o $@

firmware-verify: build/main.o build/verify.o build/monocypher.o build/parson.o
	$(CC) $(CFLAGS) $^ -o $@

