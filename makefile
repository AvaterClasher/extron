build: 
	RUSTFLAGS="-C target-cpu=native" cargo build --release

.PHONY:
wasm:
	cargo build --release --target wasm32-unknown-emscripten -Z build-std=panic_abort,std
	cp target/wasm32-unknown-emscripten/release/extron.js wasm-bin/extron.js
	cp target/wasm32-unknown-emscripten/release/extron.wasm wasm-bin/extron.wasm