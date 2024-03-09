.PHONY: all clean build-plugin


run-plugin: build-plugin
	cargo run -p plugin_example

build-plugin: 
	cargo build -p plugin_1 --release
	cargo build -p plugin_2 --release

clean:
	cargo clean
