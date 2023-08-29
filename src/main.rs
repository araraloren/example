#![allow(internal_features)]
#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate alloc;

#[link(name = "vcruntime")]
extern { }

#[link(name = "ucrt")]
extern { }

use alloc::{boxed::Box, ffi::CString};

#[no_mangle]
static _fltused: i32 = 0;

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo<'_>) -> ! {
    loop { }
}

#[lang = "eh_personality"]
pub fn personality() { }

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup () -> isize {
    let _: Box<u8> = Box::new(0u8);
    let msg = CString::new("Hello from mainCRTStartup").unwrap();
    unsafe {
        libc::printf(msg.as_c_str().as_ptr());
    }
    0
}

pub struct Allocator;

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        libc::free(ptr.cast())
    }
}
