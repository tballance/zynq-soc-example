
#![no_std]
#![feature(panic_implementation, lang_items, asm)]


#[macro_use]
extern crate board;

include!(concat!(env!("BUILDINC_DIRECTORY"), "/generated/csr.rs"));


#[no_mangle]
pub extern "C" fn main() {

    println!("hi");
    unsafe{asm!("swi 2")};
    println!("hi2");

    unsafe {
        let counter_value = csr::test::counter_read();
        println!("{:x}", counter_value);
    }
    loop {};
}


use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic!");
    loop {}
}

#[no_mangle]
#[lang = "eh_personality"] pub extern fn eh_personality() {}


#[no_mangle]
pub fn reset_handler() {
    println!("reset!");
    loop {}

}

#[no_mangle]
pub fn exception_handler() {
    println!("exception!");
    loop {}
}

use core::ptr::read_volatile;
pub const GLOBAL_TIMER_BASE: *mut u32 = 0xF8F00200 as *mut u32;

fn read_timer() -> u64 {

    unsafe {
        let lower = read_volatile(GLOBAL_TIMER_BASE.offset(0));
        let upper = read_volatile(GLOBAL_TIMER_BASE.offset(1));
    
        ((upper as u64) << 32) | lower as u64
    }
}

#[no_mangle]
pub fn swi_handler() {

    println!("swi! {:x}", read_timer());
    println!("hey");
}

#[no_mangle]
pub fn irq_handler() {
    println!("irq!");
    loop {}
}

#[no_mangle]
pub fn fiq_handler() {
    println!("fiq!");

    loop {}
}
