use	core::fmt::{Write, Result};

pub const SYS_OPEN: usize = 0x01;
pub const SYS_CLOSE: usize = 0x02;
pub const SYS_WRITEC: usize = 0x03;
pub const SYS_WRITE0: usize = 0x04;
pub const SYS_WRITE: usize = 0x05;

extern "C" {
	fn semihosting_call(operation: usize, argument: usize) -> isize;
}

pub mod open_flags {
	pub const READ: usize = 0;
	pub const BINARY: usize = 1;
	pub const READ_WRITE: usize = 2;
	pub const TRUNCATE: usize = 4;
	pub const APPEND: usize = 8;
}

pub fn open(filename: &str, mode: usize) -> isize {
	let parameters = &[filename.as_ptr() as usize, mode, filename.len() as usize];

	unsafe {
		semihosting_call(SYS_OPEN, parameters.as_ptr() as usize)
	}
}

pub fn close(handle: usize) -> isize {
	unsafe {
		semihosting_call(SYS_CLOSE, handle)
	}
}

pub fn write_string(handle: usize, value: &str) -> isize{
	let parameters = &[handle, value.as_ptr() as usize, value.len() as usize];

	unsafe {
		semihosting_call(SYS_WRITE, parameters.as_ptr() as usize)
	}
}

pub struct SemihostingStdout {
	handle: Option<usize>
}

pub static mut SEMIHOSTING_STDOUT: SemihostingStdout = SemihostingStdout{handle: None};

impl Write for SemihostingStdout {
	fn write_str(&mut self, value: &str) -> Result {
		if self.handle.is_none() {
			let handle =open(":tt\0", open_flags::TRUNCATE); 

			if handle == -1 {
				panic!("Could not open semihosting stdout");
			}

			self.handle = Some(handle as usize);
		}

		write_string(self.handle.unwrap(), value);

		Ok(())
	}
}


