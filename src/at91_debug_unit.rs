use	core::fmt::Write;
use	core::fmt::Result;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use core::ptr::read;

const MEMORY_BASE: usize = 0xFFFFF200;

pub mod registers {
	pub const CONTROL: usize = 0x00;
	pub const MODE: usize = 0x04;
	pub const INTERRUPT_DISABLE: usize = 0x0C;
	pub const STATUS: usize = 0x14;
	pub const RECEIVER_HOLDING: usize = 0x18;
	pub const TRANSMITTER_HOLDING: usize = 0x1C;
	pub const BAUD_RATE_GENERATOR: usize = 0x20; 
}

pub mod control {
	pub const RESET_RECEIVER: usize = 0x4;
	pub const RESET_TRANSMITTER: usize = 0x8;
	pub const RECEIVER_ENABLE: usize = 0x10;
	pub const RECEIVER_DISABLE: usize = 0x20;
	pub const TRANSMITTER_ENABLE: usize = 0x40;
	pub const TRANSMITTER_DISABLE: usize = 0x80;
}

pub mod mode {
	pub mod character_length {
		pub const FIVE: usize = 0x0000;
		pub const SIX: usize = 0x0040;
		pub const SEVEN: usize = 0x0080;
		pub const EIGHT: usize = 0x00C0;
	}

	pub mod parity {
		pub const EVEN: usize = 0x0000;
		pub const ODD: usize = 0x0200;
		pub const SPACE: usize = 0x0400;
		pub const MARK: usize = 0x0600;
		pub const NONE: usize = 0x0800;
	}

	pub mod stop_bits {
		pub const ONE: usize = 0x0000;
		pub const ONE_POINT_TWO: usize = 0x1000;
		pub const TWO: usize = 0x2000;
	}

	pub mod channel_mode {
		pub const NORMAL: usize = 0x0000;
		pub const AUTOMATIC_ECHO: usize = 0x4000;
		pub const LOCAL_LOOPBACK: usize = 0x8000;
		pub const REMOTE_LOOPBACK: usize = 0xC000;
	}
}

pub mod status {
	pub const RECEIVER_READY: usize = 0x01;
	pub const TRANSMITTER_READY: usize = 0x2;
}

pub struct AT91DebugUnit;

impl AT91DebugUnit {
	pub fn initialize(&mut self, baudrate: usize) {
		self.write(registers::INTERRUPT_DISABLE, 0xFFFFFFFF);

		self.write(registers::CONTROL, control::RESET_RECEIVER | control::RESET_TRANSMITTER | control::RECEIVER_DISABLE | control::TRANSMITTER_DISABLE);

		self.write(registers::BAUD_RATE_GENERATOR, baudrate);

		self.write(registers::MODE, mode::parity::NONE | mode::channel_mode::NORMAL | mode::character_length::EIGHT | mode::stop_bits::ONE);

		self.write(registers::CONTROL, control::RECEIVER_ENABLE | control::TRANSMITTER_ENABLE);
	}

	fn write(&mut self, offset: usize, value: usize) {
		let address = MEMORY_BASE + offset;

		unsafe {                                                                                                                     
			write_volatile(address as *mut usize, read(&value));                                                                                       
		}                                                                                                                            
	}

	fn read(&mut self, offset: usize) -> usize {
		let address = MEMORY_BASE + offset;
		unsafe {
			read_volatile(address as *mut usize)
		}
	}

	fn write_char(&mut self, value: u8) {
		loop {
			if (self.read(registers::STATUS) & status::TRANSMITTER_READY) != 0 {
				break
			}
		}

		self.write(registers::TRANSMITTER_HOLDING, value as usize);
	}
}

impl Write for AT91DebugUnit {
	fn write_str(&mut self, value: &str) -> Result {
		for byte in value.as_bytes() {                                                                                                 
			self.write_char(*byte);
		}

		Ok(())
	}
}

pub static mut AT91_DEBUG_UNIT: AT91DebugUnit = AT91DebugUnit{};
