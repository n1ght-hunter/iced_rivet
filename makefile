.PHONY: all clean build-plugin

# Recursive wildcard function
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

SRCDIR = examples/plugin/my_plugin/src
TARGET = target/release/my_plugin.dll
SRC = $(call rwildcard,$(SRCDIR),*.rs)

run: build-plugin run-plugin

run-plugin:
	cargo run -p example

$(TARGET): $(SRC)
	cargo build -p my_plugin

build-plugin: $(TARGET)
