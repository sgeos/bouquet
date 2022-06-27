#![no_std]

extern crate libc;

#[no_mangle]
pub extern "C" fn run() {
  const MESSAGE: &'static str = "Hello, Bouquet!\n\0";
  unsafe {
    libc::printf(MESSAGE.as_ptr() as *const _);
  }
}

