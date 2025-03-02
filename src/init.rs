use core::mem::zeroed;
use core::ptr::write_volatile;

mod panic;

use crate::kmain;

use core::arch::global_asm;

global_asm!(include_str!("init/init.s"));

fn zeros_bss() {
    unsafe extern "C" {
        static mut __bss_beg: u64;
        static mut __bss_end: u64;
    }

    let mut iter: *mut u64 = &raw mut __bss_beg;
    let end: *mut u64 = &raw mut __bss_end;

    while iter < end {
        unsafe { 
            write_volatile(iter, zeroed());
            iter = iter.add(1);
        };
    }
}

#[unsafe(no_mangle)]
unsafe fn kinit() -> ! {
    zeros_bss();
    kmain();
}