build: fmt
	cargo build

run:
	cargo run

flash:
	espflash /dev/ttyACM0 target

serial:
	espflash serial-monitor

serial-picocom:
	picocom --baud 115200 --imap lfcrlf --echo  /dev/ttyACM0

fmt:
	cargo fmt

doc:
	cargo doc --open

release: fmt
	cargo build --release

clean:
	cargo clean
