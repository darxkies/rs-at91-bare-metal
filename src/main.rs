#![no_std]
#![no_main]

#[allow(unused_imports)]
use rlibc;

use core::{
    mem::zeroed,
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

const UART0: u32 = 0x0101F1000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		continue;
	}
}

fn uart_print(value: &str) {
	let address = UART0 as *mut u8;

	for byte in value.as_bytes() {
		unsafe {
            write_volatile(address, read(byte));
		}
	}
}

#[no_mangle]
pub fn low_init(_start: u32) -> ! {
    extern "C" {
        static mut __sbss: u32; // BSS start
        static mut __ebss: u32; // BSS end
    }

    // Initialize BSS
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    main()
}

fn main() -> ! {
	uart_print("\n********************************************************\n");
	uart_print("Hello over there!\n");
	uart_print("In QEMU, press Ctrl+a and x to exit\n");
	uart_print("********************************************************\n");

	loop {}
}
