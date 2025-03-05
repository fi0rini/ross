BIN_OUT := boot

clean:
	rm -rf target $(BIN_OUT)

release:
	cargo objcopy --release -- --strip-all -O binary $(BIN_OUT)

sd:
	cp $(BIN_OUT) /media/nick/4BC2-BA51/
	umount -l /dev/sdd1