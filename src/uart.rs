use	core::fmt::{Write, Result};
use core::ptr::write_volatile;
use core::ptr::read;

const UART0: u32 = 0x0101F1000;

pub struct UART;

impl Write for UART {
	fn write_str(&mut self, value: &str) -> Result {
		 let address = UART0 as *mut u8;                                                                                                
                                                                                                                                                                                          
		for byte in value.as_bytes() {                                                                                                 
			unsafe {                                                                                                                     
				write_volatile(address, read(byte));                                                                                       
			}                                                                                                                            
		}

		Ok(())
	}
}

pub static mut UART_INSTANCE: UART = UART{};

