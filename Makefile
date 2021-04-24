run:
	cargo run

format:
	cargo fmt

clean:
	rm -f target/x86_64-barebones/release/bootimage-rust_os.bin

build:
	cargo bootimage --release
