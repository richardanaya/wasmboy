build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/hellogame.wasm .
lint:
	@cargo fmt