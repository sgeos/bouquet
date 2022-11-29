#![no_std]
#![no_main]
#![feature(lang_items)]

#![feature(default_alloc_error_handler)]
#[global_allocator]
static ALLOCATOR: ::libc_alloc::LibcAlloc = ::libc_alloc::LibcAlloc;

use lib::run;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
  run();
  0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

