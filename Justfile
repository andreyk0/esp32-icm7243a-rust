build ARG: fmt
	cargo build {{ARG}}

run:
	cargo run

flash:
	espflash /dev/ttyACM0 target/xtensa-esp32s2-espidf/release/esp32-icm7243a

serial:
	picocom --baud 115200 --imap lfcrlf --echo  /dev/ttyACM0

fmt:
	cargo fmt

doc:
	cargo doc --open

release: fmt
	cargo build --release

clean:
	cargo clean
