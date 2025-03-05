clean:
	rm -rf target kernel8.img

release:
	cargo build --release 
	cargo strip --release -- --strip-all 
	cargo objcopy --release -- -O binary kernel8.img

sd:
	cp kernel8.img /media/nick/4BC2-BA51/
	umount -l /dev/sdd1