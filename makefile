.PHONY: all clean build-plugin


run-plugin: build-plugin
	cargo run -p example

build-plugin: 
	cargo build -p my_plugin

clean:
	cargo clean
