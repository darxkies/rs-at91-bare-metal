compile:
	cargo build
	arm-none-eabi-objcopy target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal -O binary code.bin

size: compile
	arm-none-eabi-size -A target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 

disassemble: compile
	arm-none-eabi-objdump -D target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 
	
run: compile
	QEMU_AUDIO_DRV=none qemu-system-arm -M versatilepb -m 128M -nographic -kernel code.bin

debug: compile
	QEMU_AUDIO_DRV=none qemu-system-arm -M versatilepb -m 128M -nographic -gdb tcp::3333 -S -kernel target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 

gdb: 
	gdb-multiarch -tui -x debug.gdb -q target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 
