QEMU=QEMU_AUDIO_DRV=none qemu-system-arm -M versatilepb -m 128M -nographic -semihosting-config enable=on,target=auto 

compile:
	cargo build --features semihosting
	arm-none-eabi-objcopy target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal -O binary code.bin

release:
	cargo build --release 
	arm-none-eabi-strip --strip-all target/armv5te-unknown-linux-gnueabi/release/rs-at91-bare-metal 
	arm-none-eabi-objcopy target/armv5te-unknown-linux-gnueabi/release/rs-at91-bare-metal -O binary code.bin

size: compile
	arm-none-eabi-size -A target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 

disassemble: compile
	arm-none-eabi-objdump -D target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 
	
disassemble-release: compile
	arm-none-eabi-objdump -D target/armv5te-unknown-linux-gnueabi/release/rs-at91-bare-metal 
	
run: release
	$(QEMU) -kernel target/armv5te-unknown-linux-gnueabi/release/rs-at91-bare-metal 

debug: compile
	$(QEMU) -gdb tcp::3333 -S -kernel target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 

gdb: 
	gdb-multiarch -tui -x debug.gdb -q target/armv5te-unknown-linux-gnueabi/debug/rs-at91-bare-metal 

watch-and-compile:
	watchexec -e rs,S,ld -r make compile 
