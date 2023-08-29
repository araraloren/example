#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate alloc;

#[link(name = "vcruntime")]
extern { }

#[link(name = "ucrt")]
extern { }

use alloc::boxed::Box;

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
pub fn mainCRTStartup () -> isize {
    let _: Box<u8> = Box::new(0u8);
    0
}

pub struct Allocator;

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        panic!("----------------")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}