//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logging;

use log::{info, trace, debug, warn,error};
use logging::init;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() {
    clear_bss();
    init();

    trace!("hello trace");
    debug!("hello debug");
    info!("hello info");
    warn!("hello warn");
    error!("hello {}","error");

    panic!("Shutdown machine!");
}
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| 
        unsafe { (a as *mut u8).write_volatile(0) }
    );
}
