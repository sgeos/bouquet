#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]

#[global_allocator]
static ALLOCATOR: ::libc_alloc::LibcAlloc = ::libc_alloc::LibcAlloc;

use {core::ffi::c_char, lib_client_pc::run, libc::c_int};

#[no_mangle]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    run();
    0
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
