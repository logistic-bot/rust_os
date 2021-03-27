run: target/x86_64-barebones/release/bootimage-rust_os.bin
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-barebones/release/bootimage-rust_os.bin

clean:
	rm -f target/x86_64-barebones/release/bootimage-rust_os.bin

target/x86_64-barebones/release/bootimage-rust_os.bin: src/**
	cargo bootimage --release
