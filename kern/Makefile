BIN_OUT := kernel8.img

clean:
	rm -rf target $(BIN_OUT)

release:
	cargo objcopy --release -- --strip-all -O binary $(BIN_OUT)

transmit: release
	ttywrite -i $(BIN_OUT) /dev/ttyUSB0