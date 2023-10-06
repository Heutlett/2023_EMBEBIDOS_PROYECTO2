default:
	rm -rf target/
	cargo rustc -- -C link-arg=--script=./linker.ld
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/my-rusty-pi ./kernel7.img
	cp kernel7.img ../mnt/
copy_files:
	sudo cp ../mnt/bootcode.bin ../mnt/config.txt ../mnt/fixup.dat ../mnt/start.elf ../mnt/kernel7.img /media/heutlett/boot