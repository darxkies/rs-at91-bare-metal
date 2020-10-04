#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]

use rlibc;

pub mod semihosting;
pub mod uart;

use semihosting::*;
use uart::*;

use core::mem::zeroed;
use core::panic::PanicInfo;
use core::ptr::write_volatile;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		continue;
	}
}

macro_rules! printk {
    ($($arg:tt)*) => { 
		unsafe {
			#[cfg(feature = "semihosting")]
			write!(SEMIHOSTING_STDOUT, $($arg)*).expect("failed");

			#[cfg(not(feature = "semihosting"))]
			write!(UART_INSTANCE, $($arg)*).expect("failed");
		}
	};
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

	printk!("\nPC: {:#X}\n", _start);

    main()
}

fn main() -> ! {
	printk!("\n********************************************************\n");
	printk!("Hello over there!\n");
	printk!("In QEMU, press Ctrl+a and x to exit\n");
	printk!("********************************************************\n");

	loop {}
}
