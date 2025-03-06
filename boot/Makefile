ROOT := $(shell git rev-parse --show-toplevel)
BIN_OUT := kernel8.img

clean:
	rm -rf target $(BIN_OUT)

release:
	cargo objcopy --release -- --strip-all -O binary $(BIN_OUT)

sd: release
	rm -rf /media/nick/4BC2-BA51/*
	cp $(ROOT)/ext/* /media/nick/4BC2-BA51/
	cp $(BIN_OUT) /media/nick/4BC2-BA51/
	umount -l /dev/sdd1